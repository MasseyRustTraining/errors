#![allow(unused)]

/*
pub enum Option<T> {
    Some(T),
    None,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

#[allow(dead_code)]
enum LuhnError {
    SyntaxError(String),
    CheckError,
}

#[must_use]
fn luhn_check(_s: &str) -> Result<(), LuhnError> {
    todo!();
    #[allow(unreachable_code)]
    Err(LuhnError::CheckError)
}

fn luhn_prefix(s: &str) -> Result<String, LuhnError> {
    luhn_check("01234")?; // immediately returns on error
    let mut result = String::new();
    for c in s.chars().take(3) {
        result.push(c);
    }
    Ok(result)
}

#[allow(dead_code)]
fn max_plus_one(a: Vec<u32>) -> Option<u32> {
    Some(a.iter().max()? + 1)
}

fn main() {
    match luhn_prefix("012344") {
        Ok(s) => println!("{}", s),
        Err(LuhnError::SyntaxError(s)) => println!("syntax error: {}", s),
        Err(LuhnError::CheckError) => println!("check failed"),
    }
}
