mod project;

use clap::{App, Arg, SubCommand};
use colored::*;
use project::{Project, ProjectType};

fn main() {
    let matches = App::new("Crane")
        .version("0.1.0")
        .about("C/C++ Package Manager")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new crane project")
                .arg(
                    Arg::with_name("new")
                        .value_name("PROJECT_NAME")
                        .help("The project name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .value_name("PROJECT_TYPE")
                        .help("Specifies the type of project (bin, lib, dylib)")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("new").unwrap_or("");
        let project_type = ProjectType::from_string(matches.value_of("type").unwrap_or("bin"));
        let project = Project::new(project_name, &project_type);

        match project {
            Ok(project) => {
                println!("{}", "Project created successfully".green());
                println!("{}", project.path.green());
            }
            Err(error) => {
                println!("{}", "Project creation failed".red());
                println!("{}", error.red());
            }
        }
    }
}
