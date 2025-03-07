#!/bin/bash

echo "🔹 Uninstalling systemd-wizard..."

sudo rm -f /usr/local/bin/systemd-wizard
sudo rm -f /usr/share/bash-completion/completions/systemd-wizard
sudo rm -f /usr/share/zsh/site-functions/_systemd-wizard

echo "✅ systemd-wizard removed!"
