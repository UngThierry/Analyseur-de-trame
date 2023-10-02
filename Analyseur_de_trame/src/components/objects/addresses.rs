use crate::components::objects::Prelude::*;
use crate::parser::*;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Debug, Formatter};








pub struct MacAddress(Type64<48, 1>);

pub struct IpAddress(Type32<32, 0>);









impl TryFrom<&mut TrameBuffer> for MacAddress {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<MacAddress> {
        Ok(MacAddress(trame.try_into()?))
    }
}

impl TryFrom<&mut TrameBuffer> for IpAddress {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<IpAddress> {
        Ok(IpAddress(trame.try_into()?))
    }
}

impl Debug for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0 .0 >> 40,
            (self.0 .0 >> 32) & 0xFF,
            (self.0 .0 >> 24) & 0xFF,
            (self.0 .0 >> 16) & 0xFF,
            (self.0 .0 >> 8) & 0xFF,
            self.0 .0 & 0xFF
        )
    }
}

impl Debug for IpAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.0 .0 >> 24,
            (self.0 .0 >> 16) & 0xFF,
            (self.0 .0 >> 8) & 0xFF,
            self.0 .0 & 0xFF
        )
    }
}
