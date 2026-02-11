## MODIFIED Requirements

### Requirement: Sync service uploads unsynced matches
The sync service SHALL upload unsynced matches to POST /api/matches.
HTTP requests SHALL include a timeout of 30 seconds to prevent indefinite hangs.
The accepted HTTP status code range SHALL be consistent between initial request and retry (200...201 for success, 204 for no-content).

#### Scenario: Request timeout prevents indefinite hang
- **WHEN** the API server does not respond
- **THEN** the request times out after 30 seconds
- **AND** an appropriate error is surfaced

#### Scenario: Consistent status code handling on retry
- **WHEN** a request fails with 401 and is retried after token refresh
- **THEN** the retry accepts the same status codes as the original request (200-201, plus 204 for no-content)
- **AND** status codes 202-203 are not silently accepted on retry
