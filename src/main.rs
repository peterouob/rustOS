#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{println};
use bootloader::{BootInfo,entry_point};
use x86_64::structures::idt::ExceptionVector::Page;
use blog_os::memory::BootInfoFrameAllocator;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use x86_64::{structures::paging::Page,VirtAddr};
    println!("Hello World{}", "!");

    blog_os::init();
    let phy_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phy_memory_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_mapping_example(page,&mut mapper,&mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {page_ptr.offset(400).write(0x_f021_f077_f065_f04e)} //0x_f021_f077_f065_f04e 白色背景上的string New

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