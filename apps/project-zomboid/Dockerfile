ARG VERSION
FROM docker.io
FROM docker.io/steamcmd/steamcmd:$VERSION

ENV JAVA_HOME=/opt/java/openjdk
# pz expects the 64-bit jre to be in /bin/jre64
COPY --from=amd64/eclipse-temurin:22-jre-alpine $JAVA_HOME /bin/jre64
ENV PATH="${JAVA_HOME}/bin:${PATH}"

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
    USE_STEAM=true \
    TIMEOUT=60

USER root

COPY ./update_zomboid.txt /app/update_zomboid.txt

#hadolint ignore=DL3018,DL3013
RUN \
    apk add --no-cache \
        bash \
        ca-certificates \
    && mkdir $ZOMBOID_PATH \
    && chmod -R 755 $ZOMBOID_PATH

COPY ./entrypoint.sh /bin/entrypoint.sh
RUN chmod +x /bin/entrypoint.sh

VOLUME [ $ZOMBOID_PATH ]
ENTRYPOINT ["/bin/sh"]
