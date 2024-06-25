#!/bin/sh

# Ensures we can run both steamcmd installation AND our entrypoint
# whilst retaining signal handling

steamcmd +force_install_dir /app +login anonymous +app_update 380870 +quit
exec "$@"
