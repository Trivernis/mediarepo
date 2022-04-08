ARG DEBIAN_RELEASE=bullseye
ARG BASE_IMAGE=docker.io/bitnami/minideb:${DEBIAN_RELEASE}

FROM ${BASE_IMAGE} AS base
RUN apt-get update
RUN apt-get install -y \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libappindicator3-0.1-cil-dev \
    patchelf \
    librsvg2-dev \
    curl \
    wget \
    pkg-config \
    clang \
    nodejs \
    npm \
    libsoup2.4-dev \
    libwebkit2gtk-4.0-dev \
    file \
    python
RUN apt remove cmdtest -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rm -rf /var/lib/{apt,dpkg,cache,log}/ /var/cache

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

FROM sources AS build_ui
WORKDIR /usr/src
RUN python3 scripts/build.py ui --verbose