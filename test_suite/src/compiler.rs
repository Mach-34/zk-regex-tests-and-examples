use std::process::Command;

use anyhow::Context;

use crate::constants;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error compiling the code for the current regex: {0:?}")]
    ProjectCompilation(String),
}

pub fn compile_noir_project() -> anyhow::Result<()> {
    let output = Command::new("nargo")
        .arg("compile")
        .current_dir(constants::DEFAULT_PROJECT_PATH)
        .output()
        .context("error executing the compile command")?;
    if !output.status.success() {
        anyhow::bail!(Error::ProjectCompilation(
            String::from_utf8(output.stderr)
                .context("error parsing the output from the command execution")?
        ));
    }
    Ok(())
}
