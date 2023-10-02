use crate::components::objects::Prelude::*;
use std::convert::{TryFrom, TryInto};
use crate::parser::*;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct TCP {
    SourcePort: Type16<16, 0>,
    DestinationPort: Type16<16, 0>,
    SequenceNumber: Type32<32, 0>,
    AknowledgementNumber: Type32<32, 0>,
    DataOffset: TypeProduct8<4, 4>,
    Reserved: Type8<3, 1>,
    Flags: TcpFlags,
    WindowSize: Type16<16, 0>,
    Checksum: Type16<16, 1>,
    UrgentPointer: Type16<16, 0>,
    Options: Vec<TcpOption>,
    Data: Option<Data>,
}

#[derive(Debug)]
pub struct UntrackedProtocol(Data);

impl TryFrom<(usize, &mut TrameBuffer)> for TCP {
    type Error = ParserError;
    fn try_from(size_and_trame: (usize, &mut TrameBuffer)) -> Result<TCP> {
        let (size, trame) = size_and_trame;

        let mut tcp = TCP {
            SourcePort: trame.try_into()?,
            DestinationPort: trame.try_into()?,
            SequenceNumber: trame.try_into()?,
            AknowledgementNumber: trame.try_into()?,
            DataOffset: trame.try_into()?,
            Reserved: trame.try_into()?,
            Flags: trame.try_into()?,
            WindowSize: trame.try_into()?,
            Checksum: trame.try_into()?,
            UrgentPointer: trame.try_into()?,
            Options: Vec::new(),
            Data: None,
        };

        let mut remaining_bytes = tcp.DataOffset.product() as usize - 20;

        while remaining_bytes > 0 {
            let option = TcpOption::new(trame)?;
            remaining_bytes = remaining_bytes - option.get_size();
            tcp.Options.push(option);
        }

        if size - tcp.DataOffset.product() as usize > 0 {
            tcp.Data = Some((size - tcp.DataOffset.product() as usize, trame).try_into()?);
        }

        Ok(tcp)
    }
}

impl TryFrom<(usize, &mut TrameBuffer)> for UntrackedProtocol {
    type Error = ParserError;
    fn try_from(size_and_trame: (usize, &mut TrameBuffer)) -> Result<UntrackedProtocol> {
        Ok(UntrackedProtocol(size_and_trame.try_into()?))
    }
}

impl Debug for TCP {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("");
        debug_struct
            .field("Source Port", &self.SourcePort)
            .field("Destination Port", &self.DestinationPort)
            .field("Sequence Number", &self.SequenceNumber)
            .field("Aknowledgement Number", &self.AknowledgementNumber)
            .field("Data Offset", &self.DataOffset)
            .field("Reserved", &self.Reserved)
            .field("Flags", &self.Flags)
            .field("Window Size", &self.WindowSize)
            .field("Checksum", &self.Checksum)
            .field("Urgent Pointer", &self.UrgentPointer)
            .field("Options", &self.Options);

        match &self.Data {
            Some(data) => {
                let bytes_data = data
                    .get_data()
                    .iter()
                    .take_while(|data_bytes| data_bytes.0 <= 127)
                    .map(|data_bytes| data_bytes.0 as char)
                    .collect::<String>();

                let strings_vec = bytes_data
                    .split("\r\n")
                    .collect::<Vec<_>>();

                if strings_vec.len() > 2 {
                    debug_struct.field(
                        "Data string",
                        &&strings_vec[..strings_vec.len() - 2]
                    );
                }

                let data_remaining = Data::new(Vec::from(
                    &AsRef::<[Type8<8, 1>]>::as_ref(data.get_data())[bytes_data.len() - strings_vec[strings_vec.len() - 1].len()..],
                ));

                if data_remaining.get_data().len() > 0 {
                    debug_struct.field("Data bytes", &data_remaining);
                }
                debug_struct.finish()
            }
            None => debug_struct.finish(),
        }
    }
}
