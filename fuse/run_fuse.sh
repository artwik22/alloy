#!/bin/bash
# Uruchom fuse jako użytkownik artwik z dostępem do wyświetlacza
export DISPLAY=:0
if [ "$EUID" -eq 0 ]; then
    echo "Uruchamianie jako użytkownik artwik..."
    sudo -u artwik env DISPLAY=:0 /usr/local/bin/fuse "$@"
else
    /usr/local/bin/fuse "$@"
fi
