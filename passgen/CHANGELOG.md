# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Universal installer script for cross-platform installation
- GitHub Actions workflow for automated releases
- Homebrew formula for easy macOS installation
- Comprehensive documentation and installation guides
- Platform detection and automatic binary selection

### Changed

- Updated package name to `passgen-cli` for crates.io compatibility
- Enhanced README with multiple installation methods
- Improved error handling and user feedback

## [0.1.0] - 2024-01-XX

### Added

- Initial release of Password Generator CLI
- Secure password generation with customizable character sets
- Passphrase generation with word lists
- Password strength analysis and entropy calculation
- Clipboard integration for macOS
- Multiple output formats (plain, base64, hex)
- Hash generation (SHA256, SHA512, Base64)
- Character exclusion options (similar, ambiguous)
- Colored output for better user experience
- Comprehensive command-line interface with Clap

### Features

- **Generate Command**: Create secure passwords with various options
- **Passphrase Command**: Generate memorable passphrases
- **Check Command**: Analyze password strength
- **Hash Command**: Generate cryptographic hashes
- Cross-platform support (macOS, Linux, Windows)
- Cryptographically secure random number generation
- Detailed password strength analysis
- Clipboard integration for easy copying

### Technical Details

- Built with Rust for performance and security
- Uses `OsRng` for cryptographically secure randomness
- Implements entropy calculation for password strength
- Supports multiple character sets and exclusions
- Modular architecture with separate generator and utils modules
