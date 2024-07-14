#!/bin/bash

# Ensure the script fails if any command fails
set -e

# Install Rust if not already installed
if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "Rust installed successfully."
else
    echo "Rust is already installed."
fi

# Update Rust and install necessary components
echo "Updating Rust and installing components..."
rustup update
rustup component add rustfmt clippy

# Install Solana CLI tools
echo "Installing Solana CLI..."
if ! command -v solana &> /dev/null; then
    sh -c "$(curl -sSfL https://release.solana.com/v1.9.2/install)"
    echo "Solana CLI installed successfully."
else
    echo "Solana CLI is already installed."
fi

# Install Anchor CLI
echo "Installing Anchor CLI..."
if ! command -v anchor &> /dev/null; then
    cargo install --git https://github.com/project-serum/anchor --branch v0.18.0 anchor-cli --locked
    echo "Anchor CLI installed successfully."
else
    echo "Anchor CLI is already installed."
fi


echo "All dependencies installed successfully."
