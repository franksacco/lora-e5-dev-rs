# `lora-e5-dev-rs`

Ready to use template for flashing and debugging LoRa-E5-Dev Board in Rust.

The [LoRa-E5-Dev Board](https://wiki.seeedstudio.com/LoRa_E5_Dev_Board/) is a development kit that supports LoRaWAN® protocol on global frequency band and leads out full GPIOs supporting various data protocols and interfaces.
For this reason, this board turns out to be a good and inexpensive choice for fast testing and rapid prototyping of LoRa® IoT projects.

The [STM32WLE5JC](https://www.st.com/en/microcontrollers-microprocessors/stm32wle5jc.html) chip is powered by ARM® Cortex®-M4 32-bit RISC core operating at a frequency of up to 48 MHz, and Semtech SX126X LoRa® chip, supporting both LoRaWAN® and LoRa® protocol on the worldwide frequency and (G)FSK, BPSK, (G)MSK, and LoRa® modulations.
The device embed 256 KB of flash memory, 64 KB of SRAM and an extensive range of enhanced I/Os and peripherals.


## Table of Contents

* [Requirements](#requirements)
* [Available Actions](#available-actions)
    * [`cargo run`](#cargo-run)
    * [`cargo upload`](#cargo-upload)
    * [Debug with OpenOCD](#debug-with-openocd)
* [Development](#development)
    * [Developing Inside a Container](#developing-inside-a-container)
    * [Manual Setup](#manual-setup)
* [Authors](#authors)


## Requirements

* This board is a pain to get working for the first time because the flash protection bits are set.\
  Check these resources to unlock the board for programming:
    * use the [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html) as explained [here](https://wiki.seeedstudio.com/LoRa_E5_Dev_Board/#applications_1)
    * [stm32wl-unlock](https://github.com/newAM/stm32wl-unlock/)
* The OpenOCD fork by STMicroelectronics is required for debugging to work properly.\
  You can either choose to clone and compile the source code from the [official repository](https://github.com/STMicroelectronics/OpenOCD), or use the binary shipped with [STM32Cube](https://www.st.com/en/ecosystems/stm32cube.html).\
  This template assumes that STM32Cube is installed on the host system, mounts the folder containing the OpenOCD binary inside the Dev Container and adds the directory to the PATH. Edit the related lines in the [`devcontainer.json`](.devcontainer/devcontainer.json) file to match your needs or comment them out if you don't need them.


## Available Actions

### `cargo run [--release]`
This command executes `probe-run --chip STM32WLE5JCIx [--release]` that starts a [`probe-run`](https://github.com/knurling-rs/probe-run) session.\
You can set the `DEFMT_LOG` environment variable to change which logging levels are enabled.

### `cargo upload [--release]`
This command executes `cargo-flash --chip STM32WLE5JCIx [--release]` that flashes the compiled binary to the device using [`cargo-flash`]().\
You can set the `DEFMT_LOG` environment variable to change which logging levels are enabled. For example, running `DEFMT_LOG=off cargo upload --release`, you will disable all logs and produce the smallest program.

### Debug with OpenOCD
In [launch.json](.vscode/launch.json) file is defined a debug configuration called `OpenOCD` that uses the [`Cortex-Debug`](https://github.com/Marus/cortex-debug) extension, OpenOCD and `gdb` to start a debug session on Visual Studio Code.

Note that this task does not automatically build the program, so you need to run `cargo build` before debugging to use the most up-to-date binary.

## Development

To develop inside this template project, you can either choose to work in a [development container](#developing-inside-a-container) or [manually setup the enviroment](#manual-setup).

The use of a development container is strongly recommended since it allows the developers to work in the same environment and to make no additional effort to setup all the tools required during the development.

### Developing Inside a Container

Prerequisites:
  - Install [Docker](https://docs.docker.com/get-docker/), [Visual Studio Code](https://code.visualstudio.com/) and [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
  - Read the documentation at [Developing inside a Container](https://code.visualstudio.com/docs/remote/containers)

In a nutshell, you need to:
  1. Clone the repository with `git clone https://github.com/franksacco/lora-e5-dev-rs.git`.
  1. Start VS Code and run **Dev Containers: Open Folder in Container...** from the Command Palette.

Or:
  1. Start VS Code and run **Dev Containers: Clone Repository in Container Volume...** from the Command Palette.
  1. Enter `https://github.com/franksacco/lora-e5-dev-rs.git` in the input box that appears and press `Enter`.

The VS Code window will reload, clone the source code, and start building the dev container. A progress notification provides status updates.
After the build completes, VS Code will automatically connect to the container.

### Manual Setup

To develop a project based on this template, you must first make a clone of the repository:

```
git clone https://github.com/franksacco/lora-e5-dev-rs.git
```

Then, follow the instructions in [Install Rust](https://www.rust-lang.org/tools/install) to download [Rustup](https://github.com/rust-lang/rustup), the Rust toolchain installer, and setup `cargo` and `rustc`.

At this point, you need to install all the dependencies and tools required, that are specified in the [`Dockerfile`](.devcontainer/Dockerfile) and reported below.

```bash
apt-get update
# Install various dependencies.
apt-get install \
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
apt-get install binutils-multiarch
ln -s /usr/bin/objdump /usr/bin/objdump-multiarch
ln -s /usr/bin/nm /usr/bin/nm-multiarch
# Install GDB for ARM systems.
apt-get install gdb-arm-none-eabi
```

```bash
# Update default toolchain to stable.
rustup default stable
# Install cargo-binutils.
rustup component add llvm-tools-preview
cargo install cargo-binutils
# Install cargo-flash.
cargo install cargo-flash
# Install probe-run.
cargo install probe-run
# Install clippy for rust-analyzer and rustfmt.
rustup component add clippy rustfmt
# Install Rust Toolchain for Cortex-M4 without hardware floating point.
rustup target add thumbv7em-none-eabi
```


## Authors

- Francesco Saccani (francesco.saccani@unipr.it)
