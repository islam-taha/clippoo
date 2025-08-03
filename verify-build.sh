#!/bin/bash

echo "🔍 Verifying Clippoo build..."
echo

# Check if binaries exist
echo "✓ Checking binaries:"
if [ -f "target/release/clippoo-daemon" ]; then
    echo "  ✓ clippoo-daemon built successfully"
else
    echo "  ✗ clippoo-daemon not found"
    exit 1
fi

if [ -f "target/release/clippoo-ui" ]; then
    echo "  ✓ clippoo-ui built successfully"
else
    echo "  ✗ clippoo-ui not found"
    exit 1
fi

echo
echo "✓ Checking dependencies:"

# Check for wl-clipboard
if command -v wl-paste &> /dev/null; then
    echo "  ✓ wl-clipboard installed"
else
    echo "  ⚠️  wl-clipboard not installed (required for clipboard monitoring)"
fi

# Check for ydotool or wtype
if command -v ydotool &> /dev/null; then
    echo "  ✓ ydotool installed (auto-paste will work)"
elif command -v wtype &> /dev/null; then
    echo "  ✓ wtype installed (auto-paste will work)"
else
    echo "  ⚠️  Neither ydotool nor wtype installed (auto-paste won't work)"
fi

echo
echo "✓ Build verification complete!"
echo
echo "To install, run: make install"
echo "To set up keyboard shortcut, see: docs/keyboard-shortcuts.md"