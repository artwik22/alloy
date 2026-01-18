#!/usr/bin/env python3
import json
import sys
import os

# Load existing colors.json if it exists to preserve lastWallpaper
existing_data = {}
if len(sys.argv) > 6 and os.path.exists(sys.argv[6]):
    try:
        with open(sys.argv[6], 'r') as f:
            existing_data = json.load(f)
    except:
        pass

colors = {
    "background": sys.argv[1],
    "primary": sys.argv[2],
    "secondary": sys.argv[3],
    "text": sys.argv[4],
    "accent": sys.argv[5]
}

# Preserve existing values if they exist in existing data
if "lastWallpaper" in existing_data:
    colors["lastWallpaper"] = existing_data["lastWallpaper"]
if "colorPreset" in existing_data:
    colors["colorPreset"] = existing_data["colorPreset"]
if "sidebarPosition" in existing_data:
    colors["sidebarPosition"] = existing_data["sidebarPosition"]
if "sidebarVisible" in existing_data:
    colors["sidebarVisible"] = existing_data["sidebarVisible"]

# Override with provided values if they exist
# Argument 7: lastWallpaper
if len(sys.argv) > 7 and sys.argv[7]:
    colors["lastWallpaper"] = sys.argv[7]

# Argument 8: colorPreset
if len(sys.argv) > 8 and sys.argv[8]:
    colors["colorPreset"] = sys.argv[8]

# Argument 9: sidebarPosition
if len(sys.argv) > 9 and sys.argv[9]:
    colors["sidebarPosition"] = sys.argv[9]

# Preserve notification settings
if "notificationsEnabled" in existing_data:
    colors["notificationsEnabled"] = existing_data["notificationsEnabled"]
if "notificationSoundsEnabled" in existing_data:
    colors["notificationSoundsEnabled"] = existing_data["notificationSoundsEnabled"]
if "rounding" in existing_data:
    colors["rounding"] = existing_data["rounding"]
if "showHiddenFiles" in existing_data:
    colors["showHiddenFiles"] = existing_data["showHiddenFiles"]
if "presets" in existing_data:
    colors["presets"] = existing_data["presets"]

# Override with provided values if they exist
# Argument 10: notificationsEnabled
if len(sys.argv) > 10 and sys.argv[10]:
    colors["notificationsEnabled"] = sys.argv[10] == "true"

# Argument 11: notificationSoundsEnabled
if len(sys.argv) > 11 and sys.argv[11]:
    colors["notificationSoundsEnabled"] = sys.argv[11] == "true"

# Argument 12: sidebarVisible
if len(sys.argv) > 12 and sys.argv[12]:
    colors["sidebarVisible"] = sys.argv[12] == "true"

# Argument 13: rounding
if len(sys.argv) > 13 and sys.argv[13]:
    colors["rounding"] = sys.argv[13]

# Argument 14: showHiddenFiles
if len(sys.argv) > 14 and sys.argv[14]:
    colors["showHiddenFiles"] = sys.argv[14] == "true"

with open(sys.argv[6], 'w') as f:
    json.dump(colors, f, indent=2)

