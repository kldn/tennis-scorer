import 'package:flutter/material.dart';

class EmptyMatchesPlaceholder extends StatelessWidget {
  final String subtitle;

  const EmptyMatchesPlaceholder({
    super.key,
    this.subtitle = '在手錶上完成比賽後\n紀錄會自動同步到這裡',
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Icon(Icons.sports_tennis, size: 64, color: Colors.grey),
        const SizedBox(height: 16),
        const Text(
          '尚無比賽紀錄',
          style: TextStyle(color: Colors.grey, fontSize: 16),
        ),
        const SizedBox(height: 8),
        Text(
          subtitle,
          textAlign: TextAlign.center,
          style: const TextStyle(color: Colors.grey),
        ),
      ],
    );
  }
}
