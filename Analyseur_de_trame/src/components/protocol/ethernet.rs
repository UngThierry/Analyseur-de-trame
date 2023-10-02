use super::Prelude::*;
use crate::components::objects::options::id::*;
use crate::components::objects::Prelude::*;
use crate::parser::*;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Ethernet2 {
    Destination: MacAddress,
    Source: MacAddress,
    Type: Type16<16, 1>,
    IPv4: Option<IPv4>,
}

impl TryFrom<&mut TrameBuffer> for Ethernet2 {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<Ethernet2> {
        let mut ethernet = Ethernet2 {
            Destination: trame.try_into()?,
            Source: match trame.try_into() {
                Err(ParserError::EOF) => return Err(ParserError::EOFWhileParsingTrame),
                res => res?,
            },
            Type: match trame.try_into() {
                Err(ParserError::EOF) => return Err(ParserError::EOFWhileParsingTrame),
                res => res?,
            },
            IPv4: None,
        };

        if ethernet.Type.0 == 0x0800 {
            ethernet.IPv4 = Some(match trame.try_into() {
                Err(ParserError::EOF) => return Err(ParserError::EOFWhileParsingTrame),
                res => res?,
            });
        }
        Ok(ethernet)
    }
}

impl Debug for Ethernet2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Ethernet2");
        let next = debug_struct
            .field("Destination", &self.Destination)
            .field("Source", &self.Source)
            .field("Type", &self.Type)
            .field("Type Description", &get_ethernet_type(self.Type.0));

        match self.IPv4 {
            Some(ref ipv4) => next.field("IPv4", ipv4).finish(),
            None => next.finish(),
        }
    }
}
