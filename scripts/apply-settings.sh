#!/bin/bash
# =============================================================================
# Alloy Script Settings Applier
# =============================================================================
# Reads colors.json and restarts background scripts (battery, screensaver)
# with the configured arguments.

CONFIG_FILE="$HOME/.config/alloy/colors.json"
SCRIPTS_DIR="$HOME/.config/alloy/scripts"
LOG="/tmp/alloy-scripts.log"

echo "Applying script settings from $CONFIG_FILE..." > "$LOG"

# 1. Read configuration (using grep/sed/python because jq might not be there, or use python for robustness)
# Python is reliable since we use it for save-colors.py
eval $(python3 -c "
import json
import sys
import os

try:
    with open('$CONFIG_FILE', 'r') as f:
        data = json.load(f)
        
    bat_auto = data.get('scriptsAutostartBattery', False)
    ss_auto = data.get('scriptsAutostartScreensaver', False)
    bat_thresh = data.get('batteryThreshold', 10)
    ss_time = data.get('screensaverTimeout', 30)
    
    print(f'BAT_AUTO={str(bat_auto).lower()}')
    print(f'SS_AUTO={str(ss_auto).lower()}')
    print(f'BAT_THRESH={bat_thresh}')
    print(f'SS_TIME={ss_time}')
except:
    print('BAT_AUTO=false')
    print('SS_AUTO=false')
    print('BAT_THRESH=10')
    print('SS_TIME=30')
")

echo "Battery: Auto=$BAT_AUTO, Threshold=$BAT_THRESH" >> "$LOG"
echo "Screensaver: Auto=$SS_AUTO, Timeout=$SS_TIME" >> "$LOG"

# 2. Manage Battery Monitor
# Kill existing
pkill -f "battery_monitor.sh" || true

if [ "$BAT_AUTO" = "true" ]; then
    echo "Starting Battery Monitor..." >> "$LOG"
    "$SCRIPTS_DIR/battery_monitor.sh" "$BAT_THRESH" >> "/tmp/alloy-battery.log" 2>&1 &
fi

# 3. Manage Screensaver
# Kill existing
pkill -f "idle-screensaver.sh" || true
# Also kill swayidle started by it to be sure
pkill -f "swayidle.*Alloy Screensaver" || true 

if [ "$SS_AUTO" = "true" ]; then
    echo "Starting Screensaver Idle Monitor..." >> "$LOG"
    "$SCRIPTS_DIR/idle-screensaver.sh" "$SS_TIME" >> "/tmp/alloy-screensaver.log" 2>&1 &
fi
