use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::types::Question;

const DEFAULT_QUESTION: &str = "Answer the question you would have liked to be asked?";

#[derive(Default)]
pub(crate) struct QuestionLookup {
    questions: Vec<Question>,
}

impl QuestionLookup {
    pub(crate) fn populate_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            // TODO: parse the line to get the question and answer (eg `<answer>,<question>`)
            self.questions.push(Question {
                question: line?,
                answer: 0,
            });
        }
        Ok(())
    }

    pub(crate) fn get(&self) -> Question {
        // TODO: this would be better to randomize the questions and then do them in order to avoid duplicates
        let mut rng = rand::thread_rng();
        self.questions.choose(&mut rng).map_or_else(
            || Question {
                question: String::from(DEFAULT_QUESTION),
                answer: 0,
            },
            |q| q.clone(),
        )
    }
}
