use std::path::Path;

use rusqlite::Connection;

use crate::{errors::Error, Project};

pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn new(file: Option<&Path>) -> Result<Self, Error> {
        let ds = DataStore::open(file)?;
        ds.init_tables()?;

        Ok(ds)
    }

    pub fn open(file: Option<&Path>) -> Result<Self, Error> {
        let conn = match file {
            Some(file) => Connection::open(file)?,
            None => Connection::open_in_memory()?,
        };

        let ds = Self { conn };
        Ok(ds)
    }

    pub fn add_project(&self, project: &Project) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO projects (name, description) VALUES (?1, ?2)",
            (&project.name, &project.description),
        )?;

        Ok(())
    }

    pub fn delete_project(&self, project: Project) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM projects WHERE id=?1",
            &[&project.id.to_string()],
        )?;

        Ok(())
    }

    pub fn update_project(&self, project: &Project) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE projects SET name=?2, description=?3 WHERE id=?1",
            (&project.id, &project.name, &project.description),
        )?;

        Ok(())
    }

    pub fn get_project_with_id(&self, id: u64) -> Result<Option<Project>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM projects WHERE id=?1")?;
        let mut projects: Vec<Project> = stmt
            .query_map([&id.to_string()], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if projects.len() == 1 {
            return Ok(Some(projects.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_project_with_name(&self, name: &str) -> Result<Option<Project>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM projects WHERE name=?1")?;
        let mut projects: Vec<Project> = stmt
            .query_map([name], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if projects.len() == 1 {
            return Ok(Some(projects.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM projects")?;
        let projects: Vec<_> = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(projects)
    }
}

impl DataStore {
    fn init_tables(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }
}
