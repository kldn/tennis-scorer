# Tennis Scorer

A cross-platform tennis scoring engine written in Rust. Designed as a library that can be compiled to a static library for use via C FFI in Swift (watchOS), Flutter, or other frontends.

## Features

- **Complete tennis scoring rules**: Game → Set → Match state machine
- **Configurable match formats**: Best-of-3 and Best-of-5
- **Tiebreak support**: Standard (7 points), super tiebreak (10 points), or custom
- **Scoring modes**: Traditional Ad (Advantage) scoring and No-Ad (sudden death at deuce)
- **Final set options**: With or without tiebreak
- **Undo functionality**: Full point history with undo support
- **FFI-ready**: Compiles to `staticlib` for C interop

## Usage

```rust
use tennis_scorer::{MatchConfig, MatchState, MatchWithHistory, Player};

// Create a match with default config (Best-of-3, 7-point tiebreak, Ad scoring)
let config = MatchConfig::default();
let match_with_history = MatchWithHistory::new(MatchState::new(config));

// Score points
let m = match_with_history.score_point(Player::Player1);
let m = m.score_point(Player::Player1);
let m = m.score_point(Player::Player1);
let m = m.score_point(Player::Player1); // Player 1 wins the game

// Undo last point
let m = m.undo();

// Check match state
if let Some(winner) = m.current().winner() {
    println!("Match winner: {:?}", winner);
}
```

## Configuration

```rust
let config = MatchConfig {
    sets_to_win: 3,           // Best-of-5
    tiebreak_points: 10,      // Super tiebreak
    final_set_tiebreak: false, // No tiebreak in final set
    no_ad_scoring: true,      // Sudden death at deuce
};
```

## Module Structure

```
src/
├── lib.rs          # Public API exports
├── types.rs        # Player, Point enums
├── config.rs       # MatchConfig
├── game.rs         # Game-level scoring
├── tiebreak.rs     # Tiebreak scoring
├── set.rs          # Set-level scoring
├── match_state.rs  # Match-level scoring
└── history.rs      # MatchWithHistory with undo
```

## Building

```bash
cargo build --release
```

The static library will be at `target/release/libtennis_scorer.a`.

## Testing

```bash
cargo test
```

## License

MIT
