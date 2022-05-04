use core::{cmp, ffi::c_void, ptr};

// Common UEFI Data Types
type Boolean = bool;
type Uintn = usize;
type Uint8 = u8;
type Uint16 = u16;
type Uint32 = u32;
type Uint64 = u64;
type Char16 = u16;

type Tpl = Uintn;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Handle(*mut c_void);

#[repr(C)]
pub struct Guid {
    pub data1: Uint32,
    pub data2: Uint16,
    pub data3: Uint16,
    pub data4: [Uint8; 8],
}

#[derive(PartialEq, Debug)]
#[repr(usize)]
pub enum Status {
    Success = 0,
    LoadError = 1,
    InvalidParameter = 2,
    Unsupported = 3,
    BadBufferSize = 4,
    BufferTooSmall = 5,
}

#[repr(C)]
pub struct TableHeader {
    pub signature: Uint64,
    pub revision: Uint32,
    pub header_size: Uint32,
    pub crc32: Uint32,
    _reserved: Uint32,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const Char16,
    pub firmware_revision: Uint32,
    pub console_in_handle: Handle,
    _con_in: usize,
    console_out_handle: Handle,
    con_out: *mut SimpleTextOutputProtocol,
    standard_error_handle: Handle,
    std_err: *mut SimpleTextOutputProtocol,
    _runtime_services: usize,
    boot_services: *const BootServices,
    // pub number_of_table_entries: Uintn,
    // pub configuration_table: *const ConfigurationTable,
}

impl SystemTable {
    pub fn stdout(&self) -> &mut SimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }
    pub fn get_boot_services(&self) -> &BootServices {
        unsafe { &*self.boot_services }
    }
}

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,

    // Task Priority Services
    raise_tpl: unsafe extern "efiapi" fn(this: &BootServices, new_tpl: Tpl),
    restore_tpl: unsafe extern "efiapi" fn(this: &BootServices, old_tpl: Tpl),

    // Memory Servieces
    allocate_pages: unsafe extern "efiapi" fn(
        this: &BootServices,
        typ: AllocateType,
        memory_type: MemoryType,
        pages: Uintn,
        memory: PhysicalAddress,
    ) -> Status,
    free_pages: unsafe extern "efiapi" fn(
        this: &BootServices,
        memory: PhysicalAddress,
        pages: Uintn,
    ) -> Status,
    get_memory_map: unsafe extern "efiapi" fn(
        this: &BootServices,
        memory_map_size: *mut Uintn,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut Uintn,
        descriptor_size: *mut Uintn,
        descriptor_version: *mut Uintn,
    ) -> Status,

    _buf1: [usize; 11],

    handle_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        interface: &mut *mut c_void,
    ) -> Status,

    _buf2: [usize; 20],

    locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        registration: *mut c_void,
        interface: &mut *mut c_void,
    ) -> Status,
}

impl BootServices {
    pub fn get_memory_map<const BUFFER_SIZE: usize>(
        &self,
        memory_map: &mut MemoryMap<BUFFER_SIZE>,
    ) -> Status {
        unsafe {
            (self.get_memory_map)(
                self,
                &mut memory_map.map_size,
                memory_map.buffer.as_mut_ptr().cast::<MemoryDescriptor>(),
                &mut memory_map.map_key,
                &mut memory_map.descriptor_size,
                &mut memory_map.descriptor_version,
            )
        }
    }

    pub fn _open_simple_file_system_protocol(
        &self,
        handle: Handle,
    ) -> Result<SimpleFileSystemProtocol, Status> {
        let mut interface = ptr::null_mut();
        let status = unsafe {
            (self.handle_protocol)(handle, &SIMPLE_FILE_SYSTEM_PROTOCOL_GUID, &mut interface)
        };
        if status == Status::Success {
            Ok(unsafe { *interface.cast::<SimpleFileSystemProtocol>() })
        } else {
            Err(status)
        }
    }

    pub fn open_simple_file_system_protocol(&self) -> Result<SimpleFileSystemProtocol, Status> {
        let mut interface = ptr::null_mut();
        let status = unsafe {
            (self.locate_protocol)(
                &SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
                ptr::null_mut(),
                &mut interface,
            )
        };
        if status == Status::Success {
            Ok(unsafe { *interface.cast::<SimpleFileSystemProtocol>() })
        } else {
            Err(status)
        }
    }
}

