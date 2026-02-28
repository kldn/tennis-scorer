import '../models/match_model.dart';
import 'api_client.dart';

class MatchRepository {
  final ApiClient _apiClient;

  MatchRepository({required ApiClient apiClient}) : _apiClient = apiClient;

  Future<MatchListResponse> getMatches({int limit = 20, int offset = 0}) async {
    final json = await _apiClient.getMatches(limit: limit, offset: offset);
    return MatchListResponse.fromJson(json);
  }

  Future<MatchModel> getMatch(String id) async {
    final json = await _apiClient.getMatch(id);
    return MatchModel.fromJson(json);
  }
}
