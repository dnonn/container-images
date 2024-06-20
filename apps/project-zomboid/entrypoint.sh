#!/usr/bin/env bash

set +x -o pipefail

update() {
  echo "[INFO] Updating server"
  echo "We here: `dirname $0`"

  steamcmd +runscript "/app/update_zomboid.txt"

  echo "[INFO] Server updated"
}

start() {
  echo "[INFO] Starting server"
  
  if [[ -z "$BIND_IP" ]] || [[ "$BIND_IP" == "0.0.0.0" ]]; then
    BIND_IP=($(hostname -i))
    BIND_IP=${BIND_IP[0]}
  else
    BIND_IP="$BIND_IP"
  fi

  if [[ -z "$USE_STEAM" ]] || [[ "$USE_STEAM" == "true" ]]; then
    USE_STEAM=""
  else
    USE_STEAM="-nosteam"
  fi

  timeout "$TIMEOUT" /opt/pzserver/start-server.sh \
    -cachedir=$ZOMBOID_PATH \
    -Duser.home=$ZOMBOID_PATH \
    -adminpassword $ADMIN_PASSWORD \
    -ip $IP \
    -port $PORT \
    -servername $SERVER_NAME \
    -Xmx${MAX_RAM} \
    -steamvac $STEAM_VAC \
    $USE_STEAM &

  wait $!

  echo "[INFO] Server stopped"
}

update
start
