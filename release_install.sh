#!/bin/bash

PROGRAM_NAME="create-desktop-file"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (with sudo)"
    exit 1
fi

# Check if binary exists
if [ ! -f "./${PROGRAM_NAME}" ]; then
    echo "Error: ${PROGRAM_NAME} binary not found in current directory"
    exit 1
fi

# Install the program
install_program() {
    echo "Installing ${PROGRAM_NAME}..."
    
    # Create directory
    mkdir -p /usr/share/$PROGRAM_NAME
    
    # Copy binary
    cp "./${PROGRAM_NAME}" "/usr/share/${PROGRAM_NAME}/"
    chmod +x "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}"
    
    # Create symlink
    ln -sf "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}" "/usr/local/bin/${PROGRAM_NAME}"
    
    # Create desktop file
    "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}" \
        --global \
        --name "${PROGRAM_NAME}" \
        --exec-path "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}" \
        --terminal-app true \
        --app-type Application \
        --categories Development
}

# Main
main() {
    install_program
    
    echo "Installation complete!"
    echo "You can now run '${PROGRAM_NAME}' from anywhere"
}

main "$@"