use crate::parser::{ParserError, Result, TrameBuffer};
use std::convert::{TryFrom, TryInto};
use std::ops::Mul;
use std::{
    fmt,
    fmt::{Binary, Debug, Display, Formatter, LowerHex},
};

#[derive(Clone, Copy)]
pub struct Type<T, const BITS: u8, const PRINT: u8>(pub T);

pub type Type8<const BITS: u8, const PRINT: u8> = Type<u8, BITS, PRINT>;
pub type Type16<const BITS: u8, const PRINT: u8> = Type<u16, BITS, PRINT>;
pub type Type32<const BITS: u8, const PRINT: u8> = Type<u32, BITS, PRINT>;
pub type Type64<const BITS: u8, const PRINT: u8> = Type<u64, BITS, PRINT>;







#[derive(Clone, Copy)]
pub struct TypeProduct<T, const BITS: u8, const PRODUCT: u8>(Type<T, BITS, 0>);

pub type TypeProduct8<const BITS: u8, const PRODUCT: u8> = TypeProduct<u8, BITS, PRODUCT>;
pub type TypeProduct16<const BITS: u8, const PRODUCT: u8> = TypeProduct<u16, BITS, PRODUCT>;
pub type TypeProduct32<const BITS: u8, const PRODUCT: u8> = TypeProduct<u32, BITS, PRODUCT>;
pub type TypeProduct64<const BITS: u8, const PRODUCT: u8> = TypeProduct<u64, BITS, PRODUCT>;













impl<T: TryFrom<u128>, const BITS: u8, const PRINT: u8> TryFrom<&mut TrameBuffer>
    for Type<T, BITS, PRINT>
where
    <T as TryFrom<u128>>::Error: Debug,
{
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<Type<T, BITS, PRINT>> {
        Ok(Type(T::try_from(trame.get_bits(BITS)?).unwrap()))
    }
}

impl<T: TryFrom<u128> + Display + Binary + LowerHex, const BITS: u8, const PRINT: u8> Debug
    for Type<T, BITS, PRINT>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match PRINT {
            0 => write!(f, "{}", self.0),
            1 => match BITS {
                1 => write!(f, "0b{:b}", self.0),
                2 => write!(f, "0b{:02b}", self.0),
                3 => write!(f, "0b{:03b}", self.0),
                4 => write!(f, "0x{:x}", self.0),
                5 => write!(f, "0b{:05b}", self.0),
                6 => write!(f, "0b{:06b}", self.0),
                7 => write!(f, "0b{:07b}", self.0),
                8 => write!(f, "0x{:02x}", self.0),
                9 => write!(f, "0b{:09b}", self.0),
                10 => write!(f, "0b{:010b}", self.0),
                11 => write!(f, "0b{:011b}", self.0),
                12 => write!(f, "0x{:03x}", self.0),
                13 => write!(f, "0b{:013b}", self.0),
                14 => write!(f, "0b{:014b}", self.0),
                15 => write!(f, "0b{:015b}", self.0),
                16 => write!(f, "0x{:04x}", self.0),
                _ => write!(f, "0x{:x}", self.0),
            },
            2 => match BITS {
                1 => write!(f, "0b{:b} ({})", self.0, self.0),
                2 => write!(f, "0b{:02b} ({})", self.0, self.0),
                3 => write!(f, "0b{:03b} ({})", self.0, self.0),
                4 => write!(f, "0x{:x} ({})", self.0, self.0),
                5 => write!(f, "0b{:05b} ({})", self.0, self.0),
                6 => write!(f, "0b{:06b} ({})", self.0, self.0),
                7 => write!(f, "0b{:07b} ({})", self.0, self.0),
                8 => write!(f, "0x{:02x} ({})", self.0, self.0),
                9 => write!(f, "0b{:09b} ({})", self.0, self.0),
                10 => write!(f, "0b{:010b} ({})", self.0, self.0),
                11 => write!(f, "0b{:011b} ({})", self.0, self.0),
                12 => write!(f, "0x{:03x} ({})", self.0, self.0),
                13 => write!(f, "0b{:013b} ({})", self.0, self.0),
                14 => write!(f, "0b{:014b} ({})", self.0, self.0),
                15 => write!(f, "0b{:015b} ({})", self.0, self.0),
                16 => write!(f, "0x{:04x} ({})", self.0, self.0),
                _ => write!(f, "0x{:x} ({})", self.0, self.0),
            },
            3 => write!(f, "{:02x}", self.0),
            _ => unimplemented!(),
        }
    }
}

impl<T: TryFrom<u128>, const BITS: u8, const PRODUCT: u8> TryFrom<&mut TrameBuffer>
    for TypeProduct<T, BITS, PRODUCT>
where
    <T as TryFrom<u128>>::Error: Debug,
{
    type Error = ParserError;
    fn try_from(trame: &mut TrameBuffer) -> Result<TypeProduct<T, BITS, PRODUCT>> {
        Ok(TypeProduct(trame.try_into()?))
    }
}

impl<
        T: Copy + Mul<T, Output = T> + TryFrom<u128> + From<u8> + Display + Binary + LowerHex,
        const BITS: u8,
        const PRODUCT: u8,
    > Debug for TypeProduct<T, BITS, PRODUCT>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.0 .0 * PRODUCT.into(), self.0 .0)
    }
}

impl<T: Copy + Mul<T, Output = T> + From<u8>, const BITS: u8, const PRODUCT: u8>
    TypeProduct<T, BITS, PRODUCT>
{
    pub fn product(&self) -> T {
        self.0 .0 * PRODUCT.into()
    }
}
