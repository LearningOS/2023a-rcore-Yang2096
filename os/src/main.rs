#![feature(panic_info_message)]
#![no_main]
#![no_std]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
    }
    unsafe {
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    }

    

    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
    }
    unsafe {
        (sbss as usize ..ebss as usize).for_each(|a| (a as *mut u8).write_volatile(0));
        info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    }
}
