import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';

import '../models/match_model.dart';
import '../models/stats_summary.dart';
import '../providers/dashboard_provider.dart';
import '../providers/match_list_provider.dart';
import '../widgets/empty_matches_placeholder.dart';
import '../widgets/error_retry.dart';
import '../widgets/win_loss_avatar.dart';

class DashboardScreen extends ConsumerWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(dashboardProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Tennis Scorer')),
      body: _buildBody(context, ref, state),
    );
  }

  Widget _buildBody(BuildContext context, WidgetRef ref, DashboardState state) {
    if (state.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (state.error != null && state.summary == null) {
      return ErrorRetry(
        onRetry: () => ref.read(dashboardProvider.notifier).load(),
      );
    }

    final summary = state.summary;
    if (summary == null || summary.totalMatches == 0) {
      return _buildEmptyState(context, ref);
    }

    final recentMatches = ref.watch(dashboardRecentMatchesProvider);

    return RefreshIndicator(
      onRefresh: () async {
        await Future.wait([
          ref.read(dashboardProvider.notifier).refresh(),
          ref.read(matchListProvider.notifier).refresh(),
        ]);
      },
      child: ListView(
        physics: const AlwaysScrollableScrollPhysics(),
        padding: const EdgeInsets.all(16),
        children: [
          _SummaryCard(summary: summary),
          const SizedBox(height: 16),
          _RecentFormCard(summary: summary),
          const SizedBox(height: 16),
          _RecentMatchesSection(
            matches: recentMatches,
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState(BuildContext context, WidgetRef ref) {
    return RefreshIndicator(
      onRefresh: () => ref.read(dashboardProvider.notifier).refresh(),
      child: ListView(
        physics: const AlwaysScrollableScrollPhysics(),
        children: const [
          SizedBox(height: 120),
          Center(child: EmptyMatchesPlaceholder()),
        ],
      ),
    );
  }
}

class _SummaryCard extends StatelessWidget {
  final StatsSummary summary;

  const _SummaryCard({required this.summary});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('戰績摘要', style: theme.textTheme.titleMedium),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: _StatItem(
                    label: '勝率',
                    value: '${(summary.winRate * 100).toStringAsFixed(0)}%',
                    valueColor: Colors.green,
                  ),
                ),
                Expanded(
                  child: _StatItem(
                    label: '總場數',
                    value: '${summary.totalMatches}',
                  ),
                ),
                Expanded(
                  child: _StatItem(
                    label: '勝/負',
                    value: '${summary.wins}/${summary.losses}',
                  ),
                ),
                Expanded(
                  child: _StatItem(
                    label: '連勝/連敗',
                    value: summary.currentStreak.displayText,
                    valueColor: summary.currentStreak.streakType == StreakType.win
                        ? Colors.green
                        : summary.currentStreak.streakType == StreakType.loss
                            ? Colors.red
                            : null,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _StatItem extends StatelessWidget {
  final String label;
  final String value;
  final Color? valueColor;

  const _StatItem({
    required this.label,
    required this.value,
    this.valueColor,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      children: [
        Text(
          value,
          style: theme.textTheme.headlineSmall?.copyWith(
            fontWeight: FontWeight.bold,
            color: valueColor,
          ),
        ),
        const SizedBox(height: 4),
        Text(label, style: theme.textTheme.bodySmall),
      ],
    );
  }
}

class _RecentFormCard extends StatelessWidget {
  final StatsSummary summary;

  const _RecentFormCard({required this.summary});

  @override
  Widget build(BuildContext context) {
    if (summary.recentForm.isEmpty) return const SizedBox.shrink();

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('近期戰績', style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 12),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: summary.recentForm.map((result) {
                final isWin = result == 'W';
                return WinLossAvatar(isWin: isWin, radius: 16);
              }).toList(),
            ),
          ],
        ),
      ),
    );
  }
}

class _RecentMatchesSection extends StatelessWidget {
  static final _dateFormat = DateFormat('MM/dd HH:mm');

  final List<MatchModel> matches;

  const _RecentMatchesSection({required this.matches});

  @override
  Widget build(BuildContext context) {
    if (matches.isEmpty) return const SizedBox.shrink();

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    '近期比賽',
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                ),
                TextButton(
                  onPressed: () => context.go('/history'),
                  child: const Text('查看全部'),
                ),
              ],
            ),
            const SizedBox(height: 8),
            ...matches.map((match) {
              return ListTile(
                contentPadding: EdgeInsets.zero,
                dense: true,
                leading: WinLossAvatar(isWin: match.isWin, radius: 14, fontSize: 12),
                title: Text(
                  match.scoreDisplay,
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                subtitle: Text(_dateFormat.format(match.startedAt.toLocal())),
                trailing: const Icon(Icons.chevron_right, size: 20),
                onTap: () => context.push('/matches/${match.id}'),
              );
            }),
          ],
        ),
      ),
    );
  }
}
