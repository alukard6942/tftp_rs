/**
 * File: client.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */

use std::net::{UdpSocket, self, IpAddr};

#[derive(Debug)]
pub struct Client {

    socket: UdpSocket,
}

impl Client {

    pub fn new(addres: &str) -> Self {
        let soc = UdpSocket::bind(addres).expect("could not bind addres");

        Self {
            socket: soc,
        }
    }

    pub fn send_pack(&mut self, pack: Vec<u8>){
        self.socket.send(&pack);
    }

    pub fn recv_pack(&mut self) -> Vec<u8>{
        let mut buff = Vec::new();
        buff.reserve(512);

        let (bytes, addres) = self.socket.recv_from(&mut buff).expect("didnt recv");

        buff
    }
}

