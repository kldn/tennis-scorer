## Context

The scoring engine uses immutable state transitions: `GameState`, `SetState`, `MatchState`, and `MatchWithHistory`. Scoring flows top-down from `MatchState::score_point()` through `SetState` and into `GameState` or `TiebreakState`. The `Player` enum has two variants (`Player1`, `Player2`) representing the two sides.

In doubles, the two sides still score points identically. The only new concept is tracking which of 4 individual players is serving, and rotating that across games and tiebreak points.

## Goals / Non-Goals

**Goals:**
- Add match type (Singles/Doubles) to configuration
- Track a 4-player serve rotation order
- Rotate server correctly on game completion and during tiebreaks
- Expose current server through FFI

**Non-Goals:**
- No individual player statistics (points won per individual player)
- No receiver tracking (only server rotation)
- No mixed doubles special rules
- No team name or individual player name support (out of scope)
- No changes to the watchOS UI in this change (UI updates are a separate change)

## Decisions

### D1: Add MatchType enum to MatchConfig

**Choice**: Add a `MatchType` enum (`Singles`, `Doubles`) to `MatchConfig` with a `serve_order` field.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchType {
    Singles,
    Doubles,
}

pub struct MatchConfig {
    pub sets_to_win: u8,
    pub tiebreak_points: u8,
    pub final_set_tiebreak: bool,
    pub no_ad_scoring: bool,
    pub match_type: MatchType,
    pub serve_order: Vec<(Player, u8)>, // (team, player_index_within_team)
}
```

**Rationale**:
- Keeps config declarative and serializable
- `serve_order` is a Vec of 4 entries for doubles, empty for singles
- The `Player` in the tuple identifies the team (Player1 = Team 1, Player2 = Team 2)
- The `u8` distinguishes the two players within a team (0 or 1)
- Default remains Singles with empty serve_order, preserving backward compatibility

**Alternative considered**:
- Separate `DoublesConfig` struct -- adds unnecessary complexity when it is just two fields

### D2: Track serve position in MatchState

**Choice**: Add a `serve_rotation_index: usize` field to `MatchState::Playing`.

```rust
MatchState::Playing {
    sets: Vec<SetState>,
    player1_sets: u8,
    player2_sets: u8,
    config: MatchConfig,
    serve_rotation_index: usize, // 0..3, index into config.serve_order
}
```

**Rationale**:
- Simple index into the serve_order vec in config
- Incremented (mod serve_order.len()) when a game completes
- For singles matches, this field is unused (serve alternation can be derived from game count, or simply ignored since the existing code does not track server at all)

**Alternative considered**:
- Store server in each `SetState` or `GameState` -- would require threading serve info through the entire state tree, violating the principle that game/set scoring is the same for singles and doubles

### D3: Tiebreak serve rotation

**Choice**: Add a `tiebreak_serve_index: usize` and `tiebreak_points_served: u8` to `MatchState::Playing` (only meaningful during tiebreaks).

**Rationale**:
- In a tiebreak, the first server serves 1 point, then each subsequent server serves 2 points
- Tracking this at the `MatchState` level (not inside `TiebreakState`) keeps the tiebreak scoring logic unchanged
- When a tiebreak starts, `tiebreak_serve_index` is set to `serve_rotation_index`
- After tiebreak point 1: advance index; then advance every 2 points
- When tiebreak ends, `serve_rotation_index` is updated to the next position after `tiebreak_serve_index`

**Alternative considered**:
- Put serve tracking inside `TiebreakState` -- would break the clean separation where `TiebreakState` only knows about points, not about who is serving

### D4: Keep Player enum unchanged

**Choice**: Do not modify the `Player` enum. `Player::Player1` represents Team 1, `Player::Player2` represents Team 2.

**Rationale**:
- All scoring logic (game, set, match, tiebreak) operates on two sides
- Changing `Player` to a 4-player model would require rewriting every scoring function
- The serve order is the only place where individual identity within a team matters

### D5: FFI changes

**Choice**: Add `current_server: u8` (0-3) to `MatchScore` and a new constructor `tennis_match_new_doubles()`.

```c
// New FFI function
TennisMatch* tennis_match_new_doubles(
    uint8_t sets_to_win,
    uint8_t tiebreak_points,
    bool final_set_tiebreak,
    bool no_ad_scoring,
    uint8_t first_server_team,  // PLAYER_1 or PLAYER_2
);
```

**Rationale**:
- The serve order is always [Team1-PlayerA, Team2-PlayerA, Team1-PlayerB, Team2-PlayerB] (standard doubles rotation)
- A simplified constructor only needs to know which team serves first
- `current_server` in MatchScore tells the Swift layer who is serving (0-3 index)

## Risks / Trade-offs

- **[Risk] MatchState grows larger** -- Two new fields added. Acceptable since state is cloned on every point anyway and the fields are small (usize + u8).
- **[Risk] MatchConfig backward compatibility** -- Adding fields to MatchConfig changes its serialized form. Serde defaults can handle this: `match_type` defaults to Singles, `serve_order` defaults to empty vec.
- **[Trade-off] Serve tracking at MatchState level** -- Slightly inelegant that serve rotation lives in MatchState while game/set know nothing about it. But this is the right boundary: scoring is team-based, serving is individual-based.
