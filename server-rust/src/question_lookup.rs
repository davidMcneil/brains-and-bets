use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const DEFAULT_QUESTION: &str = "Answer the question you would have liked to be asked?";

#[derive(Default)]
pub(crate) struct QuestionLookup {
    questions: Vec<String>,
}

impl QuestionLookup {
    pub(crate) fn populate_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.questions.push(line?);
        }
        Ok(())
    }

    pub(crate) fn get(&self) -> String {
        let mut rng = rand::thread_rng();
        self.questions
            .choose(&mut rng)
            .map_or_else(|| String::from(DEFAULT_QUESTION), |q| q.clone())
    }
}
