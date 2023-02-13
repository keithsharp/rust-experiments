use std::error::Error;
use std::fmt;

use rand::prelude::*;

fn main() {
    // Using a String as an error type
    match string_error() {
        Err(e) => println!("String error: {e}"),
        _ => println!("No errors received"),
    };

    // Using a Unit Struct implementing Error as an error type
    match struct_error() {
        Err(e) => println!("Struct error: {e}"),
        _ => println!("No errors received"),
    };

    // Using a Struct with a source member (nesting the above Unit Struct) as an error type
    match struct_super_error() {
        Err(e) => println!(
            "Struct super error: {e}, caused by '{}'",
            e.source().unwrap()
        ),
        _ => println!("No errors received"),
    };

    // Using an Enum with two variants as an error type
    match enum_error() {
        Err(EnumError::First) => println!("EnumError::First"),
        Err(EnumError::Second) => println!("EnumError::Second"),
        _ => println!("No errors received"),
    };

    // Using an Enum with two variants and implementing Error as an error type
    // Match on Error, then match on the Enum variant, could be implmented as previous match^^
    match enum_trait_error() {
        Err(e) => match e {
            EnumTraitError::First => println!("EnumTraitError::First"),
            EnumTraitError::Second => println!("EnumTraitError::Second"),
        },
        _ => println!("No errors received"),
    };

    // Using an Enum with two variants and implementing Error as an error type
    // Match on error and use match in EnumTraitError::display to switch message
    match enum_trait_error() {
        Err(e) => println!("EnumTraitError error: {e}"),
        _ => println!("No errors received"),
    }

    // Using an Enum with one varient that contains a source value (an EnumTraitError)
    match enum_super_error() {
        Err(e) => println!("Enum super error: {e}, caused by '{}'", e.source().unwrap()),
        _ => println!("No errors received"),
    };
}

// String Error
fn string_error() -> Result<(), String> {
    Err("This is a string error".to_string())
}

// Struct Error
#[derive(Debug)]
struct StructError;

impl fmt::Display for StructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This is a struct error")
    }
}

impl Error for StructError {}

fn struct_error() -> Result<(), StructError> {
    Err(StructError)
}

// Struct Nested Error
#[derive(Debug)]
struct StructSuperError {
    source: StructError,
}

impl fmt::Display for StructSuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This is a struct super error")
    }
}

impl Error for StructSuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn struct_super_error() -> Result<(), StructSuperError> {
    Err(StructSuperError {
        source: StructError,
    })
}

// Enum Error
#[derive(Debug)]
enum EnumError {
    First,
    Second,
}

fn enum_error() -> Result<(), EnumError> {
    if random() {
        Err(EnumError::First)
    } else {
        Err(EnumError::Second)
    }
}

// Enum Error Trait
#[derive(Debug)]
enum EnumTraitError {
    First,
    Second,
}

impl fmt::Display for EnumTraitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumTraitError::First => write!(f, "This is a EnumTraitError::First"),
            EnumTraitError::Second => write!(f, "This is a EnumTraitError::Second"),
        }
    }
}

impl Error for EnumTraitError {}

fn enum_trait_error() -> Result<(), EnumTraitError> {
    if random() {
        Err(EnumTraitError::First)
    } else {
        Err(EnumTraitError::Second)
    }
}

// Enum Nested Error
#[derive(Debug)]
enum EnumSuperError {
    First { source: EnumTraitError },
}

impl fmt::Display for EnumSuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumSuperError::First { .. } => write!(f, "This is a EnumSuperError::First"),
        }
    }
}

impl Error for EnumSuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EnumSuperError::First { source } => Some(source),
        }
    }
}

fn enum_super_error() -> Result<(), EnumSuperError> {
    Err(EnumSuperError::First {
        source: EnumTraitError::Second,
    })
}
