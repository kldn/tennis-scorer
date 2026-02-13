## ADDED Requirements

### Requirement: Register FCM device token
The system SHALL allow an authenticated user to register their device's FCM token via `POST /api/notifications/register`.

#### Scenario: New device registration
- **WHEN** an authenticated user submits an FCM token and device_type for a device not previously registered
- **THEN** the system SHALL create a device_tokens record and return 200

#### Scenario: Update existing device token
- **WHEN** an authenticated user submits a new FCM token for a device_type that already has a registered token
- **THEN** the system SHALL update (UPSERT) the existing record's fcm_token and updated_at, and return 200

#### Scenario: Valid device types
- **WHEN** a registration request is submitted
- **THEN** the system SHALL only accept device_type values of 'watch_apple', 'watch_wearos', 'phone_android', or 'phone_ios'

#### Scenario: Invalid device type
- **WHEN** a registration request includes an unrecognized device_type
- **THEN** the system SHALL return 400 Bad Request

### Requirement: Device tokens table schema
The system SHALL store FCM device tokens in a `device_tokens` table.

#### Scenario: Table structure
- **WHEN** the migration runs
- **THEN** the system SHALL create a `device_tokens` table with columns: id (UUID PK), user_id (FK→users), fcm_token (TEXT), device_type (TEXT with CHECK constraint), created_at, updated_at, and UNIQUE constraint on (user_id, device_type)

### Requirement: Notification settings management
The system SHALL allow users to manage their notification preferences via `PUT /api/notifications/settings`.

#### Scenario: Update notification settings
- **WHEN** an authenticated user submits notification preferences (friend_requests, match_results, match_claims as booleans)
- **THEN** the system SHALL store the preferences and return 200

#### Scenario: Default settings
- **WHEN** a user has not configured notification settings
- **THEN** the system SHALL default all notification types to enabled (true)
