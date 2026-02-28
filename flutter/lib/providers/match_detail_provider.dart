import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/match_analysis.dart';
import '../models/match_model.dart';
import '../services/match_repository.dart';
import '../services/stats_repository.dart';
import 'auth_provider.dart';
import 'match_list_provider.dart';

final statsRepositoryProvider = Provider<StatsRepository>((ref) {
  return StatsRepository(apiClient: ref.watch(apiClientProvider));
});

class MatchDetailState {
  final MatchModel? match;
  final MatchAnalysis? analysis;
  final bool isLoading;
  final String? error;

  const MatchDetailState({
    this.match,
    this.analysis,
    this.isLoading = false,
    this.error,
  });
}

final matchDetailProvider = FutureProvider.family<MatchDetailState, String>((ref, matchId) async {
  final matchRepo = ref.watch(matchRepositoryProvider);
  final statsRepo = ref.watch(statsRepositoryProvider);

  final results = await Future.wait([
    matchRepo.getMatch(matchId),
    statsRepo.getMatchAnalysis(matchId),
  ]);

  return MatchDetailState(
    match: results[0] as MatchModel,
    analysis: results[1] as MatchAnalysis,
  );
});
