#![no_std]
#![no_main] // this attribute helps to remove main function

mod vga_buffer;

use core::panic::PanicInfo;

static PRINT_MESSAGE: &[u8] = b"WELCOME TO THE POLYGLOT PROGRAMMER OS";

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // this attribute disables the name mangling
pub extern "C" fn _start() -> ! {
    // // Firstly, we’ve cast the integer into a raw pointer(raw pointers can be immutable or mutable).
    // let buffer = 0xb8000 as *mut u8;

    // // VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen. 
    // // Secondly, we iterate over the byte string that we defined just above the _start().
    // for (num, &byte) in PRINT_MESSAGE.iter().enumerate() {
    //     // inside the loop, we use the offset method(Calculates the offset from a pointer) to write the string byte and the color byte, 
    //     // here we’ve used 0x9 for color which means light blue.

    //     // we wrapped the body of our loop in the unsafe block because we are using raw pointers and the Rust compiler can’t prove 
    //     // that the raw pointers are valid so through unsafe block we are ensuring to the compiler that all the operations are valid.
    //     unsafe {
    //         *buffer.offset(num as isize * 2) = byte;
    //         *buffer.offset(num as isize *2 + 1) = 0x9;
    //     }
    // }

    vga_buffer::print_data();

    loop{}
}