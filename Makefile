# List of subdirectories. Not used for build, but cleanup!
SUBDIRS := 	rust-eapp     \
		rust-eapp/src \
		rust-happ     \
		rust-happ/src \

# Target architecture for the host application libraries
TARGET_ARCH := riscv64gc-unknown-linux-gnu
x86_64: TARGET_ARCH := x86_64-unknown-linux-gnu

# Keystone build directory, e.g., <path>/build-generic64 for qemu builds
KEYSTONE_BUILD_DIR ?= $(error Set KEYSTONE_BUILD_DIR enviromnment)

dir2tgt = $(patsubst %, ./%/$(strip $(2)),$(strip $(1)))

all:
	cargo build --target $(TARGET_ARCH) --release

# This target builds the host application libraries for x86_64 architecture.
# Enclave application libraries will still be built for the RISC-V target.
x86_64:
	cargo build -v --target $(TARGET_ARCH) --release

# Build with SDK's internal enclave memory debugging support
debug:
	cargo build -v --features debug_memory --target $(TARGET_ARCH) --release

# Install built applications to Keystone overlay
install:
	cp ./target/riscv64gc-unknown-none-elf/release/rust-eapp $(KEYSTONE_BUILD_DIR)/overlay/root
	cp ./target/riscv64gc-unknown-linux-gnu/release/rust-happ $(KEYSTONE_BUILD_DIR)/overlay/root

# Clean build and temporary files:
clean:
	cargo clean
	rm -f *~ $(call dir2tgt, $(strip $(SUBDIRS)), *~)
