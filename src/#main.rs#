#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]

use std::{
    io
};

mod packet;
mod cap;
use crate::cap::Cap;

fn main() -> io::Result<()> {

    let mut sniff: Cap = Cap::from("wlp2s0");
    sniff.recv_packets()?;

    // packet::print_rawpacket(&mut rawpacket);

    Ok(())
}
