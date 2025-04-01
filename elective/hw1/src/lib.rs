use std::error::Error;
use std::fmt::Display;

pub mod collaborative_task;
pub mod ordinary_task;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Unstaged,
    Pending,
    Executing,
    Complete,
}
#[derive(Debug)]
pub struct BadTransition;

impl Display for BadTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Impossible transition")
    }
}

pub trait Task {
    fn description(&self) -> &str;
    fn name(&self) -> &str;
}

impl Error for BadTransition {}

#[derive(Debug)]
pub struct Collaborator {
    name: String,
    mail: String,
}

impl Collaborator {
    pub fn new(name: &str, mail: &str) -> Collaborator {
        Self {
            name: name.to_owned(),
            mail: mail.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mail(&self) -> &str {
        &self.mail
    }
}
