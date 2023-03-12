use crate::args::{CreateProject, OpenProject, ProjectCommand, ProjectSubcommand};

pub fn handle_project_command(project: ProjectCommand) {
    let command = project.command;
    match command {
        ProjectSubcommand::Create(project) => create_project(project),
        ProjectSubcommand::Open(project) => open_project(project),
        ProjectSubcommand::Show => show_all_projects(),
    }
}

fn create_project(project: CreateProject) {
    println!("Creating new project: {:?}", project);
    todo!("Create new Project")
}

fn open_project(project: OpenProject) {
    println!("Opening project: {:?}", project);
    todo!("Open project")
}

fn show_all_projects() {
    todo!("Show all projects")
}
