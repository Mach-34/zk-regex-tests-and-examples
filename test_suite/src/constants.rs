/// Path of the JSON file containing the database.
pub const DEFAULT_DATABASE_PATH: &str = "./regex_db.json";
/// Path of the main file in the Noir project.
pub const DEFAULT_PROJECT_MAIN_FILE: &str = "./execution_project/src/main.nr";
/// Path of the main project.
pub const DEFAULT_PROJECT_PATH: &str = "./execution_project";
/// Path for the Noir file that will contain the geneated code using the zk-regex tool.
pub const DEFAULT_GENERATION_PATH: &str = "./noir_code.nr";
/// Default number of random samples used to test each regex.
pub const DEFAULT_SAMPLE_NUMBER: usize = 10;
/// Default path of the JSON file that stores the decomposed version of a regex.
pub const DEFAULT_DECOMPOSED_JSON_FILE: &str = "./decomposed.json";
/// Default path of the JSON file that stored the transitions of where substrings should be extracted
pub const DEFAULT_SUBSTRS_JSON_PATH: &str = "./substrs_transitions.json";
/// Default path of the target folder for the project
pub const DEFAULT_TARJET_JSON_FILE: &str = "./execution_project/target/execution_project.json";
/// Default path for bench report
pub const DEFAULT_BENCH_RESULT_FILE: &str = "./bench_result.csv";
