#!/bin/bash
# Clippoo UI launcher script

# Set display environment if not set
if [ -z "$DISPLAY" ] && [ -z "$WAYLAND_DISPLAY" ]; then
    export WAYLAND_DISPLAY=wayland-0
fi

# Check if clippoo-ui is already running
if pgrep -x "clippoo-ui" > /dev/null; then
    # UI is already running, don't launch another instance
    exit 0
fi

# Launch the UI
exec ~/.local/bin/clippoo-ui "$@"