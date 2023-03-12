use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RobinArgs {
    #[clap(subcommand)]
    pub feature_type: FeatureType,
}

#[derive(Debug, Subcommand)]
pub enum FeatureType {
    /// Create, Open, Delete or Show all Notes
    Note(NoteCommand),
    /// Create, Open or Show all Projects
    Project(ProjectCommand),
}

// Note
#[derive(Debug, Args)]
pub struct NoteCommand {
    #[clap(subcommand)]
    pub command: NoteSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum NoteSubcommand {
    /// Create New Note
    Create(CreateNote),
    /// Open Existing Note
    Open(OpenNote),
    /// Delete a Note
    Delete(DeleteNote),
    /// Show all Notes
    Show,
}

#[derive(Debug, Args)]
pub struct CreateNote {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct OpenNote {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteNote {
    pub name: String,
}

// Project
#[derive(Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    /// Create New Project
    Create(CreateProject),
    /// Open Project
    Open(OpenProject),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateProject {
    pub project_name: String,
}

#[derive(Debug, Args)]
pub struct OpenProject {
    pub project_name: String,
}
