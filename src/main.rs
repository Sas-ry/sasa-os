#![no_std]
#![no_main]
#![feature(naked_functions)]

mod common_func;
mod types;
use core::arch::asm;
use core::arch::global_asm;
use common_func::printf_test;
use types::*;

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, {stack_top}\n",
            "j kernel_main\n",
            stack_top = sym __stack_top,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    unsafe {
        //common_func::memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as size_t - &__bss as *const u8 as size_t);
        printf_test("\n\nHello %s\n%d\n", &[&"World!", &"11"]);
    }
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
