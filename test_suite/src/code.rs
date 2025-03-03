use std::{
    fs::{self, File},
    io::{BufWriter, Write as _},
    path::Path,
    process::Command,
};

use std::fmt::Write;

use anyhow::Context;

use crate::{
    constants::{self, DEFAULT_DECOMPOSED_JSON_FILE, DEFAULT_SUBSTRS_JSON_PATH},
    db::{ComponentsWrapper, DbEntry, InputWithSubstrs, RawRegex, RegexInput},
};

/// Errors that can arise when generating the Noir code
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error is used when the generation of the code is not successful and
    /// contains the regex that produced such an error.
    #[error("error generating the code from the regex")]
    CodeGenerationFailed(String),
}

/// Represents the information to construct a noir code.
pub struct Code {
    /// Code generated using the zk-email tool.
    noir_code: String,
    /// Input size of provided to the main function in the Noir project.
    input_size: usize,
}

impl Code {
    /// Creates a new code from the inputs of the database.
    pub fn new(regex_input: &DbEntry) -> anyhow::Result<Self> {
        let noir_code = generate_noir_code(
            &regex_input.regex,
            regex_input.gen_substrs,
            Path::new(constants::DEFAULT_GENERATION_PATH),
        )
        .context("error generating the noir code")?;
        Ok(Self {
            noir_code,
            input_size: regex_input.input_size,
        })
    }

    /// Writes the current source code into a file in a given path.
    pub fn write_to_path(&self, path: &Path) -> anyhow::Result<()> {
        fs::write(path, self.print_code(None, None, false))
            .context(format!("error writing the code to the path {:?}", path))?;
        Ok(())
    }
    pub fn write_test_to_path(
        &self,
        standard_test: Option<&String>,
        gen_substr_test: Option<&InputWithSubstrs>,
        should_fail: bool,
        path: &Path,
    ) -> anyhow::Result<()> {
        fs::write(
            path,
            self.print_code(standard_test, gen_substr_test, should_fail),
        )
        .context(format!("error writing the code to the path {:?}", path))?;
        Ok(())
    }

    pub fn print_code(
        &self,
        standard_test: Option<&String>,
        gen_substr_test: Option<&InputWithSubstrs>,
        // gen_substrs tests always pass, but standard tests might be expected to fail
        should_fail: bool,
    ) -> String {
        let mut s = String::new();

        match (standard_test, gen_substr_test) {
            // Handle the standard test case
            (Some(test_case), None) => {
                write!(
                    &mut s,
                    "{}\nfn main(input: [u8; {}]) {{ regex_match(input); }}\n\n{}\nfn test() {{\n\
                  let input = {:?};\nregex_match(input);\n\
                  }}",
                    self.noir_code,  // Noir code part of `Code`
                    self.input_size, // Input size for the main function
                    if should_fail {
                        "#[test(should_fail)]"
                    } else {
                        "#[test]"
                    },
                    test_case.as_bytes() // Test case converted to byte array
                )
                .unwrap();
            }

            // Handle the substring test case
            (
                _,
                Some(InputWithSubstrs {
                    input: input_byte_array,
                    expected_substrings,
                }),
            ) => {
                write!(
                    &mut s,
                    "{}\nfn main(input: [u8; {}]) {{ regex_match(input); }}\n\n#[test]\nfn test() {{\n\
                  // Input for regex match\n\
                  let input = {:?};\n\
                  // This should contain {} substrings\n\
                  let capture_sequences = regex_match(input);\n\
                  assert(capture_sequences.len() == {});\n\
                  let substrings = extract_all_substrings::<_, _, {}>(input, capture_sequences);",
                    self.noir_code,  // Noir code part of `Code`
                    self.input_size, // Input size for the main function
                    input_byte_array.as_bytes(), // Byte array input for the regex
                    expected_substrings.len(),   // Number of expected substrings
                    expected_substrings.len(),    // Assertion: number of substrings
                    input_byte_array.as_bytes().len() // Input size reused for max substring length (just inefficient, not insecure)
                )
                .unwrap();

                // Iterate over expected substrings and generate assertions
                for (i, substr) in expected_substrings.iter().enumerate() {
                    writeln!(s, "let substr{} = substrings.get({});", i, i).unwrap();
                    for (j, byte) in substr.bytes().enumerate() {
                        writeln!(s, "assert(substr{}.get({}) == {});", i, j, byte).unwrap();
                    }
                    writeln!(s, "assert(substr{}.len() == {});", i, substr.len()).unwrap();
                }

                writeln!(s, "}}").unwrap(); // Close the test function
            }

            // Default case: no test case provided
            _ => {
                write!(
                    &mut s,
                    "{}\nfn main(input: [u8; {}]) {{ regex_match(input); }}",
                    self.noir_code, self.input_size,
                )
                .unwrap();
            }
        }
        s
    }
}

/// Function that generates the Noir code associated to a regex.
fn generate_noir_code(
    regex: &RegexInput,
    gen_substrs: bool,
    result_path: &Path,
) -> anyhow::Result<String> {
    let mut command = Command::new("zk-regex");
    match regex {
        RegexInput::Raw(RawRegex::Simple(regex_str)) => {
            command.args(["raw", "--raw-regex"]).arg(regex_str);
        }
        RegexInput::Raw(RawRegex::WithTransitions {
            regex: regex_str,
            transitions,
        }) => {
            command.args(["raw", "--raw-regex"]).arg(regex_str);
            // If substrings should be extracted, add the transitions file
            if gen_substrs {
                // Write the parts to the JSON file
                let json_file = File::create(DEFAULT_SUBSTRS_JSON_PATH)?;
                let mut writer = BufWriter::new(json_file);
                serde_json::to_writer(&mut writer, transitions)
                    .context("error writing the transitions of raw regex for gen_substrs")?;
                writer
                    .flush()
                    .context("error flushing the writer to the JSON file")?;

                command.args(["-s", DEFAULT_SUBSTRS_JSON_PATH]);
            }
        }
        RegexInput::Decomposed(parts) => {
            // Write the parts to the JSON file
            let json_file = File::create(DEFAULT_DECOMPOSED_JSON_FILE)?;
            let mut writer = BufWriter::new(json_file);
            serde_json::to_writer(&mut writer, &ComponentsWrapper::new(parts.to_vec()))
                .context("error writing the parts of the decomposed regex")?;
            writer
                .flush()
                .context("error flushing the writer to the JSON file")?;

            // Add the command arguments
            command
                .arg("decomposed")
                .arg("-d")
                .arg(DEFAULT_DECOMPOSED_JSON_FILE);
        }
    };

    // If substrings should be extracted, add the command
    if gen_substrs {
        command.args(["-g", "true"]);
    }

    let output = command
        .arg("--noir-file-path")
        .arg(result_path)
        .output()
        .context("error executing the noir generation command")?;

    if !output.status.success() {
        anyhow::bail!(Error::CodeGenerationFailed(String::from_utf8(
            output.stderr
        )?));
    }

    // Load code from stored file.
    let noir_generated_code =
        fs::read_to_string(result_path).context("error writing the noir code into the file")?;

    Ok(noir_generated_code)
}
