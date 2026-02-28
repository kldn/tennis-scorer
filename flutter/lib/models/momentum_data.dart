class MomentumData {
  final List<double> basic;
  final List<double> weighted;
  final List<List<double>> perSetBasic;
  final List<List<double>> perSetWeighted;

  MomentumData({
    required this.basic,
    required this.weighted,
    required this.perSetBasic,
    required this.perSetWeighted,
  });

  factory MomentumData.fromJson(Map<String, dynamic> json) {
    return MomentumData(
      basic: (json['basic'] as List).map((e) => (e as num).toDouble()).toList(),
      weighted: (json['weighted'] as List).map((e) => (e as num).toDouble()).toList(),
      perSetBasic: (json['per_set_basic'] as List)
          .map((set) => (set as List).map((e) => (e as num).toDouble()).toList())
          .toList(),
      perSetWeighted: (json['per_set_weighted'] as List)
          .map((set) => (set as List).map((e) => (e as num).toDouble()).toList())
          .toList(),
    );
  }
}

enum MomentumMode {
  basic('Basic'),
  weighted('Weighted'),
  perSetBasic('Per-Set Basic'),
  perSetWeighted('Per-Set Weighted');

  final String label;
  const MomentumMode(this.label);
}
