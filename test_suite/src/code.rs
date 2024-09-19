use std::{fmt::Display, fs, path::Path, process::Command};

use anyhow::Context;

use crate::{constants, db::RegexInput};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error generating the code from the regex")]
    CodeGenerationFailed(String),
}

pub struct Code {
    noir_code: String,
    input_size: usize,
    test_case: Option<String>,
}

impl Code {
    pub fn new(regex_input: &RegexInput) -> anyhow::Result<Self> {
        let noir_code = generate_noir_code(
            &regex_input.regex,
            Path::new(constants::DEFAULT_GENERATION_PATH),
        )
        .context("error generating the noir code")?;
        Ok(Self {
            noir_code,
            input_size: regex_input.input_size,
            test_case: None,
        })
    }

    pub fn set_test_case(&mut self, test_case: &str) {
        self.test_case = Some(String::from(test_case));
    }

    pub fn write_to_path(&self, path: &Path) -> anyhow::Result<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.test_case {
            Some(test_case) => {
                write!(
                    f,
                    "{}\nfn main(input: [u8; {}]) {{ regex_match(input); }}\n\n#[test]\nfn test() {{ let input = {:?}; regex_match(input); }}",
                    self.noir_code, self.input_size, test_case.as_bytes()
                )
            }
            None => {
                write!(
                    f,
                    "{}\nfn main(input: [u8; {}]) {{ regex_match(input); }}",
                    self.noir_code, self.input_size,
                )
            }
        }
    }
}

fn generate_noir_code(regex: &str, result_path: &Path) -> anyhow::Result<String> {
    let output = Command::new("zk-regex")
        .args(["raw", "--raw-regex"])
        .arg(regex)
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
