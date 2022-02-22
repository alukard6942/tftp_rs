/**
 * File: agent.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */

use std::{net::{UdpSocket, SocketAddr, ToSocketAddrs}, io::{self, ErrorKind, Read, Write}, error::Error, time::Duration, fs::{self, File}, path::Path, os::unix::prelude::FileExt, str::FromStr};

use crate::packet::{self, pack_data};



#[derive(Debug)]
pub enum RW_mod {Read, Write, None}

#[derive(Debug)]
pub struct Agent {
    socket: UdpSocket,
    addr: SocketAddr,
    port: u16,
    
    pub file: Option<File>,
    pub RW_mod: RW_mod,
    pub packet: Vec<u8>,
    pub response: Vec<u8>,
}

impl Agent {
    

    /**
     * @brief consturctor agenta 
     * addres of corespondent is defautly set to localhost
     *
     * @return Agent
     */
    pub fn new() -> io::Result<Agent>{
        
        let port_range = 1024.. 49151;
        
        let mut socket = None;
        // loop from 1024 t
        
        let mut port = 0;
        for p in port_range {
            // try binding socket
            match UdpSocket::bind(SocketAddr::from(([127,0,0,1], p))) {
                Ok(o) => socket = Some(o),
                Err(e) => continue,
            };

            port = p;
            break;
        }
        
        Ok(Agent{
            
            file: None,
            
            socket: match socket {
                None => return Err(io::Error::from(ErrorKind::AddrNotAvailable)),
                Some(o) => o,
            },
            
            addr: SocketAddr::from(([127,0,0,1], 69)),
            RW_mod: RW_mod::None,
            port: port,
            
            packet: Vec::new(),
            response: Vec::new(),
        })
    }
    
    pub fn server() -> io::Result<Agent>{

        let socket = UdpSocket::bind("127.0.0.1:69")?;
        
        Ok(Agent{
            file: None,
            socket: socket,
            addr: SocketAddr::from(([127,0,0,1], 69)),
            RW_mod: RW_mod::None,
            port: 69,
            packet: Vec::new(),
            response: Vec::new(),
        })
    }
    
    pub fn set_addr(&mut self, addr: &str ) -> io::Result<()> {
        
        self.addr = match SocketAddr::from_str(addr) {
            Ok(it) => it,
            Err(err) => return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, err.to_string())),
        };
        
        Ok(())
    }
    
    /**
     * @brief caller must insure that packt is large enough
     * size of packtes should not change only the last one shall be smaller
     *
     * @param self
     * @param str
     *
     * @return 
     */
    fn pack_data_read(&mut self) -> io::Result<()>{

        let len =  self.packet.len();

        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = &mut self.file.as_ref().unwrap().read( match self.packet.get_mut(4..){
                None => return Err(io::Error::last_os_error()),
                Some(o) => o,
            }
        )?;
        
        self.packet.truncate(*red_b + 4 + 1);

        Ok(())
    }
    
    fn pack_data_store(&mut self) -> io::Result<()>{
        
        let len =  self.response.len();
        
        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = &mut self.file.as_ref().unwrap().write( match self.response.get_mut(4..){
                None => return Err(io::Error::last_os_error()),
                Some(o) => o,
            }
        )?;
        
        self.response.truncate(*red_b + 4 +1);

        Ok(())
    }

    
    
    pub fn send_file(&mut self, path: &str) -> io::Result<()>{

        Err(io::Error::last_os_error())
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

#[test]
fn can_create_new(){
    let agent = Agent::new();
}

#[test]
fn communication_test() -> Result<(), Box<dyn Error>>{
    let mut client = Agent::new()?;
    let mut server = Agent::server()?;

    server.socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    
    use crate::packet::{Packet, self};
    client.packet = packet::pack_write("hello", "word");
    
    println!("sending {:?}",client);
    client.send_pack()?;

    println!("recving {:?}",server);
    server.recv_pack()?;
    
    return Ok(())
}

#[test]
fn can_create_succes() -> Result<(), Box<dyn Error>>{
    let agent = Agent::new()?;
    Ok(())
}

#[test]
fn readtopack() -> Result<(), Box<dyn Error>> {
    
    let mut packetare = Agent::new()?;
    
    packetare.RW_mod = RW_mod::Read;
    
    packetare.packet = packet::pack_data(4, 10);
    println!("init packet {:?}", packetare);
    println!("pack size {}", packetare.packet.len());
    
    packetare.file = Some(File::open("src/agent.rs")?);

    packetare.pack_data_read()?;
    
    println!("filled packet {:?}", packetare);
    println!("pack size {}", packetare.packet.len());

    Ok(())
}

#[test]
fn pack_write() -> Result<(), Box<dyn Error>> {
    
    let mut agent = Agent::new()?;
    
    agent.response = vec![0,0,0,0,   72, 69, 76, 76, 79,];
    
    agent.file = Some(File::create("/tmp/pack_write.log")?);
    
    agent.pack_data_store()?;
    
    Ok(())
}