use crate::components::objects::Prelude::*;
use crate::parser::*;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Data(usize, Vec<Type8<8, 1>>);

impl Data {
    pub fn new(data_bytes: Vec<Type8<8, 1>>) -> Data {
        Data(data_bytes.len(), data_bytes)
    }

    pub fn get_data(&self) -> &Vec<Type8<8, 1>> {
        &self.1
    }

    pub fn get_size(&self) -> usize {
        self.0
    }
}

impl TryFrom<&mut TrameBuffer> for Data {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<Data> {
        let length: Type8<8, 0> = trame.try_into()?;
        let mut data = Data(length.0 as usize, Vec::new());

        for _ in 0..data.0 - 2 {
            data.1.push(trame.try_into()?);
        }

        Ok(data)
    }
}

impl TryFrom<(usize, &mut TrameBuffer)> for Data {
    type Error = ParserError;
    fn try_from(size_and_trame: (usize, &mut TrameBuffer)) -> Result<Data> {
        let mut data = Data(size_and_trame.0, Vec::new());

        for _ in 0..data.0 {
            data.1.push(size_and_trame.1.try_into()?);
        }

        Ok(data)
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data_strings = Vec::new();

        let mut bytes_type = self.1.iter();
        for _ in 0.. {
            let mut format_string = String::new();
            for i in 0..40 {
                match bytes_type.next() {
                    Some(data_type) => {
                        if i != 0 {
                            format_string.push(' ');
                        }
                        format_string.push_str(&format!("{:02x}", data_type.0));
                    }
                    None => break,
                }
            }
            if format_string.is_empty() {
                break;
            } else {
                data_strings.push(format_string);
            }
        }
        data_strings.fmt(f)
    }
}
