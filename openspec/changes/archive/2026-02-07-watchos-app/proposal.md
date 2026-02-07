## Why

網球比賽時，球員或裁判需要快速記錄得分。手機放口袋不方便，但 Apple Watch 戴在手腕上可以隨時操作。結合語音輸入，甚至不需要看螢幕就能記分。core-scoring 引擎和 FFI bridge 已經完成，現在需要一個 watchOS app 來呈現 UI 並接受使用者輸入。

## What Changes

- 建立 Xcode 專案，包含 watchOS app target
- 將 Rust static library 整合到 Xcode build
- 建立 Swift wrapper 封裝 C FFI 呼叫
- 實作 SwiftUI 介面顯示比分
- 實作語音辨識功能（"Player one" / "Player two"）
- 實作 Digital Crown 與觸控操作

## Capabilities

### New Capabilities

- `swift-wrapper`: Swift class 封裝 FFI 呼叫，提供 Swift-friendly API
- `watch-ui`: SwiftUI 介面，顯示比分、提供計分按鈕、支援 Digital Crown
- `voice-input`: 語音辨識輸入，辨識 "Player one" / "Player two" 指令

### Modified Capabilities

（無）

## Impact

- **New Files**: Xcode 專案結構（WatchApp/）
- **Build**: 需要整合 Rust cross-compile for watchOS
- **Dependencies**: Rust toolchain + Xcode
- **Platforms**: watchOS 10.0+（支援最新 SwiftUI 功能）
