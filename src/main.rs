//! Utilizes a PIR sensor via Raspberry Pi GPIO to detect movement then turn on/off the
//! display as needed. This app can be auto launched with something like
//! [Tapslist.io](https://taplist.io/help/raspberry-pi-setup) to create an auto-dimming
//! beer tap display.
//!
//! Once movement is detected the display is powered on for a minimum of one minute.
//!
//! # Setup
//!
//! The pin from the PIR sensor to Pi is set via the `BCM_PIN` constant in `main.rs`.
//! This should be the GPIO pin number on the Pi using BCM numbering.
//!
//! ```
//! // BCM GPIO pin number for the motion sensor
//! const BCM_PIN: u8 = 8;
//! ```
extern crate chrono;
extern crate rppal;

mod pi;
use std::error::Error;
use std::env;

// BCM GPIO pin number for the motion sensor
const BCM_PIN: u8 = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let timout_secs = parse_args(args);
    println!("PIR motion sensor on BCM GPIO pin {}", BCM_PIN);
    println!("Press CTRL + C to exit");
    pi::watch_for_motion(BCM_PIN, timout_secs)
}

/// Parses the args to the app and returns the timeout seconds for pi::watch_for_motion().
/// Allows users to customize the display off timeout seconds.
///
///  # Arguments
///
/// * `args` - Vector of the command line args passed into app
fn parse_args(args: Vec<String>) -> i64 {
    // Arg[0] is always the path app was started in
    let default: i64 = 30;
    // if one pair argument passed
    if args.len() == 3 && (&args[1] == "--timeout" || &args[1] == "-t") {
        let num = &args[2];
        // parse the number
        let timeout: i64 = match num.parse::<i64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: Not passed an integer arg. Using default {}", default);
                help();
                default
            }
        };

        timeout
    } else if args.len() == 1 {
        // No args passed
        default
    } else {
        eprintln!("Error: Must pass args in pairs! Using default {}", default);
        help();
        default
    }
}

/// Prints a short README on how to pass in args
fn help() {
    println!("usage:
    rpi-motion-wakeup --timeout <integer>
    rpi-motion-wakeup -t <integer>
    Set the seconds with no motion before the display is turned back off.  Default is timeout 30
    
Example:
    rpi-motion-wakeup --timeout 60
    
    ");
}