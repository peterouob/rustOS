#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;

use core::panic::PanicInfo;
use blog_os::{allocator, println};
use bootloader::{BootInfo,entry_point};
use blog_os::memory::BootInfoFrameAllocator;
use blog_os::task::{Task, simple_executor::SimpleExecutor, keyboard};
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use x86_64::{VirtAddr};
    println!("Hello World{}", "!");

    blog_os::init();
    let phy_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phy_memory_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper,&mut frame_allocator).expect("heap initialization");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}",heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}",vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}",Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now ",Rc::strong_count(&cloned_reference));

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypress()));
    executor.run();

    #[cfg(test)]
    test_main();
    println!("it doesn't crash");
    blog_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task(){
    let number = async_number().await;
    println!("async number: {}",number);
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