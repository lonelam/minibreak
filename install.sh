#!/bin/bash

# GitHub repository
GITHUB_REPO="lonelam/minibreak"
BINARY_NAME="minibreak"
os=$(uname -s)
# Function to determine OS and set binary name
get_os_binary() {
    local binary
    case "$os" in
        Linux*) binary="$BINARY_NAME-linux";;
        Darwin*) binary="$BINARY_NAME-mac";;
        MINGW*|CYGWIN*|MSYS*) binary="$BINARY_NAME-windows.exe";;
        *) echo "Unsupported OS: $os"; exit 1;;
    esac
    echo $binary
}

# Function to download binary
download_binary() {
    local binary=$1
    local url="https://github.com/$GITHUB_REPO/releases/latest/download/$binary"

    echo "Downloading $binary..."
    curl -L -o "/usr/local/bin/$BINARY_NAME" "$url" || { echo "Failed to download binary"; exit 1; }
    chmod +x "/usr/local/bin/$BINARY_NAME"
    echo "$BINARY_NAME installed successfully."
}

# Main script execution
binary=$(get_os_binary)
download_binary $binary
