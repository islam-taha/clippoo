#!/usr/bin/env bash
# Clippoo shortcut setup script

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up Clippoo keyboard shortcuts...${NC}"

# Detect desktop environment
detect_desktop() {
    # Check for GNOME (including Ubuntu variants)
    if [[ "$XDG_CURRENT_DESKTOP" == *"GNOME"* ]] || [ "$DESKTOP_SESSION" = "gnome" ] || [ "$DESKTOP_SESSION" = "ubuntu" ]; then
        echo "gnome"
    elif [[ "$XDG_CURRENT_DESKTOP" == *"KDE"* ]] || [ "$DESKTOP_SESSION" = "plasma" ]; then
        echo "kde"
    elif [ -n "$SWAYSOCK" ]; then
        echo "sway"
    elif [ "$XDG_CURRENT_DESKTOP" = "Hyprland" ]; then
        echo "hyprland"
    else
        echo "unknown"
    fi
}

DESKTOP=$(detect_desktop)
echo "Detected desktop environment: $DESKTOP"

case $DESKTOP in
    gnome)
        echo -e "${GREEN}Setting up GNOME shortcut...${NC}"

        # Get current custom keybindings
        CURRENT=$(/usr/bin/gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings)

        # Check if clippoo shortcut already exists
        if [[ $CURRENT == *"clippoo"* ]]; then
            echo -e "${YELLOW}Clippoo shortcut already exists. Updating...${NC}"
        else
            # Add clippoo to the list
            if [ "$CURRENT" = "@as []" ]; then
                # No custom shortcuts yet
                NEW="['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']"
            else
                # Append to existing shortcuts
                NEW="${CURRENT%]}, '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']"
            fi
            /usr/bin/gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "$NEW"
        fi

        # Set the shortcut properties
        SCHEMA="org.gnome.settings-daemon.plugins.media-keys.custom-keybinding"
        PATH="/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/"

        /usr/bin/gsettings set ${SCHEMA}:${PATH} name 'Clippoo Clipboard Manager'
        /usr/bin/gsettings set ${SCHEMA}:${PATH} command "$HOME/.local/bin/clippoo-ui.sh"
        /usr/bin/gsettings set ${SCHEMA}:${PATH} binding '<Super><Shift>v'

        echo -e "${GREEN}✓ GNOME shortcut set: Meta+Shift+V${NC}"
        ;;

    kde)
        echo -e "${GREEN}Setting up KDE shortcut...${NC}"

        # Create a .desktop file for KDE
        DESKTOP_FILE="$HOME/.local/share/applications/clippoo-shortcut.desktop"
        cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Type=Application
Name=Clippoo Clipboard Manager
Exec=$HOME/.local/bin/clippoo-ui.sh
Icon=edit-paste
Categories=Utility;
NoDisplay=true
EOF

        # Use kwriteconfig5 to set the shortcut
        kwriteconfig5 --file kglobalshortcutsrc \
                      --group "clippoo-shortcut.desktop" \
                      --key "_launch" "Meta+Shift+V,none,Clippoo Clipboard Manager"

        # Reload shortcuts
        qdbus org.kde.kglobalaccel /kglobalaccel org.kde.KGlobalAccel.setInactive || true
        qdbus org.kde.kglobalaccel /kglobalaccel org.kde.KGlobalAccel.setActive || true

        echo -e "${GREEN}✓ KDE shortcut set: Meta+Shift+V${NC}"
        echo -e "${YELLOW}Note: You may need to log out and back in for the shortcut to take effect${NC}"
        ;;

    sway)
        echo -e "${GREEN}Setting up Sway shortcut...${NC}"

        CONFIG_FILE="$HOME/.config/sway/config"
        if [ -f "$CONFIG_FILE" ]; then
            # Check if shortcut already exists
            if grep -q "bindsym.*clippoo-ui.sh" "$CONFIG_FILE"; then
                echo -e "${YELLOW}Clippoo shortcut already exists in Sway config${NC}"
            else
                # Add shortcut to config
                echo "" >> "$CONFIG_FILE"
                echo "# Clippoo clipboard manager" >> "$CONFIG_FILE"
                echo "bindsym Mod4+Shift+v exec $HOME/.local/bin/clippoo-ui.sh" >> "$CONFIG_FILE"

                # Reload Sway config
                swaymsg reload

                echo -e "${GREEN}✓ Sway shortcut added: Meta+Shift+V${NC}"
            fi
        else
            echo -e "${RED}Sway config file not found at $CONFIG_FILE${NC}"
            echo -e "${YELLOW}Add this line to your Sway config:${NC}"
            echo "bindsym Mod4+Shift+v exec $HOME/.local/bin/clippoo-ui.sh"
        fi
        ;;

    hyprland)
        echo -e "${GREEN}Setting up Hyprland shortcut...${NC}"

        CONFIG_FILE="$HOME/.config/hypr/hyprland.conf"
        if [ -f "$CONFIG_FILE" ]; then
            # Check if shortcut already exists
            if grep -q "clippoo-ui.sh" "$CONFIG_FILE"; then
                echo -e "${YELLOW}Clippoo shortcut already exists in Hyprland config${NC}"
            else
                # Add shortcut to config
                echo "" >> "$CONFIG_FILE"
                echo "# Clippoo clipboard manager" >> "$CONFIG_FILE"
                echo "bind = SUPER SHIFT, V, exec, $HOME/.local/bin/clippoo-ui.sh" >> "$CONFIG_FILE"

                echo -e "${GREEN}✓ Hyprland shortcut added: Meta+Shift+V${NC}"
                echo -e "${YELLOW}Reload your Hyprland config for changes to take effect${NC}"
            fi
        else
            echo -e "${RED}Hyprland config file not found at $CONFIG_FILE${NC}"
            echo -e "${YELLOW}Add this line to your Hyprland config:${NC}"
            echo "bind = SUPER SHIFT, V, exec, $HOME/.local/bin/clippoo-ui.sh"
        fi
        ;;

    *)
        echo -e "${RED}Unknown desktop environment: $XDG_CURRENT_DESKTOP${NC}"
        echo -e "${YELLOW}Please set up the keyboard shortcut manually:${NC}"
        echo "  Shortcut: Meta+Shift+V"
        echo "  Command: $HOME/.local/bin/clippoo-ui.sh"
        exit 1
        ;;
esac

echo -e "${GREEN}Shortcut setup complete!${NC}"
echo -e "${YELLOW}Test it by pressing Meta+Shift+V${NC}"
