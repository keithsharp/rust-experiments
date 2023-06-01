use std::fmt::Display;

use rusqlite::Connection;

#[derive(Debug)]
struct Project {
    id: u64,
    name: String,
    description: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} - {}", self.id, self.name, self.description)
    }
}

fn main() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;

    println!("Connecting to in memory database");
    conn.execute(
        "CREATE TABLE projects (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        (),
    )?;

    let project = Project {
        id: 0,
        name: "Test Project".to_string(),
        description: "This is a test project".to_string(),
    };

    println!("Inserting a project into the database");
    conn.execute(
        "INSERT INTO projects (name, description) VALUES (?1, ?2)",
        (&project.name, &project.description),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, description FROM projects")?;
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

    for project in projects {
        println!("{}", project);
    }

    Ok(())
}
