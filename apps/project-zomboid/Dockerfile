ARG VERSION

FROM docker.io/steamcmd/steamcmd:$VERSION

ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETOS

LABEL dev.image.target_platform=$TARGETPLATFORM
LABEL dev.image.target_architecture=$TARGETARCH
LABEL dev.image.target_os=$TARGETOS
LABEL org.opencontainers.image.source="https://github.com/steamcmd/docker"

USER root
ENV HOME /app
WORKDIR $HOME

COPY ./update_zomboid.txt ./update_zomboid.txt
COPY ./entrypoint.sh /entrypoint.sh

RUN chmod -R 777 $HOME \
    && chmod -R 777 /tmp \
    && chmod +x /entrypoint.sh

USER nobody:nogroup 

VOLUME ["/data"]
ENTRYPOINT ["/entrypoint.sh"]
