## MODIFIED Requirements

### Requirement: Authentication UI for login and registration
The app SHALL provide an `AuthView` with a Sign in with Apple button using `AuthenticationServices` framework.

#### Scenario: Successful Sign in with Apple
- **WHEN** the user taps the Sign in with Apple button and confirms with double-click of the side button
- **THEN** the app SHALL send the Apple identity token to `POST /api/auth/apple`, store the returned access and refresh tokens in Keychain, and navigate to the main view

#### Scenario: Sign in error
- **WHEN** Sign in with Apple fails or the API returns an error
- **THEN** the app SHALL display an error message

#### Scenario: User cancels Sign in
- **WHEN** the user cancels the Sign in with Apple prompt
- **THEN** the app SHALL remain on the authentication screen
