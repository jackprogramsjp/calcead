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

# Uninstall
rm -rf /usr/local/bin/calcead

echo Successfully uninstalled.