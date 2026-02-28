import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../screens/login_screen.dart';
import '../screens/match_detail_screen.dart';
import '../screens/match_list_screen.dart';
import '../screens/momentum_chart_screen.dart';
import 'auth_provider.dart';

final routerProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authProvider);

  return GoRouter(
    initialLocation: '/matches',
    redirect: (context, state) {
      final isAuthenticated = authState.status == AuthStatus.authenticated;
      final isLoggingIn = state.matchedLocation == '/login';

      if (authState.status == AuthStatus.unknown) return null;

      if (!isAuthenticated && !isLoggingIn) return '/login';
      if (isAuthenticated && isLoggingIn) return '/matches';
      return null;
    },
    routes: [
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/matches',
        builder: (context, state) => const MatchListScreen(),
      ),
      GoRoute(
        path: '/matches/:id',
        builder: (context, state) {
          final matchId = state.pathParameters['id']!;
          return MatchDetailScreen(matchId: matchId);
        },
      ),
      GoRoute(
        path: '/matches/:id/momentum',
        builder: (context, state) {
          final matchId = state.pathParameters['id']!;
          return MomentumChartScreen(matchId: matchId);
        },
      ),
    ],
  );
});
