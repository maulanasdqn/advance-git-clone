# Advanced Git Clone (adc)

A powerful command-line tool that enables cloning GitHub repositories using specific SSH keys. Perfect for developers managing multiple GitHub accounts or projects that require different SSH authentication keys.

## 🚀 Overview

`adc` (Advanced Git Clone) simplifies the process of cloning repositories when you need to use specific SSH keys for authentication. Instead of manually configuring Git's SSH settings each time, `adc` handles the SSH key selection automatically.

## ✨ Features

- **🔑 SSH Key Selection**: Specify which SSH key to use for cloning
- **📁 Flexible Directory Options**: Clone to current directory or specify custom directory name
- **✅ Key Validation**: Automatically validates SSH key existence before attempting to clone
- **🛡️ Error Handling**: Comprehensive error messages for troubleshooting
- **⚡ Fast & Lightweight**: Built in Rust for optimal performance
- **📖 Built-in Help**: Comprehensive help system with usage examples

## 📋 Requirements

- **Rust**: Version 1.70+ (for building from source)
- **Git**: Must be installed and available in PATH
- **SSH Keys**: Your SSH keys should be stored in `~/.ssh/` directory

## 🔧 Installation

### Option 1: Build from Source

```bash
# Clone this repository
git clone https://github.com/yourusername/advance-git-clone.git
cd advance-git-clone

# Build the project
cargo build --release

# Install to system PATH (optional)
sudo cp ./target/release/adc /usr/local/bin/
```

### Option 2: Direct Binary Usage

After building, you can use the binary directly:

```bash
./target/release/adc --ssh your_key_name git@github.com:username/repo.git
```

## 🎯 Usage

### Basic Syntax

```bash
adc --ssh <SSH_KEY_NAME> <REPOSITORY_URL> [DIRECTORY]
```

### Examples

#### Clone to Current Directory
```bash
adc --ssh id_rsa git@github.com:username/awesome-project.git
```

#### Clone to Specific Directory
```bash
adc --ssh id_work git@github.com:company/project.git my-project-folder
```

#### Using Different SSH Keys for Different Accounts
```bash
# Personal GitHub account
adc --ssh id_personal git@github.com:personal/my-blog.git

# Work GitHub account  
adc --ssh id_work git@github.com:company/enterprise-app.git

# Client project
adc --ssh id_client git@github.com:client/secret-project.git client-work
```

### Command Options

| Option | Description | Required |
|--------|-------------|----------|
| `--ssh <KEY>` | Name of SSH key in `~/.ssh/` directory | ✅ Yes |
| `<URL>` | SSH URL of the repository to clone | ✅ Yes |
| `[DIRECTORY]` | Custom directory name for cloned repo | ❌ No |
| `--help` | Show help information | ❌ No |
| `--version` | Show version information | ❌ No |

## 🔍 How It Works

1. **Key Validation**: `adc` checks if the specified SSH key exists in `~/.ssh/`
2. **SSH Configuration**: Sets up `GIT_SSH_COMMAND` environment variable with the correct key
3. **Repository Cloning**: Executes `git clone` with the configured SSH settings
4. **Error Handling**: Provides clear feedback on success or failure

## 📂 SSH Key Management

### Expected SSH Key Location
```
~/.ssh/
├── id_rsa          # Default key
├── id_work         # Work account key
├── id_personal     # Personal account key
└── id_client       # Client project key
```

### Setting Up SSH Keys

If you need to create SSH keys for different accounts:

```bash
# Generate a new SSH key
ssh-keygen -t rsa -b 4096 -C "your.email@example.com" -f ~/.ssh/id_work

# Add to SSH agent
ssh-add ~/.ssh/id_work

# Copy public key to clipboard (Linux)
cat ~/.ssh/id_work.pub | xclip -selection clipboard
```

## 🔧 Development

### Building the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

### Project Structure

```
advance-git-clone/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main application logic
├── README.md           # This file
└── target/             # Build output (generated)
    └── release/
        └── adc         # Compiled binary
```

## 🚨 Troubleshooting

### Common Issues

#### SSH Key Not Found
```
Error: SSH key 'id_nonexistent' not found at path: /home/user/.ssh/id_nonexistent
```
**Solution**: Verify the SSH key name and ensure it exists in `~/.ssh/`

#### Permission Denied
```
Error: Git clone failed with exit code: Some(128)
```
**Solution**: Check if the SSH key is properly configured with your GitHub account

#### Git Command Not Found
```
Error: Failed to execute git command: No such file or directory
```
**Solution**: Ensure Git is installed and available in your PATH

### Debug Mode

For troubleshooting, you can check what SSH command is being used:

```bash
# The tool uses this format internally:
GIT_SSH_COMMAND="ssh -i ~/.ssh/your_key -o StrictHostKeyChecking=no" git clone <url>
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📋 Use Cases

- **Multiple GitHub Accounts**: Easily switch between personal, work, and client accounts
- **Team Development**: Different projects requiring different SSH keys
- **Security Compliance**: Projects with specific key requirements
- **Automated Scripts**: Integration into deployment or setup scripts

## 🔄 Comparison with Standard Git

| Feature | Standard Git | adc |
|---------|-------------|-----|
| SSH key selection | Manual configuration required | Single `--ssh` flag |
| Key validation | No validation | Automatic validation |
| Error messages | Generic Git errors | Specific, actionable errors |
| Multi-account support | Complex setup | Simple and intuitive |

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and reliability
- Uses [clap](https://docs.rs/clap/) for command-line argument parsing
- Inspired by the need for simplified multi-account Git workflows

---

**Made with ❤️ for developers who juggle multiple Git accounts** 