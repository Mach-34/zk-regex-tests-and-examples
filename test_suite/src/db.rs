use std::{fs, io, path::Path, str::FromStr};

use regex::Regex;

/// Errors for the regex database.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error parsing the regex: {0:?}")]
    IncorrectParseRegex(regex::Error),
    #[error("error reading the regex from file: {0:?}")]
    FailedFileRead(io::Error),
    #[error("invalid input size: {0:?}")]
    InvalidInputSize(<usize as FromStr>::Err),
}

/// Database of regular expressions that will be tested.
pub struct RegexDb(Vec<(Regex, usize)>);

impl RegexDb {
    /// Constructs a database from a file where teh file contains the information in the following format:
    ///     - Regex1
    ///     - InputSize1
    ///     - Regex2
    ///     - InputSize2
    ///     - ...
    pub fn load_from_file(file_path: &Path) -> Result<Self, Error> {
        let file_regex_content = fs::read_to_string(file_path).map_err(Error::FailedFileRead)?;
        let mut regexes = Vec::new();
        let iter_lines: Vec<&str> = file_regex_content.lines().collect();
        for pair in iter_lines.chunks(2) {
            let regex = pair[0];
            let input_size: usize = pair[1].parse().map_err(Error::InvalidInputSize)?;

            regexes.push((
                Regex::new(regex).map_err(Error::IncorrectParseRegex)?,
                input_size,
            ));
        }

        Ok(Self(regexes))
    }
}

impl IntoIterator for RegexDb {
    type Item = (Regex, usize);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
