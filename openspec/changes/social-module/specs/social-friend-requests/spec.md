## ADDED Requirements

### Requirement: Send a friend request
The system SHALL allow an authenticated user to send a friend request via `POST /api/social/friend-request`.

#### Scenario: Successful friend request by user ID
- **WHEN** an authenticated user sends a friend request with a valid `to_user_id`
- **THEN** the system SHALL create a friend_request record with status "pending" and return 201 with the request details

#### Scenario: Successful friend request by email
- **WHEN** an authenticated user sends a friend request with an `email` that matches a registered user
- **THEN** the system SHALL resolve the email to a user ID, create a friend_request record, and return 201

#### Scenario: Self-request prevention
- **WHEN** an authenticated user sends a friend request to themselves
- **THEN** the system SHALL return 400 Bad Request

#### Scenario: Duplicate request prevention
- **WHEN** an authenticated user sends a friend request to someone they already have a pending request with
- **THEN** the system SHALL return 409 Conflict

#### Scenario: Already friends
- **WHEN** an authenticated user sends a friend request to someone who is already their friend
- **THEN** the system SHALL return 409 Conflict with a message indicating they are already friends

#### Scenario: Target user not found
- **WHEN** an authenticated user sends a friend request to an email that does not exist
- **THEN** the system SHALL return 404 Not Found

### Requirement: Respond to a friend request
The system SHALL allow the recipient of a friend request to accept or reject it via `POST /api/social/friend-request/{id}`.

#### Scenario: Accept friend request
- **WHEN** the recipient of a pending friend request submits action "accept"
- **THEN** the system SHALL update the request status to "accepted", create bidirectional friendship records, and return 200

#### Scenario: Reject friend request
- **WHEN** the recipient of a pending friend request submits action "reject"
- **THEN** the system SHALL update the request status to "rejected" and return 200

#### Scenario: Non-recipient tries to respond
- **WHEN** a user who is NOT the recipient tries to accept or reject a friend request
- **THEN** the system SHALL return 403 Forbidden

#### Scenario: Request already responded to
- **WHEN** a user tries to respond to a friend request that is no longer pending
- **THEN** the system SHALL return 409 Conflict

#### Scenario: Request not found
- **WHEN** a user tries to respond to a friend request that does not exist
- **THEN** the system SHALL return 404 Not Found
