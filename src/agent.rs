/**
 * File: agent.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */

use std::{net::{UdpSocket, SocketAddr, ToSocketAddrs}, io::{self, ErrorKind}};



#[derive(Debug)]
pub enum RW_mod {Read, Write}

#[derive(Debug)]
pub struct Agent {
    socket: UdpSocket,
    addr: SocketAddr,
    RW_mod: RW_mod,
    
    packet: Vec<u8>,
    response: Vec<u8>,
}

impl Agent {

    pub fn client(addres: &str, RW_mod: RW_mod) -> io::Result<Agent> {
        for addr  in addres.to_socket_addrs()?{
            let s = UdpSocket::bind(addr);
            
            if s.is_err(){
                continue;
            }

            return Ok( Agent {
                socket: s.unwrap(),
                addr: addr,
                RW_mod: RW_mod,

                packet: Vec::new(),
                response: Vec::new(),
            });
        }
        
        Err(io::Error::new( ErrorKind::Other, "faild to bind with server" ))
    }
    
    pub fn server() -> io::Result< Agent> {

        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
    

    pub fn send_pack(&mut self) -> io::Result<usize> {
        Ok(self.socket.send_to(&self.packet, self.addr)?)
    }

    pub fn recv_pack(&mut self) -> io::Result<()>{
        let (_, add ) = self.socket.recv_from(&mut self.response)?;
        self.addr = add;

        Ok(())
    }
}