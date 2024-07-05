#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> !{
    should_failed();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_failed() {
    serial_print!("should_fail ...");
    assert_eq!(0,1);
}

#[panic_handler]
fn panic(_info : &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// cargo test --test should_panic
