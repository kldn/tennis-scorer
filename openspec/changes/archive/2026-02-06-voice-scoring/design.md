## Context

目前 watchOS app 的記分方式是透過點擊「我」和「對手」按鈕。在實際打球時，球員雙手忙碌，操作小按鈕不方便。此變更新增語音輸入功能作為替代記分方式。

現有架構：
- `ContentView.swift` - SwiftUI view，包含分數顯示和按鈕
- `TennisMatch.swift` - `@MainActor ObservableObject`，封裝 Rust FFI，提供 `scorePoint(player:)` 和 `undo()`
- 按鈕操作已使用 `WKInterfaceDevice.current().play(.click)` 提供觸覺回饋

## Goals / Non-Goals

**Goals:**
- 提供按鈕觸發的語音記分功能
- 使用 on-device 辨識，不依賴網路
- 支援中文關鍵字：「我」、「對手」、「取消」
- 清楚的狀態指示與回饋

**Non-Goals:**
- 不支援持續聆聽模式（耗電）
- 不支援自定義關鍵字
- 不支援英文或其他語言（目前 UI 為中文）
- 不修改 Rust 核心或 FFI 層

## Decisions

### D1: 使用 SFSpeechRecognizer 搭配 AVAudioEngine

**選擇**：使用 Apple 原生的 `SFSpeechRecognizer`（locale: `zh-Hant`）配合 `AVAudioEngine` 進行即時語音辨識。

**理由**：
- watchOS 原生支援，無需第三方依賴
- watchOS 10+ 支援 on-device recognition，不需要網路
- `SFSpeechRecognizer` 提供即時的 partial results，可以在說完關鍵字的瞬間就觸發動作

**替代方案**：
- 使用 `INVoiceShortcut` / Siri Intent → 需要使用者預先設定，操作繁瑣
- 使用第三方語音辨識 SDK → 增加依賴且可能不支援 watchOS

### D2: 按鈕觸發模式（非持續聆聽）

**選擇**：使用者必須先點擊麥克風按鈕才開始辨識，辨識完成或 timeout 後自動停止。

**理由**：
- 大幅節省電力，Apple Watch 電池有限
- 避免環境噪音導致誤觸發
- 使用者明確知道何時系統在聆聽

**替代方案**：
- 持續聆聽模式 → 電力消耗過大，不適合 watchOS
- 抬腕觸發 → 容易在揮拍時誤觸發

### D3: 辨識 timeout 策略

**選擇**：開始聆聽後，若約 2 秒內無語音輸入，自動停止聆聽。有語音輸入時 timeout 重置。

**理由**：
- 2 秒足夠說出任何一個關鍵字（「我」、「對手」、「取消」都是短詞）
- 使用 `SFSpeechAudioBufferRecognitionRequest` 的即時結果來判斷是否有語音活動
- 自動停止避免使用者忘記關閉

**實作方式**：
- 使用 `Task.sleep` 配合 cancellation 實現 timeout
- 每次收到 partial result 時取消並重建 timeout task

### D4: 狀態管理

**選擇**：使用 enum 管理語音辨識狀態：

```swift
enum SpeechState {
    case idle                    // 未聆聽
    case listening               // 正在聆聽
    case processing              // 辨識到語音，處理中
    case result(ScoringAction)   // 辨識成功，顯示結果
    case error                   // 辨識失敗
}

enum ScoringAction {
    case player1Point   // 辨識到「我」
    case player2Point   // 辨識到「對手」
    case undo           // 辨識到「取消」
}
```

**理由**：
- 明確的狀態機，避免非法狀態
- UI 可以根據狀態顯示對應的視覺回饋
- 與 SwiftUI 的 `@Published` 配合良好

### D5: UI 放置位置

**選擇**：將麥克風按鈕放在「我」和「對手」按鈕的下方，與 undo 按鈕同一行或相鄰。

**理由**：
- 不破壞現有的分數按鈕佈局
- 麥克風是輔助功能，不應比主要按鈕更顯眼
- 在聆聽狀態下，麥克風圖示可以用動畫（脈衝效果）表示正在聆聽

### D6: SpeechRecognizer 類別設計

**選擇**：新建 `SpeechRecognizer` 作為 `ObservableObject`，封裝所有語音辨識邏輯。

```swift
@MainActor
class SpeechRecognizer: ObservableObject {
    @Published private(set) var state: SpeechState = .idle

    func toggleListening()    // 開始或停止聆聽
    func checkPermissions() -> Bool

    private var speechRecognizer: SFSpeechRecognizer?
    private var audioEngine: AVAudioEngine
    private var recognitionTask: SFSpeechRecognitionTask?
    private var timeoutTask: Task<Void, Never>?
}
```

**理由**：
- 職責分離：`TennisMatch` 負責計分邏輯，`SpeechRecognizer` 負責語音辨識
- `ContentView` 同時觀察兩個 `ObservableObject`，根據辨識結果呼叫 `TennisMatch` 的方法

### D7: 關鍵字匹配邏輯

**選擇**：從 partial recognition results 中搜尋關鍵字，使用 `String.contains` 匹配。

**理由**：
- SFSpeechRecognizer 的 partial results 可能包含多個候選詞
- 「我」是單字，「對手」和「取消」是雙字詞，不太容易互相混淆
- 優先匹配較長的關鍵字（「對手」和「取消」先於「我」），避免「對手」被拆成包含「我」的誤判

## Risks / Trade-offs

- **[風險] watchOS 10+ 限制** → 此功能需要 watchOS 10 以上才支援 on-device 中文語音辨識。需在 Info.plist 或程式碼中做版本檢查。
- **[風險] 中文辨識準確度** → 「我」只有一個字，在嘈雜環境可能辨識率較低。可考慮未來增加替代關鍵字。
- **[取捨] 按鈕觸發 vs 持續聆聽** → 選擇按鈕觸發犧牲了一些便利性，但大幅節省電力。
- **[取捨] 僅支援中文** → 目前 UI 為中文，暫不考慮多語言，未來可擴展。
