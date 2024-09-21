use std::{
    fmt::{self, Display},
    path::Path,
    process::Command,
};

use anyhow::{bail, Context};
use rand::{self, prelude::Distribution};
use rand_regex::Regex;

use crate::{
    code::Code,
    constants,
    db::{DbEntry, InputWithSubstrs, RegexFragment, RegexInput, SamplesPass},
};
use std::fmt::Write;

/// Built-in errors for the testing phase.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This error is thrown when the test fails for at least one sample.
    #[error("the regex should recognize this string: {0:?}")]
    TestFailed(TestResult),
}

#[derive(Debug)]
pub enum TestResult {
    Standard(StandardTestResult),
    Substring(SubstringTestResult),
}

impl TestResult {
    pub fn passed(&self) -> bool {
        match self {
            TestResult::Standard(result) => result.passed(),
            TestResult::Substring(result) => result.passed(),
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Standard(standard) => write!(f, "StandardTestResult: {}", standard),
            TestResult::Substring(substring) => write!(f, "SubstringTestResult: {}", substring),
        }
    }
}

/// Result report for a standard test.
#[derive(Default, Debug)]
pub struct StandardTestResult {
    /// All inputs that were correctly accepted or correctly rejected
    successful_tests: Vec<String>,
    /// Input should have been rejected, but was accepted
    false_positives: Vec<String>,
    /// Input should have been accepted, but was rejected
    false_negatives: Vec<String>,
}

/// Result report for a test with substring generation.
#[derive(Default, Debug)]
pub struct SubstringTestResult {
    standard_test_result: StandardTestResult,
    /// Tests with substrings, but failed.
    /// These are the cases that should be rechecked manually
    incorrect_substring_tests: Vec<String>,
}

impl StandardTestResult {
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

impl Display for StandardTestResult {
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

impl Display for SubstringTestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        write!(&mut output, "{}", self.standard_test_result)?;
        if !self.incorrect_substring_tests.is_empty() {
            output.push_str(&format!(
                "These samples did not pass the test:\n{:?}\n",
                self.incorrect_substring_tests
            ));
        }
        write!(f, "{}", output)
    }
}

impl SubstringTestResult {
    pub fn new(
        successful_tests: Vec<String>,
        false_positives: Vec<String>,
        incorrect_substring_tests: Vec<String>,
    ) -> Self {
        Self {
            standard_test_result: StandardTestResult::new(
                successful_tests,
                false_positives,
                Vec::new(), // These results will be included in incorrect_substring_tests
            ),
            incorrect_substring_tests,
        }
    }

    /// Returns whether all tests passed correctly
    pub fn passed(&self) -> bool {
        self.standard_test_result.passed() && self.incorrect_substring_tests.is_empty()
    }
}

/// Tests a given regex:
/// - against randomly generate samples. Checks that they give the same outcome for Noir as for a Rust regex lib
///     (the random samples are assumed to pass in both)
/// - against input samples, of both passing and failing inputs
/// Note: raw + gen_substrs case does *not* get tested with randomly generated samples
pub fn test_regex(regex_input: &DbEntry, code: &mut Code) -> anyhow::Result<TestResult> {
    let test_result = match &regex_input.samples_pass {
        SamplesPass::WithSubstrs(samples) => {
            // Random sample testing for substrings is only done for decomposed setting
            let (random_samples_correct, incorrect_substring_tests1) = match &regex_input.regex {
                RegexInput::Decomposed(parts) => {
                    test_random_samples_gen_substrs(parts, regex_input.input_size as u32, code)?
                }
                _ => (Vec::new(), Vec::new()),
            };

            // Run tests for input samples. The test extracts substrings and compares them to the input for passing samples
            // For failing samples it does a standard test (no substring extraction)
            let (input_samples_correct, incorrect_substring_tests2, false_positives) =
                test_given_samples_gensubstr(code, samples, &regex_input.samples_fail)?;

            // Collect results
            let mut successful_tests = random_samples_correct;
            successful_tests.extend(input_samples_correct);
            let mut all_incorrect_substring_tests = incorrect_substring_tests1;
            all_incorrect_substring_tests.extend(incorrect_substring_tests2);
            TestResult::Substring(SubstringTestResult::new(
                successful_tests,
                false_positives,
                all_incorrect_substring_tests,
            ))
        }
        SamplesPass::WithoutSubstrs(samples_pass) => {
            // Test randomly generated samples: (probably) only passes are tested here
            // The result of a rust regex library is used to definitely decide whether it should be a pass or not
            // The Noir test is adjusted accordingly (if Rust says it should fail, the test fails and vice versa)
            let (random_samples_correct, random_samples_false_negatives) =
                test_for_random_samples(regex_input, code)?;

            // Test input samples
            let (input_samples_correct, false_positives, input_samples_false_negatives) =
                test_given_samples_standard(code, samples_pass, &regex_input.samples_fail)?;

            // Collect results
            let mut successful_tests = random_samples_correct;
            successful_tests.extend(input_samples_correct);
            let mut false_negatives = random_samples_false_negatives;
            false_negatives.extend(input_samples_false_negatives);

            TestResult::Standard(StandardTestResult::new(
                successful_tests,
                false_positives,
                false_negatives,
            ))
        }
    };

    if !test_result.passed() {
        bail!(Error::TestFailed(test_result));
    }
    Ok(test_result)
}

