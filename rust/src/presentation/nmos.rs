use bindings::charmap::{Pkstr, pkstr_raw};

// EWRAM layout at 0x02020000:
//   +0x00: ready flag (1 = data valid)
//   +0x01: sender label   (32 bytes, GBA charmap, 0xFF-terminated)
//   +0x21: receiver label (32 bytes, GBA charmap, 0xFF-terminated)
//   +0x41: command (0=none, 1=connect)
//   +0x42: status  (0=idle, 1=connecting, 2=connected, 3=failed)
const NMOS_BASE: *const u8 = 0x0202_0000 as *const u8;
const NMOS_CMD: *mut u8 = 0x0202_0041 as *mut u8;
const NMOS_STATUS: *const u8 = 0x0202_0042 as *const u8;

pub fn nmos_ready() -> bool {
    unsafe { *NMOS_BASE == 1 }
}

pub fn nmos_sender_label() -> &'static Pkstr {
    unsafe { pkstr_raw(core::slice::from_raw_parts(NMOS_BASE.add(0x01), 32)) }
}

pub fn nmos_receiver_label() -> &'static Pkstr {
    unsafe { pkstr_raw(core::slice::from_raw_parts(NMOS_BASE.add(0x21), 32)) }
}

pub fn nmos_status() -> u8 {
    unsafe { *NMOS_STATUS }
}

pub fn nmos_request_connect() {
    unsafe { *NMOS_CMD = 1 }
}
