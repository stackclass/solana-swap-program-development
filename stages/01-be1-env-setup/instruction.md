# Environment Setup

This stage guides you through installing and configuring all necessary tools for Solana program development. A properly configured environment is essential for efficient development workflow and avoiding setup-related issues later in the course.

## Prerequisites

Before beginning, ensure you have a modern terminal application available on your system. On macOS, the built-in Terminal.app or iTerm2 work well. On Linux, your preferred terminal emulator suffices. Windows users should consider Windows Terminal for the best experience.

You will need administrator or root access to install system-level dependencies. Ensure you have a stable internet connection as the installation process downloads several tools and packages.

## Installing Rust

Solana programs are written in Rust, making Rust tooling your first essential installation. Rust provides memory safety and concurrency guarantees that make it ideal for secure smart contract development.

Open your terminal and execute the following command to download and install Rust using rustup, the official Rust toolchain manager:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command downloads a bootstrap script that detects your system configuration and installs appropriate components. The installer will prompt you to choose installation options. The default selection works well for most users, so pressing Enter to accept defaults is appropriate.

After installation completes, you must restart your terminal or source your profile for changes to take effect:

```bash
source ~/.cargo/env
```

Verify successful installation by checking the Rust compiler version:

```bash
rustc --version
```

You should see output similar to `rustc 1.XX.X` indicating a recent stable version. Update Rust periodically using `rustup update` to receive the latest features and security patches.

## Installing Solana CLI

The Solana command-line interface provides essential tools for interacting with the Solana blockchain, managing keys, deploying programs, and testing locally. Installation requires downloading the Solana installation binary.

Execute the official installation script:

```bash
sh -c "$(curl -sSfL "https://release.solana.com/stable/install")"
```

The installer adds Solana binaries to your PATH. Restart your terminal or source your profile to ensure the solana command is available:

```bash
source ~/.bash_profile  # or ~/.zshrc for Zsh users
```

Verify installation by checking the CLI version:

```bash
solana --version
```

Configure Solana to use devnet for development, which provides test tokens without real value:

```bash
solana config set --url devnet
```

This setting ensures all subsequent commands target the devnet environment. You can later switch to localnet for faster iteration during development.

Generate a keypair for development use:

```bash
solana-keygen new
```

This command creates a file at `~/.config/solana/id.json` containing your private key. Back up this file securely and never share it. The associated public key serves as your developer identity for deploying programs and signing transactions.

## Installing Anchor Framework

Anchor is a framework that simplifies Solana program development through higher-level abstractions, automatic account validation, and IDL generation. It significantly reduces boilerplate code and prevents common programming errors.

Install Anchor using cargo, Rust's package manager:

```bash
cargo install anchor-cli anchor-lang
```

This command compiles and installs the Anchor command-line tool and library. Installation may take several minutes as it downloads dependencies and builds from source. The wait is worthwhile given Anchor's development efficiency benefits.

Verify Anchor installation:

```bash
anchor --version
```

You should see version information confirming Anchor is available. If you encounter version compatibility issues, consult the Anchor documentation for recommended Rust version requirements.

## Verifying Your Environment

With all tools installed, run a comprehensive verification to ensure everything works together correctly. Create a new Anchor project to test your setup:

```bash
anchor init my-test-program
cd my-test-program
anchor build
```

The build process compiles your empty program and generates type definitions. Successful completion confirms your Rust toolchain, Solana CLI, and Anchor framework are properly configured.

## Common Issues and Solutions

Rust installation failing on Apple Silicon Macs requires installing Rosetta 2 compatibility layer. Install using:

```bash
softwareupdate --install-rosetta
```

Solana CLI timeout errors during installation typically indicate network issues. Retry the installation command or try during off-peak hours.

Anchor build failures often stem from incompatible Rust versions. Anchor typically requires Rust version 1.XX. Check Anchor documentation for specific version requirements and adjust using:

```bash
rustup install 1.XX.X
rustup default 1.XX.X
```

## Next Steps

Your development environment is now configured and ready. Proceed to the next stage where you will learn Rust fundamentals essential for Solana program development. The skills acquired in subsequent stages build directly on this foundation.
