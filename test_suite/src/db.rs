use std::{fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Database of regular expressions that will be tested.
pub struct RegexDb(Vec<DbEntry>);

/// Represents each fragment in a decomposed regex.
#[derive(Deserialize, Serialize, Clone)]
pub struct RegexFragment {
    /// Determines if this part of the regex is private.
    pub is_public: bool,
    /// The regex string of the fragment.
    pub regex_def: String,
}

// Represents the input regex in the database.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegexInput {
    /// A raw regex.
    Raw(String),
    /// A decomposed regex defined by fragments.
    Decomposed(Vec<RegexFragment>),
}

impl RegexInput {
    /// Returns the regex whole regex associated with the input. For a [`Raw`](crate::db::RegexInput::Raw) regex,
    /// simply return the associated string. For a [`Decomposed`](crate::db::RegexInput::Decomposed) regex, we
    /// return the concatenation of all the fragments.
    pub fn complete_regex(&self) -> String {
        match self {
            Self::Raw(string) => string.clone(),
            Self::Decomposed(fragments) => fragments
                .iter()
                .map(|fragment| fragment.regex_def.clone()) // TODO: Check if we can avoid clonning.
                .collect::<String>(),
        }
    }
}

/// An entry of the test database.
#[derive(Deserialize, Serialize)]
pub struct DbEntry {
    /// The regex of the entry.
    pub regex: RegexInput,
    /// The maximum input size to generate random regexes for testing.
    pub input_size: usize,
    /// Samples that should pass that are inputted by the user.
    pub samples_pass: Vec<String>,
    /// Samples that should fail that are inputted by the user.
    pub samples_fail: Vec<String>,
}

impl RegexDb {
    /// Constructs a database from a JSON file where the file contains the information in the following format:
    pub fn load_from_file(file_path: &Path) -> anyhow::Result<Self> {
        let file_regex_content =
            fs::read_to_string(file_path).context("error reading the regex content")?;
        let json_value: Value =
            serde_json::from_str(&file_regex_content).context("error parsing the json")?;
        let regex_db: Vec<DbEntry> = serde_json::from_str(&json_value["database"].to_string())
            .context("error parsing the database array")?;

        let mut regexes = Vec::new();

        for db_element in regex_db {
            regexes.push(db_element);
        }

        Ok(Self(regexes))
    }
}

impl IntoIterator for RegexDb {
    type Item = DbEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Wrapper created as a trick to serialize the parts of a decomposed regex and
/// write them as a JSON using `serde`.
#[derive(Serialize, Deserialize)]
pub struct ComponentsWrapper {
    /// Wrapper for the parts of a decomposed regex.
    pub parts: Vec<RegexFragment>,
}

impl ComponentsWrapper {
    /// Constructs a wrapper for the decomposed regex.
    pub fn new(fragments: Vec<RegexFragment>) -> Self {
        Self { parts: fragments }
    }
}
