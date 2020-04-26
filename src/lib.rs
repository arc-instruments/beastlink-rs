mod beastlink_sys;

use std::error::Error;
use std::fmt;
use std::ffi;
use std::os::raw::{c_int, c_uint, c_char, c_void, c_ushort};
use std::sync::Once;
use std::sync::atomic::AtomicBool;
use std::env;
use beastlink_sys as bl;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use ctor::ctor;
use libc::atexit;

static _ONCE_EXEC: Once = Once::new();
#[ctor]
static _LIBRARY_INITIALISED: AtomicBool = _inner_init();

fn _inner_init() -> AtomicBool {
    _ONCE_EXEC.call_once(|| unsafe {
        let log_level = LogLevel::from_env();

        init(log_level).unwrap();
        assert_eq!(atexit(_inner_cleanup), 0);
    });

    extern "C" fn _inner_cleanup() {
        cleanup().unwrap();
    }
    AtomicBool::new(true)
}

#[derive(Debug)]
pub struct BLError {
    _code: i32,
    _msg: String
}

impl BLError {
    fn new(code: i32, msg: String) -> BLError {
        BLError{_code: code, _msg: msg}
    }
}

impl BLError {
    fn from_code(code: i32) -> BLError {
        let code = code as c_int;
        let mut msgbuf = [0 as c_char; bl::BL_IF_BUFFER_SIZE as usize];
        unsafe { bl::BlGetLastErrorText(code, msgbuf.as_mut_ptr()) };
        let msg = unsafe {
            ffi::CString::from_raw(msgbuf.as_mut_ptr()).to_str().unwrap().to_owned()
        };
        BLError::new(code as i32, msg)
    }
}

impl fmt::Display for BLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self._code, &self._msg)
    }
}

impl Error for BLError { }

#[derive(Debug,FromPrimitive)]
#[repr(i32)]
pub enum LogLevel {
    Off = 0,
    Error,
    Warning,
    Debug
}

impl LogLevel {
    fn from_num(level: i32) -> LogLevel {
        if level <= 0 {
            LogLevel::Off
        } else if level > (LogLevel::Debug as i32) {
            LogLevel::Debug
        } else {
            LogLevel::from_i32(level).unwrap()
        }
    }

    fn from_env() -> LogLevel {
        match env::var("BLRS_LOG") {
            Ok(x) => LogLevel::from_num(x.parse::<i32>().unwrap()),
            Err(_) => LogLevel::Error
        }
    }
}

#[derive(Debug,FromPrimitive)]
#[repr(u32)]
pub enum Flags {
    NoFlags = 0u32,
    ConstAddress = bl::BL_FLAG_CONST_ADDR as u32,
}

fn wrap_return_str(handle: bl::BL_DEVICE_HANDLE, func: unsafe extern "C" fn(c_int, *mut c_char) -> c_int) -> Result<String, BLError> {
    let mut resbuf = [0 as c_char; bl::BL_IF_BUFFER_SIZE as usize];

    match unsafe { func(handle, resbuf.as_mut_ptr()) } {
        0 => {
            let res = unsafe {
                ffi::CStr::from_ptr(resbuf.as_ptr()).to_str().unwrap().to_string()
            };
            Ok(res)
        },
        err => Err(BLError::from_code(err))
    }
}

fn init(log_level: LogLevel) -> Result<(), BLError> {

    let res = unsafe { bl::BlInit() } as i32;

    if res != 0 {
        return Err(BLError::from_code(res))
    }

    match unsafe { bl::BlSetLogLevel(log_level as c_int) } {
        0 => Ok(()),
        err => Err(BLError::from_code(err))
    }

}

fn cleanup() -> Result<(), BLError> {
    match unsafe { bl::BlCleanup() } {
        0 => Ok(()),
        err => Err(BLError::from_code(err))
    }
}

pub fn version() -> Result<(i32, i32, i32), BLError> {
    let mut major = 0 as c_int;
    let mut minor = 0 as c_int;
    let mut fix = 0 as c_int;

    match unsafe { bl::BlGetVersion(&mut major, &mut minor, &mut fix) } {
        0 => Ok((major, minor, fix)),
        err => Err(BLError::from_code(err))
    }
}

pub fn enumerate(vid: u16, pid: u16) -> Result<i32, BLError> {
    let mut count = -1 as c_int;

    let vid = vid as c_ushort;
    let pid = pid as c_ushort;

    match unsafe { bl::BlEnumerate(vid, pid, &mut count) } {
        0 => Ok(count as i32),
        err => Err(BLError::from_code(err))
    }
}

#[derive(Debug)]
pub struct Device {
    _handle: bl::BL_DEVICE_HANDLE
}

