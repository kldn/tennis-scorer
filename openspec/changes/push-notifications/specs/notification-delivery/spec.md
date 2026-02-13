## ADDED Requirements

### Requirement: Send push notification via FCM HTTP v1 API
The system SHALL send push notifications to users through Firebase Cloud Messaging HTTP v1 API.

#### Scenario: User has registered devices
- **WHEN** `send_push(user_id, title, body, data)` is called and the user has one or more registered device tokens
- **THEN** the system SHALL send an FCM message to each registered device token via `POST https://fcm.googleapis.com/v1/projects/{project_id}/messages:send`

#### Scenario: User has no registered devices
- **WHEN** `send_push(user_id, title, body, data)` is called and the user has no registered device tokens
- **THEN** the system SHALL silently skip sending (no error)

#### Scenario: Multiple devices
- **WHEN** a user has tokens for multiple device types (e.g., watch_apple and phone_ios)
- **THEN** the system SHALL send the notification to ALL registered devices

### Requirement: FCM OAuth 2.0 authentication
The system SHALL authenticate with FCM HTTP v1 API using OAuth 2.0 access tokens derived from a Firebase Service Account.

#### Scenario: Access token generation
- **WHEN** an FCM API call is needed and no valid access token exists
- **THEN** the system SHALL generate a JWT from the service account key, exchange it for an OAuth 2.0 access token via Google's token endpoint, and cache the token

#### Scenario: Access token caching
- **WHEN** a cached OAuth 2.0 access token exists and is not expired
- **THEN** the system SHALL reuse the cached token without requesting a new one

#### Scenario: Access token refresh
- **WHEN** the cached access token is expired or within 5 minutes of expiry
- **THEN** the system SHALL request a new access token before sending the notification

### Requirement: Invalid token cleanup
The system SHALL remove invalid FCM device tokens when FCM reports them as unregistered.

#### Scenario: FCM returns 404 for a token
- **WHEN** FCM API returns a NOT_FOUND/UNREGISTERED error for a device token
- **THEN** the system SHALL delete the corresponding device_tokens record

#### Scenario: FCM returns other errors
- **WHEN** FCM API returns a transient error (e.g., 500, 503)
- **THEN** the system SHALL log the error but NOT delete the device token

### Requirement: Notification delivery is fire-and-forget
The system SHALL not block the main request flow on notification delivery.

#### Scenario: Notification failure does not affect API response
- **WHEN** a notification send fails (FCM error, network timeout)
- **THEN** the system SHALL log the failure but the triggering API operation SHALL still succeed
