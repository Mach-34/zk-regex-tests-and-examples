use std::{fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Database of regular expressions that will be tested.
#[derive(Deserialize)]
pub struct RegexDb {
    // Regex entries in the database
    db_entries: Vec<DbEntry>,
    // Defines wether we need to benchmark all the entries in the database.
    #[serde(default)]
    pub bench_all: bool,
}

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
    /// A raw regex with optional transitions.
    Raw(RawRegex),
    /// A decomposed regex defined by fragments and whether substrings should be extracted.
    Decomposed(Vec<RegexFragment>),
}

// `RawRegex` can either be a simple string or an object with transitions.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)] // Allows deserialization of either a string or a structured object.
pub enum RawRegex {
    /// Simple string form for raw regex.
    Simple(String),
    /// Structured form with optional transitions.
    WithTransitions {
        /// The raw regex string.
        regex: String,
        /// Optional transitions.
        #[serde(skip_serializing_if = "Option::is_none")]
        transitions: Option<Transitions>,
    },
}

impl RawRegex {
    fn get_regex(&self) -> String {
        match self {
            RawRegex::Simple(str) => str.clone(),
            RawRegex::WithTransitions { regex, .. } => regex.clone(),
        }
    }
}

// Struct representing the transitions.
#[derive(Deserialize, Serialize, Debug)]
pub struct Transitions {
    /// Transitions data.
    pub transitions: Vec<Vec<Vec<u32>>>,
}

impl RegexInput {
    /// Returns the regex whole regex associated with the input. For a [`Raw`](crate::db::RegexInput::Raw) regex,
    /// simply return the associated string. For a [`Decomposed`](crate::db::RegexInput::Decomposed) regex, we
    /// return the concatenation of all the fragments.
    pub fn complete_regex(&self) -> String {
        match self {
            Self::Raw(rawregex) => rawregex.get_regex(),
            Self::Decomposed(fragments) => fragments
                .iter()
                .map(|fragment| fragment.regex_def.clone()) // TODO: Check if we can avoid clonning.
                .collect::<String>(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)] // Automatically distinguish between the two formats
pub enum SamplesPass {
    /// For cases with substrings (complex structure)
    WithSubstrs(Vec<InputWithSubstrs>),
    /// For cases without substrings (simple structure)
    WithoutSubstrs(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputWithSubstrs {
    /// The input string
    pub input: String,
    /// The expected substrings from the input
    pub expected_substrings: Vec<String>,
}

/// An entry of the test database.
#[derive(Deserialize, Serialize)]
pub struct DbEntry {
    /// The regex of the entry.
    pub regex: RegexInput,
    /// Whether substrings should be generated. Default false
    #[serde(default)]
    pub gen_substrs: bool,
    /// The maximum input size to generate random regexes for testing.
    pub input_size: usize,
    /// Samples that are provided as input by the user and expected to pass the regex
    pub samples_pass: SamplesPass,
    /// Samples that are provided as input by the user and *not* expected to pass the regex.
    pub samples_fail: Vec<String>,
    /// Defines wether you want a benchmark for the regex in the given test
    #[serde(default)]
    pub with_bench: bool,
    /// String used for the benchmarking. If you want to benchmark, this field is mandatory.
    /// If you want just testing, this field is optional.
    #[serde(default)]
    pub benchmark_str: String,
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

        // If the field in the "bench_all" is a boolean, take the boolean,
        // otherwhise, use false as a default.
        let bench_all: bool = json_value["bench_all"].as_bool().unwrap_or(false);

        let mut regexes = Vec::new();

        for db_element in regex_db {
            regexes.push(db_element);
        }

        Ok(Self {
            db_entries: regexes,
            bench_all,
        })
    }
}

impl IntoIterator for RegexDb {
    type Item = DbEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.db_entries.into_iter()
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
