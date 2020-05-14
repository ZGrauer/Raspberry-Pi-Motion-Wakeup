# Raspberry Pi Motion Wakeup

A native app written using [Rust](https://www.rust-lang.org/) that turns a [Raspberry Pi's](https://www.raspberrypi.org/) display on/off using data from GPIO pin.  This app enables a [PIR motion sensor](http://www.image.micros.com.pl/_dane_techniczne_auto/cz%20am312.pdf) connected to the Pi's [GPIO pins](https://www.raspberrypi.org/documentation/hardware/raspberrypi/gpio/README.md) to turn its display on/off based on movement.  The display is powered on for a minimum duration specified in the app. This app can be auto-launched along with [Tapslist.io](https://taplist.io/help/raspberry-pi-setup) to create an auto-dimming beer tap display.

## Getting Started

### Prerequisites

You must have Rust installed and cross-compilation configured for a Raspberry Pi.

1. Download and install [Rust](https://www.rust-lang.org/tools/install) using these [instructions](https://docs.npmjs.com/getting-started/installing-node).
2. Setup cross-compilation for Raspberry Pi as needed.
    * Instructions for MacOS & Win 10 can be [found here](https://dev.to/h_ajsf/cross-compiling-rust-for-raspberry-pi-4iai).
    * Instructions for Linux can be [found here](https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658)

### Compilation for Raspberry Pi

1. Clone this repository using the [instructions from GitHub](https://help.github.com/articles/cloning-a-repository/)
2. Update the `BCM_PIN` pin constant in `main.rs` with the input pin from your motion sensor.  See below.
3. Run this command from the project directory to build for a Pi.  See `./target/armv7-unknown-linux-gnueabihf/` for the compiled executable.

```shell
cargo build
```

### Configure Input GPIO Pin

Update the below line in `main.rs` with the pin number for the PIR sensor.  This is not the power or ground pin but the pin with data to the Pi.  This uses BCM numbering for the pin, not board numbering.  Pin numbering can be found here, [https://pinout.xyz/](https://pinout.xyz/)

```rust
// BCM GPIO pin number for the motion sensor
const BCM_PIN: u8 = 8;
```

## Deployment

Add the `--release` flag to the to build command to compile for production. This optimizes artifacts for production. See `./target/armv7-unknown-linux-gnueabihf/` for the compiled executable.

```shell
cargo build --release
```

Then copy the executable to your Rapsberry Pi and run.

## Authors

[@ZGrauer](https://github.com/ZGrauer).

## Contribute

Raspberry Pi Motion Wakeup follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.

## License

[GNU](LICENSE) © Zachary Grauerholz
