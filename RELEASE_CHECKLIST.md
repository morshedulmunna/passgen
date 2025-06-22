# Release Checklist for Password Generator CLI

This checklist will guide you through releasing your CLI tool globally for Mac users.

## Pre-Release Preparation

### 1. Update Version and Metadata

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `Formula/passgen.rb`
- [ ] Update `CHANGELOG.md` with new features/fixes
- [ ] Update author information in `Cargo.toml`
- [ ] Update repository URLs in all files

### 2. Test the Build

- [ ] Run `cargo build --release`
- [ ] Test all commands locally
- [ ] Test clipboard functionality on macOS
- [ ] Run `cargo test` to ensure all tests pass
- [ ] Test installation scripts

### 3. Documentation Updates

- [ ] Update `README.md` with installation instructions
- [ ] Update `USAGE.md` if needed
- [ ] Add installation instructions for all methods
- [ ] Update any outdated examples

## Release Methods

### Method 1: GitHub Releases (Recommended)

1. **Create a GitHub Repository**

   - [ ] Create a new repository on GitHub
   - [ ] Push your code to the repository
   - [ ] Update all URLs in files to point to your repository

2. **Set up GitHub Actions**

   - [ ] Ensure `.github/workflows/release.yml` is in place
   - [ ] Push the workflow file to trigger the setup

3. **Create a Release**

   ```bash
   # Tag and push to trigger automatic release
   git tag v0.1.0
   git push origin v0.1.0
   ```

4. **Update Homebrew Formula**
   - [ ] Fork the Homebrew/homebrew-core repository
   - [ ] Create a new formula in `Formula/passgen.rb`
   - [ ] Update the SHA256 hash with the actual hash from your release
   - [ ] Submit a pull request

### Method 2: Homebrew Tap (Easier)

1. **Create a Homebrew Tap**

   ```bash
   # Create a new repository named homebrew-tap
   # Add your formula to it
   ```

2. **Install via Tap**
   ```bash
   brew tap yourusername/tap
   brew install passgen
   ```

### Method 3: Cargo (crates.io)

1. **Publish to crates.io**

   ```bash
   cargo login
   cargo publish
   ```

2. **Install via Cargo**
   ```bash
   cargo install passgen-cli
   ```

### Method 4: Universal Installer

1. **Make installer executable**

   ```bash
   chmod +x install-universal.sh
   ```

2. **Users can install with**
   ```bash
   curl -fsSL https://raw.githubusercontent.com/yourusername/passgen/main/install-universal.sh | bash
   ```

## Post-Release Tasks

### 1. Update Documentation

- [ ] Update installation instructions in README
- [ ] Add badges for build status, version, etc.
- [ ] Update any platform-specific instructions

### 2. Marketing and Distribution

- [ ] Create a release announcement
- [ ] Share on relevant platforms (Reddit, Twitter, etc.)
- [ ] Update your personal website/portfolio
- [ ] Consider writing a blog post about the tool

### 3. Monitor and Maintain

- [ ] Monitor GitHub issues and pull requests
- [ ] Respond to user feedback
- [ ] Plan future releases
- [ ] Keep dependencies updated

## Quick Release Commands

```bash
# 1. Update version
sed -i '' 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml

# 2. Build and test
cargo build --release
cargo test

# 3. Commit changes
git add .
git commit -m "Release v0.1.1"
git push

# 4. Create and push tag
git tag v0.1.1
git push origin v0.1.1

# 5. Wait for GitHub Actions to complete
# 6. Update Homebrew formula with new SHA256
```

## Troubleshooting

### Common Issues

1. **GitHub Actions failing**

   - Check action versions in workflow file
   - Ensure repository has proper permissions
   - Verify all dependencies are available

2. **Homebrew formula rejected**

   - Ensure formula follows Homebrew guidelines
   - Test formula locally before submitting
   - Provide proper documentation

3. **Cargo publish issues**
   - Ensure package name is unique
   - Verify all metadata is correct
   - Check for any sensitive information in code

### Support

- GitHub Issues: For bug reports and feature requests
- GitHub Discussions: For general questions and community
- Email: For security issues or private matters

## Next Steps

After your first release:

1. **Monitor usage and feedback**
2. **Plan feature roadmap**
3. **Consider adding more platforms**
4. **Build a community around the tool**
5. **Consider monetization options (if applicable)**
