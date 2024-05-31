FROM espressif/idf:release-v5.1
SHELL ["/bin/bash", "-c"]

RUN apt-get update && apt-get install -y clang tmux pkg-config libudev-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN source "$HOME/.cargo/env" && \
    rustup toolchain install nightly --component rust-src && \
    cargo install ldproxy espflash

COPY dev_env.sh /dev_env.sh

RUN sh /dev_env.sh

WORKDIR /ws
