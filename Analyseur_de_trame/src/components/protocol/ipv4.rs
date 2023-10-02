use crate::components::objects::options::id::*;
use crate::components::objects::Prelude::*;
use super::Prelude::*;
use std::convert::{TryFrom, TryInto};
use crate::parser::*;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct IPv4 {
    Version: Type8<4, 0>,
    HeaderLength: TypeProduct8<4, 4>,
    DifferentiatedServicesField: DSF,
    TotalLength: Type16<16, 0>,
    Identification: Type16<16, 2>,
    Flags: IpFlags,
    FragmentOffset: Type16<13, 0>,
    TimeToLive: Type8<8, 0>,
    Protocol: Type8<8, 2>,
    HeaderChecksum: Type16<16, 1>,
    Source: IpAddress,
    Destination: IpAddress,
    Options: Vec<IpOption>,
    Tcp: Option<TCP>,
    ProtocolData: Option<UntrackedProtocol>,
}

impl Debug for IPv4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("");
        let format_tmp = debug_struct
            .field("IP Version", &self.Version)
            .field("Header Length", &self.HeaderLength)
            .field(
                "Differentiated Services Field",
                &self.DifferentiatedServicesField,
            )
            .field("Total Length", &self.TotalLength)
            .field("Identification", &self.Identification)
            .field("Flags", &self.Flags)
            .field("Fragment Offset", &self.FragmentOffset)
            .field("Time to Live", &self.TimeToLive)
            .field("Protocol", &self.Protocol)
            .field(
                "Protocol Description",
                &get_ip_protocol_description(self.Protocol.0),
            )
            .field("Header Checksum", &self.HeaderChecksum)
            .field("Source", &self.Source)
            .field("Destination", &self.Destination)
            .field("IP Options", &self.Options);

        match (&self.Tcp, &self.ProtocolData) {
            (Some(tcp), None) => format_tmp.field("TCP", tcp).finish(),
            (None, Some(protocol)) => format_tmp.field("Untracked Protocol", protocol).finish(),
            (None, None) => format_tmp.finish(),
            _ => unreachable!(),
        }
    }
}

//fonction de création d'une structure IPv4 à partir du parser
impl TryFrom<&mut TrameBuffer> for IPv4 {
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<IPv4> {
        let mut ipv4 = IPv4 {
            Version: trame.try_into()?,
            HeaderLength: trame.try_into()?,
            DifferentiatedServicesField: trame.try_into()?,
            TotalLength: trame.try_into()?,
            Identification: trame.try_into()?,
            Flags: trame.try_into()?,
            FragmentOffset: trame.try_into()?,
            TimeToLive: trame.try_into()?,
            Protocol: trame.try_into()?,
            HeaderChecksum: trame.try_into()?,
            Source: trame.try_into()?,
            Destination: trame.try_into()?,
            Options: Vec::new(),
            Tcp: None,
            ProtocolData: None,
        };

        let mut remaining_bytes = ipv4.HeaderLength.product() as usize - 20;

        while remaining_bytes > 0 {
            let option = IpOption::new(trame)?;
            remaining_bytes = remaining_bytes - option.get_size();
            ipv4.Options.push(option);
        }

        let data_bytes = ipv4.TotalLength.0 as usize - ipv4.HeaderLength.product() as usize;

        if ipv4.Protocol.0 == 6 {
            ipv4.Tcp = Some((data_bytes, trame).try_into()?);
        } else {
            ipv4.ProtocolData = Some((data_bytes, trame).try_into()?);
        }

        Ok(ipv4)
    }
}