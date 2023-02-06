FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Install various dependencies.
    && apt-get -y install --no-install-recommends \
        libudev-dev \
        libcapstone4 \
        libftdi1-dev \
        libgpiod2 \
        libhidapi-hidraw0 \
        libjaylink0 \
        libjim0.79 \
        libusb-0.1-4 \
        libusb-1.0-0-dev \
        pkg-config \
    # Install multi-arch version of objdump and nm, and create symbolic links.
    && apt-get -y install --no-install-recommends binutils-multiarch \
    && ln -s /usr/bin/objdump /usr/bin/objdump-multiarch \
    && ln -s /usr/bin/nm /usr/bin/nm-multiarch \
    # Install GDB for ARM systems.
    && apt-get -y install --no-install-recommends gdb-arm-none-eabi \
    # Clean up unused packages for a smaller final image.
    && apt-get -y autoremove \
    && apt-get clean

# Update default toolchain to stable.
RUN rustup default stable \
    # Install cargo-binutils.
    && rustup component add llvm-tools-preview \
    && cargo install cargo-binutils \
    # Install cargo-flash.
    && cargo install cargo-flash \
    # Install clippy for rust-analyzer and rustfmt.
    && rustup component add clippy rustfmt \
    # Install Rust Toolchain for Cortex-M4 without hardware floating point.
    && rustup target add thumbv7em-none-eabi