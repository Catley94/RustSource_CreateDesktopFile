#!/bin/bash

echo "Making dir ~/.local/share/CreateDesktopFile"
# Ensure the target directory exists
mkdir -p ~/.local/share/CreateDesktopFile

echo "Copying CreateDesktopFile to ~/.local/share/CreateDesktopFile/"
# Copy the release build to ~/.local/share/bin
cp ./target/release/CreateDesktopFile ~/.local/share/CreateDesktopFile/

echo "Making CreateDesktopFile executable with chmod"
# Make the binary executable
chmod +x ~/.local/share/CreateDesktopFile/CreateDesktopFile

echo "Running CreateDesktopFile to create a .desktop file for CreateDesktopFile to use the GUI, stored in ~/.local/share/applications"
~/.local/share/CreateDesktopFile/CreateDesktopFile \
--local \
--name CreateDesktopFile \
--exec-path ~/.local/share/CreateDesktopFile/CreateDesktopFile \
--terminal-app true \
--app-type Application \
--categories Development;

echo "Adding CreateDesktopFile alias to ~/.bashrc"

# Add the alias to .bashrc if it doesn't exist already
if ! grep -q "alias CreateDesktopFile=" ~/.bashrc; then
    echo 'alias CreateDesktopFile="~/.local/share/CreateDesktopFile/CreateDesktopFile"' >> ~/.bashrc
fi

echo "Installation complete!"
echo "Please run 'source ~/.bashrc' or restart your terminal to use the 'CreateDesktopFile' command"