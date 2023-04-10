mod args;
mod config;
mod notes;
mod project;

use args::{FeatureType, RobinArgs};
use clap::Parser;
use config::config::Config;
use notes::notes::handle_note_command;
use project::project::handle_project_command;

fn main() {
    let config = Config::new();
    let cli_args = RobinArgs::parse();

    match cli_args.feature_type {
        FeatureType::Note(note) => handle_note_command(note),
        FeatureType::Project(project) => handle_project_command(project),
    }
}
