FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    gcc \
    gcc-multilib \
    ruby \
    rustup \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

# --------------------------------

ARG user
ARG group
ARG home=/home/${user}

# https://askubuntu.com/questions/1513927/ubuntu-24-04-docker-images-now-includes-user-ubuntu-with-uid-gid-1000
RUN userdel -r ubuntu

RUN groupadd ${user} \
  && useradd ${user} -g ${group} -m

USER ${user}

RUN rustup toolchain install 1.82.0

WORKDIR ${home}/work

ENV IN_CONTAINER=1
ENV LANG=en_US.UTF-8
