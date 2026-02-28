// Integration test stubs for the Flutter app.
//
// Full integration tests require:
// 1. A running API backend
// 2. A valid Apple Sign In context (simulator or device)
//
// These tests verify widget structure with mocked auth state.
//
// Run with: flutter test

import 'package:flutter_test/flutter_test.dart';

import 'package:tennis_scorer/models/match_model.dart';
import 'package:tennis_scorer/models/momentum_data.dart';
import 'package:tennis_scorer/models/match_analysis.dart';

void main() {
  group('MatchModel', () {
    test('fromJson parses correctly', () {
      final json = {
        'id': '123e4567-e89b-12d3-a456-426614174000',
        'client_id': null,
        'match_type': 'singles',
        'config': {'sets_to_win': 2},
        'winner': 1,
        'player1_sets': 2,
        'player2_sets': 1,
        'started_at': '2026-02-06T10:00:00Z',
        'ended_at': '2026-02-06T11:30:00Z',
        'created_at': '2026-02-06T10:00:00Z',
      };

      final match = MatchModel.fromJson(json);
      expect(match.id, '123e4567-e89b-12d3-a456-426614174000');
      expect(match.winner, 1);
      expect(match.isWin, true);
      expect(match.scoreDisplay, '2 - 1');
      expect(match.duration.inMinutes, 90);
    });

    test('isWin returns false for player 2 win', () {
      final json = {
        'id': 'abc',
        'match_type': 'singles',
        'config': {},
        'winner': 2,
        'player1_sets': 0,
        'player2_sets': 2,
        'started_at': '2026-02-06T10:00:00Z',
        'ended_at': '2026-02-06T11:00:00Z',
        'created_at': '2026-02-06T10:00:00Z',
      };

      final match = MatchModel.fromJson(json);
      expect(match.isWin, false);
    });
  });

  group('MatchListResponse', () {
    test('fromJson parses list correctly', () {
      final json = {
        'matches': [
          {
            'id': 'match-1',
            'match_type': 'singles',
            'config': {},
            'winner': 1,
            'player1_sets': 2,
            'player2_sets': 0,
            'started_at': '2026-02-06T10:00:00Z',
            'ended_at': '2026-02-06T11:00:00Z',
            'created_at': '2026-02-06T10:00:00Z',
          },
        ],
        'total': 1,
      };

      final response = MatchListResponse.fromJson(json);
      expect(response.matches.length, 1);
      expect(response.total, 1);
    });
  });

  group('MomentumData', () {
    test('fromJson parses correctly', () {
      final json = {
        'basic': [1.0, 2.0, 1.0],
        'weighted': [1.5, 3.0, 1.5],
        'per_set_basic': [
          [1.0, 2.0],
          [0.0, -1.0],
        ],
        'per_set_weighted': [
          [1.5, 3.0],
          [0.0, -1.5],
        ],
      };

      final data = MomentumData.fromJson(json);
      expect(data.basic.length, 3);
      expect(data.weighted.length, 3);
      expect(data.perSetBasic.length, 2);
      expect(data.perSetWeighted.length, 2);
    });
  });
}
