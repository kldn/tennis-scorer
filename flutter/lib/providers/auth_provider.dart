import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import '../services/api_client.dart';
import '../services/auth_service.dart';

const _defaultBaseUrl = 'https://tennis-scorer-api-production.up.railway.app/api';

final apiClientProvider = Provider<ApiClient>((ref) {
  return ApiClient(baseUrl: _defaultBaseUrl);
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
    final loggedIn = await _authService.isLoggedIn;
    state = loggedIn
        ? const AuthState.authenticated()
        : const AuthState.unauthenticated();
  }

  Future<void> signInWithApple() async {
    try {
      await _authService.signInWithApple();
      state = const AuthState.authenticated();
    } catch (e) {
      state = AuthState.error(e.toString());
    }
  }

  Future<void> logout() async {
    await _authService.logout();
    state = const AuthState.unauthenticated();
  }
}

final authProvider = StateNotifierProvider<AuthNotifier, AuthState>((ref) {
  return AuthNotifier(ref.watch(authServiceProvider));
});
