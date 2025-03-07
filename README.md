# Systemd Wizard

A simple Rust CLI tool to generate and manage systemd service files with an easy-to-use wizard.

## Features

- Guides users through service setup with simple questions.
- Generates a valid `.service` file.
- Allows enabling and starting the service directly.

## Installation

### Prerequisites

Ensure your system has the following dependencies installed:
- **Rust & Cargo** (Install via [Rustup](https://rustup.rs/))
- **Git**

### Install Locally

```sh
cargo install --path .
```

### Install via Script

To install globally, run:

```sh
curl -fsSL https://raw.githubusercontent.com/AncientiCe/systemd-wizard/main/installation/install.sh | bash
```

For remote installations via SSH:

```sh
ssh user@server "curl -fsSL https://raw.githubusercontent.com/AncientiCe/systemd-wizard/main/installation/install.sh | bash"
```

### Uninstall

To remove `systemd-wizard`, run:

```sh
curl -fsSL https://raw.githubusercontent.com/AncientiCe/systemd-wizard/main/installation/uninstall.sh | bash
```

## Usage

```sh
systemd-wizard
```

Follow the interactive prompts to create and manage systemd service files easily.

## License

MIT

