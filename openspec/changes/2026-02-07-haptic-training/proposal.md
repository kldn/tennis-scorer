## Why

Players want to practice consistent serve rhythm. A metronome-style haptic vibration on the Apple Watch can help build muscle memory for serve timing.

## What Changes

- Add a "Practice Mode" option in the Watch app
- Configurable interval (e.g., every 6/8/10 seconds)
- Basic mode: single vibration at regular intervals
- Advanced mode: multi-step rhythm (short buzz = ready, long buzz = toss, double buzz = hit)
- Different haptic types for rhythm stages (`.click`, `.directionUp`, `.success`)
- Start/stop control with visual timer display

## Capabilities

### New Capabilities
- `haptic-training`: Metronome-style haptic rhythm for serve practice on Apple Watch

### Modified Capabilities
- Watch app navigation: Add entry point for Practice Mode

## Impact

- **Watch App**: New PracticeView, Timer-based haptic scheduling
- **Dependencies**: WKInterfaceDevice haptic API (already available)
- **Priority**: Low — implement only if time permits after core features
- **Depends on**: None (independent)
