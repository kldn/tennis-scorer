import '../models/match_analysis.dart';
import '../models/momentum_data.dart';
import '../models/pace_data.dart';
import '../models/stats_summary.dart';
import 'api_client.dart';

class StatsRepository {
  final ApiClient _apiClient;

  StatsRepository({required ApiClient apiClient}) : _apiClient = apiClient;

  Future<StatsSummary> getStatsSummary() async {
    final json = await _apiClient.getStatsSummary();
    return StatsSummary.fromJson(json);
  }

  Future<MatchAnalysis> getMatchAnalysis(String matchId) async {
    final json = await _apiClient.getMatchAnalysis(matchId);
    return MatchAnalysis.fromJson(json);
  }

  Future<MomentumData> getMatchMomentum(String matchId) async {
    final json = await _apiClient.getMatchMomentum(matchId);
    return MomentumData.fromJson(json);
  }

  Future<PaceData> getMatchPace(String matchId) async {
    final json = await _apiClient.getMatchPace(matchId);
    return PaceData.fromJson(json);
  }
}
