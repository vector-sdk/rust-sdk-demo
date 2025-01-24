# Rust SDK Demo application

This repository contains simple example applications for using the
[Rust SDK](https://github.com/vector-sdk/rust-sdk) with
[Keystone](https://keystone-enclave.org/).

# Build and Install

Clone the SDK as a subdirectory of the root directory:

    git clone https://github.com/vector-sdk/rust-sdk

Build the workspace:

    cargo build --release

or using make

    make

Copy both host and enclave applications into the overlay directory of
Keystone image build (represented by $KEYSTONE\_BUILD\_DIR):

    cp ./target/riscv64gc-unknown-none-elf/release/rust-eapp  \
         $KEYSTONE_BUILD_DIR/overlay/root/
    cp ./target/riscv64gc-unknown-linux-gnu/release/rust-happ \
         $KEYSTONE_BUILD_DIR/overlay/root/

There is also Makefile target for installation:

    make install

The host application also requires Keystone Eyrie Runtime binary
eyrie-rt and loader binary. These are built with Keystone examples.
Copy those files from $KEYSTONE\_BUILD\_DIR keystone-examples
subdirectory. The files should also be copied to
$KEYSTONE\_BUILD\_DIR/overlay/root/ directory.

Rebuild Keystone image:

    cd $KEYSTONE_BUILD_DIR/..
    make

Start the image in qemu:

    make run

Load the Keystone driver and run the program:

    modprobe keystone-driver
    ./rust-happ ./rust-eapp ./eyrie-rt ./loader

# Known problems

The host example `rust-happ` is using static linking. This will generate
the following linker warning:

    Using 'getaddrinfo' in statically linked applications requires at
    runtime the shared libraries from the glibc version used for linking

# Acknowledgment

This work is partly supported by the European Union’s Horizon Europe
research and innovation programme in the scope of the the
[CONFIDENTIAL6G](https://confidential6g.eu/) project under Grant
Agreement 101096435.
