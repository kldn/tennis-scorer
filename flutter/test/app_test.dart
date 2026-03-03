// Unit tests for Flutter app models.
//
// Run with: flutter test

import 'package:flutter_test/flutter_test.dart';

import 'package:tennis_scorer/models/match_model.dart';
import 'package:tennis_scorer/models/momentum_data.dart';

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

    test('handles null winner (match in progress)', () {
      final json = {
        'id': 'in-progress',
        'match_type': 'singles',
        'config': {},
        'winner': null,
        'player1_sets': 1,
        'player2_sets': 1,
        'started_at': '2026-02-06T10:00:00Z',
        'ended_at': null,
        'created_at': '2026-02-06T10:00:00Z',
      };

      final match = MatchModel.fromJson(json);
      expect(match.isWin, false);
      expect(match.scoreDisplay, '1 - 1');
    });

    test('handles missing optional fields', () {
      final json = {
        'id': 'minimal',
        'match_type': 'singles',
        'config': {},
        'winner': 1,
        'player1_sets': 2,
        'player2_sets': 0,
        'started_at': '2026-02-06T10:00:00Z',
        'created_at': '2026-02-06T10:00:00Z',
      };

      final match = MatchModel.fromJson(json);
      expect(match.clientId, isNull);
      expect(match.endedAt, isNull);
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

    test('throws FormatException on invalid data', () {
      final json = {
        'basic': 'not a list',
        'weighted': [1.0],
        'per_set_basic': [],
        'per_set_weighted': [],
      };

      expect(() => MomentumData.fromJson(json), throwsFormatException);
    });
  });
}
