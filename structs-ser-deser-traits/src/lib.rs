use std::io;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub trait Ser {
    fn ser(&self, buf: impl io::Write) -> io::Result<()>;
    fn ser_len(&self) -> usize;
}

pub trait Deser: Sized {
    fn deser(buf: impl io::Read) -> io::Result<Self>;
}

macro_rules! ser_deser_basic_types_generator {
    ($read_func:ident, $write_func:ident, $basic_type:ty) => {
        impl Ser for $basic_type {
            fn ser(&self, mut buf: impl io::Write) -> io::Result<()> {
                buf.$write_func::<LittleEndian>(*self)
            }
            fn ser_len(&self) -> usize {
                std::mem::size_of::<$basic_type>()
            }
        }

        impl Deser for $basic_type {
            fn deser(mut buf: impl io::Read) -> io::Result<Self> {
                buf.$read_func::<LittleEndian>()
            }
        }
    };
}

macro_rules! ser_deser_without_align_generator {
    ($read_func:ident, $write_func:ident, $basic_type:ty) => {
        impl Ser for $basic_type {
            fn ser(&self, mut buf: impl io::Write) -> io::Result<()> {
                buf.$write_func(*self)
            }
            fn ser_len(&self) -> usize {
                std::mem::size_of::<$basic_type>()
            }
        }

        impl Deser for $basic_type {
            fn deser(mut buf: impl io::Read) -> io::Result<Self> {
                buf.$read_func()
            }
        }
    };
}

ser_deser_without_align_generator!(read_i8, write_i8, i8);
ser_deser_without_align_generator!(read_u8, write_u8, u8);

ser_deser_basic_types_generator!(read_i16, write_i16, i16);
ser_deser_basic_types_generator!(read_i32, write_i32, i32);
ser_deser_basic_types_generator!(read_i64, write_i64, i64);
ser_deser_basic_types_generator!(read_i128, write_i128, i128);

ser_deser_basic_types_generator!(read_u16, write_u16, u16);
ser_deser_basic_types_generator!(read_u32, write_u32, u32);
ser_deser_basic_types_generator!(read_u64, write_u64, u64);
ser_deser_basic_types_generator!(read_u128, write_u128, u128);

ser_deser_basic_types_generator!(read_f32, write_f32, f32);
ser_deser_basic_types_generator!(read_f64, write_f64, f64);

impl Ser for String {
    fn ser(&self, mut buf: impl io::Write) -> io::Result<()> {
        buf.write_u64::<LittleEndian>(self.len() as u64)?;
        buf.write_all(self.as_ref())
    }
    fn ser_len(&self) -> usize {
        std::mem::size_of::<u64>() + self.len()
    }
}

impl Deser for String {
    fn deser(mut buf: impl io::Read) -> io::Result<Self> {
        let len = buf.read_u64::<LittleEndian>()? as usize;

        let mut result = vec![0x00; len];
        buf.read_exact(&mut result)?;

        Ok(String::from_utf8(result)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Utf8 error: {}", e)))?)
    }
}

impl Ser for bool {
    fn ser(&self, mut buf: impl io::Write) -> io::Result<()> {
        match self {
            true => buf.write_u8(1),
            false => buf.write_u8(0),
        }
    }
    fn ser_len(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

impl Deser for bool {
    fn deser(mut buf: impl io::Read) -> io::Result<Self> {
        match buf.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Got invalid bool representation",
            )),
        }
    }
}
