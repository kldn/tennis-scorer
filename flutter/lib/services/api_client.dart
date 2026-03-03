import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart' as http;

class TokenPair {
  final String accessToken;
  final String refreshToken;

  TokenPair({required this.accessToken, required this.refreshToken});
}

class ApiClient {
  final String baseUrl;
  final http.Client _httpClient;
  final FlutterSecureStorage _storage;

  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';
  static const _requestTimeout = Duration(seconds: 15);

  String? _cachedAccessToken;
  Future<bool>? _refreshFuture;

  ApiClient({
    required this.baseUrl,
    http.Client? httpClient,
    FlutterSecureStorage? storage,
  })  : _httpClient = httpClient ?? http.Client(),
        _storage = storage ?? const FlutterSecureStorage();

  void close() {
    _httpClient.close();
  }

  // MARK: - Auth

  Future<TokenPair> loginWithApple(String identityToken) async {
    final response = await _post(
      '/auth/apple',
      body: {'identity_token': identityToken},
      authenticated: false,
    );
    final json = jsonDecode(response.body) as Map<String, dynamic>;
    return TokenPair(
      accessToken: json['access_token'] as String,
      refreshToken: json['refresh_token'] as String,
    );
  }

  Future<TokenPair> refreshToken(String refreshToken) async {
    final response = await _post(
      '/auth/refresh',
      body: {'refresh_token': refreshToken},
      authenticated: false,
    );
    final json = jsonDecode(response.body) as Map<String, dynamic>;
    final newRefreshToken = json['refresh_token'] as String?;
    return TokenPair(
      accessToken: json['access_token'] as String,
      refreshToken: newRefreshToken ?? refreshToken,
    );
  }

  // MARK: - Matches

  Future<Map<String, dynamic>> getMatches({int limit = 20, int offset = 0}) async {
    final response = await _get('/matches?limit=$limit&offset=$offset');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> getMatch(String id) async {
    final response = await _get('/matches/$id');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  // MARK: - Stats

  Future<Map<String, dynamic>> getMatchAnalysis(String matchId) async {
    final response = await _get('/stats/match/$matchId/analysis');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> getMatchMomentum(String matchId) async {
    final response = await _get('/stats/match/$matchId/momentum');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> getMatchPace(String matchId) async {
    final response = await _get('/stats/match/$matchId/pace');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> getStatsSummary() async {
    final response = await _get('/stats/summary');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  // MARK: - Notifications

  Future<Map<String, dynamic>> getNotificationSettings() async {
    final response = await _get('/notifications/settings');
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<void> putNotificationSettings(Map<String, dynamic> settings) async {
    await _put('/notifications/settings', body: settings);
  }

  // MARK: - Token Cache

  Future<String?> _getAccessToken() async {
    return _cachedAccessToken ??= await _storage.read(key: _accessTokenKey);
  }

  void clearTokenCache() {
    _cachedAccessToken = null;
  }

  // MARK: - Internal

  Future<http.Response> _get(String path) async {
    return _authenticatedRequest(() async {
      final token = await _getAccessToken();
      return _httpClient
          .get(Uri.parse('$baseUrl$path'), headers: _headers(token))
          .timeout(_requestTimeout);
    });
  }

  Future<http.Response> _post(
    String path, {
    required Map<String, dynamic> body,
    bool authenticated = true,
  }) async {
    if (!authenticated) {
      final response = await _httpClient
          .post(
            Uri.parse('$baseUrl$path'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(body),
          )
          .timeout(_requestTimeout);
      _checkResponse(response);
      return response;
    }

    return _authenticatedRequest(() async {
      final token = await _getAccessToken();
      return _httpClient
          .post(
            Uri.parse('$baseUrl$path'),
            headers: _headers(token),
            body: jsonEncode(body),
          )
          .timeout(_requestTimeout);
    });
  }

  Future<http.Response> _put(
    String path, {
    required Map<String, dynamic> body,
  }) async {
    return _authenticatedRequest(() async {
      final token = await _getAccessToken();
      return _httpClient
          .put(
            Uri.parse('$baseUrl$path'),
            headers: _headers(token),
            body: jsonEncode(body),
          )
          .timeout(_requestTimeout);
    });
  }

  Future<http.Response> _authenticatedRequest(
    Future<http.Response> Function() makeRequest,
  ) async {
    try {
      var response = await makeRequest();

      if (response.statusCode == 401) {
        final refreshed = await _tryRefresh();
        if (refreshed) {
          response = await makeRequest();
        }
      }

      _checkResponse(response);
      return response;
    } on TimeoutException {
      throw ApiException(408, 'Request timed out');
    }
  }

  Future<bool> _tryRefresh() {
    return _refreshFuture ??= _doRefresh().whenComplete(() {
      _refreshFuture = null;
    });
  }

  Future<bool> _doRefresh() async {
    final token = await _storage.read(key: _refreshTokenKey);
    if (token == null) return false;

    try {
      final tokens = await refreshToken(token);
      _cachedAccessToken = tokens.accessToken;
      await Future.wait([
        _storage.write(key: _accessTokenKey, value: tokens.accessToken),
        _storage.write(key: _refreshTokenKey, value: tokens.refreshToken),
      ]);
      return true;
    } on ApiException catch (e) {
      debugPrint('Token refresh failed: $e');
      if (e.statusCode == 401) {
        final current = await _storage.read(key: _refreshTokenKey);
        if (current == token) {
          _cachedAccessToken = null;
          await Future.wait([
            _storage.delete(key: _accessTokenKey),
            _storage.delete(key: _refreshTokenKey),
          ]);
        }
      }
      return false;
    } catch (e) {
      debugPrint('Token refresh failed (transient): $e');
      return false;
    }
  }

  Map<String, String> _headers(String? token) {
    return {
      'Content-Type': 'application/json',
      if (token != null) 'Authorization': 'Bearer $token',
    };
  }

  void _checkResponse(http.Response response) {
    if (response.statusCode == 401) {
      throw ApiException(401, 'Unauthorized');
    }
    if (response.statusCode < 200 || response.statusCode >= 300) {
      String message;
      try {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        message = (json['message'] ?? json['error'] ?? 'Request failed') as String;
      } catch (_) {
        message = 'Request failed with status ${response.statusCode}';
      }
      throw ApiException(response.statusCode, message);
    }
  }
}

class ApiException implements Exception {
  final int statusCode;
  final String message;

  ApiException(this.statusCode, this.message);

  @override
  String toString() => 'ApiException($statusCode): $message';
}
