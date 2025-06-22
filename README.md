# Password Generator CLI Tool

A secure and feature-rich password generator CLI tool built in Rust, specifically designed for macOS.

## Features

- 🔐 **Secure Password Generation**: Uses cryptographically secure random number generation
- 📝 **Passphrase Generation**: Generate memorable passphrases from word lists
- 🔍 **Password Strength Analysis**: Check password strength with detailed analysis
- 📋 **Clipboard Integration**: Copy passwords directly to macOS clipboard
- 🎨 **Multiple Output Formats**: Plain text, Base64, and Hex encoding
- ⚙️ **Customizable Options**: Control character sets, length, and exclusions
- 🔒 **Hash Generation**: Generate SHA256, SHA512, and Base64 hashes

## Installation

### Method 1: Universal Installer (Recommended)

The easiest way to install on any platform:

```bash
curl -fsSL https://raw.githubusercontent.com/morshedulmunna/passgen/main/install-universal.sh | bash
```

### Method 2: Homebrew (macOS)

```bash
# Add the tap (if using custom tap)
brew tap morshedulmunna/tap

# Install the tool
brew install passgen
```

### Method 3: Cargo (Rust Package Manager)

```bash
cargo install passgen-cli
```

### Method 4: Build from Source

1. Clone or download this repository
2. Navigate to the project directory:

   ```bash
   cd passgen
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. Install globally:
   ```bash
   cargo install --path .
   ```

### Method 5: Manual Installation

1. Download the latest release for your platform from [GitHub Releases](https://github.com/morshedulmunna/passgen/releases)
2. Extract the archive
3. Move the binary to a directory in your PATH:

   ```bash
   # For macOS/Linux
   sudo mv passgen /usr/local/bin/

   # Or for user installation
   mkdir -p ~/.local/bin
   mv passgen ~/.local/bin/
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

## Quick Start

After installation, try these commands:

```bash
# Generate a secure password
passgen generate --length 16 --copy

# Generate a memorable passphrase
passgen passphrase --words 4

# Check password strength
passgen check "MyPassword123!"

# Generate a hash
passgen hash "my-secret-string" --algorithm sha256
```

## Usage

### Generate Passwords

Generate a basic 16-character password:

```bash
passgen generate
```

Generate a 32-character password with specific character sets:

```bash
passgen generate --length 32 --uppercase --lowercase --numbers --special
```

Generate a password and copy to clipboard:

```bash
passgen generate --length 20 --copy
```

Generate a password in Base64 format:

```bash
passgen generate --length 16 --format base64
```

Exclude similar and ambiguous characters:

```bash
passgen generate --exclude-similar --exclude-ambiguous
```

### Generate Passphrases

Generate a 4-word passphrase:

```bash
passgen passphrase
```

Generate a 6-word passphrase with numbers and special characters:

```bash
passgen passphrase --words 6 --numbers --special
```

Generate a passphrase with custom separator:

```bash
passgen passphrase --separator "-" --copy
```

### Check Password Strength

Analyze a password:

```bash
passgen check "MyPassword123!"
```

### Generate Hashes

Generate SHA256 hash:

```bash
passgen hash "my-secret-string"
```

Generate SHA512 hash:

```bash
passgen hash "my-secret-string" --algorithm sha512
```

Generate Base64 encoding:

```bash
passgen hash "my-secret-string" --algorithm base64
```

## Command Options

### Generate Command

- `-l, --length <LENGTH>`: Password length (default: 16)
- `-u, --uppercase`: Include uppercase letters
- `-d, --lowercase`: Include lowercase letters
- `-n, --numbers`: Include numbers
- `-s, --special`: Include special characters
- `--exclude-similar`: Exclude similar characters (l, 1, I, O, 0)
- `--exclude-ambiguous`: Exclude ambiguous characters
- `-f, --format <FORMAT>`: Output format (plain, base64, hex)
- `-c, --copy`: Copy to clipboard

### Passphrase Command

- `-w, --words <WORDS>`: Number of words (default: 4)
- `--separator <SEPARATOR>`: Word separator (default: space)
- `-n, --numbers`: Include numbers
- `-s, --special`: Include special characters
- `-c, --copy`: Copy to clipboard

### Check Command

- `password`: Password to analyze

### Hash Command

- `input`: Input string to hash
- `-a, --algorithm <ALGORITHM>`: Hash algorithm (sha256, sha512, base64)

## Examples

### Strong Password for Online Banking

```bash
passgen generate --length 24 --uppercase --lowercase --numbers --special --exclude-similar --copy
```

### Memorable Passphrase for Personal Use

```bash
passgen passphrase --words 5 --numbers --special --copy
```

### Quick Password Check

```bash
passgen check "password123"
```

### Generate API Key Hash

```bash
passgen hash "my-api-key" --algorithm sha256
```

## Security Features

- **Cryptographically Secure**: Uses `OsRng` for true randomness
- **Entropy Calculation**: Measures password strength in bits
- **Character Set Control**: Fine-grained control over character types
- **Pattern Detection**: Identifies common weak patterns
- **Exclusion Options**: Avoid confusing or problematic characters

## Password Strength Levels

- **Very Weak**: < 20 bits entropy
- **Weak**: 20-30 bits entropy
- **Medium**: 30-40 bits entropy
- **Strong**: 40-50 bits entropy
- **Very Strong**: > 50 bits entropy

## Platform Support

- ✅ macOS (Intel & Apple Silicon)
- ✅ Linux (x86_64 & ARM64)
- ✅ Windows (x86_64)

## Troubleshooting

### Clipboard Issues on macOS

If clipboard functionality doesn't work:

1. Go to System Preferences > Security & Privacy > Privacy > Accessibility
2. Click the lock icon to make changes
3. Add your terminal application (Terminal, iTerm2, etc.)
4. Restart your terminal

### Installation Issues

If you encounter installation problems:

1. Ensure you have the latest version of Rust installed
2. Try the universal installer: `curl -fsSL https://raw.githubusercontent.com/morshedulmunna/passgen/main/install-universal.sh | bash`
3. Check the [GitHub Issues](https://github.com/morshedulmunna/passgen/issues) for known problems

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This tool is provided for educational and personal use. Always follow your organization's password policies and security guidelines. The authors are not responsible for any security issues arising from the use of this tool.
