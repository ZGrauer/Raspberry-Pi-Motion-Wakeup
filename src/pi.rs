//! This module contains all the raspberry pi specific functions to 
//! watch for GPIO input and power the display on/off
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use std::{thread, time};
use std::process::Command;
use std::error::Error;
use chrono::{DateTime, Utc};

/// Monitors the GPIO input pin (`bcm_pin`) for the rising edge. 
/// This is done by setting an asynchronous interrupt trigger to execute the 
/// `handle_event(level: Level)` callback. An infinite loop is used to keep 
/// the app alive watching for input.
/// 
/// The callback function powers on/off the display depending on the pin's logic level. 
/// A `High` level powers the display on and `Low` powers off. 
/// 
/// 
/// # Arguments
/// 
/// * `bcm_pin` - BCM pin number to watch for motion
/// 
/// # Example usage
/// 
/// ```
/// pi::watch_for_motion(BCM_PIN)
/// ```
pub fn watch_for_motion(bcm_pin: u8) -> Result<(), Box<dyn Error>> {
    let now: DateTime<Utc> = Utc::now();
    let gpio = Gpio::new()?;
    let pin = gpio.get(bcm_pin)?;
    let mut input_pin = pin.into_input();
    // Clear and set async to monitor for both High & Low pin levels
    let _result = InputPin::clear_async_interrupt(&mut input_pin);
    let _result = InputPin::set_async_interrupt(&mut input_pin, Trigger::Both, handle_event)?;
    println!("{}    |   Watching for motion...", now.to_rfc3339());
    // Infinite loop to keep app alive
    loop {
        let hundred_second = time::Duration::from_millis(100);
        thread::sleep(hundred_second);
    }
}

/// Callback for changes to the GPIO pin's logic level. If the pin is `High`, power on display. 
/// Otherwise turn the display off. When the display is powered on this sleeps for 60 seconds.
/// 
/// Powering on the display is accomplished by executing command:
/// 
/// ```
/// vcgencmd display_power 1
/// ```
/// 
/// # Arguments
/// 
/// * `level` - The pin's logic level, Low or High
/// 
/// # Example usage
/// 
/// ```
/// let _result = rppal::gpio::InputPin::set_async_interrupt(&mut input_pin, Trigger::RisingEdge, handle_event)?;
/// ```
fn handle_event(level: Level) {
    let now: DateTime<Utc> = Utc::now();
    if level == Level::High {
        println!("{}    |   Motion detected, turning display on...", now.to_rfc3339());
        //Wake up. Deactivate the simple built-in X-window screensaver
        let _screensaver_status = Command::new("xset")
            .arg("s")
            .arg("reset")
            .current_dir("/usr/bin")
            .status()
            .expect("'xset s reset' command failed!");
        // Power on display
        let status = Command::new("vcgencmd")
            .arg("display_power")
            .arg("1")
            .current_dir("/usr/bin")
            .status()
            .expect("'vcgencmd display_power 1' command failed!");
        if status.success() {
            // Min 30 seconds of display time
            let thirty_seconds = time::Duration::from_secs(30);
            thread::sleep(thirty_seconds);
        } else {
            println!("{}    |   Not sleeping, display_power commmand unsucessful", now.to_rfc3339());
        }
    } else {
        // No motion, power off display
        let status = Command::new("vcgencmd")
            .arg("display_power")
            .arg("0")
            .current_dir("/usr/bin")
            .status()
            .expect("'vcgencmd display_power 0' command failed!");
        if !status.success() {
            println!("{}    |   display_power commmand unsucessful", now.to_rfc3339());
        }
        // TO turn on screensaver now...  'xset s activate'
        // Reactivate with 'xset s on s 60'
    }
}
