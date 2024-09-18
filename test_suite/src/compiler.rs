use std::{io, process::Command, string::FromUtf8Error};

use crate::constants;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error compiling the code for the current regex: {0:?}")]
    ProjectCompilation(String),
    #[error("error executing the compilation command: {0:?}")]
    CompilationCommandExecution(io::Error),
    #[error("error converting the compilation command output to string: {0:?}")]
    OutputConversion(FromUtf8Error),
}

pub fn compile_noir_project() -> Result<(), Error> {
    let output = Command::new("nargo")
        .arg("compile")
        .current_dir(constants::DEFAULT_PROJECT_PATH)
        .output()
        .map_err(Error::CompilationCommandExecution)?;
    if !output.status.success() {
        return Err(Error::ProjectCompilation(
            String::from_utf8(output.stderr).map_err(Error::OutputConversion)?,
        ));
    }
    Ok(())
}
