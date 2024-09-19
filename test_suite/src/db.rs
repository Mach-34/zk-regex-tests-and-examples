use std::{fs, path::Path};

use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;

/// Database of regular expressions that will be tested.
pub struct RegexDb(Vec<RegexInput>);

#[derive(Deserialize)]
pub struct RegexInput {
    pub regex: String,
    pub input_size: usize,
    pub format: String,
    pub samples_pass: Vec<String>,
    pub samples_fail: Vec<String>,
}

impl RegexInput {
    pub fn new(
        regex_str: &str,
        format: String,
        input_size: usize,
        samples_pass: Vec<String>,
        samples_fail: Vec<String>,
    ) -> Self {
        Self {
            regex: String::from(regex_str),
            input_size,
            format,
            samples_pass,
            samples_fail,
        }
    }
}

impl RegexDb {
    /// Constructs a database from a file where teh file contains the information in the following format:
    ///     - Regex1
    ///     - InputSize1
    ///     - Regex2
    ///     - InputSize2
    ///     - ...
    pub fn load_from_file(file_path: &Path) -> anyhow::Result<Self> {
        let file_regex_content =
            fs::read_to_string(file_path).context("error reading the regex content")?;
        let json_value: Value =
            serde_json::from_str(&file_regex_content).context("error parsing the json")?;
        let regex_db: Vec<RegexInput> = serde_json::from_str(&json_value["database"].to_string())
            .context("error parsing the database array")?;

        let mut regexes = Vec::new();

        for db_element in regex_db {
            regexes.push(db_element);
        }

        Ok(Self(regexes))
    }
}

impl IntoIterator for RegexDb {
    type Item = RegexInput;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
