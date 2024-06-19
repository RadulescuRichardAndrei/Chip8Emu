use std::cmp::Ordering;
use std::ops::{Sub, Add, BitAnd, BitOr, BitXor, Shr, Shl};

#[derive(Clone, Debug)]
pub enum RegisterValue{
    Bits8(u8),
    Bits16(u16),
}

pub struct Register{
    pub name: String,
    pub bits: RegisterValue,
}
impl Shl<i32> for RegisterValue {
    type Output = RegisterValue;

    fn shl(self, rhs: i32) -> Self::Output {
        match (self) {
            RegisterValue::Bits8(val1) => RegisterValue::from(val1 << rhs),
            RegisterValue::Bits16(val1)=> RegisterValue::from(val1 << rhs),
        }
    }
}
impl Shr<i32> for RegisterValue {
    type Output = RegisterValue;

    fn shr(self, rhs: i32) -> Self::Output {
        match (self) {
            RegisterValue::Bits8(val1) => RegisterValue::from(
                val1 >> rhs),
            RegisterValue::Bits16(val1)=> RegisterValue::from(val1 >> rhs),
        }
    }
}
impl PartialEq<Self> for RegisterValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => val1.eq(val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => val1.eq(val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}

impl PartialOrd for RegisterValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => val1.partial_cmp(val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => val1.partial_cmp(val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}
impl Sub for RegisterValue{
    type Output = RegisterValue;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => RegisterValue::Bits8(val1 - val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => RegisterValue::Bits16(val1 - val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}
impl Add for RegisterValue {
    type Output = RegisterValue;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => RegisterValue::Bits8(val1 + val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => RegisterValue::Bits16(val1 + val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}

impl BitXor for RegisterValue{
    type Output = RegisterValue;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => RegisterValue::Bits8(val1 ^ val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => RegisterValue::Bits16(val1 ^ val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}
impl BitAnd for RegisterValue {
    type Output = RegisterValue;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => RegisterValue::Bits8(val1 & val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => RegisterValue::Bits16(val1 & val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}
impl BitOr for RegisterValue{
    type Output = RegisterValue;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => RegisterValue::Bits8(val1 | val2),
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => RegisterValue::Bits16(val1 | val2),
            _ => panic!("Mismatched types for bitwise OR"),
        }
    }
}
impl From<u8> for RegisterValue{
    fn from(value: u8) -> Self {
        RegisterValue::Bits8(value)
    }
}

impl From<u16> for RegisterValue {
    fn from(value: u16) -> Self {
        RegisterValue::Bits16(value)
    }
}
impl From<RegisterValue> for u8{
    fn from(value: RegisterValue) -> Self {
        match value {
            RegisterValue::Bits8(value) => value,
            RegisterValue::Bits16(value) => value as u8,
        }
    }
}
impl From<RegisterValue> for u16{
    fn from(value: RegisterValue) -> Self {
        match value {
            RegisterValue::Bits16(value) => value,
            RegisterValue::Bits8(value) => value as u16
        }
    }
}
impl RegisterValue{
    pub fn check_add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (RegisterValue::Bits8(val1), RegisterValue::Bits8(val2)) => {
                // Check for overflow using wrapping_add
                if let Some(result) = val1.checked_add(*val2) {
                    Some(RegisterValue::Bits8(result))
                } else {
                    None // Addition overflowed
                }
            }
            (RegisterValue::Bits16(val1), RegisterValue::Bits16(val2)) => {
                // Check for overflow using wrapping_add
                if let Some(result) = val1.checked_add(*val2) {
                    Some(RegisterValue::Bits16(result))
                } else {
                    None // Addition overflowed
                }
            }
            _ => panic!("Attempt to add values of different types"),
        }
    }

}
impl Register {

    pub fn set_register_value(&mut self, value: impl Into<RegisterValue>){
        self.bits= value.into();
    }
    pub fn increment_register_value(&mut self, amount: impl Into<RegisterValue>){
        match amount.into() {
            RegisterValue::Bits8(value)=>{
                if let RegisterValue::Bits8(current) = &mut self.bits{
                    *current= current.wrapping_add(value)
                }
            }
            RegisterValue::Bits16(value)=>{
                if let RegisterValue::Bits16(current) = &mut self.bits{
                    *current= current.wrapping_add(value)
                }
            }
        }
    }
    pub fn decrement_register_value(&mut self, amount: impl Into<RegisterValue>){
        match amount.into() {
            RegisterValue::Bits8(value)=>{
                if let RegisterValue::Bits8(current) = &mut self.bits{
                    *current= current.wrapping_sub(value)
                }
            }
            RegisterValue::Bits16(value)=>{
                if let RegisterValue::Bits16(current) = &mut self.bits{
                    *current= current.wrapping_sub(value)
                }
            }
        }
    }

    pub fn get_register_value(&self)-> RegisterValue{
        return self.bits.clone();
    }

}