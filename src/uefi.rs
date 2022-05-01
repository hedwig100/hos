use core::{cmp, ffi::c_void};

// Common UEFI Data Types
type Boolean = bool;
type Uintn = usize;
type Uint32 = u32;
type Uint64 = u64;
type Char16 = u16;

type Tpl = Uintn;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Handle(*mut c_void);

#[derive(PartialEq)]
#[repr(usize)]
pub enum Status {
    Success = 0,
    LoadError = 1,
    InvalidParameter = 2,
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
    // TBC..
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
        let mut buffer = [0u16; 40];
        let size = cmp::min(40, string.len());
        for i in 0..size {
            buffer[i] = string[i] as u16;
        }
        self.output_string(buffer.as_ptr())
    }
}
