use crate::BadTransition;
use crate::Status;
use crate::Task;

#[derive(Debug)]
pub struct OrdinaryTask {
    name: String,
    description: String,
    status: Status,
}

impl Task for OrdinaryTask {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl OrdinaryTask {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Unstaged,
        }
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
        let task = OrdinaryTask::new(name.clone(), description.clone());
        assert_eq!(task.get_status(), Status::Unstaged);
        assert_eq!(task.name(), &name);
        assert_eq!(task.description(), &description);
    }

    #[test]
    fn possible_transition() -> Result<(), BadTransition> {
        let name = String::from("Task");
        let description = String::from("My new beatiful task");
        let mut task = OrdinaryTask::new(name.clone(), description.clone());

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
        let mut task = OrdinaryTask::new(name.clone(), description.clone());

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
