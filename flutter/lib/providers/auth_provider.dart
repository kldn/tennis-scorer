import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../services/api_client.dart';
import '../services/auth_service.dart';

const _defaultBaseUrl = String.fromEnvironment(
  'API_BASE_URL',
  defaultValue: 'https://tennis-scorer-api-production.up.railway.app/api',
);

final apiClientProvider = Provider<ApiClient>((ref) {
  final client = ApiClient(baseUrl: _defaultBaseUrl);
  ref.onDispose(client.close);
  return client;
});

final authServiceProvider = Provider<AuthService>((ref) {
  return AuthService(apiClient: ref.watch(apiClientProvider));
});

enum AuthStatus { unknown, authenticated, unauthenticated }

class AuthState {
  final AuthStatus status;
  final String? error;

  const AuthState({required this.status, this.error});

  const AuthState.unknown() : this(status: AuthStatus.unknown);
  const AuthState.authenticated() : this(status: AuthStatus.authenticated);
  const AuthState.unauthenticated() : this(status: AuthStatus.unauthenticated);
  AuthState.error(String message)
      : this(status: AuthStatus.unauthenticated, error: message);
}

class AuthNotifier extends StateNotifier<AuthState> {
  final AuthService _authService;

  AuthNotifier(this._authService) : super(const AuthState.unknown()) {
    _checkLoginStatus();
  }

  Future<void> _checkLoginStatus() async {
    try {
      final loggedIn = await _authService.isLoggedIn;
      if (!mounted || state.status != AuthStatus.unknown) return;
      state = loggedIn
          ? const AuthState.authenticated()
          : const AuthState.unauthenticated();
    } catch (e) {
      debugPrint('Login status check failed: $e');
      if (!mounted || state.status != AuthStatus.unknown) return;
      state = const AuthState.unauthenticated();
    }
  }

  Future<void> signInWithApple() async {
    try {
      await _authService.signInWithApple();
      if (!mounted) return;
      state = const AuthState.authenticated();
    } on ApiException catch (e) {
      if (!mounted) return;
      state = AuthState.error(e.message);
    } catch (e) {
      debugPrint('Sign in error: $e');
      if (!mounted) return;
      state = AuthState.error('登入失敗，請重試');
    }
  }

  Future<void> logout() async {
    await _authService.logout();
    if (!mounted) return;
    state = const AuthState.unauthenticated();
  }
}

final authProvider = StateNotifierProvider<AuthNotifier, AuthState>((ref) {
  return AuthNotifier(ref.watch(authServiceProvider));
});
