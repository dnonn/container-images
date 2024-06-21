ARG VERSION

FROM docker.io/steamcmd/steamcmd:$VERSION

ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETOS

LABEL dev.image.target_platform=$TARGETPLATFORM
LABEL dev.image.target_architecture=$TARGETARCH
LABEL dev.image.target_os=$TARGETOS
LABEL org.opencontainers.image.source="https://github.com/steamcmd/docker"

ENV HOME /opt/steamcmd

WORKDIR /app
COPY ./update_zomboid.txt ./update_zomboid.txt
COPY ./entrypoint.sh ./entrypoint.sh

RUN mkdir $HOME \
    && chmod -R 755 $HOME \
    && chmod -R 755 /app \
    && chmod +x ./entrypoint.sh

VOLUME ["/data"]
ENTRYPOINT ["./entrypoint.sh"]
