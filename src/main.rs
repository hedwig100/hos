#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;

mod uefi;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: uefi::Handle, st: uefi::SystemTable) -> ! {
    let stdout = st.stdout();
    stdout.reset(false);

    let string = "Hello UEFI".as_bytes();
    let mut buf = [0u16; 50];
    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }
    stdout.output_string(buf.as_ptr());

    loop {}
}
