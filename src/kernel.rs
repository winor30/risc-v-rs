#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    static __stack_top: u8;
}

fn memset(buf: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut ptr = buf;
    unsafe {
        for _ in 0..n {
            // core::ptr::write_volatile(ptr, c);
            ptr.write_volatile(c);
            ptr = ptr.add(1);
        }
    }
    buf
}

#[allow(dead_code)]
pub fn kernel_main() -> ! {
    unsafe {
        let bss_start = &mut __bss as *mut u8;
        let bss_end = &mut __bss_end as *mut u8;
        let bss_size = bss_end.offset_from(bss_start) as usize;
        memset(bss_start, 0, bss_size);
    }

    loop {}
}

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            // set __stack_top address to sp register
            // jump to kernel_main
            "mv sp, {stack_top}\n
            j {main}\n",
            stack_top = in(reg) &__stack_top,
            main = sym kernel_main,
            // The `noreturn` option indicates that this function will never return.
            options(noreturn)
        );
    }
    loop {}
}
