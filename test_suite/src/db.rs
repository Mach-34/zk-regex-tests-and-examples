use std::{fs, io, path::Path};

use regex::Regex;
use serde_json::Value;

/// Errors for the regex database.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error parsing the regex: {0:?}")]
    IncorrectParseRegex(regex::Error),
    #[error("error reading the regex from file: {0:?}")]
    FailedFileRead(io::Error),
}

/// Database of regular expressions that will be tested.
pub struct RegexDb(Vec<RegexInput>);

pub struct RegexInput {
    pub regex: Regex,
    pub input_size: usize,
}

impl RegexInput {
    pub fn new(regex_str: &str, input_size: usize) -> Result<Self, Error> {
        let regex = Regex::new(regex_str).map_err(Error::IncorrectParseRegex)?;
        Ok(Self { regex, input_size })
    }
}

impl RegexDb {
    /// Constructs a database from a file where teh file contains the information in the following format:
    ///     - Regex1
    ///     - InputSize1
    ///     - Regex2
    ///     - InputSize2
    ///     - ...
    pub fn load_from_file(file_path: &Path) -> Result<Self, Error> {
        let file_regex_content = fs::read_to_string(file_path).map_err(Error::FailedFileRead)?;
        let json_value: Value =
            serde_json::from_str(&file_regex_content).expect("failed while parsing the JSON file");
        let pairs = json_value["database"]
            .as_array()
            .expect("failed while parsing the array of regexes");

        let mut regexes = Vec::new();

        for pair in pairs {
            let regex = pair["regex"]
                .as_str()
                .expect("failed while reading the regex from the database");
            let input_size: usize = pair["input_size"]
                .as_u64()
                .expect("failed while reading the input size from the database")
                as usize;

            regexes.push(RegexInput::new(regex, input_size)?);
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
