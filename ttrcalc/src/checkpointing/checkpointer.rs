use crate::task_system::Checkpointer;
use crate::Board;
use crate::GameState;
use crate::Work;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use super::array_writing::WriteableArray;

pub struct MostPointCheckpointer<'a> {
    board: &'a Board,
}
impl<'a> MostPointCheckpointer<'a> {
    pub fn new(board: &'a Board) -> MostPointCheckpointer {
        return MostPointCheckpointer { board: board };
    }
}

fn write_gamestate<T: Write>(writer: &mut T, work: &GameState) -> std::io::Result<()> {
    writer.write_u32::<BigEndian>(work.remaining_trains())?;
    work.built_routes().write_array(writer)?;
    work.cities_reached().write_array(writer)?;
    work.routes_to_ignore().write_array(writer)?;
    return Ok(());
}

fn write_board<T: Write>(writer: &mut T, board: &Board) -> std::io::Result<()> {
    writer.write_u32::<BigEndian>(board.nr_trains())?;
    board.city_names().write_array(writer)?;
    board.routes().write_array(writer)?;
    Ok(())
}

impl<'a> Checkpointer<Work<'a>> for MostPointCheckpointer<'a> {
    fn checkpoint(&self, work: &Vec<Work<'a>>) {
        println!("Checkpointing: {} tasks queued.", work.len());
        let mut writer = BufWriter::new(File::create("checkpoint").unwrap());
        // magic : dac0
        // version: 01 (major) 00 (minor)
        writer.write_u32::<BigEndian>(0xdaca0100).unwrap();
        // board magic marker
        writer.write_u32::<BigEndian>(0x626F7264).unwrap();
        // board
        write_board(&mut writer, &self.board).unwrap();
        // work magic marker
        writer.write_u32::<BigEndian>(0x71756575).unwrap();
        // len of work items
        writer.write_u32::<BigEndian>(work.len() as u32).unwrap();
        // list of work items
        for workitem in work {
            let (header_byte, gamestate) = match workitem {
                Work::Start => (1, None),
                Work::Explore(s) => (2, Some(s)),
                Work::EvaluateScore(s) => (3, Some(s)),
            };

            // work item header/enum value
            writer.write_u8(header_byte).unwrap();
            // work item
            if let Some(gamestate) = gamestate {
                write_gamestate(&mut writer, gamestate).unwrap();
            }
        }
        println!("Checkpoint finished");
    }
}
