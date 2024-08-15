use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HighScore {
    pub username: String,
    pub score: u64,
}


#[derive(Serialize, Deserialize)]
pub struct BugReport {
    pub name: String,
    pub email: String,
    pub date: String,
    pub problem: String,
    pub severity: u8,
}


impl BugReport {
    // Ensure each parameter is within the specified bounds
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.name.len() <= 0 {
            return Err("Name field can't be empty");
        }
        if self.name.len() > 30 {
            return Err("Name must be at most 30 characters long");
        }
        if self.email.len() <= 0 {
            return Err("Email field can't be empty");
        }
        if self.email.len() > 50 {
            return Err("Email must be at most 50 characters long");
        }
        if self.problem.len() <= 0 {
            return Err("Problem field can't be empty");
        }
        if self.problem.len() > 10000 {
            return Err("Problem must be at most 10000 characters long");
        }
        if self.severity > 10 {
            return Err("Severity must be between 0 and 10");
        }

        Ok(())
    }
}
