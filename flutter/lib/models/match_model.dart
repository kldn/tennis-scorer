class MatchModel {
  final String id;
  final String? clientId;
  final String matchType;
  final Map<String, dynamic> config;
  final int winner;
  final int player1Sets;
  final int player2Sets;
  final DateTime startedAt;
  final DateTime endedAt;
  final DateTime createdAt;

  MatchModel({
    required this.id,
    this.clientId,
    required this.matchType,
    required this.config,
    required this.winner,
    required this.player1Sets,
    required this.player2Sets,
    required this.startedAt,
    required this.endedAt,
    required this.createdAt,
  });

  factory MatchModel.fromJson(Map<String, dynamic> json) {
    return MatchModel(
      id: json['id'] as String,
      clientId: json['client_id'] as String?,
      matchType: json['match_type'] as String,
      config: Map<String, dynamic>.from(json['config'] as Map),
      winner: json['winner'] as int,
      player1Sets: json['player1_sets'] as int,
      player2Sets: json['player2_sets'] as int,
      startedAt: DateTime.parse(json['started_at'] as String),
      endedAt: DateTime.parse(json['ended_at'] as String),
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }

  bool get isWin => winner == 1;

  String get scoreDisplay => '$player1Sets - $player2Sets';

  Duration get duration => endedAt.difference(startedAt);
}

class MatchListResponse {
  final List<MatchModel> matches;
  final int total;

  MatchListResponse({required this.matches, required this.total});

  factory MatchListResponse.fromJson(Map<String, dynamic> json) {
    return MatchListResponse(
      matches: (json['matches'] as List)
          .map((m) => MatchModel.fromJson(m as Map<String, dynamic>))
          .toList(),
      total: json['total'] as int,
    );
  }
}
