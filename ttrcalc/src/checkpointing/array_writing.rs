use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use ttrmodels::Route;

pub trait WriteableArray {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

impl WriteableArray for Vec<bool> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let mut build_routes_idx = vec![];
        for idx in 0..self.len() {
            if self[idx] {
                build_routes_idx.push(idx);
            }
        }
        writer.write_u16::<BigEndian>(build_routes_idx.len() as u16)?;
        for item in build_routes_idx {
            writer.write_u16::<BigEndian>(item as u16)?;
        }
        return Ok(());
    }
}
impl WriteableArray for Vec<usize> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for &item in self {
            writer.write_u16::<BigEndian>(item as u16)?;
        }
        return Ok(());
    }
}

impl WriteableArray for Vec<Route> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for item in self {
            writer.write_u16::<BigEndian>(item.city1 as u16)?;
            writer.write_u16::<BigEndian>(item.city2 as u16)?;
            writer.write_u16::<BigEndian>(item.tracks_len as u16)?;
        }
        return Ok(());
    }
}

impl WriteableArray for Vec<String> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for item in self {
            let bytes = item.clone().into_bytes();
            writer.write_u16::<BigEndian>(bytes.len() as u16)?;
            writer.write(&bytes)?;
        }
        return Ok(());
    }
}
