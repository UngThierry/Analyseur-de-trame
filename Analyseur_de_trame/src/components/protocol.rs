pub mod ethernet;
pub mod ipv4;
pub mod tcp;

pub mod Prelude {
    pub use super::ethernet::Ethernet2;
    pub use super::ipv4::IPv4;
    pub use super::tcp::{TCP, UntrackedProtocol};
}