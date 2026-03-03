import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter/foundation.dart';
import 'package:intl/intl.dart';

import '../models/match_analysis.dart';
import '../models/match_model.dart';
import '../providers/match_detail_provider.dart';

class MatchDetailScreen extends ConsumerWidget {
  final String matchId;

  const MatchDetailScreen({super.key, required this.matchId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(matchDetailProvider(matchId));

    return Scaffold(
      appBar: AppBar(title: const Text('比賽詳情')),
      body: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (err, _) {
          debugPrint('MatchDetail load error: $err');
          return const Center(child: Text('載入失敗，請稍後再試'));
        },
        data: (state) {
          final match = state.match;
          final analysis = state.analysis;
          if (match == null || analysis == null) {
            return const Center(child: Text('資料不存在'));
          }
          return _DetailContent(match: match, analysis: analysis);
        },
      ),
    );
  }
}

class _DetailContent extends StatelessWidget {
  final MatchModel match;
  final MatchAnalysis analysis;

  const _DetailContent({required this.match, required this.analysis});

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        _MatchHeader(match: match),
        const SizedBox(height: 16),
        _MomentumButton(matchId: match.id),
        const SizedBox(height: 16),
        _StatSection(
          title: '總得分',
          children: [
            _ComparisonRow(
              label: '得分',
              value1: '${analysis.player1.totalPoints.pointsWon}',
              value2: '${analysis.player2.totalPoints.pointsWon}',
            ),
            _ComparisonRow(
              label: '得分率',
              value1: _pct(analysis.player1.totalPoints.pointsWonPercentage),
              value2: _pct(analysis.player2.totalPoints.pointsWonPercentage),
            ),
          ],
        ),
        _StatSection(
          title: 'Break Point',
          children: [
            _ComparisonRow(
              label: '製造',
              value1: '${analysis.player1.breakPoints.breakPointsCreated}',
              value2: '${analysis.player2.breakPoints.breakPointsCreated}',
            ),
            _ComparisonRow(
              label: '轉換',
              value1: '${analysis.player1.breakPoints.breakPointsConverted}',
              value2: '${analysis.player2.breakPoints.breakPointsConverted}',
            ),
            _ComparisonRow(
              label: '轉換率',
              value1: _pct(analysis.player1.breakPoints.breakPointConversionRate),
              value2: _pct(analysis.player2.breakPoints.breakPointConversionRate),
            ),
          ],
        ),
        _StatSection(
          title: '發球',
          children: [
            _ComparisonRow(
              label: '保發率',
              value1: _pct(analysis.player1.service.holdPercentage),
              value2: _pct(analysis.player2.service.holdPercentage),
            ),
            _ComparisonRow(
              label: '破發率',
              value1: _pct(analysis.player1.service.breakPercentage),
              value2: _pct(analysis.player2.service.breakPercentage),
            ),
            _ComparisonRow(
              label: '優勢比',
              value1: analysis.player1.service.dominanceRatio.toStringAsFixed(2),
              value2: analysis.player2.service.dominanceRatio.toStringAsFixed(2),
            ),
          ],
        ),
        _StatSection(
          title: 'Deuce',
          children: [
            _ComparisonRow(
              label: 'Deuce 局勝率',
              value1: _pct(analysis.player1.deuce.deuceGameWinRate),
              value2: _pct(analysis.player2.deuce.deuceGameWinRate),
            ),
            _ComparisonRow(
              label: '平均 Deuce 數',
              value1: analysis.player1.deuce.averageDeucePerDeuceGame.toStringAsFixed(1),
              value2: analysis.player2.deuce.averageDeucePerDeuceGame.toStringAsFixed(1),
            ),
          ],
        ),
        _StatSection(
          title: '連續得分',
          children: [
            _ComparisonRow(
              label: '最長連續得分',
              value1: '${analysis.player1.streaks.longestPointStreak}',
              value2: '${analysis.player2.streaks.longestPointStreak}',
            ),
            _ComparisonRow(
              label: '最長連續失分',
              value1: '${analysis.player1.streaks.longestPointDrought}',
              value2: '${analysis.player2.streaks.longestPointDrought}',
            ),
            _ComparisonRow(
              label: '最長連續保發',
              value1: '${analysis.player1.streaks.longestServiceHoldStreak}',
              value2: '${analysis.player2.streaks.longestServiceHoldStreak}',
            ),
          ],
        ),
        _StatSection(
          title: '關鍵分',
          children: [
            _ComparisonRow(
              label: 'Clutch Score',
              value1: analysis.player1.clutch.clutchScore.toStringAsFixed(2),
              value2: analysis.player2.clutch.clutchScore.toStringAsFixed(2),
            ),
            _ComparisonRow(
              label: 'Break Point 勝率',
              value1: _pct(analysis.player1.clutch.breakPointWinRate),
              value2: _pct(analysis.player2.clutch.breakPointWinRate),
            ),
            _ComparisonRow(
              label: 'Set Point 勝率',
              value1: _pct(analysis.player1.clutch.setPointWinRate),
              value2: _pct(analysis.player2.clutch.setPointWinRate),
            ),
          ],
        ),
        if (analysis.player1.tiebreak.tiebreaksPlayed > 0 ||
            analysis.player2.tiebreak.tiebreaksPlayed > 0)
          _StatSection(
            title: 'Tiebreak',
            children: [
              _ComparisonRow(
                label: '勝/負',
                value1:
                    '${analysis.player1.tiebreak.tiebreaksWon}/${analysis.player1.tiebreak.tiebreaksPlayed}',
                value2:
                    '${analysis.player2.tiebreak.tiebreaksWon}/${analysis.player2.tiebreak.tiebreaksPlayed}',
              ),
              _ComparisonRow(
                label: '平均分差',
                value1: analysis.player1.tiebreak.averageTiebreakMargin.toStringAsFixed(1),
                value2: analysis.player2.tiebreak.averageTiebreakMargin.toStringAsFixed(1),
              ),
            ],
          ),
      ],
    );
  }

  String _pct(double value) => '${(value * 100).toStringAsFixed(1)}%';
}

