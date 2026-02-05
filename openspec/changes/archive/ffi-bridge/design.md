## Context

Rust 的型別系統（enums with data、Vec、Option）無法直接對應到 C。需要設計一個 C-friendly 的 API layer，同時保持記憶體安全。Swift 可以透過 C interop 呼叫這些函數。

## Goals / Non-Goals

**Goals:**

- 提供完整的計分功能給 Swift 呼叫
- 記憶體安全：明確的 create/destroy lifecycle
- 簡潔的 API：最小化必要的函數數量
- 產生可用的 C header file

**Non-Goals:**

- 不做 Swift wrapper（那是 watchos-app 的事）
- 不做 async/callback pattern（計分是同步操作）
- 不做序列化/反序列化（state 保持在記憶體中）

## Decisions

### 1. Opaque Pointer Pattern

使用 opaque pointer 隱藏 Rust 內部結構：

```c
typedef struct TennisMatch TennisMatch;

TennisMatch* tennis_match_new(void);
void tennis_match_free(TennisMatch* match);
```

Swift 端只看到指標，不需要知道內部結構。

### 2. 簡化的 Config API

提供 preset configs 而非完整的 config struct：

```c
TennisMatch* tennis_match_new_default(void);           // Best-of-3, Ad scoring
TennisMatch* tennis_match_new_best_of_5(void);         // Best-of-5, Ad scoring
TennisMatch* tennis_match_new_no_ad(void);             // Best-of-3, No-Ad
TennisMatch* tennis_match_new_custom(
    uint8_t sets_to_win,
    uint8_t tiebreak_points,
    bool final_set_tiebreak,
    bool no_ad_scoring
);
```

### 3. Player 用 uint8_t 表示

```c
#define PLAYER_1 1
#define PLAYER_2 2

void tennis_match_score_point(TennisMatch* match, uint8_t player);
```

### 4. 狀態查詢回傳 C struct

```c
typedef struct {
    uint8_t player1_sets;
    uint8_t player2_sets;
    uint8_t player1_games;  // current set
    uint8_t player2_games;  // current set
    uint8_t game_state;     // 0=playing, 1=deuce, 2=ad_p1, 3=ad_p2, 4=completed
    uint8_t player1_points; // for tiebreak or 0/15/30/40 encoded
    uint8_t player2_points;
    bool is_tiebreak;
    uint8_t winner;         // 0=none, 1=player1, 2=player2
} MatchScore;

MatchScore tennis_match_get_score(const TennisMatch* match);
```

### 5. Error Handling

大多數操作不會失敗。唯一需要處理的是 null pointer：

```c
// Returns false if match is null
bool tennis_match_score_point(TennisMatch* match, uint8_t player);
bool tennis_match_undo(TennisMatch* match);
```

### 6. Module 結構

```
src/
├── ffi.rs          # All FFI functions
└── ffi/
    └── types.rs    # C-compatible type definitions (if needed)
```

或者全部放在 `ffi.rs` 也可以，因為程式碼量不大。

## Risks / Trade-offs

**Opaque pointer 的記憶體管理** → 呼叫端必須記得呼叫 `_free()`。這是 C FFI 的標準 pattern，Swift 端可以包成 class 用 deinit 處理。

**狀態查詢的效能** → 每次查詢都會 copy 資料到 C struct。但計分 app 的查詢頻率很低（每次得分後），不是問題。

**簡化的 Point 表示** → 用數字 (0/15/30/40) 而非 enum，在 tiebreak 時就是實際分數。這讓 API 更簡單，但需要文件說明。
