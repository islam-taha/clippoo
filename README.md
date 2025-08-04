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
sudo apt install ydotool  # Recommended
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

## Building

1. Clone the repository:
```bash
git clone https://github.com/islam-taha/clippoo.git
cd clippoo
```

2. Build the project:
```bash
cargo build --release
```

3. Install binaries:
```bash
mkdir -p ~/.local/bin
cp target/release/clippoo-daemon ~/.local/bin/
cp target/release/clippoo-ui ~/.local/bin/
cp scripts/clippoo-ui.sh ~/.local/bin/
```

## Installation

### 1. Set up the systemd service:
```bash
mkdir -p ~/.config/systemd/user/
cp systemd/clippoo-daemon.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now clippoo-daemon
```

### 2. Configure keyboard shortcut (GNOME):

#### Option A: Using GNOME Settings GUI
1. Open Settings → Keyboard → View and Customize Shortcuts
2. Click "Custom Shortcuts" → "+"
3. Set:
   - Name: `Clippoo Clipboard Manager`
   - Command: `~/.local/bin/clippoo-ui.sh`
   - Shortcut: `Meta+Shift+V` (Windows/Super key + Shift + V)

#### Option B: Using command line
```bash
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings \
    "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']"

gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    name 'Clippoo Clipboard Manager'

gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    command '/home/$USER/.local/bin/clippoo-ui.sh'

gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    binding '<Super><Shift>v'
```

### 3. For ydotool auto-paste setup:
If using ydotool for auto-paste, you need to set it up:
```bash
# Start ydotool daemon
sudo systemctl enable --now ydotool

# Add your user to the input group (logout/login required)
sudo usermod -aG input $USER
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

## Testing

Run the test suite:
```bash
# Test database module
cargo test -p clippoo-daemon database_test

# Test daemon initialization
cargo test -p clippoo-daemon daemon_test

# Run all tests
cargo test --workspace
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
- Install ydotool or wtype
- For ydotool: ensure you're in the `input` group and ydotoold is running
- For wtype: should work out of the box on Wayland
- Note: Clippoo uses Ctrl+Shift+V for paste, which works in most applications including terminals

### Database location
The clipboard history is stored at: `~/.local/share/clippoo/clipboard.db`

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
