# PassGen Web Interface

This document explains how to use the PassGen password generator through its web interface.

## Features

The web interface provides all the functionality of the CLI tool in a user-friendly web browser interface:

- **Password Generation**: Generate secure passwords with customizable character sets
- **Passphrase Generation**: Create memorable passphrases from word lists
- **Password Strength Checking**: Analyze password strength and security
- **Hash Generation**: Generate SHA-256, SHA-512, and Base64 hashes

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system
- All dependencies from `Cargo.toml`

### Running the Web Server

1. **Build the web server:**

   ```bash
   cargo build --bin passgen-web
   ```

2. **Start the web server:**

   ```bash
   cargo run --bin passgen-web
   ```

3. **Access the web interface:**
   Open your web browser and navigate to: `http://localhost:8080`

The server will start on `127.0.0.1:8080` with 12 worker threads for optimal performance.

## Web Interface Usage

### 1. Generate Password

**Features:**

- Customizable password length (4-128 characters)
- Character set selection:
  - Uppercase letters (A-Z)
  - Lowercase letters (a-z)
  - Numbers (0-9)
  - Special characters
- Exclusion options:
  - Similar characters (l, 1, I, O, 0)
  - Ambiguous characters ({}, [], (), /, \, ', ", ~, ;, :, ., >, <)
- Output formats:
  - Plain text
  - Base64 encoded
  - Hexadecimal

**Usage:**

1. Select the "Generate Password" tab
2. Set your desired password length
3. Choose character sets by checking/unchecking boxes
4. Configure exclusion options if needed
5. Select output format
6. Click "Generate Password"
7. Copy the generated password to clipboard

### 2. Generate Passphrase

**Features:**

- Configurable number of words (2-20)
- Custom separator between words
- Optional numbers and special characters
- Uses a curated list of common, memorable words

**Usage:**

1. Select the "Generate Passphrase" tab
2. Set the number of words
3. Choose a separator (default: space)
4. Enable numbers and/or special characters if desired
5. Click "Generate Passphrase"
6. Copy the generated passphrase to clipboard

### 3. Check Password Strength

**Features:**

- Comprehensive password analysis
- Entropy calculation
- Strength rating (Very Weak to Very Strong)
- Detailed criteria checking:
  - Minimum length requirements
  - Character set diversity
  - Complexity assessment

**Usage:**

1. Select the "Check Strength" tab
2. Enter the password you want to analyze
3. Click "Check Strength"
4. Review the detailed analysis and recommendations

### 4. Generate Hash

**Features:**

- Multiple hash algorithms:
  - SHA-256
  - SHA-512
  - Base64 encoding
- Secure hash generation
- Copy hash to clipboard

**Usage:**

1. Select the "Generate Hash" tab
2. Enter the text you want to hash
3. Choose the hash algorithm
4. Click "Generate Hash"
5. Copy the generated hash to clipboard

## API Endpoints

The web interface is built on top of RESTful API endpoints:

### POST `/api/generate`

Generate a password with specified parameters.

**Request Body:**

```json
{
  "length": 16,
  "uppercase": true,
  "lowercase": true,
  "numbers": true,
  "special": true,
  "exclude_similar": false,
  "exclude_ambiguous": false,
  "format": "plain"
}
```

**Response:**

```json
{
  "password": "generated_password",
  "length": 16,
  "entropy": 95.2,
  "formatted_password": "generated_password"
}
```

### POST `/api/passphrase`

Generate a passphrase with specified parameters.

**Request Body:**

```json
{
  "words": 4,
  "separator": " ",
  "numbers": false,
  "special": false
}
```

**Response:**

```json
{
  "passphrase": "apple banana cherry dragon",
  "words": 4,
  "length": 23
}
```

### POST `/api/check`

Check password strength and provide analysis.

**Request Body:**

```json
{
  "password": "password_to_check"
}
```

**Response:**

```json
{
  "password": "password_to_check",
  "length": 16,
  "entropy": 85.3,
  "strength": "Strong",
  "analysis": [
    {
      "criterion": "At least 8 characters",
      "status": true
    },
    {
      "criterion": "Contains uppercase letters",
      "status": true
    }
  ]
}
```

### POST `/api/hash`

Generate a hash of the input text.

**Request Body:**

```json
{
  "input": "text_to_hash",
  "algorithm": "sha256"
}
```

**Response:**

```json
{
  "input": "text_to_hash",
  "algorithm": "sha256",
  "hash": "generated_hash_value"
}
```

## Security Features

- **Cryptographically Secure Random Generation**: Uses `OsRng` for true randomness
- **No Password Storage**: Passwords are never stored or logged
- **Client-Side Processing**: All sensitive operations happen on the server
- **HTTPS Ready**: Can be configured with SSL/TLS certificates
- **Input Validation**: All inputs are validated and sanitized

## Browser Compatibility

The web interface is compatible with:

- Chrome 60+
- Firefox 55+
- Safari 12+
- Edge 79+

## Development

### Project Structure

```
src/
├── main.rs          # CLI entry point
├── web_main.rs      # Web server entry point
├── generator.rs     # Password generation logic
└── utils.rs         # Utility functions
```

### Building for Production

```bash
cargo build --release --bin passgen-web
```

### Running with Custom Configuration

```bash
RUST_LOG=info cargo run --bin passgen-web
```

## Troubleshooting

### Common Issues

1. **Port Already in Use**

   - The default port 8080 is already occupied
   - Solution: Modify the port in `web_main.rs` or kill the process using port 8080

2. **Build Errors**

   - Ensure all dependencies are installed: `cargo build`
   - Check Rust version: `rustc --version`

3. **Web Interface Not Loading**
   - Verify the server is running: `curl http://localhost:8080`
   - Check browser console for JavaScript errors

### Logs

The web server provides detailed logging. Set the log level with:

```bash
RUST_LOG=debug cargo run --bin passgen-web
```

## Contributing

To contribute to the web interface:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
