## ADDED Requirements

### Requirement: Basic momentum calculation
The system SHALL compute a basic momentum series from `Vec<PointContext>`:
- `momentum[0] = if scorer == Player1 { +1 } else { -1 }`
- `momentum[i] = momentum[i-1] + (if scorer == Player1 { +1 } else { -1 })`
- Positive values indicate Player1 momentum, negative indicates Player2

#### Scenario: Player1 wins first 3 points
- **WHEN** Player1 wins the first 3 points
- **THEN** the basic momentum series SHALL be `[1, 2, 3]`

#### Scenario: Alternating winners
- **WHEN** Player1 wins point 1, Player2 wins point 2, Player1 wins point 3
- **THEN** the basic momentum series SHALL be `[1, 0, 1]`

### Requirement: Weighted momentum calculation
The system SHALL compute a weighted momentum series where certain points have amplified impact:
- Break point converted by returner: weight = ±3.0 (instead of ±1)
- Set point converted: weight = ±5.0
- Point won during a deuce game: weight = ±1.5
- All other points: weight = ±1.0
- Weights stack multiplicatively when multiple conditions apply (e.g., a break point that is also a set point = 3.0 × 5.0 / 1.0 = ±15.0)

The weighted momentum is cumulative: `weighted[i] = weighted[i-1] + signed_weight`

#### Scenario: Break point conversion amplifies momentum
- **WHEN** Player2 converts a break point (not a set point, not deuce)
- **THEN** the weighted momentum SHALL decrease by 3.0 at that point (instead of 1.0)

#### Scenario: Set point in a deuce game
- **WHEN** Player1 converts a set point during a deuce game
- **THEN** the weight for that point SHALL be `5.0 × 1.5 = 7.5`

### Requirement: Per-set momentum curves
The system SHALL compute independent momentum curves for each set:
- Each set's momentum starts at 0
- The momentum series is partitioned by set boundaries (when `set_number` changes in `PointContext`)
- Output is a `Vec<Vec<f64>>` (one inner vec per set) for both basic and weighted

#### Scenario: Two-set match
- **WHEN** a match has 2 sets with 50 and 60 points respectively
- **THEN** the per-set momentum SHALL have 2 inner vectors of lengths 50 and 60, each starting from 0

### Requirement: MomentumData struct
The system SHALL define a `MomentumData` struct containing:
- `basic: Vec<f64>` — full-match basic momentum series
- `weighted: Vec<f64>` — full-match weighted momentum series
- `per_set_basic: Vec<Vec<f64>>` — per-set basic momentum
- `per_set_weighted: Vec<Vec<f64>>` — per-set weighted momentum

The struct SHALL be returned from `compute_momentum(points: &[PointContext]) -> MomentumData`.

#### Scenario: Empty match
- **WHEN** `compute_momentum` is called with an empty slice
- **THEN** all fields SHALL be empty vectors
