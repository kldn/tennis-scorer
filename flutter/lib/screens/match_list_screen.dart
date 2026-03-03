import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';

import '../models/match_model.dart';
import '../providers/auth_provider.dart';
import '../providers/match_list_provider.dart';

class MatchListScreen extends ConsumerWidget {
  const MatchListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(matchListProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('比賽紀錄'),
        actions: [
          IconButton(
            icon: const Icon(Icons.logout),
            onPressed: () => ref.read(authProvider.notifier).logout(),
          ),
        ],
      ),
      body: _buildBody(context, ref, state),
    );
  }

  Widget _buildBody(BuildContext context, WidgetRef ref, MatchListState state) {
    if (state.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (state.error != null && state.matches.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('載入失敗', style: Theme.of(context).textTheme.bodyLarge),
            const SizedBox(height: 8),
            ElevatedButton(
              onPressed: () => ref.read(matchListProvider.notifier).loadMatches(),
              child: const Text('重試'),
            ),
          ],
        ),
      );
    }

    if (state.matches.isEmpty) {
      return const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.sports_tennis, size: 64, color: Colors.grey),
            SizedBox(height: 16),
            Text('尚無比賽紀錄', style: TextStyle(color: Colors.grey, fontSize: 16)),
            SizedBox(height: 8),
            Text(
              '在 Apple Watch 上完成比賽後\n紀錄會自動同步到這裡',
              textAlign: TextAlign.center,
              style: TextStyle(color: Colors.grey),
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
  final MatchModel match;

  const _MatchTile({required this.match});

  @override
  Widget build(BuildContext context) {
    final dateFormat = DateFormat('yyyy/MM/dd HH:mm');

    return ListTile(
      leading: CircleAvatar(
        backgroundColor: match.isWin ? Colors.green : Colors.red,
        child: Text(
          match.isWin ? 'W' : 'L',
          style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
        ),
      ),
      title: Text(
        match.scoreDisplay,
        style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
      ),
      subtitle: Text(dateFormat.format(match.startedAt.toLocal())),
      trailing: const Icon(Icons.chevron_right),
      onTap: () => context.push('/matches/${match.id}'),
    );
  }
}
