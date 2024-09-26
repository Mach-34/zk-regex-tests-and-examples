use std::{fmt::Display, process::Command};

use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;

use crate::constants;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error executing the gate-count command: {0:?}")]
    CommandOutputError(String),
}

#[derive(Deserialize)]
pub struct BenchResult {
    pub acir_opcodes: u32,
    pub circuit_size: u32,
    pub gates_per_opcode: Vec<u32>,
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACIR opcodes: {}\nCircuit size: {}\nGates per opcode: {:?}",
            self.acir_opcodes, self.circuit_size, self.gates_per_opcode
        )
    }
}

pub fn execute_count_gate_command() -> anyhow::Result<BenchResult> {
    let output = Command::new("bb")
        .args(["gates", "-b"])
        .arg(constants::DEFAULT_TARJET_JSON_FILE)
        .output()
        .context("the gate-count command was not executed correctly")?;
    if !output.status.success() {
        anyhow::bail!(Error::CommandOutputError(String::from_utf8(output.stderr)?));
    }

    let str_result_json = String::from_utf8(output.stdout)?;
    let output_value: Value = serde_json::from_str(&str_result_json)?;

    let bench_result: BenchResult =
        serde_json::from_str(&output_value["functions"][0].to_string())?;
    Ok(bench_result)
}
