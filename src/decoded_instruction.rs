#[derive(Debug)]
pub struct Decoded_Instruction{
    pub first_part: u16,
    pub second_part: u16,
    pub third_part: u16,
    pub fourth_part: u16,
    pub second_byte: u8,
    pub nnn_bytes: u16,
}

impl Decoded_Instruction{
    pub fn new(instruction_code: u16)->Decoded_Instruction{
        Decoded_Instruction{
            first_part: (instruction_code >>12) & 0xF,
            second_part: (instruction_code >>8) & 0xF,
            third_part: (instruction_code >>4) & 0xF,
            fourth_part: instruction_code & 0xF,
            second_byte: (instruction_code & 0xFF) as u8,
            nnn_bytes: (instruction_code &0xFFF) as u16,
        }
    }
}