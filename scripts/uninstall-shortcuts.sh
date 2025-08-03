#!/usr/bin/env bash
# Clippoo shortcut uninstall script

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Removing Clippoo keyboard shortcuts...${NC}"

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
        echo -e "${GREEN}Removing GNOME shortcut...${NC}"
        
        # Get current custom keybindings
        CURRENT=$(gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings 2>/dev/null || echo "@as []")
        
        # Check if clippoo shortcut exists
        if [[ $CURRENT == *"clippoo"* ]]; then
            # Remove clippoo from the list
            NEW=$(echo "$CURRENT" | sed "s|, '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/'||g")
            NEW=$(echo "$NEW" | sed "s|'/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/', ||g")
            NEW=$(echo "$NEW" | sed "s|'/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/'||g")
            
            # Handle empty list
            if [ "$NEW" = "[]" ]; then
                NEW="@as []"
            fi
            
            gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "$NEW" 2>/dev/null || true
            
            # Reset the shortcut properties (optional, but clean)
            SCHEMA="org.gnome.settings-daemon.plugins.media-keys.custom-keybinding"
            PATH="/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/"
            
            gsettings reset-recursively ${SCHEMA}:${PATH} 2>/dev/null || true
            
            echo -e "${GREEN}✓ GNOME shortcut removed${NC}"
        else
            echo -e "${YELLOW}No Clippoo shortcut found in GNOME${NC}"
        fi
        ;;
        
    kde)
        echo -e "${GREEN}Removing KDE shortcut...${NC}"
        
        # Remove desktop file
        DESKTOP_FILE="$HOME/.local/share/applications/clippoo-shortcut.desktop"
        if [ -f "$DESKTOP_FILE" ]; then
            rm -f "$DESKTOP_FILE"
            echo -e "${GREEN}✓ Removed desktop file${NC}"
        fi
        
        # Remove from kglobalshortcutsrc
        kwriteconfig5 --file kglobalshortcutsrc \
                      --group "clippoo-shortcut.desktop" \
                      --key "_launch" \
                      --delete 2>/dev/null || true
        
        # Reload shortcuts
        qdbus org.kde.kglobalaccel /kglobalaccel org.kde.KGlobalAccel.setInactive 2>/dev/null || true
        qdbus org.kde.kglobalaccel /kglobalaccel org.kde.KGlobalAccel.setActive 2>/dev/null || true
        
        echo -e "${GREEN}✓ KDE shortcut removed${NC}"
        ;;
        
    sway)
        echo -e "${GREEN}Removing Sway shortcut...${NC}"
        
        CONFIG_FILE="$HOME/.config/sway/config"
        if [ -f "$CONFIG_FILE" ]; then
            # Remove clippoo lines
            sed -i '/# Clippoo clipboard manager/d' "$CONFIG_FILE" 2>/dev/null || true
            sed -i '/bindsym.*clippoo-ui\.sh/d' "$CONFIG_FILE" 2>/dev/null || true
            
            # Reload Sway config
            swaymsg reload 2>/dev/null || true
            
            echo -e "${GREEN}✓ Sway shortcut removed${NC}"
        else
            echo -e "${YELLOW}Sway config file not found${NC}"
        fi
        ;;
        
    hyprland)
        echo -e "${GREEN}Removing Hyprland shortcut...${NC}"
        
        CONFIG_FILE="$HOME/.config/hypr/hyprland.conf"
        if [ -f "$CONFIG_FILE" ]; then
            # Remove clippoo lines
            sed -i '/# Clippoo clipboard manager/d' "$CONFIG_FILE" 2>/dev/null || true
            sed -i '/bind.*clippoo-ui\.sh/d' "$CONFIG_FILE" 2>/dev/null || true
            
            echo -e "${GREEN}✓ Hyprland shortcut removed${NC}"
            echo -e "${YELLOW}Reload your Hyprland config for changes to take effect${NC}"
        else
            echo -e "${YELLOW}Hyprland config file not found${NC}"
        fi
        ;;
        
    *)
        echo -e "${YELLOW}Unknown desktop environment: $XDG_CURRENT_DESKTOP${NC}"
        echo -e "${YELLOW}Please remove the keyboard shortcut manually${NC}"
        ;;
esac

echo -e "${GREEN}Shortcut removal complete!${NC}"