use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::types::{GetQuestionLocation, Question};

const DEFAULT_QUESTION: &str = "What question would you like to be asked?";

#[derive(Default)]
pub(crate) struct QuestionLookup {
    questions: Vec<Question>,
    question_idx: AtomicUsize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct NumbersApiResponse {
    text: String,
    number: u32,
    found: bool,
    r#type: String,
}

impl QuestionLookup {
    pub(crate) fn populate_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let values: Vec<&str> = line.split(',').collect();
            self.questions.push(Question {
                question: values[0].to_string(),
                answer: values[1]
                    .parse()
                    .expect(&format!("value after comma should be a number: {}", line)),
            });
            let mut rng = rand::thread_rng();
            self.questions.shuffle(&mut rng);
        }
        Ok(())
    }

    pub(crate) async fn get(&self, get_question_from: GetQuestionLocation) -> Question {
        match get_question_from {
            GetQuestionLocation::File => self.get_from_file(),
            GetQuestionLocation::NumbersApi => {
                for _ in 0..5 {
                    if let Ok(question) = get_question_from_numbers_api().await {
                        return question;
                    }
                }
                self.get_from_file()
            }
        }
    }

    pub(crate) fn get_from_file(&self) -> Question {
        if self.questions.is_empty() {
            return Question {
                question: String::from(DEFAULT_QUESTION),
                answer: 0,
            };
        }
        let index = self
            .question_idx
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| {
                Some(if v >= self.questions.len() - 1 {
                    0
                } else {
                    v + 1
                })
            });
        let index = match index {
            Ok(v) | Err(v) => v,
        };
        self.questions[index].clone()
    }
}

async fn get_question_from_numbers_api() -> Result<Question, reqwest::Error> {
    let numbers_api_response: NumbersApiResponse =
        reqwest::get("http://numbersapi.com/random/trivia?json")
            .await?
            .json()
            .await?;
    let mut question = numbers_api_response.text;
    question = question.replace(&numbers_api_response.number.to_string(), "What");
    question.pop();
    question.push('?');
    Ok(Question {
        question,
        answer: numbers_api_response.number,
    })
}
