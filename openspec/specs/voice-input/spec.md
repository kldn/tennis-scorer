## ADDED Requirements

### Requirement: Microphone button in UI

The watchOS app SHALL display a microphone button in the active match view.

#### Scenario: Microphone button visible during active match
- **WHEN** a match is in progress (no winner yet)
- **THEN** the UI SHALL display a microphone button with an SF Symbol icon (mic.fill)
- **AND** the button SHALL be visually distinct from the player score buttons

#### Scenario: Microphone button hidden after match ends
- **WHEN** the match has a winner
- **THEN** the microphone button SHALL NOT be displayed

### Requirement: Button-triggered speech recognition

The app SHALL activate speech recognition only when the user taps the microphone button. The app MUST NOT use continuous listening.

#### Scenario: Start listening on button tap
- **WHEN** the user taps the microphone button
- **AND** microphone and speech recognition permissions are granted
- **THEN** the app SHALL start capturing audio via AVAudioEngine
- **AND** the app SHALL begin real-time speech recognition via SFSpeechRecognizer
- **AND** the UI SHALL indicate the listening state (e.g., pulsing microphone icon)

#### Scenario: Stop listening on second tap
- **WHEN** the app is in the listening state
- **AND** the user taps the microphone button again
- **THEN** the app SHALL stop capturing audio and speech recognition
- **AND** the UI SHALL return to the idle state

### Requirement: Keyword recognition

The speech recognizer SHALL recognize Chinese keywords and map them to scoring actions.

#### Scenario: Recognize "我" as Player 1 point
- **WHEN** the app is listening
- **AND** the user says "我"
- **THEN** the app SHALL score a point for Player 1
- **AND** the app SHALL stop listening

#### Scenario: Recognize "對手" as Player 2 point
- **WHEN** the app is listening
- **AND** the user says "對手"
- **THEN** the app SHALL score a point for Player 2
- **AND** the app SHALL stop listening

#### Scenario: Recognize "取消" as undo
- **WHEN** the app is listening
- **AND** the user says "取消"
- **AND** undo is available (canUndo is true)
- **THEN** the app SHALL perform an undo action
- **AND** the app SHALL stop listening

#### Scenario: Undo unavailable when "取消" is spoken
- **WHEN** the app is listening
- **AND** the user says "取消"
- **AND** undo is NOT available (canUndo is false)
- **THEN** the app SHALL stop listening
- **AND** the app SHALL provide an error haptic feedback

### Requirement: Haptic and visual feedback

The app SHALL provide haptic and visual feedback upon recognition.

#### Scenario: Successful recognition feedback
- **WHEN** a keyword is successfully recognized and the action is performed
- **THEN** the app SHALL play a success haptic (WKInterfaceDevice .success)
- **AND** the UI SHALL briefly indicate which action was taken

#### Scenario: Failed recognition feedback
- **WHEN** listening ends without recognizing a valid keyword
- **THEN** the app SHALL play a failure haptic (WKInterfaceDevice .failure)
- **AND** the UI SHALL return to idle state

### Requirement: Silence timeout

The app SHALL stop listening after a period of silence to conserve battery.

#### Scenario: Timeout after silence
- **WHEN** the app is listening
- **AND** no speech is detected for approximately 2 seconds
- **THEN** the app SHALL stop listening
- **AND** the app SHALL play a failure haptic if no keyword was recognized

#### Scenario: Timeout resets on speech detection
- **WHEN** the app is listening
- **AND** partial speech is detected
- **THEN** the silence timeout SHALL reset

### Requirement: Permission handling

The app MUST request microphone and speech recognition permissions before activating voice input.

#### Scenario: First-time permission request
- **WHEN** the user taps the microphone button for the first time
- **AND** permissions have not been previously granted or denied
- **THEN** the app SHALL request both microphone and speech recognition permissions
- **AND** the app SHALL only proceed with voice recognition if both permissions are granted

#### Scenario: Permission previously denied
- **WHEN** the user taps the microphone button
- **AND** either microphone or speech recognition permission is denied
- **THEN** the app SHALL NOT start listening
- **AND** the app SHALL display a brief message indicating that permissions are required

### Requirement: On-device recognition

The app MUST use on-device speech recognition to avoid network dependency.

#### Scenario: On-device recognition available
- **WHEN** the app initializes SFSpeechRecognizer with locale zh-Hant
- **THEN** the app MUST set `requiresOnDeviceRecognition = true` on the recognition request
- **AND** speech recognition SHALL function without network connectivity

#### Scenario: On-device recognition unavailable
- **WHEN** the device does not support on-device recognition for zh-Hant
- **THEN** the microphone button SHALL be hidden or disabled
- **AND** the app SHALL NOT attempt to use network-based recognition as fallback

### Requirement: Error handling

The app SHALL handle speech recognition errors gracefully.

#### Scenario: Recognition engine error
- **WHEN** the speech recognition engine returns an error during recognition
- **THEN** the app SHALL stop listening
- **AND** the app SHALL play an error haptic
- **AND** the UI SHALL return to idle state

#### Scenario: Audio engine failure
- **WHEN** AVAudioEngine fails to start or encounters an error
- **THEN** the app SHALL NOT enter the listening state
- **AND** the app SHALL play an error haptic
