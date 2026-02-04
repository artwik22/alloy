#!/bin/bash
# =============================================================================
# Alloy Screensaver Idle Controller
# =============================================================================
# Uses swayidle.
# Wrapped in a loop to ensure it restarts if it crashes or exits.

SCREENSAVER_QML="$HOME/.config/alloy/spark/Screensaver.qml"
PIPE="/tmp/quickshell_command"
LOG="/tmp/alloy-screensaver.log"

if [ ! -f "$SCREENSAVER_QML" ]; then
    echo "Error: Screensaver not found at $SCREENSAVER_QML"
    exit 1
fi

echo "Starting Alloy Idle Monitor (swayidle)..."
echo "Timeout: 30s"
echo "Logs: $LOG"

# Commands with error suppression (|| true) and logging
# We redirect quickshell output to log so it doesn't clutter the terminal
CMD_START="echo hideSidebar > \"$PIPE\"; quickshell -p \"$SCREENSAVER_QML\" >> \"$LOG\" 2>&1"
CMD_RESUME="echo showSidebar > \"$PIPE\"; pkill -f Screensaver.qml || true"

# Infinite loop to keep it alive
while true; do
    swayidle -w \
        timeout 30 "$CMD_START" \
        resume "$CMD_RESUME" \
        before-sleep "$CMD_START"
    
    echo "Swayidle exited. Restarting in 2s..."
    sleep 2
done
