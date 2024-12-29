FROM debian
USER root

RUN mkdir /home/rust && \
    apt-get update && \
    apt-get install -y gcc  \
    git \
    curl && \
    rm -rf /var/lib/apt/lists && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    cargo install -y rustlings

WORKDIR /home/rust

