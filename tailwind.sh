#!/bin/bash

case "$1" in
    install*)  
        if [ -f tailwindcss ]; then
            rm twcss
        fi
        curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
        chmod +x tailwindcss-linux-x64
        mv tailwindcss-linux-x64 twcss
    ;;
    init*)  ./twcss init;;
    watch*) ./twcss -i input.css -o lib/static/css/styles.css --watch;;
    compile*) ./twcss -i input.css -o lib/static/css/styles.css --minify;;
    *) echo "Usage: $0 {install|init|watch|compile}"; exit 1;;
esac