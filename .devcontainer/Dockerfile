FROM rust:1.56-alpine3.14

# This Dockerfile adds a non-root user with sudo access. Update the “remoteUser” property in
# devcontainer.json to use it. More info: https://aka.ms/vscode-remote/containers/non-root-user.
ARG INSTALL_ZSH="true"
ARG USERNAME=vscode
# ARG USER_UID=1000
# ARG USER_GID=$USER_UID

RUN apk update && \
    apk add --no-cache build-base git curl

RUN \
    addgroup \
    -g 1000 \
    -S "${USERNAME}" \
    && adduser \
    -s /bin/sh \
    -u 1000 \
    -G "${USERNAME}" \
    -h "/home/${USERNAME}" \
    -D "${USERNAME}" \
    && su "${USERNAME}"