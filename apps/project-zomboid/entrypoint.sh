#!/usr/bin/env bash

set +x -o pipefail

steamcmd +runscript "/app/update_zomboid.txt"

if [ -z "$IP" ] || [ "$IP" == "0.0.0.0" ]; then
  IP=($(hostname -i))
  IP=${IP[0]}
fi

if [ -z "$USE_STEAM" ] || [ "$USE_STEAM" == "true" ]; then
  USE_STEAM=""
else
  USE_STEAM="-nosteam"
fi

exec /app/start-server.sh \
  -cachedir=/data \
  -ip $IP \
  -port ${PORT:-16261} \
  -adminpassword ${ADMIN_PASSWORD:-changeme} \
  -servername ${SERVER_NAME:-changeme} \
  -Xmx${MAX_RAM:-4096m} \
  -steamvac ${STEAM_VAC:-true} \
  "$@"
