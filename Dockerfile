#
# Dockerfile for VECTOR Rust SDK demonstrator
#
# Setup build environment and git clone VECTOR Rust SDK demonstrator
# inside the docker image path /opt/src/rust-sdk-demo
#
# Using Ubuntu 24.04 and nightly-01-25 Rust toolchain
#
# Build Docker image:
#   docker build --tag rust-sdk-demo:1.0 .
# Run Docker image:
#   docker run --rm -it rust-sdk-demo:1.0
#

# Using Ubuntu 24.04 image
FROM ubuntu:24.04

# Update and install build-essential packages
RUN apt-get update && \
    apt-get install -y build-essential && \
    apt-get install -y crossbuild-essential-riscv64

# Update toolchaoin files
RUN apt-get install -y gcc-riscv64-linux-gnu g++-riscv64-linux-gnu libc6-dev-riscv64-cross
RUN apt-get install -y gcc-riscv64-unknown-elf

# Install necessary tools
RUN apt-get install -y curl emacs git make

# Install Rust nightly-2025-01-25 (1.86.0)
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install nightly-2025-01-25
RUN rustup default nightly-2025-01-25
RUN rustup target add riscv64gc-unknown-none-elf
RUN rustup target add riscv64gc-unknown-linux-gnu

# Get rust-sdk-demo source code
RUN mkdir -p /opt/src && \
    (cd /opt/src ; git clone https://github.com/vector-sdk/rust-sdk-demo.git) && \
    (cd /opt/src/rust-sdk-demo ; git clone https://github.com/vector-sdk/rust-sdk.git)
