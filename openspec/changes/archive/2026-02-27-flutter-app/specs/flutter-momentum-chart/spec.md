## ADDED Requirements

### Requirement: Interactive momentum chart
The app SHALL display a momentum chart using data from `GET /api/stats/match/:id/momentum`.

#### Scenario: Display momentum line chart
- **WHEN** the momentum chart screen loads
- **THEN** the app SHALL render a line chart showing momentum values across all points in the match

#### Scenario: Switch between momentum modes
- **WHEN** the user selects a different mode (basic, weighted, per-set basic, per-set weighted)
- **THEN** the app SHALL re-render the chart with the selected momentum data

#### Scenario: Default mode
- **WHEN** the momentum chart screen first loads
- **THEN** the app SHALL display the basic momentum mode by default

#### Scenario: Chart visual indicators
- **WHEN** the chart is displayed
- **THEN** the app SHALL show a zero line, with positive values indicating player 1 momentum and negative values indicating player 2 momentum
