# Raspberry Pi Motion Wakeup

A native app written using [Rust](https://www.rust-lang.org/) that turns a [Raspberry Pi's](https://www.raspberrypi.org/) display on/off using data from GPIO pin.  This app enables a [PIR motion sensor](http://www.image.micros.com.pl/_dane_techniczne_auto/cz%20am312.pdf) connected to the Pi's [GPIO pins](https://www.raspberrypi.org/documentation/hardware/raspberrypi/gpio/README.md) to turn its display on/off based on movement.  The display is powered off only when there has been a minimum duration of no movement detected. This app can be auto-launched along with [Tapslist.io](https://taplist.io/help/raspberry-pi-setup) to create an auto-dimming beer tap display.

## Getting Started

### Prerequisites

You must have Rust installed and cross-compilation configured for a Raspberry Pi.

1. Download and install [Rust](https://www.rust-lang.org/tools/install) using these [instructions](https://docs.npmjs.com/getting-started/installing-node).
2. Setup cross-compilation for Raspberry Pi as needed.
    * Instructions for MacOS & Win 10 can be [found here](https://dev.to/h_ajsf/cross-compiling-rust-for-raspberry-pi-4iai).
    * Instructions for Linux can be [found here](https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658)

### Compilation for Raspberry Pi

1. Clone this repository using the [instructions from GitHub](https://help.github.com/articles/cloning-a-repository/)
    `https://github.com/ZGrauer/Raspberry-Pi-Motion-Wakeup.git`
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

### Optional Display Off Timeout

Launching the app with args `-t <seconds>` or `--timeout <seconds>` configures the seconds with no motion detected before the display is powered off.  These do not need to be included when launching the app.  If not included, then the default of 30 seconds is used.

```shell
rpi-motion-wakeup -t 60
```


## Deployment

Add the `--release` flag to the to build command to compile for production. This optimizes artifacts for production. See `./target/armv7-unknown-linux-gnueabihf/` for the compiled executable.

```shell
cargo build --release
```

Then copy the executable to your Rapsberry Pi and run.  Example:

```shell
$ ./rpi-motion-wakeup
PIR motion sensor on BCM GPIO pin 8
Press CTRL + C to exit
2020-05-15T18:34:37.851751162+00:00    |   Watching for motion...
2020-05-15T18:34:55.392830162+00:00    |   Motion detected!
2020-05-15T18:34:55.392927548+00:00    |   Powering display on
display_power=1
2020-05-15T18:35:04.774342873+00:00    |   No Motion
2020-05-15T18:35:35.364244063+00:00    |   Powering display off
display_power=0
```

## Disable the Screensaver

This app doesn't deactivate the default screensaver on the Pi. For this app to work properly you must disable the screensaver.  There are a [couple ways to do this](https://www.raspberrypi.org/documentation/configuration/screensaver.md) but the easiest method is this:

1. Install `xscreensaver`

```shell
sudo apt install xscreensaver
```

2. Find the screensaver application under the `Preferences` option on the main desktop menu, then disable it.

## Authors

[@ZGrauer](https://github.com/ZGrauer).

## Contribute

Feel free to dive in! [Open an issue](https://github.com/ZGrauer/Raspberry-Pi-Motion-Wakeup/issues/new/choose) or submit PRs.

Raspberry Pi Motion Wakeup follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.

## License

[GNU](LICENSE) © Zachary Grauerholz
