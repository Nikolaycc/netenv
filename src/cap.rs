use std::{
    mem::{MaybeUninit},
    os::fd::AsRawFd,
    io
};

use socket2::{Socket, Domain, Type};
use libc;

use super::packet;

pub struct Cap {
    buff: [MaybeUninit<u8>; 1024],
    socket: Socket,
    ifa: String,
}

impl Cap {
    pub fn from<S: Into<String>>(ifa: S) -> Self {
        Self {
            buff: MaybeUninit::uninit_array(),
            socket: Socket::new_raw(Domain::from(libc::AF_PACKET), Type::from(libc::SOCK_RAW), Some(768.into())).unwrap(),
            ifa: ifa.into(),
        }
    }

    pub fn recv_packets(&mut self) -> io::Result<()> {
        unsafe { libc::setsockopt(self.socket.as_raw_fd(), libc::SOL_SOCKET, libc::SO_BINDTODEVICE, self.ifa.as_str().as_ptr() as *const u8 as *const libc::c_void, self.ifa.len().try_into().unwrap()) } ;

        loop {
            let (size, _addr) = self.socket.recv_from(&mut self.buff).unwrap();
            let raw = unsafe {
                MaybeUninit::array_assume_init(self.buff)
            };
            let packet = &raw[..size];

            let mut rawpacket: packet::RawPacket = packet::RawPacket::from(&packet);
            packet::print_rawpacket(&mut rawpacket);
            if size == 0 {
                break;
            }
        }

        Ok(())
    }
}
