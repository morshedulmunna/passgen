# Password Generator CLI - Usage Guide

## Quick Start

After installation, you can use the `passgen` command from anywhere in your terminal.

## Basic Commands

### 1. Generate a Random Password

```bash
# Generate a 16-character password (default)
passgen generate

# Generate a 12-character password
passgen generate --length 12

# Generate a 20-character password with copy to clipboard
passgen generate --length 20 --copy
```

### 2. Generate a Passphrase

```bash
# Generate a 4-word passphrase (default)
passgen passphrase

# Generate a 6-word passphrase
passgen passphrase --words 6

# Generate a passphrase with numbers and copy to clipboard
passgen passphrase --words 5 --numbers --copy
```

### 3. Check Password Strength

```bash
# Check the strength of a password
passgen check "mypassword123"

# Check with a more complex password
passgen check "MySecureP@ssw0rd!"
```

### 4. Generate a Hash

```bash
# Generate SHA256 hash
passgen hash "my-secret-text"

# Generate SHA512 hash
passgen hash "my-secret-text" --algorithm sha512

# Generate Base64 encoding
passgen hash "my-secret-text" --algorithm base64
```

## Advanced Usage

### Password Generation Options

```bash
# Generate password with specific character sets
passgen generate --uppercase --lowercase --numbers --special

# Generate password excluding similar characters (l, 1, I, O, 0)
passgen generate --exclude-similar

# Generate password excluding ambiguous characters
passgen generate --exclude-ambiguous

# Generate password in different formats
passgen generate --format base64
passgen generate --format hex
```

### Passphrase Options

```bash
# Generate passphrase with custom separator
passgen passphrase --separator "-"

# Generate passphrase with numbers and special characters
passgen passphrase --numbers --special

# Generate passphrase with custom separator and copy
passgen passphrase --separator "_" --copy
```

## Examples

### Example 1: Generate a Strong Password for a Website

```bash
# Generate a 20-character password with all character types
passgen generate --length 20 --uppercase --lowercase --numbers --special --copy
```

### Example 2: Generate a Memorable Passphrase

```bash
# Generate a 5-word passphrase with numbers
passgen passphrase --words 5 --numbers --copy
```

### Example 3: Check Your Current Password

```bash
# Check if your current password is strong enough
passgen check "your-current-password"
```

### Example 4: Generate a Hash for API Key

```bash
# Generate SHA256 hash for an API key
passgen hash "your-api-key-here" --algorithm sha256
```

## Output Formats

### Password Formats

- **plain**: Regular text password (default)
- **base64**: Base64 encoded password
- **hex**: Hexadecimal encoded password

### Hash Algorithms

- **sha256**: SHA-256 hash (default)
- **sha512**: SHA-512 hash
- **base64**: Base64 encoding

## Clipboard Integration

The `--copy` flag automatically copies the generated password/passphrase to your clipboard on macOS.

**Note**: Make sure your terminal has clipboard permissions:

1. Go to System Preferences > Security & Privacy > Privacy > Accessibility
2. Add Terminal (or your terminal app) to the list

## Security Features

### Character Sets

- **Uppercase letters**: A-Z
- **Lowercase letters**: a-z
- **Numbers**: 0-9
- **Special characters**: !@#$%^&\*()\_+-=[]{}|;:,.<>?

### Excluded Characters

- **Similar characters**: l, 1, I, O, 0 (when using `--exclude-similar`)
- **Ambiguous characters**: {}, [], (), /, \, ', ", ~, ;, :, ., >, < (when using `--exclude-ambiguous`)

### Password Strength Analysis

The tool analyzes passwords based on:

- Length
- Character variety
- Entropy (randomness)
- Common patterns
- Dictionary words

## Tips for Best Practices

1. **Use longer passwords**: Aim for at least 16 characters
2. **Include all character types**: Use `--uppercase --lowercase --numbers --special`
3. **Use passphrases for memorability**: Combine with `--numbers --special`
4. **Check existing passwords**: Use the `check` command to verify strength
5. **Use clipboard**: The `--copy` flag makes it easy to paste passwords
6. **Exclude similar characters**: Use `--exclude-similar` for better readability

## Help and Information

```bash
# Show general help
passgen --help

# Show help for specific command
passgen generate --help
passgen passphrase --help
passgen check --help
passgen hash --help

# Show version
passgen --version
```
