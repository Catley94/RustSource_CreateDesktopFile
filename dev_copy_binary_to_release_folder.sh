#!/bin/bash

PROGRAM_NAME="create-desktop-file"

if ! command -v zip >/dev/null 2>&1; then
    echo "Error: zip command not found. Please install zip first"
    exit 1
fi

echo "Removing ${PROGRAM_NAME}.zip"
rm ./${PROGRAM_NAME}.zip

echo "Building release version..."
cargo build --release || {
    echo "Build failed!"
    exit 1
}

echo "Making release folder within project"
mkdir -p ./${PROGRAM_NAME}

echo "copying (release)create-desktop-file to ./${PROGRAM_NAME}"
cp ./target/release/create-desktop-file ./${PROGRAM_NAME}
echo "copying release_install.sh to ./${PROGRAM_NAME}"
cp release_install.sh ./${PROGRAM_NAME}
echo "copying release_uninstall.sh to ./${PROGRAM_NAME}"
cp release_uninstall.sh ./${PROGRAM_NAME}

echo "Creating zip archive..."
zip -r "${PROGRAM_NAME}.zip" "./${PROGRAM_NAME}"

rm -rf ./${PROGRAM_NAME}


echo "Done!"