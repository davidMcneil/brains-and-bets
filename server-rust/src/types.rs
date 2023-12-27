mod traits;

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    error, fmt,
    ops::Deref,
};

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type GameId = String;

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
    RoundNotInCollectingGuessesState,
    GuessedPlayerNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameConflict => write!(f, "game conflict"),
            Self::GameNotFound => write!(f, "game not found"),
            Self::PlayerConflict => write!(f, "player conflict"),
            Self::PlayerNotFound => write!(f, "player not found"),
            Self::RoundNotInStartState => write!(f, "round not in start state"),
            Self::RoundNotInCollectingAnswersState => {
                write!(f, "round not in collecting answer state")
            }
            Self::RoundNotInCollectingGuessesState => {
                write!(f, "round not in collecting guess state")
            }
            Self::GuessedPlayerNotFound => write!(f, "guessed player not found"),
        }
    }
}

impl error::Error for Error {}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: String,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            error: format!("{error:?}"),
            message: format!("{error}"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub(crate) player: Player,
}

#[cfg(test)]
impl PlayerData {
    pub(crate) fn new(player: &str) -> Self {
        Self {
            player: Player::from(player),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answer {
    /// The player who gave the answer
    player: Player,
    /// The answer to the question for the round
    pub answer: String,
}

#[cfg(test)]
impl Answer {
    pub(crate) fn new(player: &str, answer: &str) -> Self {
        Self {
            player: Player::from(player),
            answer: String::from(answer),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answers(Vec<Answer>);

impl Deref for Answers {
    type Target = Vec<Answer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Answer>> for Answers {
    fn from(value: Vec<Answer>) -> Self {
        let mut unique_answers = Vec::new();
        let mut seen_players = std::collections::HashSet::new();
        for answer in value {
            if seen_players.insert(answer.player.clone()) {
                unique_answers.push(answer);
            }
        }
        Answers(unique_answers)
    }
}

impl IntoIterator for Answers {
    type Item = Answer;
    type IntoIter = std::vec::IntoIter<Answer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Guess {
    /// The player making the guess
    pub player: Player,
    /// The list of guessed answers, one per player
    pub answers: Answers,
}

#[cfg(test)]
impl Guess {
    pub(crate) fn new(player: &str, guess: Vec<Answer>) -> Self {
        Self {
            player: Player::from(player),
            answers: Answers::from(guess),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    Start,
    CollectingAnswers,
    CollectingGuesses,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The question for the round
    question: String,
    /// The list of answers given, one per player
    pub(crate) answers: HashSet<Answer>,
    /// The list of guesses made, one per player
    pub(crate) guesses: HashSet<Guess>,
}

impl Round {
    fn new(question: String) -> Self {
        Round {
            question,
            answers: HashSet::new(),
            guesses: HashSet::new(),
        }
    }

    fn state(&self, players: usize) -> RoundState {
        if self.answers.is_empty() {
            RoundState::Start
        } else if self.answers.len() < players {
            RoundState::CollectingAnswers
        } else if self.guesses.len() < players {
            RoundState::CollectingGuesses
        } else if self.answers.len() == players && self.guesses.len() == players {
            RoundState::Complete
        } else {
            panic!("Round in unknown state")
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) players: HashSet<String>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
}

impl Game {
    pub(crate) fn add_player(&mut self, player: Player) -> Result<()> {
        // Only allow adding players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        if self.players.insert(player) {
            Ok(())
        } else {
            Err(Error::PlayerConflict)
        }
    }

    pub(crate) fn remove_player(&mut self, player: Player) -> Result<()> {
        // Only allow removing players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        self.players.remove(&player);
        Ok(())
    }

    pub(crate) fn answer(&mut self, answer: Answer) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }
        // Add or replace the answer
        let round = self.current_round_mut();
        round.answers.replace(answer);
        Ok(())
    }

    pub(crate) fn guess(&mut self, guess: Guess) -> Result<()> {
        let player = &guess.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are adding collecting for the current round
        if self.current_round_state() != RoundState::CollectingGuesses {
            return Err(Error::RoundNotInCollectingGuessesState);
        }
        // Confirm the guesses are valid
        for g in guess.answers.iter() {
            if !self.players.contains(&g.player) {
                return Err(Error::GuessedPlayerNotFound);
            }
        }
        // Add or replace the guess
        let round = self.current_round_mut();
        round.guesses.replace(guess);
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self, question: String) {
        if self.current_round_state() == RoundState::Complete {
            self.add_round(question);
        }
    }

    fn add_round(&mut self, question: String) {
        self.rounds.push(Round::new(question));
    }

    pub(crate) fn current_round(&self) -> &Round {
        let index = self.rounds.len() - 1;
        &self.rounds[index]
    }

    fn current_round_mut(&mut self) -> &mut Round {
        let index = self.rounds.len() - 1;
        &mut self.rounds[index]
    }

    fn current_round_state(&self) -> RoundState {
        let players = self.players.len();
        let round = self.current_round();
        round.state(players)
    }

    pub fn get_score(&self) -> HashMap<String, i32> {
        let mut scores = HashMap::new();
        for round in &self.rounds {
            for guess in &round.guesses {
                for answer in guess.answers.iter() {
                    let score = scores.entry(guess.player.clone()).or_insert(0);
                    if round.answers.contains(answer) {
                        *score += 1;
                    } else {
                        *score -= 1;
                    }
                }
            }
        }
        scores
    }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<String, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        initial_question: String,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round(initial_question);
            game.add_player(initial_player)?;
            self.0.insert(game_id, game);
            Ok(())
        }
    }

    pub(crate) fn get(&mut self, game_id: &str) -> Result<&mut Game> {
        self.0.get_mut(game_id).ok_or(Error::GameNotFound)
    }

    pub(crate) fn delete(&mut self, game_id: &str) {
        self.0.remove(game_id);
    }
}
