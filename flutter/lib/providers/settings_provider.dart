import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../services/api_client.dart';
import 'auth_provider.dart';

class NotificationSettings {
  final bool friendRequests;
  final bool matchResults;
  final bool matchClaims;

  const NotificationSettings({
    this.friendRequests = true,
    this.matchResults = true,
    this.matchClaims = true,
  });

  NotificationSettings copyWith({
    bool? friendRequests,
    bool? matchResults,
    bool? matchClaims,
  }) {
    return NotificationSettings(
      friendRequests: friendRequests ?? this.friendRequests,
      matchResults: matchResults ?? this.matchResults,
      matchClaims: matchClaims ?? this.matchClaims,
    );
  }

  Map<String, dynamic> toJson() => {
        'friend_requests': friendRequests,
        'match_results': matchResults,
        'match_claims': matchClaims,
      };
}

class NotificationSettingsNotifier extends StateNotifier<NotificationSettings> {
  final ApiClient _apiClient;
  bool _hasLocalEdits = false;

  NotificationSettingsNotifier(this._apiClient)
      : super(const NotificationSettings()) {
    _loadFromServer();
  }

  Future<void> _loadFromServer() async {
    try {
      final json = await _apiClient.getNotificationSettings();
      if (!mounted || _hasLocalEdits) return;
      state = NotificationSettings(
        friendRequests: json['friend_requests'] as bool? ?? true,
        matchResults: json['match_results'] as bool? ?? true,
        matchClaims: json['match_claims'] as bool? ?? true,
      );
    } catch (e) {
      debugPrint('Failed to load notification settings: $e');
    }
  }

  Future<void> updateSetting({
    bool? friendRequests,
    bool? matchResults,
    bool? matchClaims,
  }) async {
    _hasLocalEdits = true;
    final previous = state;
    final updated = state.copyWith(
      friendRequests: friendRequests,
      matchResults: matchResults,
      matchClaims: matchClaims,
    );
    state = updated;

    try {
      await _apiClient.putNotificationSettings(updated.toJson());
    } catch (e) {
      debugPrint('Failed to update notification settings: $e');
      if (mounted && identical(state, updated)) state = previous;
    }
  }
}

final notificationSettingsProvider =
    StateNotifierProvider<NotificationSettingsNotifier, NotificationSettings>(
        (ref) {
  return NotificationSettingsNotifier(ref.watch(apiClientProvider));
});
