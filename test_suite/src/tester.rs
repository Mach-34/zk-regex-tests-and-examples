use std::{fmt::Display, path::Path, process::Command};

use anyhow::{bail, Context};
use rand::{self, prelude::Distribution};
use rand_regex::Regex;

use crate::{code::Code, constants, db::DbEntry};

/// Built-in errors for the testing phase.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This error is thrown when the test fails for at least one sample.
    #[error("the regex should recognize this string: {0:?}")]
    TestFailed(TestResult),
}

/// Result report for one test.
#[derive(Default, Debug)]
pub struct TestResult {
    /// All inputs that were correctly accepted or correctly rejected
    successful_tests: Vec<String>,
    /// Input should have been rejected, but was accepted
    false_positives: Vec<String>,
    /// Input should have been accepted, but was rejected
    false_negatives: Vec<String>,
}

impl TestResult {
    /// Creates a new test result.
    pub fn new(
        successful_tests: Vec<String>,
        false_positives: Vec<String>,
        false_negatives: Vec<String>,
    ) -> Self {
        Self {
            successful_tests,
            false_positives,
            false_negatives,
        }
    }

    /// Evaluates if the test passed or not. It returns true if the test is correct
    /// for all the samples, otherwise, if some test sample failed, it returns false.
    pub fn passed(&self) -> bool {
        self.false_positives.is_empty() && self.false_negatives.is_empty()
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if !self.successful_tests.is_empty() {
            output.push_str(&format!(
                "SUCCESS: The Noir code judged the following samples correctly:\n{:?}\n",
                self.successful_tests
            ));
        }
        if !self.false_negatives.is_empty() {
            output.push_str(&format!(
                "The following samples that should match the regex did NOT pass the Noir test:\n{:?}\n",
                self.false_negatives
            ));
        }
        if !self.false_positives.is_empty() {
            output.push_str(&format!(
                "The following samples that should NOT match the regex DID pass the Noir test:\n{:?}\n",
                self.false_positives
            ));
        }
        write!(f, "{}", output)
    }
}

/// Test a sample for a given regex.
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

    // Check whether the randomly generated samples pass the Noir test (they should)
    let (random_samples_correct, random_samples_wrong) =
        evaluate_test_set(code, &regex_input.regex.complete_regex(), &random_samples)?;

    // Samples from database input
    let (samples_pass_correct, samples_pass_wrong) = evaluate_test_set(
        code,
        &regex_input.regex.complete_regex(),
        &regex_input.samples_pass,
    )?;
    let (samples_fail_correct, false_positives) = evaluate_test_set(
        code,
        &regex_input.regex.complete_regex(),
        &regex_input.samples_fail,
    )?;

    // All correct results together
    let successful_tests = random_samples_correct
        .iter()
        .chain(samples_pass_correct.iter())
        .chain(samples_fail_correct.iter())
        .cloned()
        .collect();
    // All samples (random + db input) that should have passed, but didn't
    let false_negatives = random_samples_wrong
        .iter()
        .chain(samples_pass_wrong.iter())
        .cloned()
        .collect();

    let test_result = TestResult::new(
        successful_tests,
        false_positives, // Samples that should have been rejected, but weren't
        false_negatives,
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

        // Verify whether the Noir program passes or fails sample equal to a standard Rust lib
        let ground_truth_verification = check_with_ground_truth(string, regex, test_result)?;

        if !ground_truth_verification {
            failed_samples.push(string.clone());
        } else {
            successfull_samples.push(string.clone());
        }
    }
    Ok((successfull_samples, failed_samples))
}

/// Executes the `nargo test` command on the Noir project to test the result of the regex
/// from the Noir perspective.
fn test_noir_code() -> anyhow::Result<bool> {
    let output = Command::new("nargo")
        .arg("test")
        .current_dir(constants::DEFAULT_PROJECT_PATH)
        .output()
        .context("the test command was not executed successfully")?;
    Ok(output.status.success())
}

/// Compares the Noir test result from the `nargo test` command with respect to a traditional
/// regex Rust library.
fn check_with_ground_truth(string: &str, regex: &str, noir_result: bool) -> anyhow::Result<bool> {
    let ground_truth_checker =
        regex::Regex::new(regex).context("error parsing the regex in the ground truth checker")?;
    let ground_truth_result = ground_truth_checker.captures(string).is_some();
    Ok(ground_truth_result == noir_result)
}
