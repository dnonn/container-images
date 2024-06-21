#!/usr/bin/env bash

set +x -o pipefail

update() {
  echo "[INFO] Updating server"

  steamcmd +runscript "/app/update_zomboid.txt"

  echo "[INFO] Server updated"
}

start() {
  echo "[INFO] Starting server"
  
  if [[ -z "$IP" ]] || [[ "$IP" == "0.0.0.0" ]]; then
    IP=($(hostname -i))
    IP=${IP[0]}
  else
    IP="$IP"
  fi

  if [[ -z "$USE_STEAM" ]] || [[ "$USE_STEAM" == "true" ]]; then
    USE_STEAM=""
  else
    USE_STEAM="-nosteam"
  fi

  timeout $TIMEOUT /app/start-server.sh \
    -cachedir=/data \
    -ip $IP \
    -port $PORT \
    -adminpassword $ADMIN_PASSWORD \
    -servername $SERVER_NAME \
    -Xmx${MAX_RAM} \
    -steamvac $STEAM_VAC \
    $USE_STEAM &

  server_pid=$!
  wait $server_pid

  echo "[INFO] Server stopped"
}

update
start
