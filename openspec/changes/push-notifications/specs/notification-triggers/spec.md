## ADDED Requirements

### Requirement: Friend request notification
The system SHALL send a push notification when a user receives a friend request.

#### Scenario: Friend request sent
- **WHEN** user A sends a friend request to user B
- **THEN** the system SHALL send a notification to user B with title "好友請求" and body containing user A's display name

#### Scenario: Recipient has friend_requests notifications disabled
- **WHEN** user A sends a friend request to user B and user B has friend_requests notification setting disabled
- **THEN** the system SHALL NOT send the notification

### Requirement: Match result notification
The system SHALL send push notifications to friends when a match is completed.

#### Scenario: Match created with linked opponent
- **WHEN** a match is created and opponent_user_id is set (opponent is a friend)
- **THEN** the system SHALL send a notification to the opponent with title "比賽結果" and body containing a match summary

#### Scenario: Recipient has match_results notifications disabled
- **WHEN** a match result notification would be sent but the recipient has match_results notification setting disabled
- **THEN** the system SHALL NOT send the notification

### Requirement: Match claim notification
The system SHALL send a push notification when an opponent claims a match.

#### Scenario: Match claimed by opponent
- **WHEN** a user claims a match via POST /api/matches/claim
- **THEN** the system SHALL send a notification to the match creator with title "比賽認領" and body containing the claimant's display name

#### Scenario: Creator has match_claims notifications disabled
- **WHEN** a match claim notification would be sent but the creator has match_claims notification setting disabled
- **THEN** the system SHALL NOT send the notification
