## Why

在網球比賽中，球員的雙手通常忙於握拍、擦汗、撿球，很難騰出手來點擊 Apple Watch 上的小按鈕記分。語音記分讓使用者只需點擊麥克風按鈕，然後說出關鍵字即可完成記分，大幅降低操作門檻。

此外，watchOS 螢幕小，按鈕容易誤觸。語音輸入提供了一種更自然、更不易出錯的替代操作方式。

## What Changes

- 在 ContentView 新增麥克風按鈕，觸發語音辨識
- 新增 `SpeechRecognizer` 類別，封裝 Apple Speech framework
- 支援三個中文關鍵字辨識：「我」記 Player 1 得分、「對手」記 Player 2 得分、「取消」執行 undo
- 辨識成功後提供觸覺回饋（haptic feedback）
- 使用 on-device 辨識，不需要網路連線（watchOS 10+）
- 按鈕觸發模式（非持續聆聽），節省電力
- 處理麥克風與語音辨識權限請求

## Capabilities

### New Capabilities

- `voice-input`: 透過語音關鍵字辨識來記分，支援按鈕觸發的語音輸入模式

### Modified Capabilities

無。此功能為純新增，不修改既有的手動按鈕記分功能。

## Impact

- **watchOS UI**：`ContentView.swift` - 新增麥克風按鈕與聆聽狀態指示器
- **語音辨識**：新增 `SpeechRecognizer.swift` - 封裝 SFSpeechRecognizer 與 AVAudioEngine
- **權限設定**：`Info.plist` - 新增麥克風與語音辨識使用說明
- **整合層**：`ContentView.swift` - 將語音辨識結果連接到 TennisMatch 的 scorePoint/undo
