use clap::{Parser, Subcommand};
use colored::*;

mod generator;
mod utils;

use generator::PasswordGenerator;
use utils::*;

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "A secure password generator CLI tool")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password
    Generate {
        /// Length of the password (default: 16)
        #[arg(short, long, default_value = "16")]
        length: usize,

        /// Include uppercase letters
        #[arg(short, long)]
        uppercase: bool,

        /// Include lowercase letters
        #[arg(short, long)]
        lowercase: bool,

        /// Include numbers
        #[arg(short, long)]
        numbers: bool,

        /// Include special characters
        #[arg(short, long)]
        special: bool,

        /// Exclude similar characters (l, 1, I, O, 0)
        #[arg(short, long)]
        exclude_similar: bool,

        /// Exclude ambiguous characters ({}, [], (), /, \, ', ", ~, ;, :, ., >, <)
        #[arg(short, long)]
        exclude_ambiguous: bool,

        /// Output format: plain, base64, hex
        #[arg(short, long, default_value = "plain")]
        format: String,

        /// Copy password to clipboard (macOS only)
        #[arg(short, long)]
        copy: bool,
    },

    /// Generate a passphrase
    Passphrase {
        /// Number of words in the passphrase (default: 4)
        #[arg(short, long, default_value = "4")]
        words: usize,

        /// Separator between words (default: space)
        #[arg(short, long, default_value = " ")]
        separator: String,

        /// Include numbers in the passphrase
        #[arg(short, long)]
        numbers: bool,

        /// Include special characters in the passphrase
        #[arg(short, long)]
        special: bool,

        /// Copy passphrase to clipboard (macOS only)
        #[arg(short, long)]
        copy: bool,
    },

    /// Check password strength
    Check {
        /// Password to check
        password: String,
    },

    /// Generate a secure hash
    Hash {
        /// Input string to hash
        input: String,

        /// Hash algorithm: sha256, sha512, base64
        #[arg(short, long, default_value = "sha256")]
        algorithm: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            length,
            uppercase,
            lowercase,
            numbers,
            special,
            exclude_similar,
            exclude_ambiguous,
            format,
            copy,
        } => {
            let mut generator = PasswordGenerator::new();

            // Set character sets based on flags
            if *uppercase {
                generator.include_uppercase();
            }
            if *lowercase {
                generator.include_lowercase();
            }
            if *numbers {
                generator.include_numbers();
            }
            if *special {
                generator.include_special();
            }
            if *exclude_similar {
                generator.exclude_similar();
            }
            if *exclude_ambiguous {
                generator.exclude_ambiguous();
            }

            // If no character sets specified, use all
            if !(*uppercase || *lowercase || *numbers || *special) {
                generator.include_all();
            }

            let password = generator.generate(*length)?;
            let formatted_password = format_password(&password, format)?;

            if *copy {
                copy_to_clipboard(&formatted_password)?;
                println!("{}", "Password copied to clipboard!".green());
            }

            println!("Generated Password: {}", formatted_password.cyan());
            println!("Length: {} characters", password.len());
            println!("Entropy: {:.2} bits", calculate_entropy(&password));
        }

        Commands::Passphrase {
            words,
            separator,
            numbers,
            special,
            copy,
        } => {
            let passphrase = generate_passphrase(*words, separator, *numbers, *special)?;

            if *copy {
                copy_to_clipboard(&passphrase)?;
                println!("{}", "Passphrase copied to clipboard!".green());
            }

            println!("Generated Passphrase: {}", passphrase.cyan());
            println!("Words: {}", words);
            println!("Length: {} characters", passphrase.len());
        }

        Commands::Check { password } => {
            let strength = check_password_strength(password);
            println!("Password Strength Analysis:");
            println!("Password: {}", password.cyan());
            println!("Length: {} characters", password.len());
            println!("Entropy: {:.2} bits", calculate_entropy(password));
            println!("Strength: {}", strength.to_string().color(strength.color()));

            // Show detailed analysis
            let analysis = analyze_password(password);
            println!("\nDetailed Analysis:");
            for (criterion, status) in analysis {
                let status_text = if status { "✓" } else { "✗" };
                let status_color = if status { "green" } else { "red" };
                println!("  {} {}", status_text.color(status_color), criterion);
            }
        }

        Commands::Hash { input, algorithm } => {
            let hash = generate_hash(input, algorithm)?;
            println!("Input: {}", input.cyan());
            println!("Algorithm: {}", algorithm);
            println!("Hash: {}", hash.yellow());
        }
    }

    Ok(())
}
