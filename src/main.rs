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
extern crate rppal;
extern crate chrono;

mod pi;
use std::error::Error;

// BCM GPIO pin number for the motion sensor 
const BCM_PIN: u8 = 8;

fn main() -> Result<(), Box<dyn Error>> {
    println!("PIR motion sensor on BCM GPIO pin {}", BCM_PIN);
    println!("Press CTRL + C to exit");
    pi::watch_for_motion(BCM_PIN)
}