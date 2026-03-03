import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:sign_in_with_apple/sign_in_with_apple.dart';

import 'api_client.dart';

class AuthService {
  final ApiClient _apiClient;
  final FlutterSecureStorage _storage;

  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';

  AuthService({
    required ApiClient apiClient,
    FlutterSecureStorage? storage,
  })  : _apiClient = apiClient,
        _storage = storage ?? const FlutterSecureStorage();

  Future<String?> get accessToken => _storage.read(key: _accessTokenKey);
  Future<String?> get refreshToken => _storage.read(key: _refreshTokenKey);

  Future<bool> get isLoggedIn async => (await accessToken) != null;

  Future<void> signInWithApple() async {
    final credential = await SignInWithApple.getAppleIDCredential(
      scopes: [AppleIDAuthorizationScopes.email],
    );

    final identityToken = credential.identityToken;
    if (identityToken == null) {
      throw Exception('Failed to get Apple identity token');
    }

    final tokens = await _apiClient.loginWithApple(identityToken);
    await _saveTokens(tokens.accessToken, tokens.refreshToken);
  }

  Future<String?> refreshAccessToken() async {
    final token = await refreshToken;
    if (token == null) return null;

    try {
      final tokens = await _apiClient.refreshToken(token);
      await _saveTokens(tokens.accessToken, tokens.refreshToken);
      return tokens.accessToken;
    } on ApiException catch (e) {
      // Only logout on auth rejection (401); transient errors should not force logout
      if (e.statusCode == 401) {
        debugPrint('Refresh token rejected (401), logging out');
        await logout();
      }
      return null;
    } catch (e) {
      // Network/timeout errors — don't logout
      debugPrint('Token refresh failed (transient): $e');
      return null;
    }
  }

  Future<void> logout() async {
    await _storage.delete(key: _accessTokenKey);
    await _storage.delete(key: _refreshTokenKey);
  }

  Future<void> _saveTokens(String accessToken, String refreshToken) async {
    await _storage.write(key: _accessTokenKey, value: accessToken);
    await _storage.write(key: _refreshTokenKey, value: refreshToken);
  }
}
