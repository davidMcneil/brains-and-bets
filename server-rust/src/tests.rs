use std::collections::HashMap;

use crate::{rocket, types::PlayerData, Answer, Game, Guess};
use rocket::{http::Status, local::Client};

#[test]
fn not_found() {
    let client = Client::new(rocket(None)).unwrap();
    let res = client.get("/this_is_a_bad_request").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn heartbeat() {
    let client = Client::new(rocket(None)).unwrap();
    let res = client.get("/api/v1/heartbeat").dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn simple_game() {
    let client = Client::new(rocket(None)).unwrap();
    // Create game as p1
    let p = PlayerData::new("p1");
    let res = client
        .put("/api/v1/game/my_game")
        .body(serde_json::to_string(&p).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Join
    let p = PlayerData::new("p2");
    let res = client
        .post("/api/v1/game/my_game")
        .body(serde_json::to_string(&p).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // P2 answers
    let a = Answer::new("p2", "a2");
    let res = client
        .post("/api/v1/game/my_game/answer")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Cannot join the game in the middle of a round
    let res = client
        .post("/api/v1/game/my_game")
        .body(r#"{ "player": "p3" }"#)
        .dispatch();
    assert_eq!(res.status(), Status::BadRequest);
    // P1 answers
    let a = Answer::new("p1", "a1");
    let res = client
        .post("/api/v1/game/my_game/answer")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Get the state of the game
    let mut res = client.get("/api/v1/game/my_game").dispatch();
    let game = serde_json::from_str::<Game>(&res.body_string().unwrap()).unwrap();
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.rounds.len(), 1);
    assert_eq!(game.current_round().answers.len(), 2);
    // P1 guesses
    let a = Guess::new("p1", vec![Answer::new("p1", "a1"), Answer::new("p2", "a2")]);
    let res = client
        .post("/api/v1/game/my_game/guess")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Get the state of the game
    let mut res = client.get("/api/v1/game/my_game").dispatch();
    let game = serde_json::from_str::<Game>(&res.body_string().unwrap()).unwrap();
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.rounds.len(), 1);
    assert_eq!(game.current_round().answers.len(), 2);
    assert_eq!(game.current_round().guesses.len(), 1);
    // P2 guesses
    let a = Guess::new("p2", vec![Answer::new("p1", "a1"), Answer::new("p2", "a2")]);
    let res = client
        .post("/api/v1/game/my_game/guess")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Get the state of the game
    let mut res = client.get("/api/v1/game/my_game").dispatch();
    let game = serde_json::from_str::<Game>(&res.body_string().unwrap()).unwrap();
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.rounds.len(), 2);
    assert_eq!(game.current_round().answers.len(), 0);
    assert_eq!(game.current_round().guesses.len(), 0);
    // P1 exit the game
    let p = PlayerData::new("p1");
    let res = client
        .delete("/api/v1/game/my_game/exit")
        .body(serde_json::to_string(&p).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Get the state of the game
    let mut res = client.get("/api/v1/game/my_game").dispatch();
    let game = serde_json::from_str::<Game>(&res.body_string().unwrap()).unwrap();
    assert_eq!(game.players.len(), 1);
    // Delete the game
    let res = client.delete("/api/v1/game/my_game").dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Delete the game
    let res = client.get("/api/v1/game/my_game").dispatch();
    assert_eq!(res.status(), Status::BadRequest);
}

#[test]
fn test_get_score() {
    let json_data = r#"
    {
        "players": [
            "p1",
            "p2"
        ],
        "rounds": [
            {
                "question": "What are you thankful you're not doing right now? ",
                "answers": [
                    {
                        "player": "p1",
                        "answer": "ijaefa"
                    },
                    {
                        "player": "p2",
                        "answer": "dlkamf"
                    }
                ],
                "guesses": [
                    {
                        "player": "p1",
                        "answers": [
                            {
                                "player": "p2",
                                "answer": "dlkamf"
                            }
                        ]
                    },
                    {
                        "player": "p2",
                        "answers": [
                            {
                                "player": "p1",
                                "answer": "ijaefa"
                            }
                        ]
                    }
                ]
},
            {
                "question": "What would be a great annual tradition? ",
                "answers": [
                    {
                        "player": "p1",
                        "answer": "f"
                    },
                    {
                        "player": "p2",
                        "answer": "f"
                    }
                ],
                "guesses": [
                    {
                        "player": "p1",
                        "answers": [
                            {
                                "player": "p2",
                                "answer": "f"
                            }
                        ]
                    },
                    {
                        "player": "p2",
                        "answers": [
                            {
                                "player": "p1",
                                "answer": "f"
                            }
                        ]
                    }
                ]
            },
            {
                "question": "What word or phrase is always fun to say?",
                "answers": [],
                "guesses": []
            }
        ]
    }
    "#;
    let game: Game = serde_json::from_str(json_data).unwrap();
    game.get_score();
    let mut correct_scores = HashMap::new();
    correct_scores.insert(String::from("p1"), 2);
    correct_scores.insert(String::from("p2"), 2);
    assert_eq!(game.get_score(), correct_scores)
}

#[test]
fn test_get_score_with_wrong_answer() {
    let json_data = r#"
    {
        "players": [
            "p1",
            "p2"
        ],
        "rounds": [
            {
                "question": "What are you thankful you're not doing right now? ",
                "answers": [
                    {
                        "player": "p1",
                        "answer": "ijaefa"
                    },
                    {
                        "player": "p2",
                        "answer": "dlkamf"
                    }
                ],
                "guesses": [
                    {
                        "player": "p1",
                        "answers": [
                            {
                                "player": "p2",
                                "answer": "dlkamf"
                            }
                        ]
                    },
                    {
                        "player": "p2",
                        "answers": [
                            {
                                "player": "p1",
                                "answer": "a wrong answer"
                            }
                        ]
                    }
                ]
            },
            {
                "question": "What would be a great annual tradition? ",
                "answers": [
                    {
                        "player": "p1",
                        "answer": "f"
                    },
                    {
                        "player": "p2",
                        "answer": "f"
                    }
                ],
                "guesses": [
                    {
                        "player": "p1",
                        "answers": [
                            {
                                "player": "p2",
                                "answer": "f"
                            }
                        ]
                    },
                    {
                        "player": "p2",
                        "answers": [
                            {
                                "player": "p1",
                                "answer": "f"
                            }
                        ]
                    }
                ]
            },
            {
                "question": "What word or phrase is always fun to say?",
                "answers": [],
                "guesses": []
            }
        ]
    }
    "#;
    let game: Game = serde_json::from_str(json_data).unwrap();
    game.get_score();
    let mut correct_scores = HashMap::new();
    correct_scores.insert(String::from("p1"), 2);
    correct_scores.insert(String::from("p2"), 0);
    assert_eq!(game.get_score(), correct_scores)
}
