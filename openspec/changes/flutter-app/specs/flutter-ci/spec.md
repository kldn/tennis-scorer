## ADDED Requirements

### Requirement: Flutter CI pipeline
The project SHALL have a GitHub Actions workflow (`flutter.yml`) that runs on pull requests and pushes affecting Flutter code.

#### Scenario: Trigger on Flutter code changes
- **WHEN** a PR or push modifies files under `flutter/phone_app/` or `flutter/wearos_app/`
- **THEN** the CI pipeline SHALL trigger automatically

#### Scenario: Flutter analyze
- **WHEN** the CI pipeline runs
- **THEN** the system SHALL run `flutter analyze` on both phone_app and wearos_app and fail the pipeline if any analysis issues are found

#### Scenario: Flutter test
- **WHEN** the CI pipeline runs
- **THEN** the system SHALL run `flutter test` on both phone_app and wearos_app and fail the pipeline if any tests fail

#### Scenario: Build APK
- **WHEN** the CI pipeline runs successfully
- **THEN** the system SHALL build a release APK for the phone_app (`flutter build apk --release`)

#### Scenario: Build iOS (optional)
- **WHEN** the CI pipeline runs on a macOS runner
- **THEN** the system SHALL build the iOS app (`flutter build ios --no-codesign`)

### Requirement: CI does not trigger on unrelated changes
The Flutter CI pipeline SHALL only run when Flutter code is affected.

#### Scenario: Rust-only changes
- **WHEN** a PR only modifies files under `crates/` or `migrations/`
- **THEN** the Flutter CI pipeline SHALL NOT trigger
