import 'package:flutter/material.dart';

class WinLossAvatar extends StatelessWidget {
  final bool isWin;
  final double radius;
  final double fontSize;

  const WinLossAvatar({
    super.key,
    required this.isWin,
    this.radius = 20,
    this.fontSize = 14,
  });

  @override
  Widget build(BuildContext context) {
    return CircleAvatar(
      radius: radius,
      backgroundColor: isWin ? Colors.green : Colors.red,
      child: Text(
        isWin ? 'W' : 'L',
        style: TextStyle(
          color: Colors.white,
          fontWeight: FontWeight.bold,
          fontSize: fontSize,
        ),
      ),
    );
  }
}
