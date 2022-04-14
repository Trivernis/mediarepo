ARG BASE_IMAGE=docker.io/alpine:latest

FROM ${BASE_IMAGE} AS base
RUN apk update
RUN apk add --no-cache \
    build-base \
    openssl3-dev \
    gtk+3.0-dev \
    libappindicator-dev \
    patchelf \
    librsvg-dev \
    curl \
    wget \
    clang \
    nodejs \
    npm \
    libsoup-dev \
    webkit2gtk-dev \
    file \
    python3 \
    bash \
    protoc
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rm -rf /var/lib/{cache,log}/ /var/cache

FROM base AS sources
WORKDIR /usr/src
COPY mediarepo-api ./mediarepo-api
COPY mediarepo-daemon ./mediarepo-daemon
COPY mediarepo-ui ./mediarepo-ui
COPY scripts ./scripts
RUN python3 scripts/clean.py
RUN python3 scripts/check.py --install

FROM sources AS build_daemon
WORKDIR /usr/src
RUN python3 scripts/build.py daemon --verbose
RUN mkdir ./test-repo
RUN ./out/mediarepo-daemon --repo ./test-repo init


FROM sources AS build_ui
WORKDIR /usr/src
RUN python3 scripts/build.py ui --verbose --bundles deb
