import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/momentum_data.dart';
import '../models/pace_data.dart';
import 'match_detail_provider.dart';

final momentumDataProvider =
    FutureProvider.family<MomentumData, String>((ref, matchId) async {
  final statsRepo = ref.watch(statsRepositoryProvider);
  return statsRepo.getMatchMomentum(matchId);
});

final momentumModeProvider = StateProvider<MomentumMode>((ref) => MomentumMode.basic);

final paceDataProvider =
    FutureProvider.family<PaceData, String>((ref, matchId) async {
  final statsRepo = ref.watch(statsRepositoryProvider);
  return statsRepo.getMatchPace(matchId);
});
