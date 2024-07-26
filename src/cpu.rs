use std::cmp::{Ordering, PartialOrd};
use std::ops::{Add, BitOr, Shl, Shr};
use crate::decoded_instruction::Decoded_Instruction;
use crate::memory::Memory;
use crate::register::{Register, RegisterValue};
use crate::stack::Stack;
use rand::Rng;
use crate::keypad::Keypad;

pub struct Cpu{
    pub memo: Memory,
    pub stack: Stack,
    pub registers: [Register; 21],
    pub screen: [[bool;32];64],
    pub keypad: Keypad
}

const PC_POZ :usize = 19;
const INDEX_POZ :usize =20;

const VF_POZ: usize=15;
const DT_POZ: usize=17;
const ST_POZ: usize=16;
const START_PC: usize=200;

impl Cpu{
    pub fn new() -> Cpu{
        Cpu{
        memo: Memory::new(),
        stack: Stack::new(),
        registers: [
            Register{name: String::from("V0"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V1"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V2"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V3"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V4"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V5"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V6"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V7"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V8"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("V9"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VA"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VB"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VC"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VD"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VE"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("VF"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("ST"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("DT"), bits: RegisterValue::Bits8(0x0)},
            Register{name: String::from("SP"), bits: RegisterValue::Bits16(0x0)},
            Register{name: String::from("PC"), bits: RegisterValue::Bits16(0x200)},
            Register{name: String::from("I"), bits: RegisterValue::Bits16(0x0)},
        ],
        screen: [[false;32];64],
        keypad: Keypad::new(),
        }
    }
    pub fn initialize(&mut self, data: &[u8]){
        self.memo.load(data,START_PC);
    }
    pub fn execute(&mut self){
        loop{
            let instr= self.fetch_instruction(u16::from(self.registers[PC_POZ].get_register_value()))
            let result= self.decode_instruction(instr).unwrap();
        }

    }

    fn fetch_instruction(&mut self, location: u16) -> u16{
        let first_part:u8= self.memo.get_byte(location as usize);
        let second_part:u8= self.memo.get_byte((location+1) as usize);
        self.registers[PC_POZ].increment_register_value(2u16);
        return self.combine_u8_to_u16(first_part, second_part);
    }

    fn combine_u8_to_u16(&self, high: u8, low: u8) -> u16 {
        ((high as u16) << 8) | (low as u16)
    }

    pub fn decode_instruction(&mut self, instruction_code: u16) -> Result<(), &str>{
        let decoded_instruction= Decoded_Instruction::new(instruction_code);
        println!("{:?}", decoded_instruction);
        match decoded_instruction.first_part {
            0 =>{
                match decoded_instruction.second_byte {
                    0xE0 =>  Err("clear display not implemented"),
                    0xEE => {
                        let value = self.stack.pop()?;
                        self.registers[PC_POZ].set_register_value(value);
                        Ok(())
                    },
                    _ => Err("Could not Decode")
                }
            },
            1 =>{
                self.registers[PC_POZ].set_register_value(decoded_instruction.nnn_bytes);
                Ok(())
            },
            2 =>{
                self.stack.push(u16::from(self.registers[PC_POZ].get_register_value()))?;
                self.registers[PC_POZ].set_register_value(decoded_instruction.nnn_bytes);
                Ok(())
            },
            3 =>{
              if decoded_instruction.second_byte as u16 ==
                  u16::from(self.registers[decoded_instruction.second_part as usize]
                      .get_register_value()){
                  self.registers[20].increment_register_value(2u16)
              }
                Ok(())
            },
            4 =>{
                if decoded_instruction.second_byte as u16 !=
                    u16::from(self.registers[decoded_instruction.second_part as usize]
                        .get_register_value()){
                    self.registers[PC_POZ].increment_register_value(2u16)
                }
                Ok(())
            },
            5 => {
                if u16::from(self.registers[decoded_instruction.second_part as usize]
                                 .get_register_value()) ==
                    u16::from(self.registers[decoded_instruction.third_part as usize]
                        .get_register_value()){
                    self.registers[PC_POZ].increment_register_value(2u16)
                }
                Ok(())
            }
            6 =>{
                self.registers[decoded_instruction.second_part as usize].set_register_value(decoded_instruction.second_byte);
                Ok(())
            },
            7 =>{
                self.registers[decoded_instruction.second_part as usize].increment_register_value(decoded_instruction.second_byte);
                Ok(())
            },
            8 =>{
              self.decode_instruction_logical(decoded_instruction)
            },
            9 =>{
                if u16::from(self.registers[decoded_instruction.second_part as usize]
                    .get_register_value()) !=
                    u16::from(self.registers[decoded_instruction.third_part as usize]
                        .get_register_value()){
                    self.registers[PC_POZ].increment_register_value(2u16)
                }
                Ok(())
            },
            0xA =>{
                self.registers[INDEX_POZ].set_register_value(decoded_instruction.nnn_bytes);
                Ok(())
            }
            0xB =>{
            // optional jump to nnn + vx
                self.registers[PC_POZ].set_register_value(
                    RegisterValue::Bits16(decoded_instruction.nnn_bytes)
                    + self.registers[0].get_register_value());


                Ok(())
            },
            0xC =>{
                let mut rng = rand::thread_rng();
                let random_value: u8= rng.gen();// prob 12 bits needed
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    (decoded_instruction.second_byte & random_value) as u16
                );
              Ok(())
            },
            0xD =>{
                let mut x_coord:u16= u16::from(self.registers[decoded_instruction.second_part as usize]
                    .get_register_value() & RegisterValue::Bits16(63));
                let mut y_coord:u16= u16::from(self.registers[decoded_instruction.third_part as usize]
                    .get_register_value() & RegisterValue::Bits16(31));

                self.registers[VF_POZ].set_register_value(0u16);
                for i in 0..decoded_instruction.fourth_part{
                    let bits= self.memo.get_byte((u16::from(self.registers[INDEX_POZ].get_register_value()) + i).into());

                    for j in (0..8).rev(){
                        let bit = bits & (1<<j) !=0;
                        if x_coord >=64{
                           break
                        }
                        if bit && self.screen[x_coord as usize][y_coord as usize]{
                            self.screen[x_coord as usize][y_coord as usize]= false;
                            self.registers[VF_POZ].set_register_value(1u16);
                        } else if bit && !self.screen[x_coord as usize][y_coord as usize]{
                            self.screen[x_coord as usize][y_coord as usize]= true;
                        }
                        x_coord +=1;
                    }
                    y_coord+=1;
                    x_coord= x_coord & 63;
                    if y_coord >=32{
                        break
                    }

                }

                Ok(())
            },
            0xE=>{
                match decoded_instruction.fourth_part {
                    0xE =>{
                        if self.keypad.is_key_pressed(u16::from(
                            self.registers[decoded_instruction.second_part as usize]
                                .get_register_value())){
                            self.registers[PC_POZ].increment_register_value(2u16);
                        }
                        Ok(())
                    },
                    1 =>{
                        if !self.keypad.is_key_pressed(u16::from(
                            self.registers[decoded_instruction.second_part as usize]
                                .get_register_value())){
                            self.registers[PC_POZ].increment_register_value(2u16);
                        }
                        Ok(())
                    },
                    _ => Err("Could not Decode")
                }
            },
            0xF=>{
                match decoded_instruction.second_byte {
                    0x07=>{
                        self.registers[decoded_instruction.second_part as usize]
                            .set_register_value(u16::from(
                                self.registers[DT_POZ].get_register_value()
                            ));
                        Ok(())
                    },
                    0x15=>{
                        self.registers[DT_POZ].set_register_value(u8::from(
                            self.registers[decoded_instruction.second_part as usize]
                                .get_register_value())
                        );
                        Ok(())
                    },
                    0x18=>{
                        self.registers[ST_POZ].set_register_value(u8::from(
                            self.registers[decoded_instruction.second_part as usize]
                                .get_register_value())
                        );
                        Ok(())
                    },
                    0x1E =>{
                        self.registers[INDEX_POZ].set_register_value(
                            self.registers[INDEX_POZ].get_register_value()
                                + self.registers[decoded_instruction.second_part as usize]
                                .get_register_value()
                        );
                      Ok(())
                    },
                    0x0A=>{
                        let key_hex=self.keypad.is_any_key_pressed();
                        match key_hex {
                            Some(value)=> self.registers[decoded_instruction.second_part as usize]
                                .set_register_value(value),
                            None => self.registers[PC_POZ].decrement_register_value(2u16)
                        }

                        Ok(())
                    },
                    0x29=>{
                        self.registers[INDEX_POZ].set_register_value(
                            (u16::from(self.registers[decoded_instruction.second_part as usize]
                                .get_register_value()) & 0xF)
                        );
                        Ok(())
                    },
                    0x33 =>{
                        let value= (u16::from(self.registers[decoded_instruction.second_part as usize]
                            .get_register_value()) & 0xFF) as u8;
                        let mut value_copy= value;
                        let mut digits: Vec<u8> = Vec::with_capacity(4);
                        while value_copy>0{
                            let n =value_copy %10;
                            value_copy /=10;
                            digits.push(n);
                        }
                        let mut offset=0;
                        for digit in digits{
                            self.memo.set_byte(
                                (u16::from(self.registers[INDEX_POZ]
                                    .get_register_value()) + offset).into(),
                            RegisterValue::Bits8(digit)
                            );
                            offset+=1;
                        }

                        Ok(())
                    },
                    0x55=>{
                        let mut offset=0;
                        for i in 0..decoded_instruction.second_part{
                            self.memo.set_byte(
                                (u16::from(self.registers[INDEX_POZ]
                                    .get_register_value()) + offset).into(),
                                self.registers[i as usize].get_register_value()
                            );
                            offset+=2
                        }
                        Ok(())
                    },
                    0x65=>{
                        Ok(())
                    }
                    _ => Err("Could not Decode")
                }
            }
            _ => Err("Could not Decode")
        }
    }
    fn decode_instruction_logical(&mut self, decoded_instruction: Decoded_Instruction) -> Result<(), &str> {

        match decoded_instruction.fourth_part {
            0 => {

                self.registers[decoded_instruction.second_part as usize]
                    .set_register_value(
                        self.registers[decoded_instruction.third_part as usize]
                        .get_register_value());
                Ok(())
            },
            1 =>{
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        | self.registers[decoded_instruction.third_part as usize].get_register_value()
                );

                Ok(())
            },
            2 =>{
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        & self.registers[decoded_instruction.third_part as usize].get_register_value()
                );
                Ok(())
            },
            3 =>{
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        ^ self.registers[decoded_instruction.third_part as usize].get_register_value()
                );
                Ok(())
            },
            4 =>{
                let check_overflow= self.registers[decoded_instruction.second_part as usize]
                    .get_register_value().check_add(
                    &self.registers[decoded_instruction.third_part as usize]
                        .get_register_value());

                match check_overflow {
                    Some(value)=> {},
                    None => self.registers[VF_POZ].set_register_value(1u16)
                }
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        + self.registers[decoded_instruction.third_part as usize].get_register_value()
                );
                Ok(())
            },
            5 =>{
                if self.registers[decoded_instruction.second_part as usize].get_register_value()
                    >= self.registers[decoded_instruction.third_part as usize].get_register_value(){
                    self.registers[VF_POZ].set_register_value(1u16);
                }else {
                    self.registers[VF_POZ].set_register_value(0u16);
                }

                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        - self.registers[decoded_instruction.third_part as usize].get_register_value()
                );
              Ok(())
            },
            7 =>{
                if self.registers[decoded_instruction.third_part as usize].get_register_value()
                    >= self.registers[decoded_instruction.second_part as usize].get_register_value(){
                    self.registers[VF_POZ].set_register_value(1u16);
                }else {
                    self.registers[VF_POZ].set_register_value(0u16);
                }

                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.third_part as usize].get_register_value()
                        - self.registers[decoded_instruction.second_part as usize].get_register_value()
                );
                Ok(())
            },
            6 =>{
                // optional set vx to vy
                self.registers[VF_POZ].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        & RegisterValue::from(1u16)
                );
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value() >> 1
                );
                Ok(())
            },
            0xE =>{
                // optional set vx to vy
                self.registers[VF_POZ].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value()
                        & RegisterValue::from((1 << 15)as u16)
                );
                self.registers[decoded_instruction.second_part as usize].set_register_value(
                    self.registers[decoded_instruction.second_part as usize].get_register_value() << 1
                );
                Ok(())
            }

            _ => Err("Could not Decode")
        }
    }

}



