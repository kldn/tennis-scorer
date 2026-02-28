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

  static List<double> _toDoubleList(dynamic value, String key) {
    if (value is! List) throw FormatException('Invalid momentum field: $key');
    return value.map((e) => (e as num).toDouble()).toList();
  }

  static List<List<double>> _toDoubleMatrix(dynamic value, String key) {
    if (value is! List) throw FormatException('Invalid momentum field: $key');
    return value.map((set) {
      if (set is! List) throw FormatException('Invalid momentum set in: $key');
      return set.map((e) => (e as num).toDouble()).toList();
    }).toList();
  }

  factory MomentumData.fromJson(Map<String, dynamic> json) {
    return MomentumData(
      basic: _toDoubleList(json['basic'], 'basic'),
      weighted: _toDoubleList(json['weighted'], 'weighted'),
      perSetBasic: _toDoubleMatrix(json['per_set_basic'], 'per_set_basic'),
      perSetWeighted: _toDoubleMatrix(json['per_set_weighted'], 'per_set_weighted'),
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
