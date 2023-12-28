use crate::types::{Guess, Question, Round};

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
