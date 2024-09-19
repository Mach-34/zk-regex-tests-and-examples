mod code;
mod compiler;
mod constants;
mod db;
mod tester;

use code::Code;
use db::RegexDb;
use log::{self, error, info};
use std::{error::Error, path::Path};
use tester::test_regex;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("starting regex tests");

    // Reads the database
    let database =
        RegexDb::load_from_file(Path::new(constants::DEFAULT_DATABASE_PATH)).map_err(|err| {
            error!("error creating the database: {}", err);
            err
        })?;

    for regex_input in database {
        info!("testing regex {}", regex_input.regex.as_str());
        let mut code_read_result = Code::new(&regex_input);
        match &mut code_read_result {
            Ok(code) => {
                let _ = code.write_to_path(Path::new(constants::DEFAULT_PROJECT_MAIN_FILE));
                let compilation_result = compiler::compile_noir_project();
                match compilation_result {
                    Ok(_) => info!(
                        "compilation success for regex {}",
                        regex_input.regex.as_str()
                    ),
                    Err(e) => {
                        error!(
                            "error compiling the noir project for regex {}: {:?}",
                            regex_input.regex.as_str(),
                            e
                        );
                        continue;
                    }
                }
                match test_regex(&regex_input, code) {
                    Ok(successfull_samples) => {
                        info!(
                            "sucess on checking {} samples for regex {}",
                            successfull_samples,
                            regex_input.regex.as_str()
                        );
                    }
                    Err(err) => match err.downcast_ref() {
                        Some(tester::Error::TestFailed(string_fail)) => {
                            error!(
                                "test failed for string {} for regex {}",
                                string_fail, regex_input.regex
                            )
                        }
                        Some(e) => {
                            error!("error generating the code: {:?}", e);
                        }
                        None => error!("error downcasting the anyhow::Error"),
                    },
                }
            }
            Err(err) => match err.downcast_ref() {
                Some(code::Error::CodeGenerationFailed(console_msg)) => {
                    error!("error generating the code: \n{}", console_msg);
                }
                None => error!("error downcasting the anyhow::Error"),
            },
        }
    }

    Ok(())
}
