import '../utils/format_utils.dart';

class GameDuration {
  final int setNumber;
  final int gameNumber;
  final double durationSeconds;

  GameDuration({
    required this.setNumber,
    required this.gameNumber,
    required this.durationSeconds,
  });

  factory GameDuration.fromJson(Map<String, dynamic> json) {
    return GameDuration(
      setNumber: json['set_number'] as int,
      gameNumber: json['game_number'] as int,
      durationSeconds: (json['duration_seconds'] as num).toDouble(),
    );
  }
}

class SetDuration {
  final int setNumber;
  final double durationSeconds;

  SetDuration({required this.setNumber, required this.durationSeconds});

  factory SetDuration.fromJson(Map<String, dynamic> json) {
    return SetDuration(
      setNumber: json['set_number'] as int,
      durationSeconds: (json['duration_seconds'] as num).toDouble(),
    );
  }
}

class PaceData {
  final double averagePointIntervalSeconds;
  final List<GameDuration> perGameDurations;
  final List<SetDuration> perSetDurations;
  final double totalDurationSeconds;
  final List<double> pointIntervals;

  PaceData({
    required this.averagePointIntervalSeconds,
    required this.perGameDurations,
    required this.perSetDurations,
    required this.totalDurationSeconds,
    required this.pointIntervals,
  });

  factory PaceData.fromJson(Map<String, dynamic> json) {
    return PaceData(
      averagePointIntervalSeconds:
          (json['average_point_interval_seconds'] as num).toDouble(),
      perGameDurations: (json['per_game_durations'] as List)
          .map((e) => GameDuration.fromJson(e as Map<String, dynamic>))
          .toList(),
      perSetDurations: (json['per_set_durations'] as List)
          .map((e) => SetDuration.fromJson(e as Map<String, dynamic>))
          .toList(),
      totalDurationSeconds:
          (json['total_duration_seconds'] as num).toDouble(),
      pointIntervals: (json['point_intervals'] as List)
          .map((e) => (e as num).toDouble())
          .toList(),
    );
  }

  String get averageIntervalDisplay {
    final seconds = averagePointIntervalSeconds;
    if (seconds < 60) return '${seconds.toStringAsFixed(1)}s';
    final mins = (seconds / 60).floor();
    final secs = (seconds % 60).toStringAsFixed(0);
    return '${mins}m ${secs}s';
  }

  String get totalDurationDisplay =>
      formatDuration(Duration(seconds: totalDurationSeconds.toInt()));
}
