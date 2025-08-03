# Auto-Paste Setup for Clippoo

Clippoo can automatically paste the selected clipboard entry after you choose it. This feature requires a tool to simulate keyboard input on Wayland.

## Recommended: Install wtype

`wtype` is the simplest and most reliable option for Wayland:

```bash
sudo apt install wtype
```

That's it! Auto-paste should now work.

## Alternative: Using ydotool

If you prefer ydotool, you need to run it in daemon mode. Since your version doesn't include ydotoold, you can:

1. Run ydotool in daemon mode manually:
```bash
sudo ydotool daemon &
```

2. Or create a systemd service for it:
```bash
# Create a user service file
cat > ~/.config/systemd/user/ydotool-daemon.service << EOF
[Unit]
Description=ydotool daemon
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/bin/ydotool daemon
Restart=on-failure

[Install]
WantedBy=default.target
EOF

# Enable and start the service
systemctl --user daemon-reload
systemctl --user enable ydotool-daemon
systemctl --user start ydotool-daemon
```

Note: ydotool may require root permissions or adding your user to the `input` group:
```bash
sudo usermod -aG input $USER
# Log out and back in for changes to take effect
```

## Manual Paste Mode

If auto-paste doesn't work or you prefer manual control:

1. Press `Ctrl+Shift+V` to open Clippoo
2. Select an entry (arrows/Enter or number keys 1-9)
3. The entry is copied to clipboard
4. Press `Ctrl+V` to paste where you want

This manual mode is actually more reliable and gives you better control over where to paste!