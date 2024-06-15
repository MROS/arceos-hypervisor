#![no_std]
#![no_main]

#[macro_use]
extern crate libax;
use libax::thread;
use libax::time::Duration;

#[no_mangle]
fn main() {
    let mut count = 0;
    loop {
        libax::println!("count 1");
        thread::sleep(Duration::from_millis(500));
    }
}
