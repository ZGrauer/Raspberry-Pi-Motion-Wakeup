//! This module contains all the raspberry pi specific functions to 
//! watch for GPIO input and power the display on/off
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use std::{thread, time};
use std::process::Command;
use std::error::Error;
use chrono::{DateTime, Local};

// Number of min seconds the display should be on after motion is detected
const SCREEN_TIMEOUT : i64 = 30;
// Last time display was powered on/off in seconds since 1970 
static mut LAST_ON_TIME: i64 = 0;
static mut LAST_OFF_TIME: i64 = 0;
static mut NO_MOTION_TIME: i64 = 0;

#[cfg(debug_assertions)]
macro_rules! debug {
    ($x:expr) => { dbg!($x) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($x:expr) => { std::convert::identity($x) }
}

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
    let now: DateTime<Local> = Local::now();
    let gpio = Gpio::new()?;
    let pin = gpio.get(bcm_pin)?;
    let mut input_pin = pin.into_input();
    debug!(format!("{}    |   Clear and set async interrupt to monitor for both High & Low pin levels", now.to_rfc3339()));
    let _result = InputPin::clear_async_interrupt(&mut input_pin);
    let _result = InputPin::set_async_interrupt(&mut input_pin, Trigger::Both, handle_event)?;
    println!("{}    |   Watching for motion...", now.to_rfc3339());
    debug!(format!("{}    |   No motion timeout before power off {} seconds", now.to_rfc3339(), SCREEN_TIMEOUT));
    // Infinite loop to keep app alive
    loop {
        let now: DateTime<Local> = Local::now();
        // If there has been no motion detected for at least 30 seconds
        // And the display is currently on
        unsafe {
            if (now.timestamp() - NO_MOTION_TIME) > SCREEN_TIMEOUT && LAST_ON_TIME > LAST_OFF_TIME  {
                let pin_level = input_pin.read();
                debug!(format!("{}    |   No motion in last {} seconds.  Pin Level: {:?}", now.to_rfc3339(), SCREEN_TIMEOUT, pin_level));
                // If there is still no movement
                if pin_level == Level::Low {
                    power_display(false);
                }
            }
        }
        let hundred_second = time::Duration::from_millis(500);
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
    let now: DateTime<Local> = Local::now();

    if level == Level::High {
        println!("{}    |   Motion detected!", now.to_rfc3339());
        power_display(true);
    } else {
        // Set last time no motion was detected
        unsafe {
            println!("{}    |   No Motion", now.to_rfc3339());
            NO_MOTION_TIME = now.timestamp();
            debug!(format!("{}    |   MO_MOTION_TIME: {}    LAST_OFF_TIME: {}   LAST_ON_TIME: {}", now.to_rfc3339(), NO_MOTION_TIME, LAST_OFF_TIME, LAST_ON_TIME));
        }
    }
}

/// Executes commands to power the display on or off.
/// 
/// # Arguments
/// 
/// * `on` - Determins if the display should power on or off.  True == power on
fn power_display(on: bool) {
    let now: DateTime<Local> = Local::now();
    let command_arg = if on { 
        unsafe { LAST_ON_TIME = now.timestamp(); } 
        println!("{}    |   Powering display on", now.to_rfc3339());
        "1" 
    } else { 
        unsafe { LAST_OFF_TIME = now.timestamp(); }
        println!("{}    |   Powering display off", now.to_rfc3339());
         "0" 
    };
    debug!(format!("{}    |   vcgencmd display_power {}", now.to_rfc3339(), command_arg));
    let status = Command::new("vcgencmd")
            .arg("display_power")
            .arg(command_arg)
            .current_dir("/usr/bin")
            .status()
            .expect(&format!("{}    |   'vcgencmd display_power {}' command failed!", now.to_rfc3339(), command_arg));
    if !status.success() {
        println!("{}    |   display_power commmand unsucessful", now.to_rfc3339());
    }
}