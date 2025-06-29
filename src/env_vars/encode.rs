use super::EnvironmentVariable;
use std::ops::Index;

pub(crate) enum EncodeAction {
    Default,
    Unexport,
    Unset,
}

impl EnvironmentVariable {
    pub(crate) fn encode(&self, action: EncodeAction) -> String {
        match action {
            EncodeAction::Unset => format!("unset {0}", self.name),
            _ => {
                let export = match action {
                    EncodeAction::Default => "x",
                    EncodeAction::Unexport => " +x",
                    _ => unreachable!(),
                };
                let value = ansi_c_encode(&self.value);
                let ansi_prefix = if value.contains('\\') { "$" } else { "" };
                format!("declare -g{export} {}={ansi_prefix}'{value}'", self.name)
            }
        }
    }
}

fn ansi_c_encode(input: &[u8]) -> String {
    let mut encoded = String::with_capacity(input.len() * 2);
    input.iter()
        .for_each(|&byte| {
            ENCODING_LUT[byte as usize].write(&mut encoded);
        });
    encoded
}

const ENCODING_LUT: [EncodedByte; u8::MAX as usize + 1] = encoding_lut();

const fn encoding_lut() -> [EncodedByte; u8::MAX as usize + 1] {
    let mut table = [EncodedByte::default(); u8::MAX as usize + 1];
    let mut i: usize = 0;
    while i <= u8::MAX as usize {
        table[i] = EncodedByte::new(i as u8);
        i += 1;
    }
    table
}

#[derive(Copy, Clone)]
struct EncodedByte {
    data: [u8; EncodedByte::SIZE],
}

impl EncodedByte {
    const SIZE: usize = 4;
    
    const fn new(byte: u8) -> Self {
        let encoded = match byte {
            0x20..=0x26 | 0x28..=0x5B | 0x5D..=0x7E => [byte, 0, 0, 0],
            0x0A => ['\\' as u8, 'n' as u8, 0, 0],
            0x0D => ['\\' as u8, 'r' as u8, 0, 0],
            0x09 => ['\\' as u8, 'h' as u8, 0, 0],
            0x0B => ['\\' as u8, 'v' as u8, 0, 0],
            0x0C => ['\\' as u8, 'f' as u8, 0, 0],
            0x08 => ['\\' as u8, 'b' as u8, 0, 0],
            0x07 => ['\\' as u8, 'a' as u8, 0, 0],
            0x27 => ['\\' as u8, '\'' as u8, 0, 0],
            0x5C => ['\\' as u8, '\\' as u8, 0, 0],
            _ => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                let hi = HEX[((byte >> 4) & 0xF) as usize];
                let lo = HEX[(byte & 0xF) as usize];
                ['\\' as u8, 'x' as u8, hi, lo]
            }
        };
        Self { data: encoded }
    }
    
    const fn default() -> Self {
        Self { data: [0; 4] }
    }
    
    fn iter(&self) -> EncodedByteIter {
        EncodedByteIter {
            data: &self,
            index: 0,
        }
    }

    fn write(&self, output: &mut String) {
        self.iter()
            .for_each(|byte| output.push(byte as char));
    }
}

impl Index<usize> for EncodedByte {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

struct EncodedByteIter<'a> {
    data: &'a EncodedByte,
    index: usize,
}

impl<'a> Iterator for EncodedByteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < EncodedByte::SIZE {
            let item = self.data[self.index];
            if item != 0 {
                self.index += 1;
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }
}