fn test_given_samples_gensubstr(
    code: &mut Code,
    samples_pass: &Vec<InputWithSubstrs>,
    samples_fail: &[String],
) -> anyhow::Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut correct_samples = Vec::new();
    let mut false_positives = Vec::new();
    let mut incorrect_substring_tests = Vec::new();

    // For passing samples check:
    // - regex match passes
    // - correct amount of substrings are extracted
    // - extracted substrings are correct
    for sample in samples_pass {
        let test_passed = run_single_substrs_test(code, sample)?;

        if test_passed {
            correct_samples.push(sample.input.clone());
        } else {
            // Not passing test can be because of incorrect regex match or incorrect substrings
            // Further manual testing will be needed to verify
            incorrect_substring_tests.push(sample.input.clone());
        }
    }

    // Samples fail are only checked on failing regex match;
    // No specific substrings are compared (since that doesn't make sense)
    for failing_sample in samples_fail {
        let correct_result = run_single_standard_test(code, failing_sample, true)?;

        if correct_result {
          correct_samples.push(failing_sample.clone());
        } else {
          false_positives.push(failing_sample.clone());
        }
    }

    Ok((correct_samples, incorrect_substring_tests, false_positives))
}

fn test_given_samples_standard(
    code: &mut Code,
    test_set_pass: &[String],
    test_set_fail: &[String],
) -> anyhow::Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut correct_samples = Vec::new();
    let mut false_positives = Vec::new();
    let mut false_negatives = Vec::new();

    // Helper function to process each test set
    let mut process_samples = |test_set: &[String], should_fail: bool| -> anyhow::Result<()> {
        for string in test_set {
            let correct_result = run_single_standard_test(code, string, should_fail)?;

            if correct_result {
                correct_samples.push(string.clone());
            } else if should_fail {
                false_positives.push(string.clone());
            } else {
                false_negatives.push(string.clone());
            }
        }
        Ok(())
    };

    // Process passing and failing sets
    process_samples(test_set_pass, false)?;
    process_samples(test_set_fail, true)?;

    Ok((correct_samples, false_negatives, false_positives))
}

fn test_random_samples_gen_substrs(
    regex_parts: &Vec<RegexFragment>,
    max_inputsize: u32,
    code: &mut Code,
) -> Result<(Vec<String>, Vec<String>), anyhow::Error> {
    let mut random_samples_correct = Vec::new();
    let mut incorrect_substring_tests = Vec::new();

    let mut rng = rand::thread_rng();

    // Run DEFAULT_SAMPLE_NUMBER of tests
    for _ in 0..constants::DEFAULT_SAMPLE_NUMBER {
        let mut substrings = Vec::<String>::new();
        let mut total_string = String::new();

        for part in regex_parts {
            // Create the regex for this part
            let regex_part = Regex::compile(&part.regex_def, max_inputsize);
            let sample = match regex_part {
                Ok(regex_part) => regex_part.sample(&mut rng),
                Err(err) => {
                    log::error!(
                        "ignoring the random testing - 
                the random samples were not generated due to the following error: {:?}",
                        err
                    );
                    String::new()
                }
            };

            if part.is_public {
                substrings.push(sample.clone());
            }
            // Concatenate this sample to the total_string
            total_string.push_str(&sample);
        }

        let input_with_substring = InputWithSubstrs {
            input: total_string.clone(),
            expected_substrings: substrings,
        };
        let test_passed = run_single_substrs_test(code, &input_with_substring)?;
        if test_passed {
            random_samples_correct.push(total_string);
        } else {
            incorrect_substring_tests.push(total_string);
        }
    }

    Ok((random_samples_correct, incorrect_substring_tests))
}

fn test_for_random_samples(
    regex_input: &DbEntry,
    code: &mut Code,
) -> Result<(Vec<String>, Vec<String>), anyhow::Error> {
    let str_generator_result = Regex::compile(
        &regex_input.regex.complete_regex(),
        regex_input.input_size as u32,
    );
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
    let (random_samples_correct, random_samples_wrong) =
        evaluate_test_set(code, &regex_input.regex.complete_regex(), &random_samples)?;
    Ok((random_samples_correct, random_samples_wrong))
}

/// Fill the standard testcase with given test input and run test
fn run_single_standard_test(
    code: &mut Code,
    string: &str,
    should_fail: bool,
) -> Result<bool, anyhow::Error> {
    code.set_standard_test_case(string);
    code.set_should_fail(should_fail);
    create_file_and_test(code)
}

/// Fill the substring testcase with given test input & expected substrings, and run test
fn run_single_substrs_test(
    code: &mut Code,
    sample: &InputWithSubstrs,
) -> Result<bool, anyhow::Error> {
    code.set_substrs_test_case(sample);
    create_file_and_test(code)
}

fn create_file_and_test(code: &mut Code) -> Result<bool, anyhow::Error> {
    code.write_to_path(Path::new(constants::DEFAULT_PROJECT_MAIN_FILE))?;
    let test_result = test_noir_code()?;
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
        let ground_truth_checker = regex::Regex::new(regex)
            .context("error parsing the regex in the ground truth checker")?;
        // Check with a Rust regex lib whether this input should pass
        let ground_truth_result = ground_truth_checker.captures(string).is_some();

        // Use the Rust regex result to decide whether this test should pass of fail
        let correct_result = run_single_standard_test(code, string, !ground_truth_result)?;

        if !correct_result {
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
