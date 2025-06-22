# Password Generator CLI - macOS Release Guide

## Release Methods

### Method 1: Using the Install Script (Recommended)

1. **Clone or download the project**

   ```bash
   git clone <your-repo-url>
   cd password-generate-cli
   ```

2. **Run the install script**
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

This will automatically:

- Check if Rust is installed
- Build the project in release mode
- Install the `passgen` command globally
- Make it available system-wide

### Method 2: Manual Installation

1. **Build the project**

   ```bash
   cargo build --release
   ```

2. **Install globally**
   ```bash
   cargo install --path .
   ```

### Method 3: Create a Standalone Binary

1. **Build for macOS**

   ```bash
   cargo build --release --target x86_64-apple-darwin
   # For Apple Silicon Macs
   cargo build --release --target aarch64-apple-darwin
   ```

2. **The binary will be in:**

   ```
   target/release/passgen
   ```

3. **Copy to a location in your PATH**
   ```bash
   sudo cp target/release/passgen /usr/local/bin/
   # Or for user installation
   cp target/release/passgen ~/.local/bin/
   ```

## Distribution Options

### Option 1: Homebrew Tap (Recommended for Public Distribution)

1. **Create a Homebrew formula**
2. **Host the binary on GitHub Releases**
3. **Users can install with:**
   ```bash
   brew install your-username/your-tap/passgen
   ```

### Option 2: Direct Binary Distribution

1. **Upload the release binary to GitHub Releases**
2. **Users download and install manually**

### Option 3: Cargo Install (If Published to crates.io)

1. **Publish to crates.io**

   ```bash
   cargo publish
   ```

2. **Users install with:**
   ```bash
   cargo install passgen
   ```

## Verification

After installation, verify it works:

```bash
passgen --help
passgen generate --length 12
```

## Troubleshooting

### Common Issues:

1. **Permission denied**: Make sure the binary is executable

   ```bash
   chmod +x /usr/local/bin/passgen
   ```

2. **Command not found**: Ensure the binary is in your PATH

   ```bash
   echo $PATH
   which passgen
   ```

3. **Clipboard not working**: Ensure you have clipboard permissions on macOS
   - Go to System Preferences > Security & Privacy > Privacy > Accessibility
   - Add Terminal or your terminal app

## System Requirements

- macOS 10.15 (Catalina) or later
- Rust 1.70+ (for building from source)
- Terminal with clipboard access permissions
