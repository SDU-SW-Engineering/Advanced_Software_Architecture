#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        ./latexUtils/compileLatex-UNIX.sh
elif [[ "$OSTYPE" == "darwin"* ]]; then
        ./latexUtils/compileLatex-UNIX.sh
elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        ./latexUtils/compileLatex-windows.bat
else
        echo "Unknown operating system"
        exit 1
fi
