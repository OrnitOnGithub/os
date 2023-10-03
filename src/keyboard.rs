use lazy_static::lazy_static;
use std::sync::Mutex;


pub struct KeyboardState {
    shift:    bool,
    capslock: bool,
    ctrl:     bool,
}
// Create a static mutex for the keyboard state.
lazy_static! {
    static ref KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState {
        shift:    false,
        capslock: false,
        ctrl:     false,
    });
}
/// This function should:
/// - Do something
/// - Somehow inform the API / AKI
pub fn handle_keyboard(scancode: u8) {

}