# List of subdirectories. Not used for build, but cleanup!
SUBDIRS := 	rust-eapp     \
		rust-eapp/src \
		rust-happ     \
		rust-happ/src \

# Target architecture for the host application libraries
TARGET_ARCH := riscv64gc-unknown-linux-gnu
x86_64: TARGET_ARCH := x86_64-unknown-linux-gnu

dir2tgt = $(patsubst %, ./%/$(strip $(2)),$(strip $(1)))

all:
	cargo build -v --target $(TARGET_ARCH) --release

# This target builds the host application libraries for x86_64 architecture.
# Enclave application libraries will still be built for the RISC-V target.
x86_64:
	cargo build -v --target $(TARGET_ARCH) --release

# Build with SDK's internal enclave memory debugging support
debug:
	cargo build -v --features debug_memory --target $(TARGET_ARCH) --release

# Clean build and temporary files:
clean:
	cargo clean
	rm -f *~ $(call dir2tgt, $(strip $(SUBDIRS)), *~)
