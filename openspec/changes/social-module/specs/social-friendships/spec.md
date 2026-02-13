## ADDED Requirements

### Requirement: List friends
The system SHALL return the authenticated user's friend list via `GET /api/social/friends`.

#### Scenario: User has friends
- **WHEN** an authenticated user requests their friend list
- **THEN** the system SHALL return an array of friend objects containing id, display_name, and avatar_url

#### Scenario: User has no friends
- **WHEN** an authenticated user with no friends requests their friend list
- **THEN** the system SHALL return an empty array

### Requirement: View friend's matches
The system SHALL allow viewing a friend's match history via `GET /api/social/friends/{id}/matches`.

#### Scenario: Valid friend relationship
- **WHEN** an authenticated user requests matches for a user who is their friend
- **THEN** the system SHALL return the friend's match list with pagination

#### Scenario: Not a friend
- **WHEN** an authenticated user requests matches for a user who is NOT their friend
- **THEN** the system SHALL return 403 Forbidden

#### Scenario: Friend not found
- **WHEN** an authenticated user requests matches for a user ID that does not exist
- **THEN** the system SHALL return 404 Not Found

### Requirement: Bidirectional friendship storage
The system SHALL store friendships bidirectionally in the `friendships` table.

#### Scenario: Friendship creation on accept
- **WHEN** a friend request is accepted
- **THEN** the system SHALL insert two rows: (user_id=A, friend_id=B) and (user_id=B, friend_id=A)

#### Scenario: Friendship uniqueness
- **WHEN** a friendship already exists between two users
- **THEN** the UNIQUE constraint on (user_id, friend_id) SHALL prevent duplicate entries
