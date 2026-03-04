import 'package:flutter_test/flutter_test.dart';

import 'package:tennis_scorer/models/stats_summary.dart';
import 'package:tennis_scorer/models/pace_data.dart';

void main() {
  group('StatsSummary', () {
    test('fromJson parses correctly', () {
      final json = {
        'total_matches': 10,
        'wins': 7,
        'losses': 3,
        'win_rate': 0.7,
        'current_streak': {'streak_type': 'win', 'count': 3},
        'recent_form': ['W', 'W', 'W', 'L', 'W'],
      };

      final summary = StatsSummary.fromJson(json);
      expect(summary.totalMatches, 10);
      expect(summary.wins, 7);
      expect(summary.losses, 3);
      expect(summary.winRate, 0.7);
      expect(summary.currentStreak.streakType, StreakType.win);
      expect(summary.currentStreak.count, 3);
      expect(summary.recentForm.length, 5);
    });

    test('zero matches', () {
      final json = {
        'total_matches': 0,
        'wins': 0,
        'losses': 0,
        'win_rate': 0.0,
        'current_streak': {'streak_type': 'none', 'count': 0},
        'recent_form': <String>[],
      };

      final summary = StatsSummary.fromJson(json);
      expect(summary.totalMatches, 0);
      expect(summary.currentStreak.displayText, '-');
    });
  });

  group('CurrentStreak', () {
    test('displayText for win streak', () {
      final streak = CurrentStreak(streakType: StreakType.win, count: 5);
      expect(streak.displayText, '5W');
    });

    test('displayText for loss streak', () {
      final streak = CurrentStreak(streakType: StreakType.loss, count: 2);
      expect(streak.displayText, '2L');
    });

    test('displayText for no streak', () {
      final streak = CurrentStreak(streakType: StreakType.none, count: 0);
      expect(streak.displayText, '-');
    });
  });

  group('PaceData', () {
    test('fromJson parses correctly', () {
      final json = {
        'average_point_interval_seconds': 25.5,
        'per_game_durations': [
          {'set_number': 1, 'game_number': 1, 'duration_seconds': 180.0},
          {'set_number': 1, 'game_number': 2, 'duration_seconds': 240.0},
        ],
        'per_set_durations': [
          {'set_number': 1, 'duration_seconds': 1800.0},
          {'set_number': 2, 'duration_seconds': 2400.0},
        ],
        'total_duration_seconds': 4200.0,
        'point_intervals': [20.0, 25.0, 30.0],
      };

      final pace = PaceData.fromJson(json);
      expect(pace.averagePointIntervalSeconds, 25.5);
      expect(pace.perGameDurations.length, 2);
      expect(pace.perSetDurations.length, 2);
      expect(pace.totalDurationSeconds, 4200.0);
      expect(pace.pointIntervals.length, 3);
    });

    test('averageIntervalDisplay formats seconds', () {
      final pace = PaceData(
        averagePointIntervalSeconds: 25.5,
        perGameDurations: [],
        perSetDurations: [],
        totalDurationSeconds: 4200.0,
        pointIntervals: [],
      );
      expect(pace.averageIntervalDisplay, '25.5s');
    });

    test('averageIntervalDisplay formats minutes', () {
      final pace = PaceData(
        averagePointIntervalSeconds: 90.0,
        perGameDurations: [],
        perSetDurations: [],
        totalDurationSeconds: 4200.0,
        pointIntervals: [],
      );
      expect(pace.averageIntervalDisplay, '1m 30s');
    });

    test('totalDurationDisplay formats hours and minutes', () {
      final pace = PaceData(
        averagePointIntervalSeconds: 25.0,
        perGameDurations: [],
        perSetDurations: [],
        totalDurationSeconds: 5400.0,
        pointIntervals: [],
      );
      expect(pace.totalDurationDisplay, '1h 30m');
    });

    test('totalDurationDisplay formats minutes only', () {
      final pace = PaceData(
        averagePointIntervalSeconds: 25.0,
        perGameDurations: [],
        perSetDurations: [],
        totalDurationSeconds: 2700.0,
        pointIntervals: [],
      );
      expect(pace.totalDurationDisplay, '45m');
    });
  });

  group('GameDuration', () {
    test('fromJson parses correctly', () {
      final json = {
        'set_number': 2,
        'game_number': 5,
        'duration_seconds': 300.0,
      };

      final gd = GameDuration.fromJson(json);
      expect(gd.setNumber, 2);
      expect(gd.gameNumber, 5);
      expect(gd.durationSeconds, 300.0);
    });
  });

  group('SetDuration', () {
    test('fromJson parses correctly', () {
      final json = {
        'set_number': 1,
        'duration_seconds': 1800.0,
      };

      final sd = SetDuration.fromJson(json);
      expect(sd.setNumber, 1);
      expect(sd.durationSeconds, 1800.0);
    });
  });
}
