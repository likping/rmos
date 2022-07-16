#![no_std]//禁用标准库
#![no_main]//不使用预定义的入口
#![allow(dead_code, unused_imports,non_snake_case,unused_variables,non_upper_case_globals)]
use core::panic::PanicInfo;
use core::arch::{asm, global_asm};
// mod common;
// use crate::common::port::Port::InputOutput;
// use common::{Port};
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
   loop{} 
}
/*整个生命周期有效*/
static mut vga_x :i8=0;
static mut vga_y :i8 =0; 
/*打印字符*/
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}