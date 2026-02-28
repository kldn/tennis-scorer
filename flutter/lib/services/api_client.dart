import 'dart:convert';

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

  ApiClient({
    required this.baseUrl,
    http.Client? httpClient,
    FlutterSecureStorage? storage,
  })  : _httpClient = httpClient ?? http.Client(),
        _storage = storage ?? const FlutterSecureStorage();

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

  Future<String> refreshToken(String refreshToken) async {
    final response = await _post(
      '/auth/refresh',
      body: {'refresh_token': refreshToken},
      authenticated: false,
    );
    final json = jsonDecode(response.body) as Map<String, dynamic>;
    return json['access_token'] as String;
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

  // MARK: - Internal

  Future<http.Response> _get(String path) async {
    return _authenticatedRequest(() async {
      final token = await _storage.read(key: _accessTokenKey);
      return _httpClient.get(
        Uri.parse('$baseUrl$path'),
        headers: _headers(token),
      );
    });
  }

  Future<http.Response> _post(
    String path, {
    required Map<String, dynamic> body,
    bool authenticated = true,
  }) async {
    if (!authenticated) {
      final response = await _httpClient.post(
        Uri.parse('$baseUrl$path'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(body),
      );
      _checkResponse(response);
      return response;
    }

    return _authenticatedRequest(() async {
      final token = await _storage.read(key: _accessTokenKey);
      return _httpClient.post(
        Uri.parse('$baseUrl$path'),
        headers: _headers(token),
        body: jsonEncode(body),
      );
    });
  }

  Future<http.Response> _authenticatedRequest(
    Future<http.Response> Function() makeRequest,
  ) async {
    var response = await makeRequest();

    if (response.statusCode == 401) {
      final refreshed = await _tryRefresh();
      if (refreshed) {
        response = await makeRequest();
      }
    }

    _checkResponse(response);
    return response;
  }

  Future<bool> _tryRefresh() async {
    final token = await _storage.read(key: _refreshTokenKey);
    if (token == null) return false;

    try {
      final newAccessToken = await refreshToken(token);
      await _storage.write(key: _accessTokenKey, value: newAccessToken);
      return true;
    } catch (_) {
      await _storage.delete(key: _accessTokenKey);
      await _storage.delete(key: _refreshTokenKey);
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
      throw ApiException(response.statusCode, response.body);
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
