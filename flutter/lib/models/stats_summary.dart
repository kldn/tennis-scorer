enum StreakType { win, loss, none }

class CurrentStreak {
  final StreakType streakType;
  final int count;

  CurrentStreak({required this.streakType, required this.count});

  factory CurrentStreak.fromJson(Map<String, dynamic> json) {
    final typeStr = json['streak_type'] as String;
    final type = StreakType.values.firstWhere(
      (e) => e.name == typeStr,
      orElse: () => StreakType.none,
    );
    return CurrentStreak(
      streakType: type,
      count: json['count'] as int,
    );
  }

  String get displayText {
    if (streakType == StreakType.none || count == 0) return '-';
    final label = streakType == StreakType.win ? 'W' : 'L';
    return '$count$label';
  }
}

class StatsSummary {
  final int totalMatches;
  final int wins;
  final int losses;
  final double winRate;
  final CurrentStreak currentStreak;
  final List<String> recentForm;

  StatsSummary({
    required this.totalMatches,
    required this.wins,
    required this.losses,
    required this.winRate,
    required this.currentStreak,
    required this.recentForm,
  });

  factory StatsSummary.fromJson(Map<String, dynamic> json) {
    return StatsSummary(
      totalMatches: json['total_matches'] as int,
      wins: json['wins'] as int,
      losses: json['losses'] as int,
      winRate: (json['win_rate'] as num).toDouble(),
      currentStreak: CurrentStreak.fromJson(
        json['current_streak'] as Map<String, dynamic>,
      ),
      recentForm: (json['recent_form'] as List).cast<String>(),
    );
  }
}
