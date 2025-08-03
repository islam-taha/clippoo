# Keyboard Shortcuts Setup Guide

This guide covers how to set up the `Ctrl+Shift+V` keyboard shortcut for Clippoo on various desktop environments.

## GNOME (Wayland)

### Method 1: GUI Settings
1. Open Settings → Keyboard → Keyboard Shortcuts
2. Scroll down and click "View and Customize Shortcuts"
3. Click on "Custom Shortcuts"
4. Click the "+" button to add a new shortcut
5. Fill in:
   - **Name:** `Clippoo Clipboard Manager`
   - **Command:** `~/.local/bin/clippoo-ui.sh`
   - **Shortcut:** Click "Set Shortcut" and press `Ctrl+Shift+V`

### Method 2: Command Line
```bash
# Create custom binding
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings \
    "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']"

# Set name
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    name 'Clippoo Clipboard Manager'

# Set command
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    command "$HOME/.local/bin/clippoo-ui.sh"

# Set binding
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/ \
    binding '<Control><Shift>v'
```

## KDE Plasma (Wayland)

1. Open System Settings → Shortcuts → Custom Shortcuts
2. Right-click in the list → New → Global Shortcut → Command/URL
3. Name it "Clippoo Clipboard Manager"
4. In the "Trigger" tab, click "None" and press `Ctrl+Shift+V`
5. In the "Action" tab, set command to `~/.local/bin/clippoo-ui.sh`
6. Click "Apply"

## Sway

Add to your `~/.config/sway/config`:
```
bindsym Ctrl+Shift+v exec ~/.local/bin/clippoo-ui.sh
```

Then reload Sway config:
```bash
swaymsg reload
```

## Hyprland

Add to your `~/.config/hypr/hyprland.conf`:
```
bind = CTRL SHIFT, V, exec, ~/.local/bin/clippoo-ui.sh
```

## Troubleshooting

### Shortcut not working?
1. Check if another application is using `Ctrl+Shift+V`
2. Try a different shortcut like `Super+V` or `Ctrl+Alt+V`
3. Ensure the script is executable:
   ```bash
   chmod +x ~/.local/bin/clippoo-ui.sh
   ```

### Testing the shortcut
Run this command to test if the UI launches:
```bash
~/.local/bin/clippoo-ui.sh
```

If it works manually but not with the shortcut, the issue is with the keyboard binding configuration.