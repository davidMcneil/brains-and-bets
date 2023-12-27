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
pub(crate) type ScoreAmount = u32;
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

#[derive(Clone, Deserialize, Serialize)]
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
    fn add_or_replace(&mut self, guess: Guess) {
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
    /// The guess the player is wagering on
    pub guess: AnswerAmount,
    /// The players wager amount
    pub wager: ScoreAmount,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, Deref, IntoIterator)]
pub(crate) struct Wagers(Vec<Wager>);

impl Wagers {
    fn add_or_replace(&mut self, wager: Wager) {
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
    fn new(question: Question) -> Self {
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
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub players: HashSet<Player>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub rounds: Vec<Round>,
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
        if self.current_round_state() <= RoundState::CollectingGuesses {
            return Err(Error::RoundNotInCollectingGuessesState);
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
        // Confirm we are adding collecting for the current round
        if self.current_round_state() != RoundState::CollectingWagers {
            return Err(Error::RoundNotInCollectingWagersState);
        }
        // Confirm the wagers are valid
        let round = self.current_round_mut();
        if !round.guesses.contains(wager.guess) {
            return Err(Error::GuessNotFound);
        }
        // TODO: check that the amount is less than or equal to what the user could wager
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
        todo!("implement this")
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
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
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
