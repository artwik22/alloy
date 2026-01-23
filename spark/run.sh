#!/bin/bash
# Skrypt uruchamiający quickshell z wymaganą zmienną środowiskową
export QML_XHR_ALLOW_FILE_READ=1
export QUICKSHELL_PROJECT_PATH="$(cd "$(dirname "$0")" && pwd)"

# Ustaw głośność na 35% przy starcie
pactl set-sink-volume @DEFAULT_SINK@ 35% 2>/dev/null || true

quickshell --path "$QUICKSHELL_PROJECT_PATH"

