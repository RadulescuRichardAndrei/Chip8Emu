pub struct Keypad{
    //later use
    keys_state: [bool;16],
}

impl Keypad{
    pub fn new() -> Keypad{
        Keypad{
            keys_state: [false;16],
        }
    }
    pub fn press_key(&mut self, key_hex: u16) {
        // Set the corresponding key to pressed (true)
        if key_hex < 16 {
            self.keys_state[key_hex as usize] = true;
        }
    }
    pub fn release_key(&mut self, key_hex: u16) {
        // Set the corresponding key to not pressed (false)
        if key_hex < 16 {
            self.keys_state[key_hex as usize] = false;
        }
    }
    pub fn is_key_pressed(&self, key_hex:u16)-> bool{
        if key_hex < 16 {
            self.keys_state[key_hex as usize]
        } else {
            false
        }
    }
    pub fn is_any_key_pressed(&self) -> Option<u16>{
        for (index, &pressed) in self.keys_state.iter().enumerate() {
            if pressed {
                return Some(index as u16);
            }
        }
        None
    }
}

