#!/usr/bin/env bash

set +x -o pipefail

steamcmd +runscript "/app/update_zomboid.txt"

if [ -z "$IP" ] || [ "$IP" == "0.0.0.0" ]; then
  IP=($(hostname -i))
  IP=${IP[0]}
else
  IP=$IP
fi

if [ -z "$USE_STEAM" ] || [ "$USE_STEAM" == "true" ]; then
  USE_STEAM=""
else
  USE_STEAM="-nosteam"
fi

exec /app/start-server.sh \
  -cachedir=/data \
  -ip $IP \
  -port $PORT \
  -adminpassword $ADMIN_PASSWORD \
  -servername $SERVER_NAME \
  -Xmx${MAX_RAM} \
  -steamvac $STEAM_VAC \
  $USE_STEAM \
  "$@"
