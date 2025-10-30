#!/bin/bash
# Setup development environment för teamet (i internet-zon)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=========================================="
echo "Setting up Development Environment"
echo "=========================================="

# Create team-scripts directory
mkdir -p "$SCRIPT_DIR/team-scripts"

# Create Python virtual environment
if command -v python3 &> /dev/null; then
    echo "Creating Python virtual environment..."
    python3 -m venv "$SCRIPT_DIR/python-env"
    
    echo "Activating virtual environment..."
    source "$SCRIPT_DIR/python-env/bin/activate" || source "$SCRIPT_DIR/python-env/Scripts/activate"
    
    echo "Installing common dependencies..."
    pip install --upgrade pip
    pip install ansible pyvmomi pyyaml requests
    
    echo "✅ Python environment ready"
    echo ""
    echo "To activate: source python-env/bin/activate"
else
    echo "⚠️  Python3 not found, skipping Python setup"
fi

# Create example script
cat > "$SCRIPT_DIR/team-scripts/example.py" << 'EXAMPLE_SCRIPT'
#!/usr/bin/env python3
"""Example script för teamet"""

import sys
import json

def main():
    print("Hello from team script!")
    if len(sys.argv) > 1:
        print(f"Arguments: {sys.argv[1:]}")
    
    # Teamet kan här använda alla dependencies de behöver
    # t.ex.:
    # import requests
    # import yaml
    # etc.

if __name__ == '__main__':
    main()
EXAMPLE_SCRIPT

chmod +x "$SCRIPT_DIR/team-scripts/example.py"

# Create PowerShell example
cat > "$SCRIPT_DIR/team-scripts/Example.ps1" << 'EXAMPLE_PS'
# Example PowerShell script för teamet

param(
    [string]$InputFile,
    [string]$OutputFile
)

Write-Host "Hello from PowerShell script!"

# Teamet kan här använda alla modules de behöver
# t.ex.:
# Import-Module ImportExcel
# etc.

if ($InputFile) {
    Write-Host "Input: $InputFile"
}

if ($OutputFile) {
    Write-Host "Output: $OutputFile"
}
EXAMPLE_PS

echo ""
echo "=========================================="
echo "✅ Development environment ready!"
echo "=========================================="
echo ""
echo "Team scripts directory: $SCRIPT_DIR/team-scripts"
echo "Python venv: $SCRIPT_DIR/python-env"
echo ""
echo "Teamet kan nu:"
echo "1. Utveckla scripts i team-scripts/"
echo "2. Använda pip install / Install-Module som vanligt"
echo "3. Du paketerar med package-for-airgap.sh"
echo "4. Deploy i airgapped miljö med executor"

