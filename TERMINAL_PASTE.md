# Terminal Paste Support

When using Clippoo with terminal applications, please note:

## Pasting in Terminals

Most terminal emulators use **Ctrl+Shift+V** for pasting (not Ctrl+V), including:
- GNOME Terminal
- Konsole
- Tilix
- Alacritty
- Kitty
- Most other Linux terminals

## How Clippoo Works with Terminals

1. Press **Ctrl+Shift+V** to open Clippoo's clipboard manager
2. Select your desired clipboard entry
3. The content is copied to your clipboard
4. Clippoo will attempt to auto-paste using ydotool or wtype
5. If auto-paste fails in terminal, manually paste with **Ctrl+Shift+V**

## Auto-paste in Terminals

Auto-paste may not work in all terminals due to security restrictions. If auto-paste fails:
- The content is still in your clipboard
- Simply press **Ctrl+Shift+V** again to paste manually

## Installing Auto-paste Tools

For better auto-paste support, install one of these tools:

```bash
# For ydotool (works on both Wayland and X11)
sudo apt install ydotool
sudo systemctl enable ydotool
sudo systemctl start ydotool

# For wtype (Wayland only)
sudo apt install wtype
```

Note: Some distributions may require additional setup for ydotool to work properly.