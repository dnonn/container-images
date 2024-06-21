ARG VERSION

FROM docker.io/steamcmd/steamcmd:ubuntu-24

ARG TARGETPLATFORM
ARG TARGETARCH
ARG TARGETOS

LABEL dev.image.target_platform=$TARGETPLATFORM
LABEL dev.image.target_architecture=$TARGETARCH
LABEL dev.image.target_os=$TARGETOS
LABEL org.opencontainers.image.source="https://github.com/steamcmd/docker"

ARG PUID=5000
ARG PGID=5000

ENV USER steam
ENV GROUP steam

RUN groupadd -g $PGID $GROUP \
    && useradd -m -u $PUID -g $PGID $USER \
    && mkdir /data \
    && chown -R $USER:$GROUP /data

COPY --chown=$USER:$GROUP ./update_zomboid.txt /app/update_zomboid.txt
COPY --chown=$USER:$GROUP ./entrypoint.sh /app/entrypoint.sh

USER $USER:$GROUP

WORKDIR /app
VOLUME ["/data"]
ENTRYPOINT ["/app/entrypoint.sh"]
