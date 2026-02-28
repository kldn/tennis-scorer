class BreakPointStats {
  final int breakPointsCreated;
  final int breakPointsConverted;
  final int breakPointsFaced;
  final int breakPointsSaved;
  final double breakPointConversionRate;

  BreakPointStats({
    required this.breakPointsCreated,
    required this.breakPointsConverted,
    required this.breakPointsFaced,
    required this.breakPointsSaved,
    required this.breakPointConversionRate,
  });

  factory BreakPointStats.fromJson(Map<String, dynamic> json) {
    return BreakPointStats(
      breakPointsCreated: json['break_points_created'] as int,
      breakPointsConverted: json['break_points_converted'] as int,
      breakPointsFaced: json['break_points_faced'] as int,
      breakPointsSaved: json['break_points_saved'] as int,
      breakPointConversionRate: (json['break_point_conversion_rate'] as num).toDouble(),
    );
  }
}

class ServiceStats {
  final int serviceGamesPlayed;
  final int serviceGamesHeld;
  final double holdPercentage;
  final int returnGamesPlayed;
  final int returnGamesWon;
  final double breakPercentage;
  final double dominanceRatio;

  ServiceStats({
    required this.serviceGamesPlayed,
    required this.serviceGamesHeld,
    required this.holdPercentage,
    required this.returnGamesPlayed,
    required this.returnGamesWon,
    required this.breakPercentage,
    required this.dominanceRatio,
  });

  factory ServiceStats.fromJson(Map<String, dynamic> json) {
    return ServiceStats(
      serviceGamesPlayed: json['service_games_played'] as int,
      serviceGamesHeld: json['service_games_held'] as int,
      holdPercentage: (json['hold_percentage'] as num).toDouble(),
      returnGamesPlayed: json['return_games_played'] as int,
      returnGamesWon: json['return_games_won'] as int,
      breakPercentage: (json['break_percentage'] as num).toDouble(),
      dominanceRatio: (json['dominance_ratio'] as num).toDouble(),
    );
  }
}

class DeuceStats {
  final int deuceGamesCount;
  final int deuceGamesWon;
  final double deuceGameWinRate;
  final int totalDeuceCount;
  final double averageDeucePerDeuceGame;

  DeuceStats({
    required this.deuceGamesCount,
    required this.deuceGamesWon,
    required this.deuceGameWinRate,
    required this.totalDeuceCount,
    required this.averageDeucePerDeuceGame,
  });

  factory DeuceStats.fromJson(Map<String, dynamic> json) {
    return DeuceStats(
      deuceGamesCount: json['deuce_games_count'] as int,
      deuceGamesWon: json['deuce_games_won'] as int,
      deuceGameWinRate: (json['deuce_game_win_rate'] as num).toDouble(),
      totalDeuceCount: json['total_deuce_count'] as int,
      averageDeucePerDeuceGame: (json['average_deuces_per_deuce_game'] as num).toDouble(),
    );
  }
}

class StreakStats {
  final int longestPointStreak;
  final int longestPointDrought;
  final int longestServiceHoldStreak;
  final int maxGamesInARow;

  StreakStats({
    required this.longestPointStreak,
    required this.longestPointDrought,
    required this.longestServiceHoldStreak,
    required this.maxGamesInARow,
  });

  factory StreakStats.fromJson(Map<String, dynamic> json) {
    return StreakStats(
      longestPointStreak: json['longest_point_streak'] as int,
      longestPointDrought: json['longest_point_drought'] as int,
      longestServiceHoldStreak: json['longest_service_hold_streak'] as int,
      maxGamesInARow: json['max_games_in_a_row'] as int,
    );
  }
}

class ClutchStats {
  final double breakPointWinRate;
  final double setPointWinRate;
  final double matchPointWinRate;
  final double normalPointWinRate;
  final double clutchScore;

  ClutchStats({
    required this.breakPointWinRate,
    required this.setPointWinRate,
    required this.matchPointWinRate,
    required this.normalPointWinRate,
    required this.clutchScore,
  });

  factory ClutchStats.fromJson(Map<String, dynamic> json) {
    return ClutchStats(
      breakPointWinRate: (json['break_point_win_rate'] as num).toDouble(),
      setPointWinRate: (json['set_point_win_rate'] as num).toDouble(),
      matchPointWinRate: (json['match_point_win_rate'] as num).toDouble(),
      normalPointWinRate: (json['normal_point_win_rate'] as num).toDouble(),
      clutchScore: (json['clutch_score'] as num).toDouble(),
    );
  }
}

class TiebreakStats {
  final int tiebreaksPlayed;
  final int tiebreaksWon;
  final double tiebreakWinRate;
  final double averageTiebreakMargin;

  TiebreakStats({
    required this.tiebreaksPlayed,
    required this.tiebreaksWon,
    required this.tiebreakWinRate,
    required this.averageTiebreakMargin,
  });

  factory TiebreakStats.fromJson(Map<String, dynamic> json) {
    return TiebreakStats(
      tiebreaksPlayed: json['tiebreaks_played'] as int,
      tiebreaksWon: json['tiebreaks_won'] as int,
      tiebreakWinRate: (json['tiebreak_win_rate'] as num).toDouble(),
      averageTiebreakMargin: (json['average_tiebreak_margin'] as num).toDouble(),
    );
  }
}

class TotalPointsStats {
  final int pointsWon;
  final int totalPoints;
  final double pointsWonPercentage;

  TotalPointsStats({
    required this.pointsWon,
    required this.totalPoints,
    required this.pointsWonPercentage,
  });

  factory TotalPointsStats.fromJson(Map<String, dynamic> json) {
    return TotalPointsStats(
      pointsWon: json['points_won'] as int,
      totalPoints: (json['total_points'] ?? json['total'] ?? 0) as int,
      pointsWonPercentage: (json['points_won_percentage'] as num).toDouble(),
    );
  }
}

class PlayerStats {
  final BreakPointStats breakPoints;
  final ServiceStats service;
  final DeuceStats deuce;
  final StreakStats streaks;
  final ClutchStats clutch;
  final TiebreakStats tiebreak;
  final TotalPointsStats totalPoints;

  PlayerStats({
    required this.breakPoints,
    required this.service,
    required this.deuce,
    required this.streaks,
    required this.clutch,
    required this.tiebreak,
    required this.totalPoints,
  });

  factory PlayerStats.fromJson(Map<String, dynamic> json) {
    return PlayerStats(
      breakPoints: BreakPointStats.fromJson(json['break_points'] as Map<String, dynamic>),
      service: ServiceStats.fromJson(json['service'] as Map<String, dynamic>),
      deuce: DeuceStats.fromJson(json['deuce'] as Map<String, dynamic>),
      streaks: StreakStats.fromJson(json['streaks'] as Map<String, dynamic>),
      clutch: ClutchStats.fromJson(json['clutch'] as Map<String, dynamic>),
      tiebreak: TiebreakStats.fromJson(json['tiebreak'] as Map<String, dynamic>),
      totalPoints: TotalPointsStats.fromJson(json['total_points'] as Map<String, dynamic>),
    );
  }
}

class MatchAnalysis {
  final PlayerStats player1;
  final PlayerStats player2;

  MatchAnalysis({required this.player1, required this.player2});

  factory MatchAnalysis.fromJson(Map<String, dynamic> json) {
    return MatchAnalysis(
      player1: PlayerStats.fromJson(json['player1'] as Map<String, dynamic>),
      player2: PlayerStats.fromJson(json['player2'] as Map<String, dynamic>),
    );
  }
}
