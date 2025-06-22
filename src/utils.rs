use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use hex;
use rand::Rng;
use sha2::{Digest, Sha256, Sha512};
use std::process::Command;

// Common word list for passphrases
const COMMON_WORDS: &[&str] = &[
    "apple",
    "banana",
    "cherry",
    "dragon",
    "eagle",
    "forest",
    "garden",
    "house",
    "island",
    "jungle",
    "knight",
    "lemon",
    "mountain",
    "ocean",
    "planet",
    "queen",
    "river",
    "sunset",
    "tiger",
    "umbrella",
    "village",
    "window",
    "yellow",
    "zebra",
    "anchor",
    "bridge",
    "castle",
    "diamond",
    "elephant",
    "flower",
    "guitar",
    "hammer",
    "iceberg",
    "jacket",
    "kangaroo",
    "lighthouse",
    "moonlight",
    "notebook",
    "orange",
    "penguin",
    "rainbow",
    "sailboat",
    "treasure",
    "umbrella",
    "volcano",
    "waterfall",
    "xylophone",
    "yacht",
    "zeppelin",
];

#[derive(Debug, Clone, Copy)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    pub fn to_string(&self) -> &'static str {
        match self {
            PasswordStrength::VeryWeak => "Very Weak",
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Medium => "Medium",
            PasswordStrength::Strong => "Strong",
            PasswordStrength::VeryStrong => "Very Strong",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            PasswordStrength::VeryWeak => "red",
            PasswordStrength::Weak => "yellow",
            PasswordStrength::Medium => "blue",
            PasswordStrength::Strong => "green",
            PasswordStrength::VeryStrong => "bright_green",
        }
    }
}

pub fn format_password(password: &str, format: &str) -> Result<String> {
    match format.to_lowercase().as_str() {
        "plain" => Ok(password.to_string()),
        "base64" => Ok(general_purpose::STANDARD.encode(password.as_bytes())),
        "hex" => Ok(hex::encode(password.as_bytes())),
        _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut child = Command::new("pbcopy")
        .arg("-Prefer")
        .arg("txt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait_with_output()?;
    if !status.status.success() {
        return Err(anyhow::anyhow!("Failed to copy to clipboard"));
    }

    Ok(())
}

pub fn calculate_entropy(password: &str) -> f64 {
    let mut charset_size = 0;
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_numbers = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_ascii_uppercase() && !has_uppercase {
            charset_size += 26;
            has_uppercase = true;
        } else if ch.is_ascii_lowercase() && !has_lowercase {
            charset_size += 26;
            has_lowercase = true;
        } else if ch.is_ascii_digit() && !has_numbers {
            charset_size += 10;
            has_numbers = true;
        } else if !ch.is_alphanumeric() && !has_special {
            charset_size += 32; // Common special characters
            has_special = true;
        }
    }

    if charset_size == 0 {
        charset_size = 1; // Avoid log(0)
    }

    password.len() as f64 * (charset_size as f64).log2()
}

pub fn check_password_strength(password: &str) -> PasswordStrength {
    let entropy = calculate_entropy(password);
    let length = password.len();

    // Check for common patterns
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_numbers = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    let complexity_score = [has_uppercase, has_lowercase, has_numbers, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    match (entropy, length, complexity_score) {
        (entropy, _, _) if entropy < 20.0 => PasswordStrength::VeryWeak,
        (entropy, length, score) if entropy < 30.0 || length < 8 || score < 2 => {
            PasswordStrength::Weak
        }
        (entropy, length, score) if entropy < 40.0 || length < 10 || score < 3 => {
            PasswordStrength::Medium
        }
        (entropy, length, score) if entropy < 50.0 || length < 12 || score < 4 => {
            PasswordStrength::Strong
        }
        _ => PasswordStrength::VeryStrong,
    }
}

pub fn analyze_password(password: &str) -> Vec<(&'static str, bool)> {
    let mut analysis = Vec::new();

    // Length checks
    analysis.push(("At least 8 characters", password.len() >= 8));
    analysis.push(("At least 12 characters", password.len() >= 12));
    analysis.push(("At least 16 characters", password.len() >= 16));

    // Character set checks
    analysis.push((
        "Contains uppercase letters",
        password.chars().any(|c| c.is_ascii_uppercase()),
    ));
    analysis.push((
        "Contains lowercase letters",
        password.chars().any(|c| c.is_ascii_lowercase()),
    ));
    analysis.push((
        "Contains numbers",
        password.chars().any(|c| c.is_ascii_digit()),
    ));
    analysis.push((
        "Contains special characters",
        password.chars().any(|c| !c.is_alphanumeric()),
    ));

    // Entropy check
    let entropy = calculate_entropy(password);
    analysis.push(("Entropy >= 30 bits", entropy >= 30.0));
    analysis.push(("Entropy >= 40 bits", entropy >= 40.0));

    // Common pattern checks
    let has_repeating = password
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|window| window[0] == window[1] && window[1] == window[2]);
    analysis.push(("No repeating characters (3+ consecutive)", !has_repeating));

    // Check for common sequences
    let common_sequences = ["123", "abc", "qwe", "asd", "password", "admin"];
    let has_common_sequence = common_sequences
        .iter()
        .any(|seq| password.to_lowercase().contains(seq));
    analysis.push(("No common sequences", !has_common_sequence));

    analysis
}

pub fn generate_passphrase(
    words: usize,
    separator: &str,
    include_numbers: bool,
    include_special: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut passphrase_parts = Vec::new();

    for _ in 0..words {
        let word = COMMON_WORDS[rng.gen_range(0..COMMON_WORDS.len())];
        passphrase_parts.push(word.to_string());
    }

    if include_numbers {
        let num_position = rng.gen_range(0..=passphrase_parts.len());
        let number = rng.gen_range(100..1000).to_string();
        passphrase_parts.insert(num_position, number);
    }

    if include_special {
        let special_chars = "!@#$%^&*";
        let special_char = special_chars
            .chars()
            .nth(rng.gen_range(0..special_chars.len()))
            .unwrap();
        let special_position = rng.gen_range(0..=passphrase_parts.len());
        passphrase_parts.insert(special_position, special_char.to_string());
    }

    Ok(passphrase_parts.join(separator))
}

pub fn generate_hash(input: &str, algorithm: &str) -> Result<String> {
    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            Ok(hex::encode(hasher.finalize()))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            Ok(hex::encode(hasher.finalize()))
        }
        "base64" => Ok(general_purpose::STANDARD.encode(input.as_bytes())),
        _ => Err(anyhow::anyhow!("Unsupported hash algorithm: {}", algorithm)),
    }
}
