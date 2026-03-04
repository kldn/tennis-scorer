import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/match_model.dart';
import '../services/match_repository.dart';
import 'auth_provider.dart';

final matchRepositoryProvider = Provider<MatchRepository>((ref) {
  return MatchRepository(apiClient: ref.watch(apiClientProvider));
});

class MatchListState {
  final List<MatchModel> matches;
  final int total;
  final bool isLoading;
  final bool isLoadingMore;
  final String? error;

  const MatchListState({
    this.matches = const [],
    this.total = 0,
    this.isLoading = false,
    this.isLoadingMore = false,
    this.error,
  });

  bool get hasMore => matches.length < total;

  MatchListState copyWith({
    List<MatchModel>? matches,
    int? total,
    bool? isLoading,
    bool? isLoadingMore,
    String? error,
  }) {
    return MatchListState(
      matches: matches ?? this.matches,
      total: total ?? this.total,
      isLoading: isLoading ?? this.isLoading,
      isLoadingMore: isLoadingMore ?? this.isLoadingMore,
      error: error,
    );
  }
}

class MatchListNotifier extends StateNotifier<MatchListState> {
  final MatchRepository _repository;
  static const _pageSize = 20;

  MatchListNotifier(this._repository) : super(const MatchListState()) {
    loadMatches();
  }

  Future<void> loadMatches() => _fetch(showLoading: true);

  Future<void> _fetch({bool showLoading = false}) async {
    if (showLoading) state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _repository.getMatches(limit: _pageSize, offset: 0);
      if (!mounted) return;
      state = MatchListState(
        matches: response.matches,
        total: response.total,
      );
    } catch (e) {
      if (!mounted) return;
      state = state.copyWith(isLoading: false, error: e.toString());
    }
  }

  Future<void> loadMore() async {
    if (state.isLoadingMore || !state.hasMore) return;

    state = state.copyWith(isLoadingMore: true);
    try {
      final response = await _repository.getMatches(
        limit: _pageSize,
        offset: state.matches.length,
      );
      if (!mounted) return;
      state = state.copyWith(
        matches: [...state.matches, ...response.matches],
        total: response.total,
        isLoadingMore: false,
      );
    } catch (e) {
      if (!mounted) return;
      state = state.copyWith(isLoadingMore: false, error: e.toString());
    }
  }

  Future<void> refresh() => _fetch();
}

final matchListProvider =
    StateNotifierProvider<MatchListNotifier, MatchListState>((ref) {
  return MatchListNotifier(ref.watch(matchRepositoryProvider));
});
