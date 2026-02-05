#!/bin/bash
# =============================================================================
# Alloy Screensaver Idle Controller
# =============================================================================
# Uses swayidle.
# Wrapped in a loop to ensure it restarts if it crashes or exits.

TIMEOUT="${1:-30}"
USE_LOCKSCREEN="${2:-false}"
SCREENSAVER_QML="$HOME/.config/alloy/spark/Screensaver.qml"
PIPE="/tmp/quickshell_command"
LOG="/tmp/alloy-screensaver.log"

if [ ! -f "$SCREENSAVER_QML" ]; then
    echo "Error: Screensaver not found at $SCREENSAVER_QML"
    exit 1
fi

echo "Starting Alloy Idle Monitor (swayidle)..."
echo "Timeout: ${TIMEOUT}s"
echo "Lock Mode: $USE_LOCKSCREEN"
echo "Logs: $LOG"

# Determine commands based on mode
if [ "$USE_LOCKSCREEN" = "true" ]; then
    # Try to find a lock command
    if command -v hyprlock &> /dev/null; then
        LOCK_CMD="hyprlock"
    elif command -v swaylock &> /dev/null; then
        LOCK_CMD="swaylock -f -c 000000"
    else
        echo "Error: No lock command found (hyprlock/swaylock)."
        LOCK_CMD="sh -c 'echo Lock command missing >> $LOG'"
    fi
    
    # Check if we should also hide sidebar when locked? 
    # Usually lockscreen covers everything so maybe not strictly needed, 
    # but good to be consistent with "idle" state.
    # However, hyprlock will just overlay.
    
    CMD_START="$LOCK_CMD >> \"$LOG\" 2>&1"
    # Resume is handled by unlocking, swayidle doesn't need explicit resume cmd for lock usually
    # unless we want to do something after unlock. 
    # But wait, swayidle 'timeout' runs a command. If that command is 'hyprlock', 
    # swayidle waits for it to finish? No, usually 'lock' is a separate event in swayidle config,
    # but here we are using 'timeout' to trigger it.
    
    # If we use 'timeout' to run hyprlock, it will run.
    # We don't usually need a resume command to 'kill' the lockscreen, the user unlocks it.
    CMD_RESUME="echo 'Resumed from lock' >> \"$LOG\""
else
    # QML Screensaver Mode
    CMD_START="echo hideSidebar > \"$PIPE\"; quickshell -p \"$SCREENSAVER_QML\" >> \"$LOG\" 2>&1"
    CMD_RESUME="echo showSidebar > \"$PIPE\"; pkill -f Screensaver.qml || true"
fi

# Infinite loop to keep it alive
while true; do
    swayidle -w \
        timeout "$TIMEOUT" "$CMD_START" \
        resume "$CMD_RESUME" \
        before-sleep "$CMD_START"
    
    echo "Swayidle exited. Restarting in 2s..."
    sleep 2
done
