import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/momentum_data.dart';
import '../providers/momentum_provider.dart';

class MomentumChartScreen extends ConsumerWidget {
  final String matchId;

  const MomentumChartScreen({super.key, required this.matchId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final momentumAsync = ref.watch(momentumDataProvider(matchId));
    final currentMode = ref.watch(momentumModeProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Momentum')),
      body: momentumAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (err, _) => Center(child: Text('載入失敗: $err')),
        data: (data) => _ChartContent(data: data, mode: currentMode, ref: ref),
      ),
    );
  }
}

class _ChartContent extends StatelessWidget {
  final MomentumData data;
  final MomentumMode mode;
  final WidgetRef ref;

  const _ChartContent({
    required this.data,
    required this.mode,
    required this.ref,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Mode selector
        Padding(
          padding: const EdgeInsets.all(12),
          child: SegmentedButton<MomentumMode>(
            segments: MomentumMode.values
                .map((m) => ButtonSegment(value: m, label: Text(m.label)))
                .toList(),
            selected: {mode},
            onSelectionChanged: (selected) {
              ref.read(momentumModeProvider.notifier).state = selected.first;
            },
          ),
        ),
        // Chart
        Expanded(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(8, 0, 16, 16),
            child: _buildChart(context),
          ),
        ),
        // Legend
        Padding(
          padding: const EdgeInsets.only(bottom: 16),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              _LegendItem(color: Colors.green, label: 'P1 優勢'),
              const SizedBox(width: 24),
              _LegendItem(color: Colors.red, label: 'P2 優勢'),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildChart(BuildContext context) {
    final isPerSet = mode == MomentumMode.perSetBasic || mode == MomentumMode.perSetWeighted;

    if (isPerSet) {
      return _buildPerSetChart(context);
    }
    return _buildSingleChart(context);
  }

  Widget _buildSingleChart(BuildContext context) {
    final values = mode == MomentumMode.basic ? data.basic : data.weighted;

    if (values.isEmpty) {
      return const Center(child: Text('無資料'));
    }

    final spots = values.asMap().entries
        .map((e) => FlSpot(e.key.toDouble(), e.value))
        .toList();

    final maxY = values.map((v) => v.abs()).reduce((a, b) => a > b ? a : b);
    final yBound = (maxY * 1.1).ceilToDouble().clamp(1.0, double.infinity);

    return LineChart(
      LineChartData(
        minY: -yBound,
        maxY: yBound,
        gridData: FlGridData(
          drawHorizontalLine: true,
          drawVerticalLine: false,
          getDrawingHorizontalLine: (value) {
            if (value == 0) {
              return const FlLine(
                color: Colors.grey,
                strokeWidth: 2,
              );
            }
            return FlLine(
              color: Colors.grey.withValues(alpha: 0.2),
              strokeWidth: 0.5,
            );
          },
        ),
        titlesData: const FlTitlesData(
          topTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
          rightTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
          bottomTitles: AxisTitles(
            axisNameWidget: Text('Point'),
            sideTitles: SideTitles(showTitles: false),
          ),
        ),
        borderData: FlBorderData(show: false),
        lineBarsData: [
          LineChartBarData(
            spots: spots,
            isCurved: true,
            preventCurveOverShooting: true,
            barWidth: 2,
            color: Colors.blue,
            dotData: const FlDotData(show: false),
            belowBarData: BarAreaData(
              show: true,
              color: Colors.red.withValues(alpha: 0.15),
              cutOffY: 0,
              applyCutOffY: true,
            ),
            aboveBarData: BarAreaData(
              show: true,
              color: Colors.green.withValues(alpha: 0.15),
              cutOffY: 0,
              applyCutOffY: true,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildPerSetChart(BuildContext context) {
    final sets = mode == MomentumMode.perSetBasic
        ? data.perSetBasic
        : data.perSetWeighted;

    if (sets.isEmpty) {
      return const Center(child: Text('無資料'));
    }

    final colors = [Colors.blue, Colors.orange, Colors.purple, Colors.teal, Colors.pink];

    double globalMaxY = 1.0;
    for (final setData in sets) {
      for (final v in setData) {
        if (v.abs() > globalMaxY) globalMaxY = v.abs();
      }
    }
    final yBound = (globalMaxY * 1.1).ceilToDouble();

    final lineBars = <LineChartBarData>[];
    for (var i = 0; i < sets.length; i++) {
      final setData = sets[i];
      final spots = setData.asMap().entries
          .map((e) => FlSpot(e.key.toDouble(), e.value))
          .toList();
      lineBars.add(LineChartBarData(
        spots: spots,
        isCurved: true,
        preventCurveOverShooting: true,
        barWidth: 2,
        color: colors[i % colors.length],
        dotData: const FlDotData(show: false),
      ));
    }

    return Column(
      children: [
        // Per-set legend
        Wrap(
          spacing: 12,
          children: List.generate(sets.length, (i) {
            return _LegendItem(
              color: colors[i % colors.length],
              label: 'Set ${i + 1}',
            );
          }),
        ),
        const SizedBox(height: 8),
        Expanded(
          child: LineChart(
            LineChartData(
              minY: -yBound,
              maxY: yBound,
              gridData: FlGridData(
                drawHorizontalLine: true,
                drawVerticalLine: false,
                getDrawingHorizontalLine: (value) {
                  if (value == 0) {
                    return const FlLine(color: Colors.grey, strokeWidth: 2);
                  }
                  return FlLine(
                    color: Colors.grey.withValues(alpha: 0.2),
                    strokeWidth: 0.5,
                  );
                },
              ),
              titlesData: const FlTitlesData(
                topTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                rightTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                bottomTitles: AxisTitles(
                  axisNameWidget: Text('Point'),
                  sideTitles: SideTitles(showTitles: false),
                ),
              ),
              borderData: FlBorderData(show: false),
              lineBarsData: lineBars,
            ),
          ),
        ),
      ],
    );
  }
}

class _LegendItem extends StatelessWidget {
  final Color color;
  final String label;

  const _LegendItem({required this.color, required this.label});

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 12,
          height: 12,
          decoration: BoxDecoration(color: color, shape: BoxShape.circle),
        ),
        const SizedBox(width: 4),
        Text(label, style: Theme.of(context).textTheme.bodySmall),
      ],
    );
  }
}
