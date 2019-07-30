# wfpi

> Brain-Body Interface On Raspberry Pi

## Installation

This project uses nightly rust - this will make it easier to eventually transition to a `no_std` environment for embedded deployment. Rust should automatically switch to the nightly channel when your CWD is inside this project folder, thanks to the `rust-toolchain` file.

### Clippy (Rust Linter)

```sh
rustup component add clippy
```

### Cross-Compilation

These installation steps require a gcc cross-compilation linker. The instructions provide details for a Linux environment (including WSL).

To cross-compile for a Raspberry Pi target, we specify the `armv7-unknown-linux-gnueabihf` target in `.cargo/config`. This also specifies a gcc cross-compilation linker to use. To install the linker:

```sh
sudo apt-get install gcc-multilib-arm-linux-gnueabihf
```

Once these are set, you can run `cargo build` and build a target for the Raspberry Pi!

## Plans

Current functionality is built for a Raspberry Pi model 2/3/4. Raspberry Pi models 3 and 4 use ARMv8 processors, but ARMv8 is compatible with ARMv7, so there's not much difference. However, we can also target Raspberry Pi models 3 and 4 by specifying the AARCH64 architecture (this is what LLVM and Rust call ARMv8). If we want to target Raspberry Pi Zero, we can also target `arm-unknown-linux-gnueabihf` - however, as we're using Raspberry Pi as a stepping-stone to embedded devices, changing targets doesn't strike me as super necessary.

Once the calibration app is moved to an external device (XBox/PC) we can start making this application into a `no_std` application suitable for use on embedded hardware. For details about Rust development on embedded devices, see <https://rust-embedded.github.io/book/>.

## TODO

- [ ] Convert everything over to use `embedded-hal` - this way the conversion between microcontrollers relies solely upon switching the driver. <https://github.com/rust-embedded/linux-embedded-hal>
- [ ] Include an ADC driver to retrieve signals from the MYO sensors <https://github.com/pcein/adc-mcp3008> & <http://pramode.in/2018/02/24/an-introduction-to-writing-embedded-hal-based-drivers-in-rust/>
