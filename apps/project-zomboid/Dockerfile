ARG VERSION

FROM docker.io/rust:alpine as builder
WORKDIR /app
COPY ./zomboid-manager .
RUN apk add alpine-sdk
RUN cargo build --release

FROM docker.io/steamcmd/steamcmd:ubuntu-24

ENV INSTALL_PATH=/app

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
ENV HOME /home/$USER

RUN groupadd -g $PGID $GROUP \
    && useradd -m -d $HOME -u $PUID -g $PGID $USER

WORKDIR /app
COPY --from=builder --chown=$USER:$GROUP /app/target/release/server-manager .
COPY --chown=$USER:$GROUP ./entrypoint.sh /entrypoint.sh

RUN mkdir /data \
    && chown -R $USER:$GROUP /data \
    && chown -R $USER:$GROUP /app

USER $USER:$GROUP

ENV INSTALL_PATH /app
ENV SERVER_ARGUMENTS -adminpassword,CHANGEME,-cachedir,/data
ENV EXIT_TIMEOUT 900
ENV RUST_LOG "info"

VOLUME ["/data"]
ENTRYPOINT ["/bin/sh", "/entrypoint.sh", "/app/server-manager"]
