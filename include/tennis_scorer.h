/* Warning: this file should be kept in sync with src/ffi.rs */

#ifndef TENNIS_SCORER_H
#define TENNIS_SCORER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Constants
 * ============================================================================ */

#define PLAYER_1 1
#define PLAYER_2 2

/* GameState codes */
#define GAME_STATE_PLAYING 0
#define GAME_STATE_DEUCE 1
#define GAME_STATE_ADVANTAGE_P1 2
#define GAME_STATE_ADVANTAGE_P2 3
#define GAME_STATE_COMPLETED 4

/* ============================================================================
 * Types
 * ============================================================================ */

/**
 * Opaque handle to a tennis match with history.
 * Use tennis_match_new_*() to create and tennis_match_free() to destroy.
 */
typedef struct TennisMatch TennisMatch;

/**
 * Current score of a tennis match.
 * All numeric values are unsigned 8-bit integers.
 */
typedef struct {
    uint8_t player1_sets;    /* Sets won by player 1 */
    uint8_t player2_sets;    /* Sets won by player 2 */
    uint8_t player1_games;   /* Games won in current set by player 1 */
    uint8_t player2_games;   /* Games won in current set by player 2 */
    uint8_t player1_points;  /* Points in current game (0/15/30/40 or tiebreak points) */
    uint8_t player2_points;  /* Points in current game (0/15/30/40 or tiebreak points) */
    uint8_t game_state;      /* GAME_STATE_* constant */
    bool is_tiebreak;        /* True if currently in a tiebreak */
    uint8_t winner;          /* 0 = no winner, PLAYER_1 or PLAYER_2 */
    uint8_t deuce_count;     /* Number of times deuce occurred in current game */
} MatchScore;

/* ============================================================================
 * Lifecycle Functions
 * ============================================================================ */

/**
 * Create a new match with default settings.
 * Default: Best-of-3, 7-point tiebreak, Ad scoring.
 * @return New match handle. Caller must call tennis_match_free() when done.
 */
TennisMatch* tennis_match_new_default(void);

/**
 * Create a new Best-of-5 match.
 * @return New match handle. Caller must call tennis_match_free() when done.
 */
TennisMatch* tennis_match_new_best_of_5(void);

/**
 * Create a new match with No-Ad scoring (sudden death at deuce).
 * @return New match handle. Caller must call tennis_match_free() when done.
 */
TennisMatch* tennis_match_new_no_ad(void);

/**
 * Create a new match with custom settings.
 * @param sets_to_win Number of sets to win (2 for best-of-3, 3 for best-of-5)
 * @param tiebreak_points Points to win tiebreak (typically 7 or 10)
 * @param final_set_tiebreak Whether to use tiebreak in final set
 * @param no_ad_scoring Whether to use No-Ad scoring (sudden death at deuce)
 * @return New match handle. Caller must call tennis_match_free() when done.
 */
TennisMatch* tennis_match_new_custom(
    uint8_t sets_to_win,
    uint8_t tiebreak_points,
    bool final_set_tiebreak,
    bool no_ad_scoring
);

/**
 * Free a match and release its memory.
 * Safe to call with NULL.
 * @param match Match handle to free (can be NULL)
 */
void tennis_match_free(TennisMatch* match);

/* ============================================================================
 * Scoring Functions
 * ============================================================================ */

/**
 * Score a point for the given player.
 * @param match Match handle
 * @param player PLAYER_1 or PLAYER_2
 * @return true if successful, false if match is NULL or player is invalid
 */
bool tennis_match_score_point(TennisMatch* match, uint8_t player);

/**
 * Undo the last point scored.
 * @param match Match handle
 * @return true if successful, false if match is NULL
 */
bool tennis_match_undo(TennisMatch* match);

/* ============================================================================
 * Query Functions
 * ============================================================================ */

/**
 * Get the current score.
 * @param match Match handle (can be NULL)
 * @return Current score. Returns zeroed struct if match is NULL.
 */
MatchScore tennis_match_get_score(const TennisMatch* match);

/**
 * Check if undo is available.
 * @param match Match handle
 * @return true if undo is possible, false otherwise or if match is NULL
 */
bool tennis_match_can_undo(const TennisMatch* match);

/**
 * Check if the match is complete.
 * @param match Match handle
 * @return true if match has a winner, false otherwise or if match is NULL
 */
bool tennis_match_is_complete(const TennisMatch* match);

/**
 * Get the winner of the match.
 * @param match Match handle
 * @return PLAYER_1, PLAYER_2, or 0 if no winner yet or match is NULL
 */
uint8_t tennis_match_get_winner(const TennisMatch* match);

#ifdef __cplusplus
}
#endif

#endif /* TENNIS_SCORER_H */