class _MatchHeader extends StatelessWidget {
  final MatchModel match;

  const _MatchHeader({required this.match});

  @override
  Widget build(BuildContext context) {
    final dateFormat = DateFormat('yyyy/MM/dd HH:mm');
    final duration = match.duration;
    final durationStr =
        '${duration.inHours}h ${duration.inMinutes.remainder(60)}m';

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Text(
              match.scoreDisplay,
              style: Theme.of(context).textTheme.headlineLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
              decoration: BoxDecoration(
                color: match.isWin ? Colors.green : Colors.red,
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                match.isWin ? '勝利' : '失敗',
                style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              dateFormat.format(match.startedAt.toLocal()),
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            Text(
              '時長: $durationStr',
              style: Theme.of(context).textTheme.bodySmall,
            ),
          ],
        ),
      ),
    );
  }
}

class _MomentumButton extends StatelessWidget {
  final String matchId;

  const _MomentumButton({required this.matchId});

  @override
  Widget build(BuildContext context) {
    return FilledButton.icon(
      onPressed: () => context.push('/matches/$matchId/momentum'),
      icon: const Icon(Icons.show_chart),
      label: const Text('Momentum 圖表'),
    );
  }
}

class _StatSection extends StatelessWidget {
  final String title;
  final List<Widget> children;

  const _StatSection({required this.title, required this.children});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(child: Text(title, style: Theme.of(context).textTheme.titleMedium)),
                SizedBox(
                  width: 60,
                  child: Text('P1', textAlign: TextAlign.center,
                      style: Theme.of(context).textTheme.labelSmall),
                ),
                SizedBox(
                  width: 60,
                  child: Text('P2', textAlign: TextAlign.center,
                      style: Theme.of(context).textTheme.labelSmall),
                ),
              ],
            ),
            const Divider(),
            ...children,
          ],
        ),
      ),
    );
  }
}

class _ComparisonRow extends StatelessWidget {
  final String label;
  final String value1;
  final String value2;

  const _ComparisonRow({
    required this.label,
    required this.value1,
    required this.value2,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          Expanded(child: Text(label, style: Theme.of(context).textTheme.bodySmall)),
          SizedBox(
            width: 60,
            child: Text(value1, textAlign: TextAlign.center,
                style: const TextStyle(fontWeight: FontWeight.w600)),
          ),
          SizedBox(
            width: 60,
            child: Text(value2, textAlign: TextAlign.center,
                style: const TextStyle(fontWeight: FontWeight.w600)),
          ),
        ],
      ),
    );
  }
}
