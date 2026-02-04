#!/bin/bash

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
             NOTIFIED_10="false"
             NOTIFIED_5="false"
             
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
            if [ "$CURRENT_PERCENT" -le 5 ]; then
                if [ "$NOTIFIED_5" != "true" ]; then
                    notify-send "Low Battery" "Battery critical ($CURRENT_PERCENT%). Please connect charger!" -i battery-caution
                    NOTIFIED_5="true"
                    NOTIFIED_10="true" # logic implies we passed 10
                fi
            elif [ "$CURRENT_PERCENT" -le 10 ]; then
                if [ "$NOTIFIED_10" != "true" ]; then
                    notify-send "Low Battery" "Battery low ($CURRENT_PERCENT%). Please connect charger." -i battery-low
                    NOTIFIED_10="true"
                fi
            fi
        fi
    fi
}

# Initialize
LAST_STATUS=""
NOTIFIED_10="false"
NOTIFIED_5="false"

# Run an initial check
check_status

# Monitor for changes efficiently
# We listen for both line_power and battery events now
upower --monitor | while read line; do
    if echo "$line" | grep -qE "line_power|battery"; then
        check_status
    fi
done
