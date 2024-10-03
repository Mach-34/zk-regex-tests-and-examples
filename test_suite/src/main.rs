mod bench;
mod code;
mod compiler;
mod constants;
mod db;
mod tester;

use bench::{execute_count_gate_command, BenchReport};
use clap::Parser;
use code::Code;
use db::RegexDb;
use log::{self, error, info};
use std::{error::Error, path::Path};
use tester::test_regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional path to the regex database file
    #[arg(long, short, default_value_t = String::from(constants::DEFAULT_DATABASE_PATH))]
    db: String,
    /// If you want to run the testing
    #[arg(long, short)]
    test: bool,
    /// If you want to run the benchmarking
    #[arg(long, short)]
    bench: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // Parse command-line arguments
    let args = Args::parse();
    info!("starting regex tests");
    // Reads the database from the given path or use the default one
    let database_path = Path::new(&args.db);
    let database = RegexDb::load_from_file(database_path).map_err(|err| {
        error!("error creating the database: {}", err);
        err
    })?;

    let benchmark_all = database.bench_all;
    let mut bench_report = BenchReport::default();
    for regex_input in database {
        info!("testing regex {}", regex_input.regex.complete_regex());
        let mut code_read_result = Code::new(&regex_input);
        match &mut code_read_result {
            Ok(code) => {
                let _ = code.write_to_path(Path::new(constants::DEFAULT_PROJECT_MAIN_FILE));
                let compilation_result = compiler::compile_noir_project();
                match compilation_result {
                    Ok(_) => info!(
                        "compilation success for regex {}",
                        regex_input.regex.complete_regex()
                    ),
                    Err(e) => {
                        error!(
                            "error compiling the noir project for regex {}: {:?}",
                            regex_input.regex.complete_regex(),
                            e
                        );
                        continue;
                    }
                }
                if args.test {
                    match test_regex(&regex_input, code) {
                        Ok(test_result) => {
                            info!(
                                "test passed correctly for regex {}:\n{}",
                                regex_input.regex.complete_regex(),
                                test_result
                            );
                        }
                        Err(err) => match err.downcast_ref() {
                            Some(tester::Error::TestFailed(test_result)) => {
                                error!(
                                    "test failed for regex {}:\n{}",
                                    regex_input.regex.complete_regex(),
                                    test_result
                                )
                            }
                            None => error!("error downcasting the anyhow::Error"),
                        },
                    }
                }
                if args.bench {
                    if regex_input.with_bench || benchmark_all {
                        match execute_count_gate_command() {
                            Ok(mut bench_result) => {
                                info!("benchmark results:\n{}", bench_result);
                                // Changes the data needed to write the report.
                                bench_result.regex = regex_input.regex.complete_regex();
                                bench_result.with_gen_substr = regex_input.gen_substrs;
                                bench_report.push_result(bench_result);
                            }
                            Err(err) => {
                                error!(
                                    "error running the benchmark for regex {}: {:?}",
                                    regex_input.regex.complete_regex(),
                                    err
                                )
                            }
                        }
                    }
                }
            }
            Err(err) => match err.downcast_ref() {
                Some(code::Error::CodeGenerationFailed(console_msg)) => {
                    error!("error generating the code: \n{}", console_msg);
                }
                None => error!("error downcasting the anyhow::Error"),
            },
        }
    }

    // Save the bench results.
    if !bench_report.is_empty() {
        info!("saving benchmark results into CSV");
        bench_report.save(Path::new(constants::DEFAULT_BENCH_RESULT_FILE))?;
    }

    Ok(())
}
