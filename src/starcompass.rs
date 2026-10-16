//! Validation, rendering, and drift detection for agent-protocol imports.
//! Each child module owns one concern; this module owns the shared error
//! type and text and JSON helpers.

mod catalog;
mod check;
mod cli;
mod manifest;
mod render;
#[cfg(test)]
mod test_support;

pub(crate) use cli::run;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{fmt, fs, path::Path};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

fn present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

fn digest(data: &[u8]) -> String {
    let bytes = Sha256::digest(data);
    let mut output = String::with_capacity(64);
    for byte in bytes {
        use fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing to a string cannot fail");
    }
    output
}

fn read_text(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(Error::new(format!("missing file: {}", path.display())))
        }
        Err(error) => Err(error.into()),
    }
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = read_text(path)?;
    serde_json::from_str(&text)
        .map_err(|error| Error::new(format!("invalid JSON in {}: {error}", path.display())))
}

fn json_text<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)? + "\n")
}

fn markdown_content(value: &str) -> &str {
    value.trim_start_matches(['\r', '\n']).trim_end()
}

fn valid_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_inline_code(value: &str) -> bool {
    !value.trim().is_empty() && !value.contains(['`', '\r', '\n'])
}

fn valid_commit_id(value: &str) -> bool {
    valid_lower_hex(value, 40)
}
