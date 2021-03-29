#![feature(abi_efiapi)]
#![no_std]
#![no_main]
use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::prelude::*;
extern crate rlibc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, world!").unwrap();

    loop {}
    // Status::SUCCESS
}
