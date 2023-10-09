use spin;
use crate::print;

#[derive(Debug)]
pub struct KeyboardState {
    alt      : bool,
    ctrl     : bool,
    shift    : bool,
    capslock : bool,
}
pub static KEYBOARD_STATE: spin::Mutex<KeyboardState> =
    spin::Mutex::new(
        KeyboardState {
            alt      : false,
            ctrl     : false,
            shift    : false,
            capslock : false,
        }
    );

/// This function:
/// - Updates the KEYBOARD_STATE Mutex
pub fn handle_keyboard(scancode: u8) {
    match scancode {
        // Alt
        56  => KEYBOARD_STATE.lock().alt      = true ,
        184 => KEYBOARD_STATE.lock().alt      = false, // 56 + 128 = Key released
        // Control
        29  => KEYBOARD_STATE.lock().ctrl     = true ,
        157 => KEYBOARD_STATE.lock().ctrl     = false,
        // Shift
        42  => KEYBOARD_STATE.lock().shift    = true ,
        170 => KEYBOARD_STATE.lock().shift    = false,
        // Caps Lock
        58  => KEYBOARD_STATE.lock().capslock = true ,
        186 => KEYBOARD_STATE.lock().capslock = false,

        _   => {}
    }
}