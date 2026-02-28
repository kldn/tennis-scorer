import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/momentum_data.dart';
import '../services/stats_repository.dart';
import 'match_detail_provider.dart';

final momentumDataProvider =
    FutureProvider.family<MomentumData, String>((ref, matchId) async {
  final statsRepo = ref.watch(statsRepositoryProvider);
  return statsRepo.getMatchMomentum(matchId);
});

final momentumModeProvider = StateProvider<MomentumMode>((ref) => MomentumMode.basic);
