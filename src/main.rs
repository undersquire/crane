mod project;

use clap::{App, Arg};
use colored::*;
use project::{Project, ProjectType};

fn main() {
    let matches = App::new("Crane")
        .version("0.1.0")
        .about("C/C++ Package Manager")
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("new")
                .value_name("PROJECT_NAME")
                .help("Creates a new crane project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("init")
                .short("i")
                .long("init")
                .help("Initializes the current directory as a new crane project"),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("PROJECT_TYPE")
                .help("Specifies the type of project (bin, lib, dylib)")
                .takes_value(true),
        )
        .get_matches();

    if let Some(proj_name) = matches.value_of("new") {
        let proj_type = match matches.value_of("type") {
            Some(x) => ProjectType::from_string(x),
            None => ProjectType::from_string("bin"),
        };

        match Project::new(proj_name, &proj_type) {
            Ok(_) => println!(
                "{} {} `{}` project",
                "Created".bright_green(),
                proj_type.to_string(),
                proj_name
            ),
            Err(e) => println!("{}: {}", "error".bright_red(), e),
        }
    }
}
