use chrono::Local;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use std::time::Duration;

fn now() -> String {
    Local::now()
        .to_string()
        .split_once('.')
        .unwrap()
        .0
        .to_string()
}

#[cfg(windows)]
fn to_hour(h: u64) -> Duration {
    Duration::from_secs(h * 60 * 60)
}

#[cfg(windows)]
fn prevent() {
    use std::thread;
    use winapi::um::{
        winbase::SetThreadExecutionState,
        winnt::{ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED},
    };

    println!("{}: Start Prevent!!!", now());
    unsafe {
        let result = SetThreadExecutionState(
            ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED | ES_AWAYMODE_REQUIRED | ES_CONTINUOUS,
        );

        println!("{}: Call SetThreadExecutionState", now());
        println!("{}: Result -> {}", now(), result);
    }

    loop {
        thread::sleep(to_hour(1));
    }
}

#[cfg(not(windows))]
fn prevent() {
    println!("{}: No Prevent...", now());
}

fn main() {
    prevent();
}
