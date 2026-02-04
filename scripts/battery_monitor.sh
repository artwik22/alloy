#!/bin/bash

# Arguments:
# $1 = Critical Threshold (default 10)
CRITICAL_THRESHOLD="${1:-10}"
WARNING_THRESHOLD=$((CRITICAL_THRESHOLD + 5))

# Function to check and notify status
check_status() {
    # Get current status of AC power
    # robust methodology: check if any line_power source is online
    ONLINE=$(upower -i $(upower -e | grep line_power) | grep "online:" | awk '{print $2}')
    
    # If we can't find line_power device, try inferring from battery state
    if [ -z "$ONLINE" ]; then
         BAT_STATE=$(upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep "state:" | awk '{print $2}')
         if [ "$BAT_STATE" = "charging" ] || [ "$BAT_STATE" = "fully-charged" ]; then
             ONLINE="yes"
         else
             ONLINE="no"
         fi
    fi

    # Handle connection/disconnection notifications
    if [ "$ONLINE" != "$LAST_STATUS" ]; then
        LAST_STATUS="$ONLINE"
        
        if [ "$ONLINE" = "yes" ]; then
             # Charger connected
             # Get battery percentage
             BAT_PERCENT=$(upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep "percentage" | awk '{print $2}')
             notify-send "Power" "Charger connected ($BAT_PERCENT)" -i battery-charging
             
             # Reset low battery notification flags
             NOTIFIED_WARNING="false"
             NOTIFIED_CRITICAL="false"
             
        elif [ "$ONLINE" = "no" ]; then
             notify-send "Power" "Charger disconnected" -i battery-missing
        fi
    fi

    # Handle Low Battery notifications (only if discharging)
    if [ "$ONLINE" = "no" ]; then
        # Get raw percentage number (remove %)
        CURRENT_PERCENT=$(upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep "percentage" | awk '{print $2}' | tr -d '%')
        
        # Ensure we have a number
        if [[ "$CURRENT_PERCENT" =~ ^[0-9]+$ ]]; then
            if [ "$CURRENT_PERCENT" -le "$CRITICAL_THRESHOLD" ]; then
                if [ "$NOTIFIED_CRITICAL" != "true" ]; then
                    notify-send "Low Battery" "Battery critical ($CURRENT_PERCENT%). Please connect charger!" -i battery-caution
                    NOTIFIED_CRITICAL="true"
                    NOTIFIED_WARNING="true" # logic implies we passed warning
                fi
            elif [ "$CURRENT_PERCENT" -le "$WARNING_THRESHOLD" ]; then
                if [ "$NOTIFIED_WARNING" != "true" ]; then
                    notify-send "Low Battery" "Battery low ($CURRENT_PERCENT%). Please connect charger." -i battery-low
                    NOTIFIED_WARNING="true"
                fi
            fi
        fi
    fi
}

# Initialize
LAST_STATUS=""
NOTIFIED_WARNING="false"
NOTIFIED_CRITICAL="false"

# Run an initial check
check_status

# Monitor for changes efficiently
# We listen for both line_power and battery events now
upower --monitor | while read line; do
    if echo "$line" | grep -qE "line_power|battery"; then
        check_status
    fi
done
