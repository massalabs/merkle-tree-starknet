use std::path::PathBuf;

use clap::{arg, command, value_parser};

pub struct Cli {
    pub scenario_dir: Option<PathBuf>,
    pub scenario_file: Option<PathBuf>,
}

impl Cli {
    pub fn parse() -> Cli {
        let matches = command!() // requires `cargo` feature
            .arg(
                arg!(
                    -d --scenario_dir <SCENARIO_DIR_PATH> "Sets scenario directory"
                )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(arg!(-f --file <SCENARIO_FILE_PATH> "Sets scenario file, disable if directory is provided").required(false).value_parser(value_parser!(PathBuf)))
            .get_matches();

        if let Some(dir) = matches.get_one::<PathBuf>("scenario_dir") {
            println!("scenario_dir: {:?}", dir);
            Cli {
                scenario_dir: Some(dir.clone()),
                scenario_file: None,
            }
        } else if let Some(file_path) = matches.get_one::<PathBuf>("file") {
            println!("scenario_file: {:?}", file_path);
            Cli {
                scenario_dir: None,
                scenario_file: Some(file_path.clone()),
            }
        } else {
            panic!("No scenario_dir or file provided")
        }
    }
}
