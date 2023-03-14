// virtual key code reference
// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
use std::mem::MaybeUninit;
use winapi::um::winuser::{GetAsyncKeyState, GetKeyState, MapVirtualKeyA, MapVirtualKeyW, VK_SHIFT, VK_CONTROL, VK_MENU};
use winapi::um::winuser::{keybd_event, VK_RETURN};

use anyhow::Result;

fn handle_keyboard_event() -> Option<u8> {
    let mut buffer = [0u8; 256];
    let mut key_code: Option<u8> = None;
    unsafe {
        if GetKeyState(buffer.as_mut_ptr()) == 0 {
            return key_code;
        }
        for i in 0..255 {
            let state = buffer[i] as i32;
            if state == -128 || state == -127 {
                key_code = Some(i as u8);
                break;
            }
        }
    }
    key_code
}

fn keyboard_output(key: u8) {
    unsafe {
        keybd_event(key, 0, 0, 0);
        keybd_event(key, 0, 2, 0);
    }
}

fn main() -> Result<()> {
    loop {
        if let Some(key_code) = handle_keyboard_event() {
            println!("Key code: {}", key_code);

            keyboard_output(VK_RETURN);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
