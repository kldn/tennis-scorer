## MODIFIED Requirements

### Requirement: Permission handling
The system SHALL request microphone and speech recognition permissions before use.
When permission is denied, the system SHALL display an alert that can be properly dismissed by the user.
The alert SHALL use a mutable `@State` binding driven by `onChange(of: speechRecognizer.permissionDenied)`, not `.constant()`.

#### Scenario: Permission denied alert dismisses correctly
- **WHEN** speech recognition permission is denied
- **AND** the permission alert is shown
- **AND** the user taps "好"
- **THEN** the alert dismisses and does not reappear until permissionDenied changes again

### Requirement: Silence timeout
The system SHALL stop listening after approximately 2 seconds of silence.
The idle return task SHALL be stored as a cancellable property and capture `self` weakly.
Previous idle tasks SHALL be cancelled before scheduling a new one.
The idle task SHALL also be cancelled in `stopListening()`.

#### Scenario: Idle task cancellation on new recognition
- **WHEN** an idle return task is scheduled
- **AND** a new speech recognition session starts
- **THEN** the previous idle task is cancelled before the new session begins

#### Scenario: No retain cycle from idle task
- **WHEN** an idle return task is scheduled
- **AND** the SpeechRecognizer is deallocated before the task completes
- **THEN** the task does not prevent deallocation (weak self reference)
