# This image is for build purposes only, it will be used to build the parachain binary

# The command with docker buildx to create the image must be something like this with the args
# docker buildx build --build-arg USER_ID=$(id -u) --build-arg GROUP_ID=$(id -g) -t g6_parachain_builder:1.0 .
# And the execution command is like this
# docker run -it --name g6_parachain_builder --hostname g6 --rm -v $(pwd):/home/parachain/workspace -v $HOME/runner/cargo_cache/:/home/parachain/.cargo g6_parachain_builder:1.0

FROM debian:bookworm

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

# Create a user and group to run the application as a non-root user for security reasons 
# The id of the user and group is the same as the one on the host machine to avoid permission issues this should be taken from args
#ARG USER_ID
#ARG GROUP_ID
#
## Usar la instrucción RUN para crear el usuario y grupo en un solo comando, asegurando que el directorio home se crea correctamente.
#RUN groupadd -g $GROUP_ID parachain && \
#    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID parachain

USER parachain:parachain

# Set the working directory
WORKDIR /home/solo/workspace
RUN echo "alias ll='ls -la'" >> /home/parachain/.bashrc
RUN echo "alias ..='cd ..'" >> /home/parachain/.bashrc
RUN echo "alias ...='cd ../..'" >> /home/parachain/.bashrc

# Set the environment
#ENV USER=parachain
#ENV GROUP=parachain
#ENV HOME=/home/parachain
#ENV PATH=$HOME/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Update rust toolchains
RUN rustup default stable
RUN rustup update

# Install the wasm target
RUN rustup update nightly
# RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rust-src
RUN rustup component add clippy

RUN cargo install cargo-audit
