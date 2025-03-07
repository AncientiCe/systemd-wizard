#!/bin/bash

set -e

echo "🔹 Installing systemd-wizard..."

# Ensure dependencies are installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust (cargo) is not installed. Install it first: https://rustup.rs/"
    exit 1
fi

# Clone repo if not already present
if [ ! -d "systemd-wizard" ]; then
    git clone https://github.com/AncientiCe/systemd-wizard.git
fi

cd systemd-wizard

# Build the binary
cargo build --release

# Move binary to /usr/local/bin for global access
sudo mv target/release/systemd-wizard /usr/local/bin/systemd-wizard

# Ensure permissions
sudo chmod +x /usr/local/bin/systemd-wizard

echo "✅ systemd-wizard installed successfully!"

# Add completion scripts
if [ -d "/usr/share/bash-completion/completions" ]; then
    sudo cp completions/systemd-wizard.bash /usr/share/bash-completion/completions/systemd-wizard
fi

if [ -d "/usr/share/zsh/site-functions" ]; then
    sudo cp completions/_systemd-wizard /usr/share/zsh/site-functions/
fi

echo "🎉 Done! Run 'systemd-wizard' to start."
