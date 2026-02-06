## ADDED Requirements

### Requirement: Authentication UI for login and registration
The app SHALL provide an `AuthView` where the user can enter email and password to register or log in.

#### Scenario: Successful login
- **WHEN** the user enters valid credentials and taps login
- **THEN** the app SHALL store the access and refresh tokens in Keychain and navigate to the main view

#### Scenario: Successful registration
- **WHEN** the user enters a new email and password (min 8 chars) and taps register
- **THEN** the app SHALL create the account, automatically log in, and store tokens

#### Scenario: Login error
- **WHEN** credentials are invalid
- **THEN** the app SHALL display an error message without revealing which field was wrong

### Requirement: Tokens stored securely in Keychain
The app SHALL store JWT access and refresh tokens in the iOS/watchOS Keychain.

#### Scenario: Tokens persist across app launches
- **WHEN** the user logs in and later relaunches the app
- **THEN** the stored tokens SHALL be available for API requests

#### Scenario: Logout clears tokens
- **WHEN** the user logs out
- **THEN** the app SHALL delete all tokens from Keychain

### Requirement: API client attaches auth headers
The `APIClient` SHALL attach `Authorization: Bearer <access_token>` to all authenticated requests.

#### Scenario: Authenticated request
- **WHEN** an API request is made and tokens exist
- **THEN** the request SHALL include the Bearer token header

#### Scenario: Token refresh on 401
- **WHEN** an API request returns 401
- **THEN** the client SHALL attempt to refresh the token and retry the original request once
