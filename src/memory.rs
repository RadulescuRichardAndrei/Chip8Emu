use crate::register::RegisterValue;

pub struct Memory{
    buffer: [u8; 4096],
}

impl Memory{
    pub fn new() -> Self {
        let mut buffer = [0u8; 4096];
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        buffer[..fontset.len()].copy_from_slice(&fontset);

        Memory{buffer}
    }

    pub fn get_byte(&self, address: usize) -> u8{
        self.buffer[address]
    }
    pub fn load(&mut self, data: &[u8], start_addr: usize){
        let end_addr = start_addr+ data.len();
        self.buffer[start_addr..end_addr].copy_from_slice(data)

    }
    pub fn set_byte(&mut self, address: usize, value: RegisterValue) {
        match value {
            RegisterValue::Bits8(val) => {self.buffer[address] = val;}
            RegisterValue::Bits16(val) => {
                self.buffer[address] = (val & 0xFF) as u8;
                self.buffer[address+1] = ((val>>8) & 0xFF) as u8;
            }
        }

    }

}
