#![allow(dead_code)]

use anyhow::{Error, Result, Ok};
use rand::prelude::*;
use thiserror::Error;

use std::fmt;
use std::error::Error as StdError;

fn main() -> Result<()> {
    // let msg = maybe_error()?;
    // let msg = always_error()?;
    let msg = always_random_error()?;
    println!("Message: {msg}");

    Ok(())
}

fn maybe_error() -> Result<String> {
    if random() {
        always_error()?;
    }
    Ok("Hello, World!".to_string())
}

fn always_error() -> Result<String> {
    Err(Error::from(ThisTestError::NestedStringError { msg: "This is the outer error".to_string(), source: EnumTraitError::Second("Hello from the EnumTraitError".to_string()) }).context("ThisError Enum with message and nested error"))
}

fn always_random_error() -> Result<String> {
    match rand::thread_rng().gen_range(0..3) {
        0 => Err(Error::from(EnumSuperError::First { source: EnumTraitError::Second("Hello, Keith".to_string()) }).context("Handmade Enum erros")),
        1 => Err(Error::from(ThisTestError::ContainsString("Hello, Keith".to_string())).context("ThisError Enum with String value")),
        2 => Err(Error::from(ThisTestError::NestedError(EnumTraitError::Second("Hello, Keith".to_string()))).context("ThisError Enum with nested error")),
        3 => Err(Error::from(ThisTestError::NestedStringError { msg: "This is the outer error".to_string(), source: EnumTraitError::Second("Hello from the EnumTraitError".to_string()) }).context("ThisError Enum with message and nested error")),
        x => panic!("Got unexpected random value: {x}"),
    }
}

// Enum Error Trait
#[derive(Debug)]
enum EnumTraitError {
    First(String),
    Second(String),
}

impl fmt::Display for EnumTraitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumTraitError::First(s) => write!(f, "EnumTraitError::First with message: {s}"),
            EnumTraitError::Second(s) => write!(f, "EnumTraitError::Second with message: {s}"),
        }
    }
}

impl StdError for EnumTraitError {}

// Enum Nested Error
#[derive(Debug)]
enum EnumSuperError {
    First { source: EnumTraitError},
}

impl fmt::Display for EnumSuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumSuperError::First { .. } => write!(f, "This is a EnumSuperError::First"),
        }
    }
}

impl StdError for EnumSuperError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            EnumSuperError::First { source } => Some(source)
        }
    }
}

// ThisError
#[derive(Debug, Error)]
enum ThisTestError {
    #[error("'ThisTestError::ContainsString' with message: {0}")]
    ContainsString(String),
    #[error("'ThisTestError::NestedError' with a nested error")]
    NestedError(#[from] EnumTraitError),
    #[error("'ThisTestError::NestedStringError' with message: {msg}")]
    NestedStringError{msg: String, source: EnumTraitError}
}