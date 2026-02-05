## Context

已有 Rust library（`libtennis_scorer.a`）和 C header（`tennis_scorer.h`）。需要建立 watchOS app 來使用這個 library。watchOS 開發使用 Swift + SwiftUI，需要透過 C interop 呼叫 Rust FFI。

## Goals / Non-Goals

**Goals:**

- 建立可在 Apple Watch 上運行的計分 app
- 提供清晰的比分顯示（sets、games、points）
- 支援觸控按鈕計分
- 支援語音指令計分（"Player one" / "Player two"）
- 支援 Undo 功能
- 流暢的 60fps UI 體驗

**Non-Goals:**

- iPhone companion app（可後續加入）
- 雲端同步、歷史記錄
- 複雜的統計分析
- 多語言支援（先做英文）

## Decisions

### 1. Xcode 專案結構

```
WatchApp/
├── TennisScorer.xcodeproj
├── TennisScorer Watch App/
│   ├── TennisScorerApp.swift      # App entry point
│   ├── ContentView.swift          # Main UI
│   ├── ScoreView.swift            # Score display component
│   ├── TennisMatch.swift          # Swift wrapper for FFI
│   └── SpeechRecognizer.swift     # Voice input handler
├── Bridge/
│   ├── tennis_scorer.h            # Copy from include/
│   └── module.modulemap           # For Swift import
└── Libraries/
    └── libtennis_scorer.a         # Cross-compiled for watchOS
```

### 2. Cross-compile Rust for watchOS

需要為 watchOS 編譯 Rust library：

```bash
# 安裝 target
rustup target add aarch64-apple-watchos

# 編譯
cargo build --release --target aarch64-apple-watchos
```

**注意**：watchOS target 目前是 Tier 3，可能需要 nightly Rust 或額外設定。如果有問題，備案是使用 watchOS simulator target 先開發。

### 3. Swift Wrapper 設計

使用 class 封裝 FFI，提供 Swift-friendly API：

```swift
class TennisMatch: ObservableObject {
    @Published private(set) var score: MatchScore

    private var handle: OpaquePointer?

    init(config: MatchConfig = .default) { ... }
    deinit { tennis_match_free(handle) }

    func scorePoint(player: Player) { ... }
    func undo() { ... }
}
```

使用 `ObservableObject` + `@Published` 讓 SwiftUI 自動更新。

### 4. UI 設計

**主畫面佈局：**
```
┌─────────────────────┐
│    Sets: 1 - 0      │  ← 小字
│   Games: 5 - 4      │  ← 中字
│                     │
│      40 - 30        │  ← 大字，當前 game 分數
│                     │
│  [P1]        [P2]   │  ← 兩個計分按鈕
│                     │
│      [Undo]         │  ← 小按鈕
└─────────────────────┘
```

- 使用 SF Symbols（`person.fill`）作為 Player 圖示
- Digital Crown 可用於選擇 player（轉動切換 focus）
- 長按 Undo 確認（防止誤觸）

### 5. 語音辨識

使用 Apple Speech framework：

```swift
import Speech

class SpeechRecognizer: ObservableObject {
    func startListening() { ... }
    func stopListening() { ... }
}
```

辨識關鍵詞：
- "Player one" / "One" / "First" → Player 1 得分
- "Player two" / "Two" / "Second" → Player 2 得分
- "Undo" / "Cancel" → 回退

**權限**：需要在 Info.plist 加入 `NSSpeechRecognitionUsageDescription`。

### 6. 狀態管理

使用 SwiftUI 的 `@StateObject` 管理：

```swift
@main
struct TennisScorerApp: App {
    @StateObject private var match = TennisMatch()
    @StateObject private var speech = SpeechRecognizer()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(match)
                .environmentObject(speech)
        }
    }
}
```

## Risks / Trade-offs

**Rust watchOS cross-compile** → Tier 3 target，可能不穩定。備案：先用 simulator 開發，或考慮將 scoring logic 用 Swift 重寫（但這樣就失去 cross-platform 優勢）。

**語音辨識準確度** → 球場環境嘈雜，辨識可能不準。設計上以觸控為主，語音為輔。

**電池消耗** → 持續語音辨識很耗電。使用 push-to-talk 模式，按住說話再放開。

**螢幕大小** → Apple Watch 螢幕小，資訊要精簡。優先顯示當前 game 分數，sets/games 用小字。
