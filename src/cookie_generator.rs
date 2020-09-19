//! Cookie Generator to generate a unique cookie
use std::vec::Vec;
use super::error::Error;
use super::config::MAX_COOKIE_NUM;

/// Cookie Generator
/// Used to generate unique cookie
pub struct CookieGenerator {
    used_cookie: Vec<usize>,
    current: usize,
    size: u8,
}

impl CookieGenerator {
    /// Create a new 'CookieGenerator'
    pub fn new() -> Self {
        Self {
            used_cookie: Vec::new(),
            current: random_number::random!(),
            size: 0,
        }
    }

    /// Generate a new 'cookie'
    pub fn generate(&mut self) -> Result<usize, Error> {
        // too much cookie
        if self.used_cookie.len() > MAX_COOKIE_NUM {
            println!("Too much used cookie, stop generate.");
            return Err(Error::CookieGenerateError)
        }
        // generate a unique cookie
        while self.used_cookie.contains(&(self.current)) {
            self.current = random_number::random!();
        }
        self.used_cookie.push(self.current);
        self.size += 1;
        Ok(self.current)
    }

    /// Reset the 'CookieGenerator'
    /// Not used in this crate yet
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.used_cookie.clear();
        self.size = 0;
        self.current = 0;
    }
}

