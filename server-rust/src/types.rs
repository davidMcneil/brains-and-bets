use derive_more::{Deref, IntoIterator};
use displaydoc::Display;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type AnswerAmount = u32;
pub(crate) type ScoreAmount = i32;
pub(crate) type GameId = String;
pub(crate) type Scores = HashMap<Player, ScoreAmount>;

#[derive(Deserialize, Serialize, Debug, Display, Error)]
pub(crate) enum Error {
    /// game conflict
    GameConflict,
    /// game not found
    GameNotFound,
    /// player conflict
    PlayerConflict,
    /// player not found
    PlayerNotFound,
    /// round not in start state
    RoundNotInStartState,
    /// round not in collecting guesses state
    RoundNotInCollectingGuessesState,
    /// round not in collecting wagers state
    RoundNotInCollectingWagersState,
    /// guess not found
    GuessNotFound,
    /// invalid wager
    InvalidWager,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: Error,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            message: format!("{error}"),
            error,
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = BadRequest::new(self);
        let body = serde_json::to_string(&body).expect("to BadRequest serialize");
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub player: Player,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CreateGameData {
    /// The player with which the request is associated
    pub player: Player,
    /// The location to get questions from
    pub get_questions_from: GetQuestionLocation,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct Question {
    /// The question for the round
    pub question: String,
    /// The correct answer to the question
    pub answer: AnswerAmount,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Guess {
    /// The player making the guess
    pub player: Player,
    /// The players guess for the round
    pub guess: AnswerAmount,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, Deref, IntoIterator)]
pub(crate) struct Guesses(Vec<Guess>);

impl Guesses {
    pub fn add_or_replace(&mut self, guess: Guess) {
        if let Some(existing_guess_index) = self.iter().position(|g| g.player == guess.player) {
            self.0[existing_guess_index] = guess;
        } else {
            self.0.push(guess);
        }
    }

    fn contains(&mut self, guess: AnswerAmount) -> bool {
        self.iter().any(|g| g.guess == guess)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Wager {
    /// The player making the wager
    pub player: Player,
    /// The guess the player is wagering on, None is a wager that the correct value is below all guesses
    pub guess: Option<AnswerAmount>,
    /// The players wager amount
    pub wager: ScoreAmount,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, Deref, IntoIterator)]
pub(crate) struct Wagers(Vec<Wager>);

impl Wagers {
    pub fn add_or_replace(&mut self, wager: Wager) {
        if let Some(existing_wager_index) = self.iter().position(|w| w.player == wager.player) {
            self.0[existing_wager_index] = wager;
        } else {
            self.0.push(wager);
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RoundState {
    Start,
    CollectingGuesses,
    CollectingWagers,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The question for the round
    pub question: Question,
    /// The list of guesses given, one per player
    pub guesses: Guesses,
    /// The list of wagers made, one per player
    pub wagers: Wagers,
}

impl Round {
    pub fn new(question: Question) -> Self {
        Round {
            question,
            guesses: Guesses::default(),
            wagers: Wagers::default(),
        }
    }

    fn state(&self, players: usize) -> RoundState {
        match (self.guesses.len(), self.wagers.len()) {
            (0, 0) => RoundState::Start,
            (guesses, 0) if guesses < players => RoundState::CollectingGuesses,
            (guesses, wagers) if guesses == players && wagers < players => {
                RoundState::CollectingWagers
            }
            (guesses, wagers) if guesses == players && wagers == players => RoundState::Complete,
            _ => panic!("Round in unknown state"),
        }
    }

    pub fn get_closest_guess(&self) -> Option<u32> {
        // Get the greatest guess that is not greater than the actual answer
        self.guesses
            .iter()
            .map(|guess| guess.guess)
            .filter(|guess| guess <= &self.question.answer)
            .max()
    }

    pub fn get_score_changes(&self, payout_ratio: i32, closest_guess_bonus: i32) -> Scores {
        let closest_guess = self.get_closest_guess();
        let mut score_changes = HashMap::new();
        for wager in self.wagers.iter() {
            let score_change = if wager.guess == closest_guess {
                // With the correct wager, the player gets a payout proportional to the wager amount
                wager.wager * payout_ratio
            } else if wager.wager >= 1 {
                // With an incorrect wager of at least 1, the player loses all but 1 of the wager amount
                -wager.wager + 1
            } else {
                // With a wager of 0, there is no gain or loss
                0
            };
            score_changes.insert(wager.player.clone(), score_change);
        }
        // Add an extra bonus to the players with the closest guess
        if let Some(closest_guess) = closest_guess {
            for guess in self.guesses.iter() {
                if guess.guess == closest_guess {
                    let closest_player_score =
                        score_changes.entry(guess.player.clone()).or_insert(0);
                    *closest_player_score += closest_guess_bonus;
                }
            }
        }
        score_changes
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub players: HashSet<Player>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub rounds: Vec<Round>,
    /// The location to get questions from
    pub question_location: GetQuestionLocation,
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

    pub(crate) fn guess(&mut self, guess: Guess) -> Result<()> {
        let player = &guess.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting guesses for the current round
        match self.current_round_state() {
            RoundState::Start | RoundState::CollectingGuesses => (),
            _ => return Err(Error::RoundNotInCollectingGuessesState),
        }
        // Add or replace the answer
        let round = self.current_round_mut();
        round.guesses.add_or_replace(guess);
        Ok(())
    }

    pub(crate) fn wager(&mut self, wager: Wager) -> Result<()> {
        let player = &wager.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting wagers for the current round
        if self.current_round_state() != RoundState::CollectingWagers {
            return Err(Error::RoundNotInCollectingWagersState);
        }
        // Confirm the wagers are valid
        let scores = self.get_score();
        let round = self.current_round_mut();
        if let Some(some_wager_guess) = wager.guess {
            if !round.guesses.contains(some_wager_guess) {
                return Err(Error::GuessNotFound);
            }
        }
        // Check that the amount is less than or equal to their score so far
        match scores.get(&wager.player) {
            Some(score) => {
                if &wager.wager > score {
                    return Err(Error::InvalidWager);
                }
            }
            None => return Err(Error::PlayerNotFound),
        }
        // Add or replace the guess
        round.wagers.add_or_replace(wager);
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self, question: Question) {
        if self.rounds.is_empty() || self.current_round_state() == RoundState::Complete {
            self.rounds.push(Round::new(question));
        }
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

    pub fn get_score(&self) -> Scores {
        let mut scores = HashMap::new();
        // Everyone start off with a score of 1
        for player in &self.players {
            scores.insert(player.clone(), 1);
        }
        // Do not score the current round if it is not Complete
        let last_round_to_score = if self.current_round_state() == RoundState::Complete {
            self.rounds.len()
        } else {
            self.rounds.len() - 1
        };
        for round in &self.rounds[..last_round_to_score] {
            let round_score_changes = round.get_score_changes(3, 3);
            for (player, round_score_change) in &round_score_changes {
                let score = scores.entry(player.clone()).or_insert(1);
                *score += round_score_change;
            }
        }
        scores
    }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<GameId, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        question: Question,
        get_questions_from: GetQuestionLocation,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game {
                question_location: get_questions_from,
                ..Default::default()
            };
            game.add_round_if_complete(question);
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

#[derive(Default, Clone, Copy, Deserialize, Serialize, Debug)]
pub(crate) enum GetQuestionLocation {
    #[default]
    File,
    NumbersApi,
}
