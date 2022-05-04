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

    stdout.print("Hello UEFI");

    // get memory map
    let bs = st.get_boot_services();
    let mut memory_map = uefi::MemoryMap::<{ 4096 * 4 }> {
        buffer_size: 4096 * 4,
        buffer: &mut [0; 4096 * 4],
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0,
    };

    if bs.get_memory_map(&mut memory_map) == uefi::Status::Success {
        stdout.print("suceeded");
    }

    let _ = match memory_map.get(1) {
        Some(_) => stdout.print("successful getting a memory map"),
        None => stdout.print("failed to get a memory map"),
    };

    // open file
    let mut sfsp = bs.open_simple_file_system_protocol().unwrap();
    stdout.print("successful opening a simple file system protocol.");
    let root = sfsp.open_volume();
    let _ = match root {
        Ok(_) => stdout.print("success"),
        Err(_) => stdout.print("failed"),
    };
    // stdout.print("successful opening a root directory.");
    // let _ = root.open(
    //     "kernel.elf",
    //     uefi::OpenMode::Read,
    //     uefi::FileAttributes::Null,
    // );
    // stdout.print("successful opening a file.");

    loop {}
}
