#![no_std]//禁用标准库
#![no_main]//不使用预定义的入口
#![allow(dead_code, unused_imports,non_snake_case,unused_variables,non_upper_case_globals)]
use core::panic::PanicInfo;
mod common;
use crate::common::port::Port::InputOutput;
use common::{Port};
#[panic_handler]
fn panic(_info:&PanicInfo)->!{
   loop{} 
}
/*整个生命周期有效*/
static mut vga_x :i8=0;
static mut vga_y :i8 =0; 
/*打印字符*/
pub fn printf(_str:&[u8]){
    let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in _str.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((vga_y*80+vga_x) as isize * 2) = byte;
            *vga_buffer.offset((vga_y*80+vga_x)  as isize * 2 + 1) = 0xb;
            vga_x+=1;
            if(vga_x>=80 &&vga_y<25){
                vga_y+=1;
                vga_x=0;
            }
        }
    }
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let Port8bit=Port::Bit8{port_number:10};
    // Port8bit.write(b'a');
    // Port8bit.read();
    loop {}
}
