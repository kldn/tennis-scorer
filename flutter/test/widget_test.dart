import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:tennis_scorer/main.dart';

void main() {
  testWidgets('App boots without crashing', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(child: TennisScorerApp()),
    );

    // Verify the app widget is present
    expect(find.byType(TennisScorerApp), findsOneWidget);
  });
}
