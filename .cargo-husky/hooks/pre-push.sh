#!/bin/sh

echo "--- Checks Xyn Code Quality ---"

cargo clippy 

status=$?

if [ $status -ne 0 ]; then
    echo "ERROR: Detected some problems on the code!"
    echo "Failed to push. Fix the code before pushing again."    
    exit 1
fi

echo "No problem detected. Continuing push.."
exit 0