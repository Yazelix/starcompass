//! Consumer file formats: the import manifest, the exceptions document, and
//! consumer-relative path normalization.

use super::{Error, Result, present, valid_inline_code};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

pub(super) const LOCAL_OVERLAY: &str = ".agent-protocols.local.md";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ExceptionsDocument {
    pub(super) schema: u64,
    pub(super) exceptions: Vec<Exception>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Exception {
    pub(super) approved_at: String,
    pub(super) approved_by: String,
    pub(super) protocol: String,
    pub(super) reason: String,
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    pub(super) review_condition: Option<String>,
    pub(super) scope: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Manifest {
    pub(super) schema: u64,
    pub(super) source: Source,
    pub(super) profiles: Vec<String>,
    pub(super) protocols: Vec<ImportRecord>,
    pub(super) exceptions: Vec<Exception>,
    pub(super) overlay: Overlay,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) nested: Vec<Nested>,
    /// Consumer file, beside the root `AGENTS.md`, that holds the canonical
    /// protocol text instead of the always-loaded `AGENTS.md`.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    pub(super) protocol_file: Option<String>,
}

/// A directory whose generated `AGENTS.md` adds local rules to the root import.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Nested {
    pub(super) path: String,
    pub(super) sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Source {
    pub(super) repository: String,
    pub(super) commit: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ImportRecord {
    pub(super) id: String,
    pub(super) version: u64,
    pub(super) sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Overlay {
    pub(super) sha256: String,
}

pub(super) fn validate_exceptions(
    document: &ExceptionsDocument,
    selected: &BTreeSet<String>,
) -> Result<()> {
    if document.schema != 1 {
        return Err(Error::new(
            "exceptions document must use schema 1 and an exceptions list",
        ));
    }
    let mut identities = BTreeSet::new();
    for (index, exception) in document.exceptions.iter().enumerate() {
        let number = index + 1;
        if [
            exception.approved_at.as_str(),
            exception.approved_by.as_str(),
            exception.protocol.as_str(),
            exception.reason.as_str(),
            exception.scope.as_str(),
        ]
        .into_iter()
        .any(|value| value.trim().is_empty())
            || exception
                .review_condition
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(Error::new(format!(
                "exception {number} fields must be non-empty strings"
            )));
        }
        if !selected.contains(&exception.protocol) {
            return Err(Error::new(format!(
                "exception {number} targets a protocol outside the profile"
            )));
        }
        if !valid_date(&exception.approved_at) {
            return Err(Error::new(format!(
                "exception {number} approved_at must be YYYY-MM-DD"
            )));
        }
        let identity = (exception.protocol.clone(), exception.scope.clone());
        if !identities.insert(identity) {
            return Err(Error::new(format!(
                "duplicate exception scope for {}",
                exception.protocol
            )));
        }
    }
    Ok(())
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes
            .iter()
            .enumerate()
            .any(|(index, byte)| !matches!(index, 4 | 7) && !byte.is_ascii_digit())
    {
        return false;
    }
    let Ok(year) = value[..4].parse::<u32>() else {
        return false;
    };
    if year == 0 {
        return false;
    }
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..].parse::<u32>() else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

/// Normalizes the protocol file to a consumer-relative path of plain names.
/// Codex loads `AGENTS.md` and `AGENTS.override.md` itself, so a protocol file
/// with either name would escape the chain limit or displace the import.
pub(super) fn protocol_file_path(value: &str) -> Result<String> {
    let path = nested_path(value).map_err(|_| {
        Error::new(format!(
            "protocol file must be a relative path of plain names without backticks or line breaks: {value:?}"
        ))
    })?;
    let name = path.rsplit('/').next().unwrap_or_default();
    if ["AGENTS.md", "AGENTS.override.md"]
        .iter()
        .any(|loaded| name.eq_ignore_ascii_case(loaded))
    {
        return Err(Error::new(format!(
            "protocol file must not be named like an auto-loaded instruction file: {value:?}"
        )));
    }
    Ok(path)
}

/// One directory's overlay and generated `AGENTS.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct NestedFiles {
    pub(super) path: String,
    pub(super) overlay: String,
    pub(super) agents: String,
}

/// Normalizes a consumer directory to plain relative names joined by `/`.
pub(super) fn nested_path(value: &str) -> Result<String> {
    let invalid = || {
        Error::new(format!(
            "nested directory must be a relative path of plain directory names without backticks or line breaks: {value:?}"
        ))
    };
    let mut names = Vec::new();
    for component in Path::new(value).components() {
        match component {
            Component::Normal(name) => names.push(name.to_str().ok_or_else(invalid)?),
            Component::CurDir if !names.is_empty() => {}
            _ => return Err(invalid()),
        }
    }
    let path = names.join("/");
    if names.is_empty() || !valid_inline_code(&path) {
        return Err(invalid());
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starcompass::test_support::*;

    #[test]
    fn nested_directories_must_be_plain_relative_names() {
        assert_eq!(nested_path("orbit").unwrap(), "orbit");
        assert_eq!(nested_path("orbit/./crates/").unwrap(), "orbit/crates");
        for invalid in [
            "",
            ".",
            "/orbit",
            "../orbit",
            "orbit/../venus",
            "or`bit",
            "or\nbit",
        ] {
            assert_error(nested_path(invalid), "nested directory");
        }
    }
}
