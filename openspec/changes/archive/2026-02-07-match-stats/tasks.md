## 1. Statistics Data Model

- [ ] 1.1 Create `MatchStats` struct with totalMatches, wins, losses properties
- [ ] 1.2 Add computed property `winRate: Double`

## 2. Statistics Calculation

- [ ] 2.1 Add `getStats() -> MatchStats` method to MatchHistoryStore
- [ ] 2.2 Implement counting logic: wins = matches where winner == 1
- [ ] 2.3 Handle empty history case (return zeros)

## 3. UI Integration

- [ ] 3.1 Add statistics display to match completion view in ContentView.swift
- [ ] 3.2 Format as "戰績: N勝 M敗 (P%)"
- [ ] 3.3 Calculate percentage and round to integer

## 4. State Management

- [ ] 4.1 Add `@Published var stats: MatchStats` to TennisMatch or create StatsViewModel
- [ ] 4.2 Update stats when match completes and is saved
- [ ] 4.3 Load stats on app launch

## 5. Build & Verify

- [ ] 5.1 Build watchOS app to verify Swift changes compile
- [ ] 5.2 Manual test: complete matches, verify stats display correctly
- [ ] 5.3 Manual test: verify stats update after each completed match
