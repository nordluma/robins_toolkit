use cmd_args::cmd_parser::ArgParse;
use serde_derive::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fmt::format,
    fs::{self, create_dir},
    path::Path,
    process,
};
use whoami;

mod cmd_args;
mod notes;

#[derive(Deserialize)]
struct Config {
    setup: Setup,
    defaults: DefaultValues,
    paths: Paths,
}

#[derive(Deserialize)]
struct Setup {
    complete: bool,
}

#[derive(Deserialize)]
struct DefaultValues {
    root_folder: String,
    notes_handle: String,
    projects_handle: String,
}

#[derive(Deserialize, Serialize)]
struct Paths {
    notes_folder: Option<String>,
    projects_folder: Option<String>,
}

fn main() {
    const CONFIG_FILE: &str = "settings.toml";
    let app_config = load_config(String::from(CONFIG_FILE));

    let mut configs = match app_config {
        Ok(c) => c,
        Err(_) => process::exit(1),
    };

    if configs.setup.complete == false {
        set_paths(&mut configs).expect("Should Created new paths");
    }

    let arguments: Vec<String> = env::args().collect();
    let args_str = ArgParse::build(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(args_str);
}

fn load_config(config_path: String) -> Result<Config, Box<dyn Error>> {
    let contents = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not load configurations");
            process::exit(1);
        }
    };

    let config = toml::from_str(&contents)?;
    Ok(config)
}

fn set_paths(config: &Config) -> Result<(), Box<dyn Error>> {
    let user = whoami::username();

    let notes_path = format!(
        "{}{}{}",
        &config.defaults.root_folder, &user, &config.defaults.notes_handle
    );
    if !Path::new(&notes_path).exists() {
        create_dir(notes_path)?;
    }

    let projects_path = format!(
        "{}{}{}",
        &config.defaults.root_folder, &user, &config.defaults.projects_handle
    );
    if !Path::new(&projects_path).exists() {
        create_dir(projects_path)?;
    }

    Ok(())
}

fn run(cmd: ArgParse) /*-> Result<(), Box<dyn Error>>*/
{
    todo!();
}
