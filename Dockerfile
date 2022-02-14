ARG DEBIAN_RELEASE=bullseye

FROM bitnami/minideb:${DEBIAN_RELEASE} AS builder

WORKDIR /usr/src
COPY mediarepo-api ./mediarepo-api
COPY mediarepo-daemon ./mediarepo-daemon
COPY mediarepo-ui ./mediarepo-ui
COPY build.py .

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
    libavutil-dev \
    libavformat-dev \
    libavcodec-dev \
    libavfilter-dev \
    libavdevice-dev \
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

RUN python3 build.py build
