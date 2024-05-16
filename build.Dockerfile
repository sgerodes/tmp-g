FROM debian:bookworm as builder

# Install system dependencies
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
    build-essential \
    clang \
    curl \
    git \
    make \
    libssl-dev \
    protobuf-compiler \
    llvm \
    libudev-dev

WORKDIR /workspace
ENV PATH=$HOME/.cargo/bin:$PATH

RUN echo "alias ll='ls -la'" >> $HOME/.bashrc && \
    echo "alias ..='cd ..'" >> $HOME/.bashrc && \
    echo "alias ...='cd ../..'" >> $HOME/.bashrc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# Source cargo environment directly in the Dockerfile
ENV PATH="/root/.cargo/bin:${PATH}"
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Update rust toolchains and install targets and components
RUN rustup default stable && \
    rustup update && \
    rustup update nightly && \
    rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu && \
    rustup component add rust-src && \
    rustup component add clippy && \
    cargo install cargo-audit

COPY . .

# Use a local target directory to avoid cross-device link error
RUN cargo build --release --target-dir /workspace/target