#[repr(C)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Address,
    MaxType,
}

#[repr(C)]
pub enum MemoryType {
    ReservedMemoryType,
    // TBC...
}

type PhysicalAddress = Uint64;
type VirtualAddress = Uint64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub typ: Uint32,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: Uint64,
    pub attribute: Uint64,
}

/// MemoryMap is used for getting memory map
///
/// buffer_size is an allocated memory map size
pub struct MemoryMap<'a, const BUFFER_SIZE: usize> {
    pub buffer_size: usize,
    pub buffer: &'a mut [u8; BUFFER_SIZE],
    pub map_size: usize,
    pub map_key: usize,
    pub descriptor_size: usize,
    pub descriptor_version: usize,
}

impl<'a, const BUFFER_SIZE: usize> MemoryMap<'a, BUFFER_SIZE> {
    pub fn get(&self, i: usize) -> Option<&'a MemoryDescriptor> {
        if self.descriptor_size * i < self.map_size {
            let ptr = self.buffer.as_ptr() as usize + self.descriptor_size * i;
            let descriptor = unsafe { &*(ptr as *const MemoryDescriptor) };
            Some(descriptor)
        } else {
            None
        }
    }
}

const SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: Guid = Guid {
    data1: 0x0964e5b22,
    data2: 0x6459,
    data3: 0x11d2,
    data4: [0x8eu8, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: Uint64,
    open_volume: unsafe extern "efiapi" fn(
        this: &mut SimpleFileSystemProtocol,
        root: &mut *mut FileProtocol,
    ) -> Status,
}

impl SimpleFileSystemProtocol {
    pub fn open_volume(&mut self) -> Result<FileProtocol, Status> {
        let mut root = ptr::null_mut();
        let status = unsafe { (self.open_volume)(self, &mut root) };
        if status == Status::Success {
            Ok(unsafe { *root.cast::<FileProtocol>() })
        } else {
            Err(status)
        }
    }
}

#[repr(u64)]
pub enum OpenMode {
    Read = 0x0000000000000001,
    Write = 0x0000000000000002,
    Create = 0x8000000000000000,
}

#[repr(u64)]
pub enum FileAttributes {
    Null = 0,
    ReadOnly = 0x0000000000000001,
    Hidden = 0x0000000000000002,
    System = 0x0000000000000004,
    Reserved = 0x0000000000000008,
    Directory = 0x0000000000000010,
    Archive = 0x0000000000000020,
    ValidAttr = 0x0000000000000037,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FileProtocol {
    pub revision: Uint64,
    open: unsafe extern "efiapi" fn(
        this: &FileProtocol,
        new_handle: &mut *mut FileProtocol,
        filename: *const Char16,
        open_mode: *const OpenMode,
        attributes: *const FileAttributes,
    ) -> Status,
}

impl FileProtocol {
    pub fn open(
        &self,
        filename: &str,
        open_mode: OpenMode,
        attributes: FileAttributes,
    ) -> Result<FileProtocol, Status> {
        let mut ptr = ptr::null_mut();
        let status = unsafe {
            (self.open)(
                &self,
                &mut ptr,
                filename.as_ptr().cast::<Char16>(),
                &open_mode,
                &attributes,
            )
        };
        if status == Status::Success {
            Ok(unsafe { *ptr })
        } else {
            Err(status)
        }
    }
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: unsafe extern "efiapi" fn(
        this: &SimpleTextOutputProtocol,
        extended_verification: Boolean,
    ) -> Status,
    output_string:
        unsafe extern "efiapi" fn(this: &SimpleTextOutputProtocol, string: *const Char16) -> Status,
}

impl SimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> Status {
        unsafe { (self.reset)(self, extended_verification) }
    }
    fn output_string(&self, string: *const u16) -> Status {
        unsafe { (self.output_string)(self, string) }
    }
    pub fn print(&self, string: &str) -> Status {
        let string = string.as_bytes();
        let mut buffer = [0u16; 100];
        let size = cmp::min(100, string.len());
        for i in 0..size {
            buffer[i] = string[i] as u16;
        }
        self.output_string(buffer.as_ptr())
    }
}
