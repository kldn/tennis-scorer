import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';

import '../models/match_model.dart';
import '../providers/match_list_provider.dart';
import '../widgets/empty_matches_placeholder.dart';
import '../widgets/error_retry.dart';
import '../widgets/win_loss_avatar.dart';

class MatchListScreen extends ConsumerWidget {
  const MatchListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(matchListProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('比賽紀錄')),
      body: _buildBody(context, ref, state),
    );
  }

  Widget _buildBody(BuildContext context, WidgetRef ref, MatchListState state) {
    if (state.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (state.error != null && state.matches.isEmpty) {
      return ErrorRetry(
        onRetry: () => ref.read(matchListProvider.notifier).loadMatches(),
      );
    }

    if (state.matches.isEmpty) {
      return RefreshIndicator(
        onRefresh: () => ref.read(matchListProvider.notifier).refresh(),
        child: ListView(
          physics: const AlwaysScrollableScrollPhysics(),
          children: const [
            SizedBox(height: 120),
            Center(
              child: EmptyMatchesPlaceholder(
                subtitle: '在 Apple Watch 上完成比賽後\n紀錄會自動同步到這裡',
              ),
            ),
          ],
        ),
      );
    }

    return RefreshIndicator(
      onRefresh: () => ref.read(matchListProvider.notifier).refresh(),
      child: NotificationListener<ScrollNotification>(
        onNotification: (notification) {
          if (notification is ScrollEndNotification &&
              notification.metrics.extentAfter < 200 &&
              !state.isLoadingMore) {
            ref.read(matchListProvider.notifier).loadMore();
          }
          return false;
        },
        child: ListView.builder(
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: state.matches.length + (state.isLoadingMore ? 1 : 0),
          itemBuilder: (context, index) {
            if (index == state.matches.length) {
              return const Padding(
                padding: EdgeInsets.all(16),
                child: Center(child: CircularProgressIndicator()),
              );
            }
            return _MatchTile(match: state.matches[index]);
          },
        ),
      ),
    );
  }
}

class _MatchTile extends StatelessWidget {
  static final _dateFormat = DateFormat('yyyy/MM/dd HH:mm');

  final MatchModel match;

  const _MatchTile({required this.match});

  @override
  Widget build(BuildContext context) {

    return ListTile(
      leading: WinLossAvatar(isWin: match.isWin),
      title: Text(
        match.scoreDisplay,
        style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
      ),
      subtitle: Text(_dateFormat.format(match.startedAt.toLocal())),
      trailing: const Icon(Icons.chevron_right),
      onTap: () => context.push('/matches/${match.id}'),
    );
  }
}
