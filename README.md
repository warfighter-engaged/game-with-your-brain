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
rustup target add armv7-unknown-linux-gnueabihf
sudo apt-get install gcc-multilib-arm-linux-gnueabihf
```

Once these are set, you can run `cargo build` and build a target for the Raspberry Pi!

### Raspberry Pi Setup

The inputs uses serial/UART, I2C, and SPI communications, which have to be enabled in the Pi's settings before they can be used.

#### Serial

By default, the Pi's primary UART (serial0) is assigned to the Linux console. We need it to be available for the EEG headset, so we need to disable the serial console.

This can be done with the `raspi-config` utility:

```sh
sudo raspi-config
```

Select "Interfacing Options", then "Serial", then select "No" to disable the serial console.

You can also make these changes manually by editing the kernel command line:

```sh
sudo nano /boot/cmdline.txt
```

Find the console entry that refers to the serial0 device and remove it, including the baud rate setting. It should look something like `console=serial0,115200`. Make sure the rest of the line remains the same.

Remove any lines containing `enable_uart=0` or `enable_uart=1` from `/boot/config.txt`.

On Raspberry Pi models with a Bluetooth module, an extra step is required to either disable Bluetooth or move it to `/dev/ttyS0`, so `/dev/ttyAMA0` becomes available for serial communication. To disable Bluetooth, add `dtoverlay=pi3-disable-bt` to `/boot/config.txt`. You'll also need to disable the service that initializes Bluetooth with `sudo systemctl disable hciuart`.

To move the Bluetooth module to `/dev/ttyS0`, instead of disabling it with the above-mentioned steps, add `dtoverlay=pi3-miniuart-bt` and `core_freq=250` to `/boot/config.txt`.

Remember to reboot the Raspberry Pi after making any changes.

#### I2C

The Raspberry Pi's BCM283x SoC supports three hardware I2C buses, however only the I2C bus on physical pins 3 and 5 should be used to communicate with slave devices. The other two buses are used internally as an HDMI interface, and for HAT identification.

The I2C bus connected to physical pins 3 (SDA) and 5 (SCL) is disabled by default. You can enable it through sudo raspi-config, or by manually adding dtparam=i2c_arm=on to /boot/config.txt. Remember to reboot the Raspberry Pi afterwards.

#### SPI

SPI is simpler; all we need to do is to enable the SPI functionality on the Pi.

```sh
sudo raspi-config
```

Select "Interfacing Options", then "SPI", then select "Yes."

Alternately, the settings can be edited manually:

```sh
sudo nano /boot/config.txt
```

Add the following line at the bottom:

```text
dtparam=spi=on
```

Save the file and exit. Reboot the Pi using:

```sh
sudo reboot
```

To make sure the SPI module is loaded by the system, run the following command:

```sh
lsmod | grep spi_
```

You should see `spi_bcm2708` or `spi_bcm2835`.

## Plans

Current functionality is built for a Raspberry Pi model 2/3/4. Raspberry Pi models 3 and 4 use ARMv8 processors, but ARMv8 is compatible with ARMv7, so there's not much difference. However, we can also target Raspberry Pi models 3 and 4 by specifying the AARCH64 architecture (this is what LLVM and Rust call ARMv8). If we want to target Raspberry Pi Zero, we can also target `arm-unknown-linux-gnueabihf` - however, as we're using Raspberry Pi as a stepping-stone to embedded devices, changing targets doesn't strike me as super necessary.

Once the calibration app is moved to an external device (XBox/PC) we can start making this application into a `no_std` application suitable for use on embedded hardware. For details about Rust development on embedded devices, see <https://rust-embedded.github.io/book/>.

## TODO

- [ ] Convert everything over to use `embedded-hal` - this way the conversion between microcontrollers relies solely upon switching the driver. <https://github.com/rust-embedded/linux-embedded-hal>
- [ ] Include an ADC driver to retrieve signals from the MYO sensors <https://github.com/pcein/adc-mcp3008> & <http://pramode.in/2018/02/24/an-introduction-to-writing-embedded-hal-based-drivers-in-rust/>
