use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::types::Question;

const DEFAULT_QUESTION: &str = "What question would you like to be asked?";

#[derive(Default)]
pub(crate) struct QuestionLookup {
    questions: Vec<Question>,
    question_idx: usize,
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
                    .expect("value after comma should be a number"),
            });
            let mut rng = rand::thread_rng();
            self.questions.shuffle(&mut rng);
        }
        Ok(())
    }

    pub(crate) fn get(&mut self) -> Question {
        if self.questions.is_empty() {
            return Question {
                question: String::from(DEFAULT_QUESTION),
                answer: 0,
            };
        }
        let question = self.questions[self.question_idx].clone();
        self.question_idx += 1;
        if self.question_idx == self.questions.len() {
            self.question_idx = 0;
        }
        question
    }
}
