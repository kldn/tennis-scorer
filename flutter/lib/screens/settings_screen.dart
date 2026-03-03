import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/auth_provider.dart';
import '../providers/settings_provider.dart';

class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifSettings = ref.watch(notificationSettingsProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('設定')),
      body: ListView(
        children: [
          // Account section
          _SectionHeader(title: '帳號'),
          ListTile(
            leading: const Icon(Icons.person),
            title: const Text('已登入'),
            subtitle: const Text('Apple ID'),
          ),
          const Divider(),

          // Notification preferences section
          _SectionHeader(title: '通知偏好'),
          SwitchListTile(
            secondary: const Icon(Icons.person_add),
            title: const Text('好友請求'),
            subtitle: const Text('收到好友請求時通知'),
            value: notifSettings.friendRequests,
            onChanged: (value) {
              ref
                  .read(notificationSettingsProvider.notifier)
                  .updateSetting(friendRequests: value);
            },
          ),
          SwitchListTile(
            secondary: const Icon(Icons.sports_tennis),
            title: const Text('比賽結果'),
            subtitle: const Text('比賽完成時通知'),
            value: notifSettings.matchResults,
            onChanged: (value) {
              ref
                  .read(notificationSettingsProvider.notifier)
                  .updateSetting(matchResults: value);
            },
          ),
          SwitchListTile(
            secondary: const Icon(Icons.link),
            title: const Text('對手認領'),
            subtitle: const Text('對手認領比賽時通知'),
            value: notifSettings.matchClaims,
            onChanged: (value) {
              ref
                  .read(notificationSettingsProvider.notifier)
                  .updateSetting(matchClaims: value);
            },
          ),
          const Divider(),

          // Sign out
          ListTile(
            leading: const Icon(Icons.logout, color: Colors.red),
            title: const Text('登出', style: TextStyle(color: Colors.red)),
            onTap: () => _confirmLogout(context, ref),
          ),
        ],
      ),
    );
  }

  void _confirmLogout(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('確認登出'),
        content: const Text('確定要登出嗎？'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('取消'),
          ),
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              ref.read(authProvider.notifier).logout();
            },
            child: const Text('登出', style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  final String title;

  const _SectionHeader({required this.title});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
      child: Text(
        title,
        style: Theme.of(context).textTheme.titleSmall?.copyWith(
              color: Theme.of(context).colorScheme.primary,
            ),
      ),
    );
  }
}
