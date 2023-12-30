use crate::types::{Game, Round};
use serde_json::from_str;
use std::collections::HashMap;

#[test]
fn test_get_closest_guess_multiple_guesses() {
    let round_json = r#"{
        "question": {
            "question": "What is the capital of France?",
            "answer": 5
        },
        "guesses": [
            {
                "player": "Player1",
                "guess": 2
            },
            {
                "player": "Player2",
                "guess": 4
            },
            {
                "player": "Player3",
                "guess": 6
            }
        ],
        "wagers": []
    }"#;

    let round: Round = from_str(round_json).expect("Failed to deserialize Round");

    assert_eq!(round.get_closest_guess(), Some(4));
}

#[test]
fn test_get_closest_guess_no_valid_guess() {
    let round_json = r#"{
        "question": {
            "question": "What is the capital of France?",
            "answer": 5
        },
        "guesses": [
            {
                "player": "Player1",
                "guess": 6
            },
            {
                "player": "Player2",
                "guess": 8
            }
        ],
        "wagers": []
    }"#;

    let round: Round = from_str(round_json).expect("Failed to deserialize Round");

    assert_eq!(round.get_closest_guess(), None);
}

#[test]
fn test_get_score_changes_correct_wager() {
    let round_json = r#"{
        "question": {
            "question": "What is the capital of France?",
            "answer": 5
        },
        "guesses": [
            {
                "player": "Player1",
                "guess": 5
            }
        ],
        "wagers": [
            {
                "player": "Player1",
                "guess": 5,
                "wager": 10
            }
        ]
    }"#;

    let round: Round = from_str(round_json).expect("Failed to deserialize Round");

    let payout_ratio = 3;
    let closest_guess_bonus = 5;
    let score_changes = round.get_score_changes(payout_ratio, closest_guess_bonus);

    let expected_changes_json = r#"{
        "Player1": 35
    }"#;
    let expected_changes: HashMap<String, i32> =
        from_str(expected_changes_json).expect("Failed to deserialize expected changes");

    assert_eq!(score_changes, expected_changes);
}

#[test]
fn test_get_score_changes_incorrect_wager() {
    let round_json = r#"{
        "question": {
            "question": "What is the capital of France?",
            "answer": 5
        },
        "guesses": [
            {
                "player": "Player1",
                "guess": 3
            }
        ],
        "wagers": [
            {
                "player": "Player1",
                "guess": 10,
                "wager": 5
            }
        ]
    }"#;

    let round: Round = from_str(round_json).expect("Failed to deserialize Round");

    let payout_ratio = 3;
    let closest_guess_bonus = 2;
    let score_changes = round.get_score_changes(payout_ratio, closest_guess_bonus);

    let expected_changes_json = r#"{
        "Player1": -2
    }"#;
    let expected_changes: HashMap<String, i32> =
        from_str(expected_changes_json).expect("Failed to deserialize expected changes");

    assert_eq!(score_changes, expected_changes);
}

#[test]
fn test_get_score_changes_all_guesses_too_high() {
    let round_json = r#"{
        "question": {
            "question": "What is the capital of France?",
            "answer": 5
        },
        "guesses": [
            {
                "player": "Player1",
                "guess": 10
            }
        ],
        "wagers": [
            {
                "player": "Player1",
                "guess": null,
                "wager": 5
            }
        ]
    }"#;

    let round: Round = from_str(round_json).expect("Failed to deserialize Round");

    let payout_ratio = 3;
    let closest_guess_bonus = 2;
    let score_changes = round.get_score_changes(payout_ratio, closest_guess_bonus);

    let expected_changes_json = r#"{
        "Player1": 15
    }"#;
    let expected_changes: HashMap<String, i32> =
        from_str(expected_changes_json).expect("Failed to deserialize expected changes");

    assert_eq!(score_changes, expected_changes);
}

#[test]
fn test_get_score() {
    let game_json = r#"{
        "players": ["Player1", "Player2", "Player3"],
        "rounds": [
            {
                "question": {
                    "question": "What is 2 + 2?",
                    "answer": 4
                },
                "guesses": [
                    {
                        "player": "Player1",
                        "guess": 3
                    },
                    {
                        "player": "Player2",
                        "guess": 1
                    },
                    {
                        "player": "Player3",
                        "guess": 10
                    }
                ],
                "wagers": [
                    {
                        "player": "Player1",
                        "guess": 1,
                        "wager": 10
                    },
                    {
                        "player": "Player2",
                        "guess": 10,
                        "wager": 8
                    },
                    {
                        "player": "Player3",
                        "guess": 3,
                        "wager": 5
                    }
                ]
            }
        ],
        "question_location": "File"
    }"#;

    let game: Game = from_str(game_json).expect("Failed to deserialize Game");

    let scores = game.get_score();

    // -5 = -9 (wrong wager) + 3 (closest guess) + 1 (everyone starts with 1)
    // -6 = -7 (wrong wager) + 1 (everyone starts with 1)
    // 16 = 5*3 (correct wager) + 1 (everyone starts with 1)
    let expected_scores_json = r#"{
        "Player1": -5,
        "Player2": -6,
        "Player3": 16
    }"#;
    let expected_scores: HashMap<String, i32> =
        from_str(expected_scores_json).expect("Failed to deserialize expected scores");

    assert_eq!(scores, expected_scores);
}
