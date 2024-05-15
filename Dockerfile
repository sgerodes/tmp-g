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

RUN echo "alias ll='ls -la'" >> $HOME/.bashrc
RUN echo "alias ..='cd ..'" >> $HOME/.bashrc
RUN echo "alias ...='cd ../..'" >> $HOME/.bashrc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# Source cargo environment directly in the Dockerfile
ENV PATH="/root/.cargo/bin:${PATH}"

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN cat $HOME/.bashrc
RUN echo $HOME
RUN which rustup

# Update rust toolchains
RUN rustup default stable
RUN rustup update

# Install the wasm target
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rust-src
RUN rustup component add clippy

RUN cargo install cargo-audit

COPY . .

RUN cargo build --release

# Second stage: Runtime
FROM debian:bookworm as runner

# Install runtime dependencies
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
    libssl-dev \
    protobuf-compiler \
    llvm \
    libudev-dev

# Set the working directory
WORKDIR /workspace
RUN echo "alias ll='ls -la'" >> $HOME/.bashrc
RUN echo "alias ..='cd ..'" >> $HOME/.bashrc
RUN echo "alias ...='cd ../..'" >> $HOME/.bashrc


# Copy the built binary from the builder stage
COPY --from=builder /workspace/target $HOME/workspace/
ENV NODE_EXECUTABLE="./target/release/g6-solo-node"
ENV STATE_DATA_BASE_FOLDER=./chain-data/

# Set the default command to start the node
#CMD ["/workspace/target/release/g6-solo-node", "--dev", "--base-path", "./chain-data/"]
