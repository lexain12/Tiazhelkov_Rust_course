use crate::BadTransition;
use crate::Collaborator;
use crate::Status;
use crate::Task;

#[derive(Debug)]
pub struct CollaborativeTask<'a> {
    name: String,
    description: String,
    status: Status,
    collaborators: Vec<&'a Collaborator>,
}

impl Task for CollaborativeTask<'_> {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl<'a> CollaborativeTask<'a> {
    pub fn new(name: String, description: String, collaborator: Option<&'a Collaborator>) -> Self {
        Self {
            name,
            description,
            status: Status::Unstaged,
            collaborators: if collaborator.is_some() {
                vec![collaborator.unwrap()]
            } else {
                vec![]
            },
        }
    }
    pub fn add_collaborator(&mut self, collaborator: &'a Collaborator) {
        self.collaborators.push(collaborator);
    }

    pub fn get_collaborators(&self) -> &Vec<&'a Collaborator> {
        &self.collaborators
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn stage(&mut self) -> Result<(), BadTransition> {
        match self.status {
            Status::Unstaged => {
                self.status = Status::Pending;
                Ok(())
            }
            _ => Err(BadTransition),
        }
    }

    pub fn execute(&mut self) -> Result<(), BadTransition> {
        match self.status {
            Status::Pending => {
                self.status = Status::Executing;
                Ok(())
            }
            _ => Err(BadTransition),
        }
    }

    pub fn complete(&mut self) -> Result<(), BadTransition> {
        match self.status {
            Status::Executing => {
                self.status = Status::Complete;
                Ok(())
            }
            _ => Err(BadTransition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let name = String::from("Task");
        let description = String::from("My new beatiful task");
        let collaborator = Collaborator::new("Viktor", "lexain12@gmail.com");
        let task = CollaborativeTask::new(name.clone(), description.clone(), Some(&collaborator));
        assert_eq!(task.get_status(), Status::Unstaged);
        assert_eq!(task.name(), &name);
        assert_eq!(task.description(), &description);
    }
    #[test]
    fn add_collaborator() {
        let name = String::from("Task");
        let description = String::from("My new beatiful task");
        let collaborator = Collaborator::new("Viktor", "lexain12@gmail.com");
        let mut task =
            CollaborativeTask::new(name.clone(), description.clone(), Some(&collaborator));

        let new_collaborator = Collaborator::new("Alex", "alex12@gmail.com");
        task.add_collaborator(&new_collaborator);
        let collaborators = vec![&collaborator, &new_collaborator];
        assert_eq!(
            task.get_collaborators()
                .iter()
                .zip(&collaborators)
                .filter(|&(a, b)| a.name == b.name && a.mail == b.mail)
                .count(),
            2
        );
    }

    #[test]
    fn possible_transition() -> Result<(), BadTransition> {
        let name = String::from("Task");
        let description = String::from("My new beatiful task");
        let mut task = CollaborativeTask::new(name.clone(), description.clone(), None);

        task.stage()?;
        assert_eq!(task.get_status(), Status::Pending);
        task.execute()?;
        assert_eq!(task.get_status(), Status::Executing);
        task.complete()?;
        assert_eq!(task.get_status(), Status::Complete);

        Ok(())
    }
    #[test]
    fn impossible_transitions() -> Result<(), BadTransition> {
        let name = String::from("Task");
        let description = String::from("My new beatiful task");
        let mut task = CollaborativeTask::new(name.clone(), description.clone(), None);

        task.execute().expect_err("");
        task.complete().expect_err("");

        task.stage()?;
        task.complete().expect_err("");
        task.stage().expect_err("");

        task.execute()?;
        task.execute().expect_err("");
        task.stage().expect_err("");

        task.complete()?;
        task.complete().expect_err("");
        task.execute().expect_err("");
        task.stage().expect_err("");

        Ok(())
    }
}
