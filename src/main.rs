#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kubos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kubos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello my fellow {} kids!", 10 / 3);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kubos::test_panic_handler(info);
}
