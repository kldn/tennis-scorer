import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../screens/dashboard_screen.dart';
import '../screens/login_screen.dart';
import '../screens/match_detail_screen.dart';
import '../screens/match_list_screen.dart';
import '../screens/momentum_chart_screen.dart';
import '../screens/settings_screen.dart';
import '../screens/shell_screen.dart';
import 'auth_provider.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();

final routerProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authProvider);

  return GoRouter(
    navigatorKey: _rootNavigatorKey,
    initialLocation: '/loading',
    redirect: (context, state) {
      final isAuthenticated = authState.status == AuthStatus.authenticated;
      final isLoggingIn = state.matchedLocation == '/login';
      final isLoading = state.matchedLocation == '/loading';

      if (authState.status == AuthStatus.unknown) {
        return isLoading ? null : '/loading';
      }

      if (!isAuthenticated && !isLoggingIn) return '/login';
      if (isAuthenticated && (isLoggingIn || isLoading)) return '/dashboard';
      return null;
    },
    routes: [
      GoRoute(
        path: '/loading',
        builder: (context, state) => const Scaffold(
          body: Center(child: CircularProgressIndicator()),
        ),
      ),
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginScreen(),
      ),
      StatefulShellRoute.indexedStack(
        builder: (context, state, navigationShell) {
          return ShellScreen(navigationShell: navigationShell);
        },
        branches: [
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/dashboard',
                builder: (context, state) => const DashboardScreen(),
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/history',
                builder: (context, state) => const MatchListScreen(),
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/settings',
                builder: (context, state) => const SettingsScreen(),
              ),
            ],
          ),
        ],
      ),
      GoRoute(
        parentNavigatorKey: _rootNavigatorKey,
        path: '/matches/:id',
        builder: (context, state) {
          final matchId = state.pathParameters['id']!;
          return MatchDetailScreen(matchId: matchId);
        },
      ),
      GoRoute(
        parentNavigatorKey: _rootNavigatorKey,
        path: '/matches/:id/momentum',
        builder: (context, state) {
          final matchId = state.pathParameters['id']!;
          return MomentumChartScreen(matchId: matchId);
        },
      ),
    ],
  );
});
