#!/bin/bash

# Ensure the target directory exists
mkdir -p ~/.local/share/CreateDesktopFile

# Copy the release build to ~/.local/share/bin
cp ./target/release/CreateDesktopFile ~/.local/share/CreateDesktopFile/

# Make the binary executable
chmod +x ~/.local/share/CreateDesktopFile/CreateDesktopFile

# Add the alias to .bashrc if it doesn't exist already
if ! grep -q "alias CreateDesktopFile=" ~/.bashrc; then
    echo 'alias CreateDesktopFile="~/.local/share/CreateDesktopFile/CreateDesktopFile"' >> ~/.bashrc
fi

echo "Installation complete!"
echo "Please run 'source ~/.bashrc' or restart your terminal to use the 'create-desktop-file' command"