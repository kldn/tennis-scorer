## 1. Rust Cross-Compile 設定

- [ ] 1.1 確認 Rust nightly 安裝（watchOS target 需要）
- [ ] 1.2 安裝 watchOS targets：`aarch64-apple-watchos`、`aarch64-apple-watchos-sim`
- [ ] 1.3 建立 `.cargo/config.toml` 設定 watchOS linker 參數
- [ ] 1.4 測試編譯 `cargo build --target aarch64-apple-watchos-sim`
- [ ] 1.5 建立 build script 自動化 cross-compile

## 2. Xcode 專案建立

- [ ] 2.1 使用 Xcode 建立新專案：watchOS App（SwiftUI）
- [ ] 2.2 設定 Deployment Target 為 watchOS 10.0
- [ ] 2.3 建立 `Bridge/` 目錄，複製 `tennis_scorer.h`
- [ ] 2.4 建立 `module.modulemap` 讓 Swift import C header
- [ ] 2.5 建立 `Libraries/` 目錄放置 `.a` 檔
- [ ] 2.6 設定 Build Settings：Header Search Paths、Library Search Paths
- [ ] 2.7 設定 Build Phases：Link Binary With Libraries 加入 `libtennis_scorer.a`
- [ ] 2.8 驗證專案可成功 build（先用空 UI）

## 3. Swift Wrapper（TennisMatch.swift）

- [ ] 3.1 定義 `Player` Swift enum（`.player1`, `.player2`）
- [ ] 3.2 定義 `Score` Swift struct（包含所有比分資訊）
- [ ] 3.3 建立 `TennisMatch` class，conform to `ObservableObject`
- [ ] 3.4 實作 `init()` 呼叫 `tennis_match_new_default()`
- [ ] 3.5 實作 `init(config:)` 呼叫 `tennis_match_new_custom()`
- [ ] 3.6 實作 `deinit` 呼叫 `tennis_match_free()`
- [ ] 3.7 實作 `scorePoint(player:)` 呼叫 FFI 並更新 `@Published score`
- [ ] 3.8 實作 `undo()` 呼叫 FFI 並更新 score
- [ ] 3.9 實作 `canUndo` computed property
- [ ] 3.10 實作 private `updateScore()` 從 FFI 讀取 MatchScore 轉換為 Swift Score
- [ ] 3.11 撰寫 Swift unit tests 驗證 wrapper

## 4. 主畫面 UI（ContentView.swift）

- [ ] 4.1 建立 `ScoreView` 顯示比分（sets、games、points）
- [ ] 4.2 實作分數顯示邏輯：一般分數、Deuce、Advantage、Tiebreak
- [ ] 4.3 建立兩個計分按鈕（P1、P2）
- [ ] 4.4 按鈕加入 haptic feedback（`WKInterfaceDevice.current().play(.click)`）
- [ ] 4.5 建立 Undo 按鈕（disable when `!canUndo`）
- [ ] 4.6 實作 Undo 確認機制（長按或 alert）
- [ ] 4.7 建立 match 結束畫面（顯示 winner、New Match 按鈕）
- [ ] 4.8 調整字體大小適應 Apple Watch 螢幕

## 5. 設定畫面（SettingsView.swift）

- [ ] 5.1 建立 `SettingsView` 頁面
- [ ] 5.2 加入賽制選擇：Best-of-3 / Best-of-5
- [ ] 5.3 加入 Scoring 模式選擇：Ad / No-Ad
- [ ] 5.4 加入 Tiebreak 分數選擇：7 / 10
- [ ] 5.5 使用 `@AppStorage` 持久化設定
- [ ] 5.6 在 ContentView 加入設定入口（gear icon）

## 6. 語音辨識（SpeechRecognizer.swift）

- [ ] 6.1 建立 `SpeechRecognizer` class，conform to `ObservableObject`
- [ ] 6.2 加入 `NSSpeechRecognitionUsageDescription` 到 Info.plist
- [ ] 6.3 加入 `NSMicrophoneUsageDescription` 到 Info.plist
- [ ] 6.4 實作權限請求邏輯
- [ ] 6.5 實作 `startListening()` 開始辨識
- [ ] 6.6 實作 `stopListening()` 停止辨識並處理結果
- [ ] 6.7 實作關鍵詞比對：player one/two、undo
- [ ] 6.8 加入麥克風按鈕到 UI（push-to-talk 模式）
- [ ] 6.9 實作辨識成功/失敗的 haptic 和視覺回饋
- [ ] 6.10 處理權限被拒的情況（隱藏語音功能）

## 7. 整合與測試

- [ ] 7.1 在 Simulator 測試完整流程
- [ ] 7.2 測試一場完整的 match（含 tiebreak）
- [ ] 7.3 測試 Undo 功能
- [ ] 7.4 測試語音辨識功能
- [ ] 7.5 在實機測試（如果有開發者帳號）
- [ ] 7.6 調整 UI 細節（間距、顏色、動畫）

## 8. 打包與部署

- [ ] 8.1 設定 app icon
- [ ] 8.2 設定 app display name
- [ ] 8.3 建立 Archive 並測試 ad-hoc 部署
