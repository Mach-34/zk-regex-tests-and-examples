use std::{fmt::Display, path::Path, process::Command};

use anyhow::{bail, Context};
use rand::{self, prelude::Distribution};
use rand_regex::Regex;

use crate::{code::Code, constants, db::DbEntry};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the regex should recognize this string: {0:?}")]
    TestFailed(TestResult),
}

#[derive(Default, Debug)]
pub struct TestResult {
    successful_fails: Vec<String>,
    successful_passes: Vec<String>,
    unsuccessful_fails: Vec<String>,
    unsuccessful_passes: Vec<String>,
}

impl TestResult {
    pub fn new(
        successful_fails: Vec<String>,
        successful_passes: Vec<String>,
        unsuccessful_fails: Vec<String>,
        unsuccessful_passes: Vec<String>,
    ) -> Self {
        Self {
            successful_fails,
            successful_passes,
            unsuccessful_fails,
            unsuccessful_passes,
        }
    }

    pub fn passed(&self) -> bool {
        self.unsuccessful_fails.is_empty() && self.unsuccessful_passes.is_empty()
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if !self.successful_passes.is_empty() {
            output.push_str(&format!(
                "The following samples that should pass the regex passed the Noir test:\n{:?}\n",
                self.successful_passes
            ));
        }
        if !self.successful_fails.is_empty() {
            output.push_str(&format!(
                "The following samples that should not match the regex did not pass the Noir test:\n{:?}\n",
                self.successful_fails
            ));
        }
        if !self.unsuccessful_passes.is_empty() {
            output.push_str(&format!(
                "The following samples that should match the regex did NOT pass the Noir test:\n{:?}\n",
                self.unsuccessful_passes
            ));
        }
        if !self.unsuccessful_fails.is_empty() {
            output.push_str(&format!(
                "The following samples that should NOT match the regex DID pass the Noir test:\n{:?}\n",
                self.unsuccessful_fails
            ));
        }
        write!(f, "{}", output)
    }
}

pub fn test_regex(regex_input: &DbEntry, code: &mut Code) -> anyhow::Result<TestResult> {
    // Generate the random strings
    let str_generator_result = Regex::compile(
        &regex_input.regex.complete_regex(),
        regex_input.input_size as u32,
    );

    // Generate the random samples. If somehow the compiler of the library does
    // not accept the provided regex, we log an error and we set the random samples to
    // be empty to continue the test with the user provided samples.
    let random_samples = match str_generator_result {
        Ok(str_generator) => {
            let rng = rand::thread_rng();
            str_generator
                .sample_iter(rng)
                .take(constants::DEFAULT_SAMPLE_NUMBER)
                .collect::<Vec<String>>()
        }
        Err(err) => {
            log::error!(
                "ignoring the random testing - 
                the random samples were not generated due to the following error: {:?}",
                err
            );
            Vec::new()
        }
    };

    let (random_succ_pass, random_unsucc_pass) =
        evaluate_test_set(code, &regex_input.regex.complete_regex(), &random_samples)?;
    let (passes_that_passed, passes_that_not_passed) = evaluate_test_set(
        code,
        &regex_input.regex.complete_regex(),
        &regex_input.samples_pass,
    )?;
    let (failures_that_failed, failures_that_not_fail) = evaluate_test_set(
        code,
        &regex_input.regex.complete_regex(),
        &regex_input.samples_fail,
    )?;

    let successful_passes = random_succ_pass
        .iter()
        .chain(passes_that_passed.iter())
        .cloned()
        .collect();
    let unsuccessful_passes = random_unsucc_pass
        .iter()
        .chain(passes_that_not_passed.iter())
        .cloned()
        .collect();

    let test_result = TestResult::new(
        failures_that_failed,
        successful_passes,
        failures_that_not_fail,
        unsuccessful_passes,
    );

    if !test_result.passed() {
        bail!(Error::TestFailed(test_result));
    }
    Ok(test_result)
}

/// Evaluates a test set of samples and returns a pair of the samples that were successful
/// and the samples that were not successful.
fn evaluate_test_set(
    code: &mut Code,
    regex: &str,
    test_set: &Vec<String>,
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let mut failed_samples = Vec::new();
    let mut successfull_samples = Vec::new();
    for string in test_set {
        // Set the testcase the current sample and write the main file.
        code.set_test_case(string);
        code.write_to_path(Path::new(constants::DEFAULT_PROJECT_MAIN_FILE))?;
        let test_result = test_noir_code()?;

        let ground_truth_verification = check_with_ground_truth(string, regex, test_result)?;

        if !ground_truth_verification {
            failed_samples.push(string.clone());
        } else {
            successfull_samples.push(string.clone());
        }
    }
    Ok((successfull_samples, failed_samples))
}

fn test_noir_code() -> anyhow::Result<bool> {
    let output = Command::new("nargo")
        .arg("test")
        .current_dir(constants::DEFAULT_PROJECT_PATH)
        .output()
        .context("the test command was not executed successfully")?;
    Ok(output.status.success())
}

fn check_with_ground_truth(string: &str, regex: &str, noir_result: bool) -> anyhow::Result<bool> {
    let ground_truth_checker =
        regex::Regex::new(regex).context("error parsing the regex in the ground truth checker")?;
    let ground_truth_result = ground_truth_checker.captures(string).is_some();
    Ok(ground_truth_result == noir_result)
}
