
// * ? < > | " : / \
const NAME_ILLEGAL_BYTE: [u8; 9] = [0x2a,0x3f,0x3c,0x3e,0x7c,0x22,0x3a,0x2f,0x5c];

pub const NAME_MAX: usize = 255;
pub struct Name([u8; NAME_MAX]);

impl Name {}


#[inline]
pub fn is_illegal_byte(byte:u8)->bool{
    if NAME_ILLEGAL_BYTE.iter().position(|&x| x==byte).is_some(){
        true
    }else{
        false
    }
}
