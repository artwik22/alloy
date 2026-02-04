#!/bin/bash

# Konfiguracja rozmiaru okna pływającego
# Konfiguracja rozmiaru okna pływającego
# Pobieranie z argumentów, domyślnie 1000x700
TARGET_WIDTH=${1:-1000}
TARGET_HEIGHT=${2:-700}

handle_refresh() {
    if ! command -v jq &> /dev/null; then
        return
    fi

    # Pobierz stan okien raz
    JSON=$(hyprctl -j clients)

    # 1. Znajdź okna, które powinny być pływające (są jedyne na workspace i jeszcze nie są floating)
    # Filtrujemy workspace > 0 (pomijamy scratchpady itp)
    TO_FLOAT=$(echo "$JSON" | jq -r '
        map(select(.workspace.id > 0)) 
        | group_by(.workspace.id) 
        | .[] 
        | select(length == 1) 
        | .[] 
        | select(.floating == false) 
        | .address')

    # 2. Znajdź okna, które powinny być kafelkowe (jest ich więcej niż 1 na workspace i są floating)
    TO_TILE=$(echo "$JSON" | jq -r '
        map(select(.workspace.id > 0)) 
        | group_by(.workspace.id) 
        | .[] 
        | select(length > 1) 
        | .[] 
        | select(.floating == true) 
        | .address')

    # Aplikowanie zmian
    for addr in $TO_FLOAT; do
        if [ ! -z "$addr" ]; then
            hyprctl dispatch setfloating address:$addr
            hyprctl dispatch resizewindowpixel exact $TARGET_WIDTH $TARGET_HEIGHT,address:$addr
            hyprctl dispatch centerwindow address:$addr
        fi
    done

    for addr in $TO_TILE; do
        if [ ! -z "$addr" ]; then
            hyprctl dispatch settiled address:$addr
        fi
    done
}

# Sprawdzenie środowiska
if [ -z "$HYPRLAND_INSTANCE_SIGNATURE" ]; then
    exit 1
fi

SOCKET="/tmp/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock"
if [ -n "$XDG_RUNTIME_DIR" ]; then
    SOCKET="$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock"
fi

if [ ! -S "$SOCKET" ]; then
    exit 1
fi

# Inicjalizacja
handle_refresh

# Pętla nasłuchująca
socat -U - UNIX-CONNECT:"$SOCKET" | while read -r line; do
    case "$line" in
        openwindow*|closewindow*|movewindow*|changefloatingmode*|activewindow*)
            handle_refresh
            ;;
    esac
done
