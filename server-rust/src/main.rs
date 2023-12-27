mod question_lookup;
#[cfg(test)]
mod tests;
mod types;

use parking_lot::{Mutex, RwLock};
use question_lookup::QuestionLookup;
use rocket::{
    self, config::LogLevel, delete, get, http::Method, post, put, routes, serde::json::Json,
    Config, State,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use types::{Game, Guess, PlayerData, Result, Scores, Wager};

type Games = Mutex<types::Games>;
type Questions = RwLock<QuestionLookup>;

#[get("/heartbeat")]
fn heartbeat() -> &'static str {
    "heartbeat"
}

#[put("/game/<game_id>", data = "<player>")]
fn create_game(
    game_id: String,
    player: Json<PlayerData>,
    games: &State<Games>,
    questions: &State<Questions>,
) -> Result<()> {
    let mut games = games.lock();
    let player = player.into_inner();
    games.create(game_id, player.player, questions.read().get())
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(game_id: String, player: Json<PlayerData>, games: &State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let player = player.into_inner();
    game.add_player(player.player)
}

#[get("/game/<game_id>")]
fn game(game_id: String, games: &State<Games>) -> Result<Json<Game>> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/guess", data = "<guess>")]
fn guess(game_id: String, guess: Json<Guess>, games: &State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let guess = guess.into_inner();
    game.guess(guess)
}

#[post("/game/<game_id>/wager", data = "<wager>")]
fn wager(
    game_id: String,
    wager: Json<Wager>,
    games: &State<Games>,
    questions: &State<Questions>,
) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let wager = wager.into_inner();
    game.wager(wager)?;
    game.add_round_if_complete(questions.read().get());
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(game_id: String, player: Json<PlayerData>, games: &State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let player = player.into_inner();
    game.remove_player(player.player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: String, games: &State<Games>) {
    let mut games = games.lock();
    games.delete(&game_id)
}

#[get("/game/<game_id>/score")]
fn get_score(game_id: String, games: &State<Games>) -> Result<Json<Scores>> {
    let mut games = games.lock();
    let game = games.get(&game_id)?.clone();
    Ok(Json(game.get_score()))
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The path to a file containing newline delimited questions.
    #[structopt(long = "questions-file")]
    questions_file: Option<PathBuf>,
    /// An IP address the application will listen on.
    #[structopt(long = "host", short = "H", default_value = "0.0.0.0")]
    address: IpAddr,
    /// A port number to listen on.
    #[structopt(long = "port", short = "P", default_value = "8172")]
    port: u16,
    /// The log level.
    #[structopt(
        default_value = "normal",
        long = "log-level",
        possible_values = &["off", "debug", "normal", "critical"]
    )]
    log_level: LogLevel,
}

#[rocket::launch]
fn rocket() -> _ {
    let opt = Opt::from_args();
    let config = Config {
        address: opt.address,
        port: opt.port,
        log_level: opt.log_level,
        ..Config::default()
    };

    // Populate the questions
    let mut questions = QuestionLookup::default();
    if let Some(questions_file) = opt.questions_file {
        if let Err(e) = questions.populate_from_file(&questions_file) {
            eprintln!("Failed to populate questions from file {questions_file:?}, err: {e}");
            std::process::exit(1);
        }
    }

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .configure(config)
        .attach(cors.to_cors().expect("known valid cors config"))
        .mount(
            "/api/v1",
            routes![
                heartbeat,
                create_game,
                join_game,
                game,
                guess,
                wager,
                exit_game,
                delete_game,
                get_score,
            ],
        )
        .manage(Questions::default())
        .manage(Games::default())
}
