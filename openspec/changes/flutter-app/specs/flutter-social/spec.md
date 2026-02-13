## ADDED Requirements

### Requirement: Social screen displays friends and requests
The phone app SHALL provide a Social screen with friend list and pending friend requests.

#### Scenario: User with friends
- **WHEN** the user opens the Social screen and has friends
- **THEN** the app SHALL display a list of friends with display_name and avatar_url from `GET /api/social/friends`

#### Scenario: Pending friend requests
- **WHEN** the user has pending incoming friend requests
- **THEN** the app SHALL display a badge/count and a list of pending requests with accept/reject actions

#### Scenario: No friends or requests
- **WHEN** the user has no friends and no pending requests
- **THEN** the app SHALL display an empty state with instructions on how to add friends

### Requirement: Send friend request
The phone app SHALL allow users to send friend requests by email.

#### Scenario: Send request by email
- **WHEN** the user enters an email address and taps "Send Request"
- **THEN** the app SHALL call `POST /api/social/friend-request` with the email and show a success confirmation

#### Scenario: Invalid email or user not found
- **WHEN** the API returns an error (user not found, already friends, etc.)
- **THEN** the app SHALL display the appropriate error message

### Requirement: Respond to friend request
The phone app SHALL allow users to accept or reject incoming friend requests.

#### Scenario: Accept friend request
- **WHEN** the user taps "Accept" on a pending friend request
- **THEN** the app SHALL call `POST /api/social/friend-request/{id}` with action "accept" and update the UI

#### Scenario: Reject friend request
- **WHEN** the user taps "Reject" on a pending friend request
- **THEN** the app SHALL call `POST /api/social/friend-request/{id}` with action "reject" and remove it from the list

### Requirement: View friend details and head-to-head
The phone app SHALL allow viewing a friend's match history and head-to-head statistics.

#### Scenario: View friend's matches
- **WHEN** the user taps a friend in the list
- **THEN** the app SHALL navigate to a detail screen showing the friend's recent matches from `GET /api/social/friends/{id}/matches`

#### Scenario: View head-to-head stats
- **WHEN** the user views a friend's detail screen
- **THEN** the app SHALL display head-to-head statistics (wins, losses, total matches) from `GET /api/social/head-to-head/{id}`
