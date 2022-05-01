use core::ffi::c_void;

#[repr(usize)]
pub enum Status {
    Success = 0,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Handle(*mut c_void);

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    _con_in: usize,
    pub console_out_handle: Handle,
    pub con_out: *mut SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub std_err: *mut SimpleTextOutputProtocol,
    // pub runtime_services: *const RuntimeServices,
    // pub boot_services: *const BootServices,
    // pub number_of_table_entries: u64,
    // pub configuration_table: *const ConfigurationTable,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(
        this: &SimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> Status,
    pub output_string:
        unsafe extern "efiapi" fn(this: &SimpleTextOutputProtocol, string: *const u16) -> Status,
}

impl SystemTable {
    pub fn stdout(&self) -> &mut SimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }
}

impl SimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> Status {
        unsafe { (self.reset)(self, extended_verification) }
    }
    pub fn output_string(&self, string: *const u16) -> Status {
        unsafe { (self.output_string)(self, string) }
    }
}