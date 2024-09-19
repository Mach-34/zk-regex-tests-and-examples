use std::{path::Path, process::Command};

use rand::{self, prelude::Distribution};
use rand_regex::Regex;

use crate::{code::Code, constants, db::RegexInput};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the regex should recognize this string: {0:?}")]
    TestFailed(String),
    #[error("error constructing the random generator: {0:?}")]
    WrongGeneratorConstruction(rand_regex::Error),
}

pub fn test_regex(regex_input: &RegexInput, code: &mut Code) -> anyhow::Result<usize> {
    // Generate the random strings
    let str_generator = Regex::compile(regex_input.regex.as_str(), regex_input.input_size as u32)
        .map_err(Error::WrongGeneratorConstruction)?;
    let rng = rand::thread_rng();
    let samples: Vec<String> = str_generator
        .sample_iter(rng)
        .take(constants::DEFAULT_SAMPLE_NUMBER)
        .collect();

    for string in &samples {
        // Set the testcase the current sample and write the main file.
        code.set_test_case(string);
        code.write_to_path(Path::new(constants::DEFAULT_PROJECT_MAIN_FILE))?;
        let test_result = test_noir_code();

        let ground_truth_verification =
            check_with_ground_truth(string, regex_input.regex.as_str(), test_result);
        if !ground_truth_verification {
            anyhow::bail!(Error::TestFailed(string.clone()));
        }
    }

    Ok(samples.len())
}

fn test_noir_code() -> bool {
    let output = Command::new("nargo")
        .arg("test")
        .current_dir(constants::DEFAULT_PROJECT_PATH)
        .output()
        .expect("the test command should be executed to get the test result");
    output.status.success()
}

fn check_with_ground_truth(string: &str, regex: &str, noir_result: bool) -> bool {
    let ground_truth_checker =
        regex::Regex::new(regex).expect("error parsing the regex in the ground truth checker");
    ground_truth_checker.captures(string).is_some() == noir_result
}
