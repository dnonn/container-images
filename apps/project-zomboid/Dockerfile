ARG VERSION

FROM docker.io/steamcmd/steamcmd:$VERSION

ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETOS

LABEL dev.image.target_platform=$TARGETPLATFORM
LABEL dev.image.target_architecture=$TARGETARCH
LABEL dev.image.target_os=$TARGETOS
LABEL org.opencontainers.image.source="https://github.com/steamcmd/docker"

ENV HOME /opt

WORKDIR /app
COPY ./update_zomboid.txt ./update_zomboid.txt
COPY ./entrypoint.sh ./entrypoint.sh

RUN chmod -R 755 /opt \
    && chmod -R 755 /app \
    && chmod +x ./entrypoint.sh

VOLUME ["/data"]
ENTRYPOINT ["./entrypoint.sh"]
