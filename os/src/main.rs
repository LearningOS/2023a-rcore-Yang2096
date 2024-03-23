#![feature(panic_info_message)]
#![no_main]
#![no_std]
mod console;
mod lang_items;
mod sbi;

use core::arch::{asm, global_asm};
global_asm!(include_str!("entry.asm"));

extern "C" {
    // high address

    fn ebss(); // end addr of BSS segment
    fn sbss(); // start addr of BSS segment
    fn boot_stack_top();
    fn boot_stack_lower_bound();
    fn edata(); // end addr of data segment
    fn sdata(); // start addr of data segment
    fn erodata(); // end addr of Read-Only data ssegment
    fn srodata(); // start addr of Read-Only data segment
    fn etext(); // end addr of text segment
    fn stext(); // begin addr of text segment

    // low address
}

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    error!(
        ".stack [{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    function_a(0b01010);

    panic!("Shutdown machine!");
}

fn clear_bss() {
    unsafe {
        (sbss as usize..ebss as usize).for_each(|a| (a as *mut u8).write_volatile(0));
    }
}

fn function_a(n: u32) {
    if n & 1 == 0 {
        function_b(n >> 1)
    } else {
        function_a(n >> 1)
    }
}

fn function_b(n: u32) {
    if n & 1 == 1 {
        function_a(n >> 1)
    } else {
        back_trace()
    }
}

fn back_trace() {
    let mut fp: usize;
    unsafe {
        asm!("mv {}, s0", out(reg) fp);
    }
    while fp < boot_stack_top as usize {
        println!("s0 {:#x}", fp);
        println!("ra {:#x}", unsafe { *((fp - 8) as *const usize) });
        fp = unsafe { *((fp - 16) as *const usize) };
    }
}
