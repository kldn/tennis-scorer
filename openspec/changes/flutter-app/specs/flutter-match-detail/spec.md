## ADDED Requirements

### Requirement: Match detail with full statistics
The app SHALL display a match detail screen showing complete statistics fetched from `GET /api/stats/match/:id/analysis`.

#### Scenario: Display match header
- **WHEN** the user opens a match detail
- **THEN** the app SHALL display the match date, final score (sets), match duration, and win/loss result

#### Scenario: Display break point statistics
- **WHEN** the match detail screen loads
- **THEN** the app SHALL display break point opportunities and conversion rates for both players

#### Scenario: Display service statistics
- **WHEN** the match detail screen loads
- **THEN** the app SHALL display service-related statistics for both players

#### Scenario: Display deuce statistics
- **WHEN** the match detail screen loads
- **THEN** the app SHALL display deuce count and outcomes

#### Scenario: Display streak statistics
- **WHEN** the match detail screen loads
- **THEN** the app SHALL display longest winning streaks for both players

#### Scenario: Display clutch moment statistics
- **WHEN** the match detail screen loads
- **THEN** the app SHALL display clutch/critical point conversion rates

#### Scenario: Display tiebreak statistics
- **WHEN** the match detail screen loads and the match contains tiebreaks
- **THEN** the app SHALL display tiebreak results and point breakdowns

#### Scenario: Navigate to momentum chart
- **WHEN** the user taps the momentum chart section or button
- **THEN** the app SHALL navigate to the full momentum chart view for that match
