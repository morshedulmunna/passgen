use anyhow::Result;
use rand::Rng;
use rand_core::OsRng;
use std::collections::HashSet;

pub struct PasswordGenerator {
    charset: String,
    exclude_chars: HashSet<char>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self {
            charset: String::new(),
            exclude_chars: HashSet::new(),
        }
    }

    pub fn include_uppercase(&mut self) -> &mut Self {
        self.charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        self
    }

    pub fn include_lowercase(&mut self) -> &mut Self {
        self.charset.push_str("abcdefghijklmnopqrstuvwxyz");
        self
    }

    pub fn include_numbers(&mut self) -> &mut Self {
        self.charset.push_str("0123456789");
        self
    }

    pub fn include_special(&mut self) -> &mut Self {
        self.charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        self
    }

    pub fn include_all(&mut self) -> &mut Self {
        self.include_uppercase()
            .include_lowercase()
            .include_numbers()
            .include_special();
        self
    }

    pub fn exclude_similar(&mut self) -> &mut Self {
        self.exclude_chars.extend(['l', '1', 'I', 'O', '0']);
        self
    }

    pub fn exclude_ambiguous(&mut self) -> &mut Self {
        self.exclude_chars.extend([
            '{', '}', '[', ']', '(', ')', '/', '\\', '\'', '"', '~', ';', ':', '.', '>', '<',
        ]);
        self
    }

    pub fn generate(&self, length: usize) -> Result<String> {
        if self.charset.is_empty() {
            return Err(anyhow::anyhow!("No character set specified"));
        }

        let mut rng = OsRng;
        let mut password = String::with_capacity(length);
        let available_chars: Vec<char> = self
            .charset
            .chars()
            .filter(|&c| !self.exclude_chars.contains(&c))
            .collect();

        if available_chars.is_empty() {
            return Err(anyhow::anyhow!("No characters available after exclusions"));
        }

        for _ in 0..length {
            let index = rng.gen_range(0..available_chars.len());
            password.push(available_chars[index]);
        }

        Ok(password)
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}
