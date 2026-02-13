## ADDED Requirements

### Requirement: FCM token registration on app launch
The phone app SHALL register its FCM device token with the backend on each app launch.

#### Scenario: Successful token registration
- **WHEN** the app launches and the user is authenticated
- **THEN** the app SHALL obtain an FCM token via Firebase Messaging, determine the device_type (phone_android or phone_ios), and call `POST /api/notifications/register`

#### Scenario: Token refresh
- **WHEN** the FCM token is refreshed by Firebase
- **THEN** the app SHALL re-register the new token with the backend

### Requirement: Display push notifications
The phone app SHALL display incoming push notifications appropriately.

#### Scenario: App in background
- **WHEN** a push notification arrives while the app is in the background
- **THEN** the system notification tray SHALL display the notification with title and body

#### Scenario: App in foreground
- **WHEN** a push notification arrives while the app is in the foreground
- **THEN** the app SHALL display an in-app notification banner

#### Scenario: Notification tap navigation
- **WHEN** the user taps a push notification
- **THEN** the app SHALL navigate to the relevant screen (e.g., Social screen for friend requests, Match detail for match results)

### Requirement: Notification permissions
The phone app SHALL request notification permissions from the user.

#### Scenario: First launch permission request
- **WHEN** the user launches the app for the first time after authentication
- **THEN** the app SHALL request push notification permissions from the OS

#### Scenario: Permission denied
- **WHEN** the user denies notification permissions
- **THEN** the app SHALL continue to function normally without push notifications
