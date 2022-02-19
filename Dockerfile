ARG DEBIAN_RELEASE=bullseye

FROM bitnami/minideb:${DEBIAN_RELEASE} AS builder

WORKDIR /usr/src
COPY mediarepo-api ./mediarepo-api
COPY mediarepo-daemon ./mediarepo-daemon
COPY mediarepo-ui ./mediarepo-ui
COPY scripts ./scripts

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

RUN python3 scripts/clean.py
RUN python3 scripts/check.py --install
RUN python3 scripts/build.py all --verbose
