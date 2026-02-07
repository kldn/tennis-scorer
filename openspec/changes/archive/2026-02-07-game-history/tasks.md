## 1. Data Model

- [ ] 1.1 Create `CompletedMatch` struct with Codable conformance
- [ ] 1.2 Add fields: id (UUID), date (Date), winner (Int), player1Sets (Int), player2Sets (Int)

## 2. Persistence Layer

- [ ] 2.1 Create `MatchHistoryStore` class to manage persistence
- [ ] 2.2 Implement `save(_ match: CompletedMatch)` method using UserDefaults
- [ ] 2.3 Implement `loadAll() -> [CompletedMatch]` method
- [ ] 2.4 Implement 100-match limit with FIFO removal
- [ ] 2.5 Add UserDefaults key constant for history storage

## 3. Integration with TennisMatch

- [ ] 3.1 Add `MatchHistoryStore` instance to TennisMatch or make it a singleton
- [ ] 3.2 Modify `newMatch()` to check if current match is completed
- [ ] 3.3 If completed, create CompletedMatch from current score and save
- [ ] 3.4 Then proceed with creating new match

## 4. API for UI

- [ ] 4.1 Add `getMatchHistory() -> [CompletedMatch]` method accessible from UI
- [ ] 4.2 Ensure history is sorted by date descending

## 5. Build & Verify

- [ ] 5.1 Build watchOS app to verify Swift changes compile
- [ ] 5.2 Manual test: complete a match, start new match, verify history saved
- [ ] 5.3 Manual test: force quit app, relaunch, verify history persists
