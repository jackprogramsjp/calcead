#!/bin/bash

abort() {
    printf "%s\n" "$@"
    exit 1
}

# Make sure BASH Is installed
if [ -z "${BASH_VERSION:-}" ]
then
    abort "Bash is required to execute this script."
fi

# Check OS
OS="$(uname)"
MACHINE="$(uname -m)"
if [[ "${OS}" != "Darwin" && "${MACHINE}" != "arm64" ]]
then
    abort "For now, Calcead is only supported for MacOS arm64 machines. You need to build from source if you are on another machine."
fi

# Download
rm -rf /usr/local/bin/calcead # Make sure calcead is removed before installing it with curl
curl -f -L https://github.com/jackprogramsjp/calcead/releases/download/v0.1.0/calcead -o /usr/local/bin/calcead && echo "Successfully installed." || echo "Failed to install with curl."
chmod +x /usr/local/bin/calcead # make executable be able to execute
