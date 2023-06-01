use sql_rusqlite::{data::DataStore, Project};

fn main() -> anyhow::Result<()> {
    println!("Creating new in-memory datastore");
    let ds = DataStore::new(None)?;

    let project = Project::new(
        "Test Project".to_string(),
        "This is a test project".to_string(),
    );

    println!("Adding a project to the datastore");
    ds.add_project(&project)?;

    println!("Getting all projects from the datastore");
    let projects = ds.get_projects()?;
    for project in projects {
        println!("  {}", project);
    }

    println!(
        "Getting project with name '{}' from the datastore",
        project.name()
    );
    let mut project = ds
        .get_project_with_name("Test Project")?
        .expect("should always be able to get a project");
    println!("  {}", project);

    println!("Updating the project description");
    project.set_description("There has been a change of plan".to_string());
    ds.update_project(&project)?;

    println!(
        "Getting project with ID '{}' from the datastore",
        project.id()
    );
    let project = ds
        .get_project_with_id(1)?
        .expect("should always be able to get a project");
    println!("  {}", project);

    println!(
        "Deleting project with ID '{}' from the datastore",
        project.id()
    );
    ds.delete_project(project)?;

    let projects = ds.get_projects()?;
    println!("There are {} projects in the datastore", projects.len());

    Ok(())
}
