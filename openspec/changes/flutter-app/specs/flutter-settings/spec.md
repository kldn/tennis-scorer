## ADDED Requirements

### Requirement: Settings screen displays user profile
The phone app SHALL provide a Settings screen with user account information.

#### Scenario: View profile info
- **WHEN** the user opens the Settings screen
- **THEN** the app SHALL display the user's display_name, email, and avatar from the cached user data

### Requirement: Notification preferences
The phone app SHALL allow users to manage push notification preferences.

#### Scenario: View notification settings
- **WHEN** the user opens the notification preferences section
- **THEN** the app SHALL display toggles for friend_requests, match_results, and match_claims notifications

#### Scenario: Update notification setting
- **WHEN** the user toggles a notification preference
- **THEN** the app SHALL call `PUT /api/notifications/settings` with the updated preferences

### Requirement: Sign out
The phone app SHALL allow users to sign out.

#### Scenario: Successful sign out
- **WHEN** the user taps "Sign Out"
- **THEN** the app SHALL sign out from Firebase Auth, clear local state, and navigate to the login screen

#### Scenario: Sign out confirmation
- **WHEN** the user taps "Sign Out"
- **THEN** the app SHALL show a confirmation dialog before proceeding
