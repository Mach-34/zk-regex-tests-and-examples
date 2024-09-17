mod code;
mod compiler;
mod db;

use code::Code;
use db::RegexDb;
use log::{self, error, info};
use std::{error::Error, path::Path};

const DEFAULT_DATABASE_PATH: &str = "./regex_db.txt";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Starting regex tests");

    // Reads the database
    let database = RegexDb::load_from_file(Path::new(DEFAULT_DATABASE_PATH)).map_err(|err| {
        error!("error creating the database: {}", err);
        err
    })?;

    for (regex, input_size) in database {
        info!("Testing regex {}", regex.as_str());
        let code_read_result = Code::new(regex.clone(), input_size);
        match code_read_result {
            Ok(code) => {
                let _ = compiler::write_code_to_project(code).map_err(|err| {
                    error!("error writing the code into the noir project: {err}");
                    err
                });
                let compilation_result = compiler::compile_noir_project().map_err(|err| {
                    error!("error compiling the noir project: {err}");
                    err
                });
                if compilation_result.is_ok() {
                    info!("Compilation success for regex {}", regex.as_str());
                }
            }
            Err(err) => error!("error generating the code: {}", err),
        }
    }

    Ok(())
}
