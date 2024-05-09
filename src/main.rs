#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]

use std::{
    io
};

mod packet;
mod cap;
use crate::cap::Cap;

// fn print_packet(raw_packet: &[u8]) {
//     println!("Ethernet (");
//     println!("\tDstMac: {:X}:{:X}:{:X}:{:X}:{:X}:{:X}", raw_packet[0], raw_packet[1], raw_packet[2], raw_packet[3], raw_packet[4], raw_packet[5]);
//     println!("\tSrcMac: {:X}:{:X}:{:X}:{:X}:{:X}:{:X}", raw_packet[6], raw_packet[7], raw_packet[8], raw_packet[9], raw_packet[10], raw_packet[11]);
//     let raw_proto: &[u8; 2] = &[raw_packet[12], raw_packet[13]];
//     let proto = u16::from_le_bytes(*raw_proto);
//     println!("\tProto: {}", proto);
//     println!(")");

//     if proto == 8 {
//         let raw_totallen: &[u8; 2] = &[raw_packet[16], raw_packet[17]];
//         let totallen = u16::from_le_bytes(*raw_totallen);
//         let raw_id: &[u8; 2] = &[raw_packet[18], raw_packet[19]];
//         let id = u16::from_le_bytes(*raw_id);
//         let raw_flags: &[u8; 2] = &[raw_packet[20], raw_packet[21]];
//         let flags = u16::from_le_bytes(*raw_flags);
//         let raw_checksum: &[u8; 2] = &[raw_packet[24], raw_packet[25]];
//         let checksum = u16::from_le_bytes(*raw_checksum);
//         println!("IP (");
//         println!("\tVersionAndIHL: {}", raw_packet[14]);
//         println!("\tTypeOfService: {}", raw_packet[15]);
//         println!("\tTotalLength: {}", totallen);
//         println!("\tID: {}", id);
//         println!("\tFlags: {}", flags);
//         println!("\tTTL: {}", raw_packet[22]);
//         println!("\tProtocol: {}", raw_packet[23]);
//         println!("\tCheckSum: {}", checksum);
//         println!("\tSrcIP: {}.{}.{}.{}", raw_packet[26], raw_packet[27], raw_packet[28], raw_packet[29]);
//         println!("\tDstIP: {}.{}.{}.{}", raw_packet[30], raw_packet[31], raw_packet[32], raw_packet[33]);
//         println!(")");
//     }
// }

fn main() -> io::Result<()> {

    let mut sniff: Cap = Cap::from("wlp2s0");
    sniff.recv_packets()?;

    // packet::print_rawpacket(&mut rawpacket);

    Ok(())
}
