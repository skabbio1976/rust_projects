#!/bin/bash
# Package scripts-executor och teamets scripts för airgapped deployment

set -e

PACKAGE_DIR="scripts-airgap-package"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=========================================="
echo "Packaging Scripts for Airgap Deployment"
echo "=========================================="

# Clean up old package
if [ -d "$PACKAGE_DIR" ]; then
    rm -rf "$PACKAGE_DIR"
fi

mkdir -p "$PACKAGE_DIR"

# Build executor binary (Rust eller Go)
echo "Building executor binary..."
cd "$SCRIPT_DIR"

# Try Rust first (preferred)
if command -v cargo &> /dev/null && [ -f "Cargo.toml" ]; then
    echo "Building Rust executor..."
    RUSTFLAGS='-C link-arg=-static' cargo build --release --target x86_64-unknown-linux-musl 2>/dev/null || \
    cargo build --release
    cp target/release/executor "$PACKAGE_DIR/executor" 2>/dev/null || \
    cp target/x86_64-unknown-linux-musl/release/executor "$PACKAGE_DIR/executor" 2>/dev/null || \
    echo "⚠️  Rust build failed, trying Go..."
fi

# Fallback to Go if Rust not available or failed
if [ ! -f "$PACKAGE_DIR/executor" ] && command -v go &> /dev/null; then
    echo "Building Go executor..."
    CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o "$PACKAGE_DIR/executor" executor.go
    echo "✅ Executor binary built (Go)"
elif [ ! -f "$PACKAGE_DIR/executor" ]; then
    echo "⚠️  Neither Rust nor Go found, skipping executor build"
    echo "⚠️  You'll need to build executor manually"
fi

if [ -f "$PACKAGE_DIR/executor" ]; then
    chmod +x "$PACKAGE_DIR/executor"
    echo "✅ Executor binary ready"
fi

# Copy team scripts
echo "Copying team scripts..."
if [ -d "team-scripts" ]; then
    cp -r team-scripts "$PACKAGE_DIR/"
    echo "✅ Team scripts copied"
else
    mkdir -p "$PACKAGE_DIR/team-scripts"
    echo "⚠️  No team-scripts directory found, created empty one"
fi

# Package Python dependencies if venv exists
if [ -d "python-env" ]; then
    echo "Packaging Python environment..."
    tar -czf "$PACKAGE_DIR/python-env.tar.gz" python-env/
    echo "✅ Python environment packaged"
fi

# Copy config
if [ -f "config.json" ]; then
    cp config.json "$PACKAGE_DIR/"
    echo "✅ Config copied"
else
    echo "⚠️  No config.json found"
fi

# Create installation script
cat > "$PACKAGE_DIR/install.sh" << 'INSTALL_SCRIPT'
#!/bin/bash
# Install script for airgapped environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing scripts-executor..."

# Extract Python environment if exists
if [ -f "$SCRIPT_DIR/python-env.tar.gz" ]; then
    echo "Extracting Python environment..."
    tar -xzf "$SCRIPT_DIR/python-env.tar.gz" -C "$SCRIPT_DIR"
fi

# Make executor executable
chmod +x "$SCRIPT_DIR/executor"

echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  $SCRIPT_DIR/executor config.json <script_name> [args...]"
INSTALL_SCRIPT

chmod +x "$PACKAGE_DIR/install.sh"

# Create tarball
echo ""
echo "Creating tarball..."
tar -czf scripts-airgap-package.tar.gz "$PACKAGE_DIR"

echo ""
echo "=========================================="
echo "✅ Package created: scripts-airgap-package.tar.gz"
echo "=========================================="

