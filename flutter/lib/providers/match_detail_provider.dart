import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/match_analysis.dart';
import '../models/match_model.dart';
import '../services/stats_repository.dart';
import 'auth_provider.dart';
import 'match_list_provider.dart';

final statsRepositoryProvider = Provider<StatsRepository>((ref) {
  return StatsRepository(apiClient: ref.watch(apiClientProvider));
});

class MatchDetailState {
  final MatchModel? match;
  final MatchAnalysis? analysis;

  const MatchDetailState({this.match, this.analysis});
}

final matchDetailProvider = FutureProvider.family<MatchDetailState, String>((ref, matchId) async {
  final matchRepo = ref.watch(matchRepositoryProvider);
  final statsRepo = ref.watch(statsRepositoryProvider);

  final (match, analysis) = await (
    matchRepo.getMatch(matchId),
    statsRepo.getMatchAnalysis(matchId),
  ).wait;

  return MatchDetailState(
    match: match,
    analysis: analysis,
  );
});
