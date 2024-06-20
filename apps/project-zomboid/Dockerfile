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
    ZOMBOID_PATH=/zomboid \
    ADMIN_PASSWORD=changemee \
    IP=0.0.0.0 \
    PORT=16261 \
    SERVER_NAME=changeme \
    MAX_RAM=4096m \
    STEAM_VAC=true \
    USE_STEAM=true

USER root

COPY ./update_zomboid.txt /app/update_zomboid.txt

#hadolint ignore=DL3018,DL3013
RUN \
    apk add --no-cache \
        bash \
        ca-certificates \
    && mkdir $ZOMBOID_PATH \
    && chmod -R 755 $ZOMBOID_PATH

COPY ./entrypoint.sh /entrypoint.sh

VOLUME [ $ZOMBOID_PATH ]
CMD ["/entrypoint.sh"]
