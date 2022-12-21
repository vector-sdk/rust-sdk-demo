# Rust SDK Demo application

This repository contains simple example applications for using the [Rust SDK](https://github.com/vector-sdk/rust-sdk) with [Keystone](https://keystone-enclave.org/).

# Build and Install

Clone the SDK into the root directory and build the workspace:

      git clone https://github.com/vector-sdk/rust-sdk
      cargo build --release

Copy both the applications, as well as Keystone
[Eyrie Runtime](https://github.com/keystone-enclave/keystone-runtime) into the
overlay directory of Keystone image build (represented by $KEYSTONE_BUILD_DIR)

      cp ./target/riscv64gc-unknown-none-elf/release/rust-eapp  \
         $KEYSTONE_BUILD_DIR/overlay/root/
      cp ./target/riscv64gc-unknown-linux-gnu/release/rust-happ \
         $KEYSTONE_BUILD_DIR/overlay/root/

Build Keystone image:

      cd $KEYSTONE_BUILD_DIR
      make image

Start the image in qemu:

      ./scripts/run-qemu.sh

Load the Keystone driver and run the program:

      insmod keystone-driver.ko
      ./rust-happ ./rust-eapp ./eyrie-rt
