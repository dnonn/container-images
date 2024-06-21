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
# The USER directive is docker doesn't set the HOME environment variable
# so we set it explicitly.
ENV HOME /home/$USER

RUN groupadd -g $PGID $GROUP \
    && useradd -m -d $HOME -u $PUID -g $PGID $USER

COPY --chown=$USER:$GROUP ./update_zomboid.txt /app/update_zomboid.txt
COPY --chown=$USER:$GROUP ./entrypoint.sh /entrypoint.sh

RUN mkdir /data \
    && chown -R $USER:$GROUP /data \
    && chown -R $USER:$GROUP /app

USER $USER:$GROUP

WORKDIR $HOME
VOLUME ["/data"]
ENTRYPOINT ["/entrypoint.sh"]
