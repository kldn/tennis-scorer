import 'package:flutter/material.dart';

class ErrorRetry extends StatelessWidget {
  final VoidCallback onRetry;

  const ErrorRetry({super.key, required this.onRetry});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text('載入失敗', style: Theme.of(context).textTheme.bodyLarge),
          const SizedBox(height: 8),
          ElevatedButton(
            onPressed: onRetry,
            child: const Text('重試'),
          ),
        ],
      ),
    );
  }
}
