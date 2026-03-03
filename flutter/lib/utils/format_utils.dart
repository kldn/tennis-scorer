String formatDuration(Duration duration) {
  final totalMinutes = duration.inMinutes;
  if (totalMinutes < 60) return '${totalMinutes}m';
  final hours = duration.inHours;
  final mins = totalMinutes.remainder(60);
  return '${hours}h ${mins}m';
}
