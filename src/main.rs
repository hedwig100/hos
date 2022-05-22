#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr;

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
    let mut handle_buffer = uefi::HandleBuffer {
        buffer_size: 1024,
        buffer: &mut [uefi::Handle(ptr::null_mut() as *mut c_void); 1024],
    };
    let _ = bs.get_handle(&mut handle_buffer, &uefi::LOADED_IMAGE_PROTOCOL_GUID);
    let loaded_image = bs.open_loadedimage(handle_buffer.buffer[0]).unwrap();

    let mut handle_buffer2 = uefi::HandleBuffer {
        buffer_size: 1024,
        buffer: &mut [uefi::Handle(ptr::null_mut() as *mut c_void); 1024],
    };
    let _ = bs.get_handle(&mut handle_buffer2, &uefi::SIMPLE_FILE_SYSTEM_PROTOCOL_GUID);
    let mut sfsp = bs
        .open_sfsp(handle_buffer2.buffer[0], loaded_image.device_handle)
        .unwrap();
    stdout.print("successful opening a simple file system protocol.");

    if sfsp.revision != 0x00010000 {
        stdout.print("III");
    }
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
