FROM debian
USER root

RUN mkdir /home/Rust && \
    apt-get update && \
    apt-get install -y gcc  \
    git \
    curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    cargo install rustlings && \
    rm -rf /var/lib/apt/lists 

WORKDIR /home/Rust

