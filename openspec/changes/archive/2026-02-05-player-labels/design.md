## Context

目前 watchOS App 使用「P1」和「P2」作為玩家標籤，這是通用的設計，但對於個人追蹤自己比賽的使用場景不夠直覺。使用者需要記住自己是 P1 還是 P2。

現有架構：
- Rust 核心：使用 `Player::Player1` / `Player::Player2` enum
- FFI 層：使用 `PLAYER_1` / `PLAYER_2` 常數 (u8)
- Swift 層：使用 `Player.player1` / `Player.player2` enum
- UI 層：硬編碼「P1」/「P2」字串

## Goals / Non-Goals

**Goals:**
- 將 UI 標籤改為「我」和「對手」，更符合個人使用場景
- 保持所有底層邏輯不變

**Non-Goals:**
- 不增加可自訂標籤功能（未來可能做）
- 不修改 Rust 核心或 FFI 層
- 不做多語系支援（目前僅中文）

## Decisions

### D1: 只修改 UI 層

**選擇**：僅修改 `ContentView.swift` 中的字串

**理由**：
- 這是純顯示層變更，不影響任何邏輯
- Swift `Player` enum 和 Rust `Player` enum 應保持語義中立（player1/player2）
- 最小變更範圍，最低風險

**替代方案**：
- 在 Swift `Player` enum 加 `displayName` 屬性 → 過度設計
- 新增 Localizable.strings → 目前不需要多語系

### D2: 標籤選擇

**選擇**：
- P1 → 「我」
- P2 → 「對手」
- 「P1 Wins!」→「我贏了！」
- 「P2 Wins!」→「對手贏了」

**理由**：
- 簡潔明瞭
- 符合中文使用習慣

## Risks / Trade-offs

- **[風險] 未來多語系** → 目前硬編碼中文，未來若需多語系需重構。接受此風險，因為目前需求僅為中文。
- **[風險] 使用者可能想自訂** → 未來可加設定頁面，目前不做。
