ARG VERSION
FROM docker.io/steamcmd/steamcmd:$VERSION

ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETOS

LABEL dev.image.target_platform=$TARGETPLATFORM
LABEL dev.image.target_architecture=$TARGETARCH
LABEL dev.image.target_os=$TARGETOS
LABEL org.opencontainers.image.source="https://github.com/steamcmd/docker"

ENV \
    ADMIN_PASSWORD=changeme \
    IP=0.0.0.0 \
    PORT=16261 \
    SERVER_NAME=changeme \
    MAX_RAM=4096m \
    STEAM_VAC=true \
    USE_STEAM=true \
    TIMEOUT=60 \
    TZ="Etc/UTC"

WORKDIR /app
COPY ./update_zomboid.txt ./update_zomboid.txt
COPY ./entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

VOLUME ["/data"]
ENTRYPOINT ["./entrypoint.sh"]
