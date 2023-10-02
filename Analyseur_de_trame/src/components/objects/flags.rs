use crate::components::objects::Prelude::*;
use crate::parser::*;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Debug, Formatter};




fn is_set(bit: u8) -> &'static str {
    match bit {
        0 => "Not set",
        1 => "Set",
        _ => unreachable!(),
    }
}





pub struct IpFlags(Type8<3, 0>);

pub struct TcpFlags(Type16<9, 0>);

pub struct DSF(Type8<8, 0>);








impl TryFrom<&mut TrameBuffer> for IpFlags {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<IpFlags> {
        Ok(IpFlags(trame.try_into()?))
    }
}

impl TryFrom<&mut TrameBuffer> for TcpFlags {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<TcpFlags> {
        Ok(TcpFlags(trame.try_into()?))
    }
}

impl TryFrom<&mut TrameBuffer> for DSF {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<DSF> {
        Ok(DSF(trame.try_into()?))
    }
}

impl Debug for IpFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("Reserved Bit", &is_set(self.0 .0 >> 2))
            .field("Don't Fragment", &is_set((self.0 .0 >> 1) & 1))
            .field("More Fragments", &is_set(self.0 .0 & 1))
            .finish()
    }
}

impl Debug for TcpFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("Nonce", &is_set((self.0 .0 >> 8) as u8))
            .field("Congestion Window Reduced", &is_set(((self.0 .0 >> 7) & 1) as u8))
            .field("ECN-Echo", &is_set(((self.0 .0 >> 6) & 1) as u8))
            .field("Urgent", &is_set(((self.0 .0 >> 5) & 1) as u8))
            .field("Acknowledgment", &is_set(((self.0 .0 >> 4) & 1) as u8))
            .field("Push", &is_set(((self.0 .0 >> 3) & 1) as u8))
            .field("Reset", &is_set(((self.0 .0 >> 2) & 1) as u8))
            .field("Synchronize", &is_set(((self.0 .0 >> 1) & 1) as u8))
            .field("Fin", &is_set(((self.0 .0 >> 0) & 1) as u8))
            .finish()
    }
}

impl Debug for DSF {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field(
                "Differentiated Services Codepoint",
                &Type::<u8, 6, 0>(self.0 .0 >> 2),
            )
            .field(
                "Explicit Congestion Notification",
                &Type::<u8, 2, 0>(self.0 .0 & 0b11),
            )
            .finish()
    }
}
