import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/match_model.dart';
import '../models/stats_summary.dart';
import 'match_detail_provider.dart';
import 'match_list_provider.dart';

class DashboardState {
  final StatsSummary? summary;
  final bool isLoading;
  final String? error;

  const DashboardState({
    this.summary,
    this.isLoading = false,
    this.error,
  });

  DashboardState copyWith({
    StatsSummary? summary,
    bool? isLoading,
    String? error,
  }) {
    return DashboardState(
      summary: summary ?? this.summary,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

class DashboardNotifier extends StateNotifier<DashboardState> {
  final Ref _ref;

  DashboardNotifier(this._ref) : super(const DashboardState()) {
    load();
  }

  Future<void> load() => _fetch(showLoading: true);
  Future<void> refresh() => _fetch();

  Future<void> _fetch({bool showLoading = false}) async {
    if (showLoading) state = state.copyWith(isLoading: true, error: null);
    try {
      final statsRepo = _ref.read(statsRepositoryProvider);
      final summary = await statsRepo.getStatsSummary();

      if (!mounted) return;
      state = DashboardState(summary: summary);
    } catch (e) {
      debugPrint('Dashboard fetch error: $e');
      if (!mounted) return;
      state = state.copyWith(isLoading: false, error: '載入失敗，請稍後再試');
    }
  }
}

final dashboardProvider =
    StateNotifierProvider<DashboardNotifier, DashboardState>((ref) {
  return DashboardNotifier(ref);
});

/// Recent matches for the dashboard, derived from [matchListProvider].
final dashboardRecentMatchesProvider = Provider<List<MatchModel>>((ref) {
  final matchListState = ref.watch(matchListProvider);
  final matches = matchListState.matches;
  return matches.length > 5 ? matches.sublist(0, 5) : matches;
});
