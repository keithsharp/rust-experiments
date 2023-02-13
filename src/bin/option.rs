use std::ops::Mul;

use rand::prelude::*;

// The examples use random() to return Some(String) or None, this means
// you might need to run multple times to see different output
fn main() {
    // Simple handling of an Option<String>
    match maybe_option_string() {
        Some(msg) => println!("Message: {msg}"),
        None => println!("Did not get a message"),
    }

    // Using map() to run a closure on the value contained in the Option
    // Note, res is Option<&str> rather than the return value of Option<String>
    let res = maybe_option_string().map(|_| "Hello, Keith");
    match res {
        Some(msg) => println!("Map: {msg}"),
        None => println!("Did not get a message"),
    }

    // Using map_or() to provide a default of run a closure
    let msg = maybe_option_string().map_or("Hello, Keith".to_string(), |s| s.chars().rev().collect::<String>());
    println!("Map Or: {msg}");

    // Use expect to panic on None
    // Uncomment  the call to failure_option() to panic
    let msg = success_option().expect("should always return a msg");
    // let msg = failure_option().expect("should always return a msg");
    println!("Expect: {msg}");

    // Using unwrap_or() to provide provide a value
    let msg = maybe_option_string().unwrap_or("Hello, Keith".to_string());
    println!("Unwrap or: {msg}");

    // Using unwrap_or_else() to provide provide a value
    let msg = maybe_option_string().unwrap_or_else(|| "Hello, Keith".chars().rev().collect::<String>());
    println!("Unwrap or else: {msg}");

    // Using unwrap_or_default() to provide provide a value - an empty string 
    let msg = maybe_option_string().unwrap_or_default();
    println!("Unwrap or default: {msg}");

    // Using and_then() to run a closure if there's a value
    let val = maybe_option_integer().and_then(|v| Some(v.mul(v)));
    match val {
        Some(v) => println!("And then: {v}"),
        None => println!("No numbers to square"),
    }
}

// 50:50 chance of Some(String) or None
fn maybe_option_string() -> Option<String> {
    if random() {
        Some("Hello, World".to_string())
    } else {
        None
    }
}

// 50:50 chance of Some(Integer) or None
fn maybe_option_integer() -> Option<u32> {
    if random() {
        Some(42)
    } else {
        None
    }
}

fn success_option() -> Option<String> {
    Some("Hello, World".to_string())
}

#[allow(dead_code)]
fn failure_option() -> Option<String> {
    None
}