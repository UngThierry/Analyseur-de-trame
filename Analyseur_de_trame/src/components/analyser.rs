#![allow(non_snake_case)]
use super::protocol::Prelude::*;

use crate::parser::{ParserError, TrameBuffer};

use std::fmt;

use std::convert::TryInto;

pub enum TrameError {
    ParserError(ParserError, usize, usize),
}

impl fmt::Debug for TrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrameError::ParserError(err, l, ch) => write!(f, "Erreur ligne {}, charactere {} : {:?}", l, ch + 1, err)
        }
    }
}

pub fn analyse(ref mut trame: TrameBuffer) -> Vec<Result<Ethernet2, TrameError>> {
    std::iter::from_fn(|| match trame.try_into() {
        Err(ParserError::EOF) => None,
        Err(err) => {
            let res = Some(Err(TrameError::ParserError(err, trame.get_line(), trame.get_line_offset())));
            trame.reset_offset();
            res
        },
        Ok(res) => {
            trame.reset_offset();
            Some(Ok(res))
        }
    })
    .collect()
}

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn read_trame(path: &str) {
    let save_path = path.replace("\\", "/");
    let save_path = match save_path.rsplitn(2, "/").nth(1) {
        Some(path) => path.to_string() + "/",
        None => "".to_string(),
    };
    let save_path = save_path + "trame_analyser.log";


    let file = File::open(path).expect("Impossible d'ouvrir le fichier choisi !");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let trame_buffer = TrameBuffer::new(Box::new(lines.map(|s| s.expect("Cannot read lines"))));

    let analyse = analyse(trame_buffer);

    println!("{:#?}", analyse);

    let other_file = File::create(save_path).expect("Impossible de créer le fichier trame_analyser.log");
    let mut buffer = BufWriter::new(other_file);
    buffer.write(format!("{:#?}", analyse).as_bytes()).expect("Impossible d'écrire dans le fichier trame_analyser.log");
}