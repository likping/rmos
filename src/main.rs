#![no_std]//禁用标准库
#![no_main]//不使用预定义的入口

#![allow(dead_code, unused_imports,non_snake_case,unused_variables,non_upper_case_globals)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success=0x10,
    Failed=0x11,
}

pub fn exit_qemu(exit_code:QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port=Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

use core::panic::PanicInfo;
use core::arch::{asm, global_asm};
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion(){
    print!("trival assertion...");
    assert_eq!(1,1);
    println!("[ok]");
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}



