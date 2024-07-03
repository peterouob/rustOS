#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();
    //double fault
    // unsafe {
    //     *(0xdeadbeff as *mut u8) = 42;
    // }
    let ptr = 0x2031b2 as *mut u8;
    unsafe {
        let x= *ptr;
    }
    println!("read work");
    unsafe {
        *ptr = 42;
    }
    println!("write work");

    #[cfg(test)]
    test_main();
    println!("it doesn't crash");
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}