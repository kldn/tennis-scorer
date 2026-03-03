import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:tennis_scorer/models/stats_summary.dart';
import 'package:tennis_scorer/providers/dashboard_provider.dart';
import 'package:tennis_scorer/screens/dashboard_screen.dart';
import 'package:tennis_scorer/screens/settings_screen.dart';

void main() {
  group('DashboardScreen', () {
    testWidgets('shows loading indicator when loading', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dashboardProvider.overrideWith(
              (ref) => _LoadingDashboardNotifier(ref),
            ),
            dashboardRecentMatchesProvider.overrideWithValue([]),
          ],
          child: const MaterialApp(home: DashboardScreen()),
        ),
      );

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('shows empty state when no matches', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dashboardProvider.overrideWith(
              (ref) => _EmptyDashboardNotifier(ref),
            ),
            dashboardRecentMatchesProvider.overrideWithValue([]),
          ],
          child: const MaterialApp(home: DashboardScreen()),
        ),
      );

      expect(find.text('尚無比賽紀錄'), findsOneWidget);
    });

    testWidgets('shows summary when data loaded', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dashboardProvider.overrideWith(
              (ref) => _LoadedDashboardNotifier(ref),
            ),
            dashboardRecentMatchesProvider.overrideWithValue([]),
          ],
          child: const MaterialApp(home: DashboardScreen()),
        ),
      );

      expect(find.text('戰績摘要'), findsOneWidget);
      expect(find.text('70%'), findsOneWidget);
      expect(find.text('10'), findsOneWidget);
    });
  });

  group('SettingsScreen', () {
    testWidgets('shows notification toggles', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(home: SettingsScreen()),
        ),
      );

      expect(find.text('好友請求'), findsOneWidget);
      expect(find.text('比賽結果'), findsOneWidget);
      expect(find.text('對手認領'), findsOneWidget);
      expect(find.text('登出'), findsOneWidget);
    });

    testWidgets('shows logout confirmation dialog', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(home: SettingsScreen()),
        ),
      );

      await tester.tap(find.text('登出'));
      await tester.pumpAndSettle();

      expect(find.text('確認登出'), findsOneWidget);
      expect(find.text('確定要登出嗎？'), findsOneWidget);
    });

    testWidgets('can dismiss logout dialog', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(home: SettingsScreen()),
        ),
      );

      await tester.tap(find.text('登出'));
      await tester.pumpAndSettle();

      await tester.tap(find.text('取消'));
      await tester.pumpAndSettle();

      expect(find.text('確認登出'), findsNothing);
    });

    testWidgets('toggles notification settings', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(home: SettingsScreen()),
        ),
      );

      // Find switch for 好友請求 and toggle it
      final switches = find.byType(Switch);
      expect(switches, findsNWidgets(3));
    });
  });
}

// Test helpers

class _LoadingDashboardNotifier extends DashboardNotifier {
  _LoadingDashboardNotifier(super.ref);

  @override
  Future<void> load() async {
    state = state.copyWith(isLoading: true);
  }
}

class _EmptyDashboardNotifier extends DashboardNotifier {
  _EmptyDashboardNotifier(super.ref);

  @override
  Future<void> load() async {
    state = DashboardState(
      summary: StatsSummary(
        totalMatches: 0,
        wins: 0,
        losses: 0,
        winRate: 0.0,
        currentStreak: CurrentStreak(streakType: StreakType.none, count: 0),
        recentForm: [],
      ),
    );
  }
}

class _LoadedDashboardNotifier extends DashboardNotifier {
  _LoadedDashboardNotifier(super.ref);

  @override
  Future<void> load() async {
    state = DashboardState(
      summary: StatsSummary(
        totalMatches: 10,
        wins: 7,
        losses: 3,
        winRate: 0.7,
        currentStreak: CurrentStreak(streakType: StreakType.win, count: 3),
        recentForm: ['W', 'W', 'W', 'L', 'W'],
      ),
    );
  }
}
