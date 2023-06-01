use std::fmt::Display;

pub mod data;
pub mod errors;

pub use errors::Error;

#[derive(Debug)]
pub struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} - {}", self.id, self.name, self.description)
    }
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 0,
            name,
            description,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
