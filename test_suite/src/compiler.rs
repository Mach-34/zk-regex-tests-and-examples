use std::{fs, io, path::Path, process::Command, string::FromUtf8Error};

use crate::code::Code;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error compiling the code for the current regex: {0:?}")]
    ProjectCompilation(String),
    #[error("error writing the code into the project file: {0:?}")]
    WriteToProject(io::Error),
    #[error("error executing the compilation command: {0:?}")]
    CompilationCommandExecution(io::Error),
    #[error("error converting the compilation command output to string: {0:?}")]
    OutputConversion(FromUtf8Error),
}

pub const DEFAULT_PROJECT_MAIN_FILE: &str = "./execution_project/src/main.nr";
pub const DEFAULT_PROJECT_PATH: &str = "./execution_project";

pub fn write_code_to_project(code: Code) -> Result<(), Error> {
    fs::write(Path::new(DEFAULT_PROJECT_MAIN_FILE), code.to_string())
        .map_err(Error::WriteToProject)?;
    Ok(())
}

pub fn compile_noir_project() -> Result<(), Error> {
    let output = Command::new("nargo")
        .arg("compile")
        .current_dir(DEFAULT_PROJECT_PATH)
        .output()
        .map_err(Error::CompilationCommandExecution)?;
    if !output.status.success() {
        return Err(Error::ProjectCompilation(
            String::from_utf8(output.stderr).map_err(Error::OutputConversion)?,
        ));
    }
    Ok(())
}
