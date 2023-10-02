use crate::components::objects::options::id::*;
use crate::components::objects::Prelude::*;
use crate::parser::*;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Formatter};
use ParserError::*;

pub mod id;






pub enum IpOption {
    EOOL,
    NOP,
    UntrackedOption(Type8<8, 2>, Data),
}

pub enum TcpOption {
    EOOL,
    NOP,
    UntrackedOption(Type8<8, 2>, Data),
}






impl IpOption {
    pub fn new(trame: &mut TrameBuffer) -> Result<IpOption> {
        use IpOption::*;

        let option_id: Type8<8, 2> = trame.try_into()?;
        if let Err(_) = is_ip_option_correct(option_id.0) {
            Err(EOF)
        } else {
            Ok(match option_id.0 & 0b1_1111 {
                0 => EOOL,
                1 => NOP,
                2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
                | 20 | 21 | 23 | 24 | 25 => UntrackedOption(option_id, trame.try_into()?),
                _ => unreachable!(),
            })
        }
    }

    pub fn get_size(&self) -> usize {
        use IpOption::*;

        match self {
            EOOL | NOP => 1,
            UntrackedOption(_, data) => data.get_size(),
        }
    }
}

impl TcpOption {
    pub fn new(trame: &mut TrameBuffer) -> Result<TcpOption> {
        use TcpOption::*;

        let option_id: Type8<8, 2> = trame.try_into()?;
        if let Err(_) = is_ip_option_correct(option_id.0) {
            Err(EOF)
        } else {
            Ok(match option_id.0 & 0b1_1111 {
                0 => EOOL,
                1 => NOP,
                2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
                | 20 | 21 | 23 | 24 | 25 => UntrackedOption(option_id, trame.try_into()?),
                _ => unreachable!(),
            })
        }
    }

    pub fn get_size(&self) -> usize {
        use TcpOption::*;

        match self {
            EOOL | NOP => 1,
            UntrackedOption(_, data) => data.get_size(),
        }
    }
}

impl Debug for IpOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use IpOption::*;

        match self {
            EOOL => write!(f, "End of Options List"),
            NOP => write!(f, "No Operation"),
            UntrackedOption(option_id, data) => f
                .debug_struct(get_ip_description(self))
                .field(
                    "copy",
                    &(match option_id.0 >> 7 {
                        0 => "Do not copy",
                        1 => "Copy",
                        _ => unreachable!(),
                    }),
                )
                .field(
                    "class",
                    &(match (option_id.0 >> 5) & 0b11 {
                        0 => "Control",
                        2 => "Debugging and Measurement",
                        _ => unreachable!(),
                    }),
                )
                .field("length", &data.get_size())
                .field("data", &data)
                .finish(),
        }
    }
}

impl Debug for TcpOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use TcpOption::*;
        match self {
            EOOL => write!(f, "End of Options List"),
            NOP => write!(f, "No Operation"),
            UntrackedOption(_, data) => f
                .debug_struct(get_tcp_description(self))
                .field("length", &data.get_size())
                .field("data", &data)
                .finish(),
        }
    }
}
