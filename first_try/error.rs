use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct LexerError {
    kind: ErrorKind,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.kind {
            ErrorKind::Match { step, source } => {
                write!(
                    f,
                    "Couldn't match. Failed working on step: {}. Source is: {}.",
                    step, source
                )
            }
            ErrorKind::Choose { tries } => {
                let chains = tries.iter().map(|e| e.chain());
                todo!()
            }
        }
    }
}

impl Error for LexerError {}

impl LexerError {
    pub fn matches(step: &str, source: &str) -> Self {
        Self {
            kind: ErrorKind::Match {
                step: step.into(),
                source: source.into(),
            },
        }
    }
    pub fn choose(tries: Vec<anyhow::Error>) -> Self {
        Self {
            kind: ErrorKind::Choose { tries },
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Match { step: String, source: String },
    Choose { tries: Vec<anyhow::Error> },
}
