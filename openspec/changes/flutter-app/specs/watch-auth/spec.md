## MODIFIED Requirements

### Requirement: Authentication UI for login and registration
The Watch app SHALL provide an `AuthView` with Firebase Auth Sign-In (Apple Sign-In on watchOS).

#### Scenario: Successful Firebase Sign-In
- **WHEN** the user taps the Sign In button and completes Apple Sign-In authentication
- **THEN** the app SHALL obtain a Firebase ID Token, send it to `GET /api/auth/me` in the Authorization header, store the Firebase ID Token locally, and navigate to the main view

#### Scenario: Sign in error
- **WHEN** Firebase Sign-In fails or the API returns an error
- **THEN** the app SHALL display an error message on the authentication screen

#### Scenario: User cancels Sign in
- **WHEN** the user cancels the Sign-In prompt
- **THEN** the app SHALL remain on the authentication screen

#### Scenario: Token refresh
- **WHEN** the stored Firebase ID Token is expired or about to expire
- **THEN** the Firebase SDK SHALL automatically refresh the token without user interaction
