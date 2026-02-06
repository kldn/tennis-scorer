## 1. Permission Setup

- [x] 1.1 Add `NSSpeechRecognitionUsageDescription` to Info.plist with Chinese description
- [x] 1.2 Add `NSMicrophoneUsageDescription` to Info.plist with Chinese description
- [x] 1.3 Implement permission check and request flow in SpeechRecognizer.checkPermissions()
- [x] 1.4 Handle permission denied state: show message and disable mic button

## 2. Speech Recognition Service

- [x] 2.1 Create `SpeechRecognizer.swift` with `SpeechState` enum and `ScoringAction` enum
- [x] 2.2 Initialize `SFSpeechRecognizer` with locale `zh-Hant` and `requiresOnDeviceRecognition = true`
- [x] 2.3 Implement `startListening()`: configure AVAudioEngine, install tap, create recognition request
- [x] 2.4 Implement `stopListening()`: stop audio engine, cancel recognition task, clean up
- [x] 2.5 Implement keyword matching from partial results ("對手" and "取消" matched before "我")
- [x] 2.6 Implement ~2 second silence timeout using Task.sleep with cancellation
- [x] 2.7 Implement `toggleListening()` to switch between idle and listening states
- [x] 2.8 Handle recognition errors and update state accordingly

## 3. UI Changes

- [x] 3.1 Add `@StateObject` for `SpeechRecognizer` in ContentView
- [x] 3.2 Add microphone button below the player score buttons with mic.fill SF Symbol
- [x] 3.3 Add listening state indicator: pulsing animation on microphone icon when state is .listening
- [x] 3.4 Add brief result feedback: show recognized action text before returning to idle
- [x] 3.5 Hide or disable mic button when on-device recognition is unavailable
- [x] 3.6 Hide mic button on match complete view

## 4. Integration with TennisMatch

- [x] 4.1 Wire SpeechRecognizer results to TennisMatch.scorePoint(player:) and TennisMatch.undo()
- [x] 4.2 Add haptic feedback: .success for recognized keyword, .failure for timeout/error
- [x] 4.3 Check canUndo before executing undo from voice; play error haptic if unavailable
- [x] 4.4 Ensure UI updates after voice-triggered scoring (score display, undo button state)

## 5. Tests

- [x] 5.1 Unit test keyword matching logic: "我" maps to .player1Point, "對手" maps to .player2Point, "取消" maps to .undo
- [x] 5.2 Unit test that longer keywords ("對手", "取消") are matched before "我"
- [x] 5.3 Unit test state transitions: idle -> listening -> processing -> result -> idle
- [x] 5.4 Unit test timeout behavior: state returns to idle after silence
- [x] 5.5 Build watchOS app to verify new UI elements compile and render correctly