impl Device {
    pub fn open(id: i32) -> Result<Device, BLError> {
        let id = id as c_int;
        let mut handle: bl::BL_DEVICE_HANDLE = bl::BL_INVALID_DEVICE;

        match unsafe { bl::BlOpen(id, &mut handle) } {
            0 => Ok(Device{ _handle: handle }),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn close(&self) -> Result<(), BLError> {
        match unsafe { bl::BlClose(self._handle) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn read_register(&self, register: u32) -> Result<u32, BLError> {
        let register = register as c_uint;
        let mut result = 0 as c_uint;

        match unsafe { bl::BlReadRegister(self._handle, register, &mut result) } {
            0 => Ok(result as u32),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn write_register(&self, register: u32, value: u32) -> Result<(), BLError> {
        let register = register as c_uint;
        let value = value as c_uint;

        match unsafe { bl::BlWriteRegister(self._handle, register, value) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn write_block(&self, addr: u32, buffer: &mut [u8], f: Flags) -> Result<(), BLError> {
        let data = (*buffer).as_mut_ptr() as *mut c_void;
        let size = buffer.len() as c_int;
        let flags = f as i32;

        match unsafe { bl::BlWriteBlock(self._handle, addr, data, size, flags) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn read_block(&self, addr: u32, size: i32, f: Flags) -> Result<Vec<u8>, BLError> {
        let mut buffer: Vec<u8> = vec![0u8; size as usize];
        let bufptr = buffer.as_mut_ptr() as *mut c_void;
        let flags = f as i32;

        match unsafe { bl::BlReadBlock(self._handle, addr, bufptr, size, flags) } {
            0 => Ok(buffer),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn reset_fpga(&self) -> Result<(), BLError> {
        match unsafe { bl::BlResetFpga(self._handle) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn set_timeout(&self, timeout: u32) -> Result<(), BLError> {
        let timeout = timeout as c_uint;
        match unsafe { bl::BlSetTimeout(self._handle, timeout) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn firmware_version(&self) -> Result<(i32, i32, i32), BLError> {
        let mut major = 0 as c_int;
        let mut minor = 0 as c_int;
        let mut fix = 0 as c_int;

        match unsafe { bl::BlGetFirmwareVersion(self._handle, &mut major, &mut minor, &mut fix) } {
            0 => Ok((major, minor, fix)),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn serial_number(&self) -> Result<String, BLError> {
        wrap_return_str(self._handle, bl::BlGetSerialNumber)
    }

    pub fn derivate_info(&self) -> Result<String, BLError> {
        wrap_return_str(self._handle, bl::BlGetDerivateInfo)
    }

    pub fn get_derivate_id(&self) -> Result<u32, BLError> {
        let mut result = 0 as c_uint;

        match unsafe { bl::BlGetDerivateId(self._handle, &mut result) } {
            0 => Ok(result as u32),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn get_user_id(&self) -> Result<String, BLError> {
        wrap_return_str(self._handle, bl::BlGetUserId)
    }

    pub fn set_user_id(&self, id: &str) -> Result<(), BLError> {
        let c_id = ffi::CString::new(id).unwrap();
        let c_id_p = c_id.into_raw();

        let res = match unsafe { bl::BlSetUserId(self._handle, c_id_p) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        };

        // put the pointer back into Rust-management to prevent it from leaking
        let _ = unsafe { ffi::CString::from_raw(c_id_p) };
        res
    }

    pub fn program_from_file(&self, path: &str) -> Result<(), BLError> {
        let c_path = ffi::CString::new(path).unwrap();
        let c_path_p = c_path.as_ptr();

        match unsafe { bl::BlProgramFpgaFromBin(self._handle, c_path_p) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }

    pub fn program_from_memory(&self, bytes: &mut [u8], compressed: bool) -> Result<(), BLError> {
        let data = (*bytes).as_mut_ptr() as *mut c_void;
        let size = bytes.len() as c_int;
        let compressed = compressed as c_int;

        match unsafe { bl::BlProgramFpgaFromMemory(self._handle, data, size, compressed) } {
            0 => Ok(()),
            err => Err(BLError::from_code(err))
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {

    use std::env;
    use std::fs;
    use serial_test::serial;
    use super::*;

    const VID: u16 = 0x10f8;
    const PID: u16 = 0xc583;

    /*#[test]
    #[serial]
    fn initialisation() {
        match init(LogLevel::Debug) {
            Err(err) => panic!("Initialisation failed {}", err),
            _ => {}
        }

        match cleanup() {
            Err(err) => panic!("cleanupialisation failed {}", err),
            _ => {}
        }

    }*/

    #[test]
    #[serial]
    fn enumeration() {

        match enumerate(VID, PID) {
            Ok(count) => {
                println!("Enumeration result: {}", count);
                assert!( count >= 0, "Enumeration is negative!" )
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        }

    }

    #[test]
    #[serial]
    fn open_close_devices() {

        let count = match enumerate(VID, PID) {
            Ok(count) => {
                assert!( count >= 0, "Number of enumerated devices is negative!" );
                println!("Enumerated {} devices", count);
                count
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        };

        for enum_id in 0..count {
            println!("Trying device {}", enum_id);
            let dev = match Device::open(enum_id) {
                Ok(dev) => dev,
                Err(err) => {
                    panic!("Error while opening device: {}", err)
                }
            };

            println!("Opened device {:?}", dev);

            match dev.close() {
                Err(err) => {
                    panic!("Error while closing device {:?}: {}", dev, err)
                },
                _ => {}
            }

        }

    }

    #[test]
    #[serial]
    fn serial_number() {

        let count = match enumerate(VID, PID) {
            Ok(count) => {
                assert!( count >= 0, "Number of enumerated devices is negative!" );
                println!("Enumerated {} devices", count);
                count
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        };

        for enum_id in 0..count {
            println!("Trying device {}", enum_id);
            let dev = match Device::open(enum_id) {
                Ok(dev) => dev,
                Err(err) => {
                    panic!("Error while opening device: {}", err)
                }
            };

            println!("Opened device {:?}", dev);

            match dev.serial_number() {
                Ok(serialnum) => println!("Serial number: {}", serialnum),
                Err(err) => {
                    panic!("Could not read serial number: {}", err)
                }
            }

            match dev.close() {
                Err(err) => {
                    panic!("Error while closing device {:?}: {}", dev, err)
                },
                _ => {}
            }

        }

    }

    #[test]
    #[serial]
    fn firmware_rev() {

        let count = match enumerate(VID, PID) {
            Ok(count) => {
                assert!( count >= 0, "Number of enumerated devices is negative!" );
                println!("Enumerated {} devices", count);
                count
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        };

        for enum_id in 0..count {
            println!("Trying device {}", enum_id);
            let dev = match Device::open(enum_id) {
                Ok(dev) => dev,
                Err(err) => {
                    panic!("Error while opening device: {}", err)
                }
            };

            println!("Opened device {:?}", dev);

            match dev.firmware_version() {
                Ok(fwver) => println!("Firmware revision: {}.{}.{}", fwver.0, fwver.1, fwver.2),
                Err(err) => {
                    panic!("Could not read firmware revision: {}", err)
                }
            }

            match dev.close() {
                Err(err) => {
                    panic!("Error while closing device {:?}: {}", dev, err)
                },
                _ => {}
            }

        }
    }

    #[test]
    #[ignore]
    #[serial]
    fn efm03_comprehensive() {

        // Check if the FPGA design is provided and is valid
        // Unfortunately due to IP considerations we cannot
        // provide the .bin file so the user must point us
        // to it!
        let path = match env::var("EFM03_BL_TEST_FILE") {
            Ok(p) => p,
            Err(_) => { panic!("EFM03_BL_TEST_FILE is not set"); }
        };

        match fs::metadata(&path) {
            Ok(meta) => {
                if !meta.is_file() {
                    panic!("{} is not a file", path);
                }
            },
            Err(_) => {
                panic!("{} is not a valid file", path);
            }
        };

        // Find EFM03 devices
        let count = match enumerate(VID, PID) {
            Ok(count) => {
                assert!( count >= 0, "Number of enumerated devices is negative!" );
                println!("Enumerated {} devices", count);
                count
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        };

        if count == 0 {
            panic!("No EFM03 devices found");
        }

        for enum_id in 0..count {
            println!("Trying device {}", enum_id);
            let dev = match Device::open(enum_id) {
                Ok(dev) => dev,
                Err(err) => {
                    panic!("Error while opening device: {}", err)
                }
            };

            println!("Opened device {:?}", dev);

            // Load the design unto the FPGA
            match dev.program_from_file(&path) {
                Ok(_) => { println!("FPGA design loaded"); },
                Err(err) => {
                    panic!("Error while programming FPGA: {}", err)
                }
            };

            let ram_base: u32 = 0x00000000;
            let block_size: usize = 1024 * 1024;

            // Make a 1 MiB buffer of bytes 0x11
            let mut outb: Vec<u8> = vec![0x11; block_size];

            // Write them to the FPGA
            match dev.write_block(ram_base, &mut outb, Flags::NoFlags) {
                Ok(_) => { println!("Block written") }
                Err(err) => {
                    panic!("Failure while writing to RAM: {}", err);
                }
            }

            // Read them back
            let inb: Vec<u8>;
            inb = match dev.read_block(ram_base, block_size as i32, Flags::NoFlags) {
                Ok(buf) => buf,
                Err(err) => {
                    panic!("Error while reading buffer: {}", err);
                }
            };

            // They should be the same
            if inb == outb {
                println!("Written buffer is read back succesfully");
            } else {
                panic!("Buffers differ");
            }

            match dev.close() {
                Err(err) => {
                    panic!("Error while closing device {:?}: {}", dev, err)
                },
                _ => {}
            }

        }

    }
}
