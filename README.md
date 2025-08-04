# Disclaimer
This code was fully AI written, I have only tested it for my use case.
Just wanted a flycut replica on linux, and Claude code did the rest, use it on your own responsibility!

# Clippoo

A Flycut-style clipboard manager for Linux (Wayland + GNOME), providing persistent clipboard history with a fast, distraction-free UI.

## Features

- 🔄 Continuous clipboard monitoring on Wayland
- 💾 Persistent SQLite-based clipboard history (survives reboots)
- 🎨 Semi-transparent modal UI with keyboard navigation
- ⚡ Lightning-fast clipboard switching with `Meta+Shift+V`
- 📋 Automatic paste simulation after selection
- 🔢 Quick selection with number keys (1-9)
- 🛡️ Lightweight daemon with resource limits
- 🚀 100% Wayland-compatible

## Prerequisites

- Rust 1.70+ and Cargo
- GTK4 development libraries
- wl-clipboard tools
- ydotool or wtype (for auto-paste)
- systemd (for service management)

### Install Dependencies

#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    libgtk-4-dev \
    libsqlite3-dev \
    wl-clipboard \
    pkg-config
    
# For auto-paste (choose one):
sudo apt install ydotool ydotoold  # Recommended
# OR
sudo apt install wtype

# Optional: For better terminal detection on X11
sudo apt install xdotool
```

#### Fedora:
```bash
sudo dnf install -y \
    gtk4-devel \
    sqlite-devel \
    wl-clipboard \
    gcc \
    pkg-config
    
# For auto-paste:
sudo dnf install ydotool

# Optional: For better terminal detection on X11
sudo dnf install xdotool
```

#### Arch Linux:
```bash
sudo pacman -S \
    gtk4 \
    sqlite \
    wl-clipboard \
    base-devel \
    pkg-config
    
# For auto-paste:
yay -S ydotool-bin
# OR
sudo pacman -S wtype

# Optional: For better terminal detection on X11
sudo pacman -S xdotool
```

## Building and Installation

### Quick Install

1. Clone the repository:
```bash
git clone https://github.com/islam-taha/clippoo.git
cd clippoo
```

2. Install Clippoo (builds, installs binaries, sets up systemd service, and configures keyboard shortcuts):
```bash
make install
```

That's it! Clippoo is now installed and running.

### Manual Build Options

```bash
make build      # Build the project
make rebuild    # Clean and rebuild
make test       # Run tests
make help       # Show all available commands
```

### Setting up ydotool for Auto-paste

For auto-paste functionality to work, you need to set up ydotool:

1. **Install ydotool:**
```bash
# Ubuntu/Debian
sudo apt install ydotool

# Fedora
sudo dnf install ydotool

# Arch
yay -S ydotool-bin
```

2. **Set up ydotool daemon:**

First, add your user to the input group:
```bash
sudo usermod -aG input $USER
```

Then create the ydotool service:
```bash
# Create systemd service for ydotool
sudo tee /etc/systemd/system/ydotoold.service > /dev/null << 'EOF'
[Unit]
Description=ydotool daemon
After=default.target

[Service]
ExecStart=/usr/bin/ydotoold
Restart=always
RestartSec=2
Environment=PATH=/usr/bin:/usr/local/bin:/bin

[Install]
WantedBy=default.target
EOF

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable ydotoold
sudo systemctl start ydotoold
```

3. **Logout and login again** for the group changes to take effect.

4. **Verify ydotool is working:**
```bash
# Test ydotool
ydotool type "test"
```

### Alternative: Using wtype (Wayland only)

If you prefer wtype over ydotool:
```bash
# Ubuntu/Debian
sudo apt install wtype

# No additional setup required
```

## Usage

1. **Copy text normally** with `Ctrl+C`
2. **Open clipboard history** with `Meta+Shift+V` (Windows/Super key + Shift + V)
3. **Navigate** with:
   - ↑/↓ arrow keys
   - Number keys 1-9 for quick selection
4. **Select an entry** with Enter
5. **Cancel** with Escape

The selected entry will be automatically pasted into the active application.

## Uninstalling

To completely remove Clippoo:
```bash
make uninstall
```

This will:
- Stop and disable the daemon service
- Remove all installed binaries
- Remove keyboard shortcuts
- Clean up systemd configuration

## Testing

Run the test suite:
```bash
make test
```

Or run specific tests:
```bash
# Test database module
cargo test -p clippoo-daemon database_test

# Test daemon initialization
cargo test -p clippoo-daemon daemon_test
```

## Project Structure

```
clippoo/
├── daemon/              # Background clipboard monitoring service
│   ├── src/
│   │   ├── main.rs     # Daemon entry point
│   │   └── clipboard_watcher.rs
│   └── Cargo.toml
├── ui/                  # GTK4 modal interface
│   ├── src/
│   │   ├── main.rs     # UI entry point
│   │   ├── popup.rs    # Modal window implementation
│   │   └── style.css   # UI styling
│   └── Cargo.toml
├── src/
│   └── database/       # Shared SQLite database module
├── systemd/            # Service configuration
├── scripts/            # Helper scripts
├── tests/              # Integration tests
└── docs/               # Documentation
```

## Troubleshooting

### Daemon not starting
```bash
# Check service status
systemctl --user status clippoo-daemon

# View logs
journalctl --user -u clippoo-daemon -f
```

### UI not appearing
- Ensure the keyboard shortcut is properly configured
- Check if the UI binary is executable: `chmod +x ~/.local/bin/clippoo-ui*`
- Run manually to test: `~/.local/bin/clippoo-ui.sh`

### Auto-paste not working

1. **Check if ydotool is installed and running:**
```bash
# Check if ydotoold is running
systemctl status ydotoold

# Check if you're in the input group
groups | grep input
```

2. **If ydotool isn't working:**
   - Make sure you followed the ydotool setup instructions above
   - Ensure you logged out and back in after adding yourself to the input group
   - Try running `ydotool type test` to verify it's working

3. **Alternative: Use wtype**
   - Install wtype: `sudo apt install wtype`
   - Clippoo will automatically fall back to wtype if ydotool fails

Note: Clippoo uses Ctrl+Shift+V for paste, which works in most applications including terminals

### Database location
The clipboard history is stored at: `~/.local/share/clippoo/clipboard.db`

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
