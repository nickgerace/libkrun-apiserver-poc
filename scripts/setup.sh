#!/usr/bin/env bash

# Ubuntu 20.10+ contains "buildah" in its sources.
# Do not modify dependencies as they are needed for each "make" target.
# Install all major dependencies: buildah libkrunfw libkrun krunvm
apt update
apt upgrade -y
apt install \
    apt-transport-https \
    bison \
    build-essential \
    buildah \
    ca-certificates \
    cargo \
    curl \
    flex \
    git \
    gnupg \
    libssl-dev \
    lsb-release \
    make \
    python3 \
    python3-pyelftools \
    -y
apt autoremove -y
git clone https://github.com/containers/libkrunfw.git
( cd libkrunfw; make; sudo make install )
git clone https://github.com/containers/libkrun.git
( cd libkrun; make; sudo make install )
git clone https://github.com/containers/krunvm.git
( cd krunvm; make; sudo make install )
