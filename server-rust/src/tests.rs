use crate::types::{Game, Guess, Question, Round, Wager};
use std::collections::HashMap;

#[test]
fn test_get_closest_guess_multiple_guesses() {
    let question = Question {
        question: "What is the capital of France?".to_string(),
        answer: 5,
    };
    let mut round = Round::new(question);
    let guess1 = Guess {
        player: "Player1".to_string(),
        guess: 2,
    };
    let guess2 = Guess {
        player: "Player2".to_string(),
        guess: 4,
    };
    let guess3 = Guess {
        player: "Player3".to_string(),
        guess: 6,
    };
    round.guesses.add_or_replace(guess1.clone());
    round.guesses.add_or_replace(guess2.clone());
    round.guesses.add_or_replace(guess3.clone());
    assert_eq!(round.get_closest_guess(), Some(&guess2));
}

#[test]
fn test_get_closest_guess_no_valid_guess() {
    let question = Question {
        question: "What is the capital of France?".to_string(),
        answer: 5,
    };
    let mut round = Round::new(question);
    let guess1 = Guess {
        player: "Player1".to_string(),
        guess: 6,
    };
    let guess2 = Guess {
        player: "Player2".to_string(),
        guess: 8,
    };
    round.guesses.add_or_replace(guess1.clone());
    round.guesses.add_or_replace(guess2.clone());
    assert_eq!(round.get_closest_guess(), None);
}

#[test]
fn test_get_score_changes_correct_wager() {
    let question = Question {
        question: "What is the capital of France?".to_string(),
        answer: 5,
    };
    let mut round = Round::new(question);

    let guess = Guess {
        player: "Player1".to_string(),
        guess: 5,
    };
    round.guesses.add_or_replace(guess);

    let wager = Wager {
        player: "Player1".to_string(),
        guess: 5,
        wager: 10,
    };
    round.wagers.add_or_replace(wager);

    let payout_ratio = 3;
    let closest_guess_bonus = 5;
    let score_changes = round.get_score_changes(payout_ratio, closest_guess_bonus);

    let mut expected_changes = HashMap::new();
    expected_changes.insert("Player1".to_string(), 35); // 10 * 3 + 5 (correct wager and closest guess)

    assert_eq!(score_changes, expected_changes);
}

#[test]
fn test_get_score_changes_incorrect_wager() {
    let question = Question {
        question: "What is the capital of France?".to_string(),
        answer: 5,
    };
    let mut round = Round::new(question);

    let guess = Guess {
        player: "Player1".to_string(),
        guess: 3,
    };
    round.guesses.add_or_replace(guess);

    let wager = Wager {
        player: "Player1".to_string(),
        guess: 10,
        wager: 5,
    };
    round.wagers.add_or_replace(wager);

    let payout_ratio = 3;
    let closest_guess_bonus = 2;
    let score_changes = round.get_score_changes(payout_ratio, closest_guess_bonus);

    let mut expected_changes = HashMap::new();
    expected_changes.insert("Player1".to_string(), -2); // (-5 + 1) + 2 (incorrect wager but still had closest guess)

    assert_eq!(score_changes, expected_changes);
}

#[test]
fn test_get_score() {
    // Create a game with three players
    let mut game = Game::default();

    // Add rounds with questions
    game.add_round_if_complete(Question {
        question: "What is 2 + 2?".to_string(),
        answer: 4,
    });

    game.add_player("Player1".to_string()).unwrap();
    game.add_player("Player2".to_string()).unwrap();
    game.add_player("Player3".to_string()).unwrap();

    // Simulate guesses and wagers for each round
    game.guess(Guess {
        player: "Player1".to_string(),
        guess: 3,
    })
    .unwrap();
    game.guess(Guess {
        player: "Player2".to_string(),
        guess: 1,
    })
    .unwrap();
    game.guess(Guess {
        player: "Player3".to_string(),
        guess: 10,
    })
    .unwrap();

    game.wager(Wager {
        player: "Player1".to_string(),
        guess: 1,
        wager: 10,
    })
    .unwrap();
    game.wager(Wager {
        player: "Player2".to_string(),
        guess: 10,
        wager: 8,
    })
    .unwrap();
    game.wager(Wager {
        player: "Player3".to_string(),
        guess: 3,
        wager: 5,
    })
    .unwrap();

    // Calculate scores with a payout ratio of 3
    let scores = game.get_score();

    // Verify the expected scores
    let mut expected_scores = HashMap::new();
    expected_scores.insert("Player1".to_string(), -7); // -9 (wrong wager) + 1 (closest guess) + 1 (everyone starts with 1)
    expected_scores.insert("Player2".to_string(), -6); // -7 (wrong wager) + 1 (everyone starts with 1)
    expected_scores.insert("Player3".to_string(), 16); // 5*3 (correct wager) + 1 (everyone starts with 1)

    assert_eq!(scores, expected_scores);
}
