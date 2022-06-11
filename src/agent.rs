/**
 * File: agent.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 12.04.2022
 */


#[ path ="./error.rs" ]
mod error;
use error::{TftpError, Result};

#[ path = "./packet.rs" ]
mod packet;
use packet::{pack_ack, pack_data, Packet};


use std::{
    io::{Read, Write},
    net::{ToSocketAddrs, UdpSocket},
    time::Duration,
};

use std::fs::File;
use std::net::SocketAddr;

use crate::log;


#[derive(Debug, Clone)]
pub enum RwMod {
    Read,
    Write,
    None,
}

#[derive(Debug)]
pub struct Agent {

    pub packlen: usize,

    failed_tryes_count: u16,
    socket: UdpSocket,
    addr: SocketAddr,
    pub file: File,
    pub rw_mod: RwMod,
    pub packet: Vec<u8>,
    pub response: Vec<u8>,
}

impl Agent {
    /**
     * @brief creates the initial configuration of client
     *
     * @param str addres and port "addr:port"
     * @param rw_mod (read or write)
     * @param str (name of trasfering file) "example.txt"
     *
     *
     *
     * @return Agent
     */
    pub fn client(addr: &impl ToSocketAddrs, rw_mod: RwMod, filename: &str) -> Result<Agent> {
        // let len = 512;
        let len = 512;

        use packet::{pack_read, pack_write};

        // assuming 'localhost' resolves to 127.0.0.1
        let addrs = addr.to_socket_addrs()?.next().unwrap();

        Ok(Agent {
            packlen: len,
            failed_tryes_count: 0,

            // vildcard bind to localhost random port
            socket: {
                let soc = UdpSocket::bind("[::]:0")?;
                soc.set_read_timeout(Some(Duration::from_secs(5)))?;
                soc
            },

            addr: addrs,

            file: match rw_mod.clone() {
                RwMod::Read  => File::create(filename)?,
                RwMod::Write => File::open(filename)?,
                RwMod::None  => return Err(TftpError::InvalidData),
            },

            rw_mod: rw_mod.clone(),

            packet: match rw_mod.clone() {
                RwMod::Read  => pack_read(filename, "octet"),
                RwMod::Write => pack_write(filename, "octet"),
                RwMod::None  => return Err(TftpError::InvalidMode),
            },

            response: match rw_mod {
                RwMod::Read  => pack_ack(1),
                RwMod::Write => pack_data(1, len),
                RwMod::None  => return Err(TftpError::InvalidMode),
            },
        })
    }

    pub fn server(packet: &mut Vec<u8>, addr: SocketAddr) -> Result<(Agent, RwMod)> {

        let len = 512;

        let filename = {
            let mut filename = String::new();
            for c in packet.get(2..).expect("packet too short"){
                if *c == 0 as u8 {
                    break;
                }

                // seams too convoluted
                filename.push(char::from(*c));
            }

            filename
        };
        
        let mode = {
            match packet.opcode() {
                1 => RwMod::Write,
                2 => RwMod::Read,
                _ => RwMod::None,
            }
        };

        log!("{addr} reqvests {:?} {filename}", mode.clone());

        packet.resize(len +4, 0);
        let mut buff2: Vec<u8> = Vec::new();
        let (pout, pin) = match mode.clone() {
                RwMod::Read => (packet, &mut buff2),
                RwMod::Write => (&mut buff2, packet),
                RwMod::None  => return Err(TftpError::InvalidMode),
        };

        Ok((Agent {
            packlen: len,
            failed_tryes_count: 0,
            socket: {
                let soc = UdpSocket::bind("[::]:0")?;
                soc.set_read_timeout(Some(Duration::from_secs(5)))?;
                soc
            },
            //bind
            addr: addr.to_owned(),
            file: match mode.clone() {
                    RwMod::Read  => File::open(filename)?,
                    RwMod::Write => File::create(filename)?,
                    RwMod::None  => return Err(TftpError::InvalidMode),
            },
            rw_mod: mode.clone(),
            packet: pout.to_vec() ,
            response: pin.to_vec(),
        }, mode))
    }

    pub fn send_header(&mut self) -> Result<()> {
        self.socket.send_to(&self.packet, &self.addr)?;
        Ok(())
    }

    pub fn client_set_addr(&mut self) -> Result<(usize, SocketAddr)> {
            let (size, add) = match self.socket.recv_from(&mut self.response) {
                Ok(it) => it,
                Err(err) =>{
                    log!("recv fail {err}");

                    self.failed_tryes_count += 1;
                    if self.failed_tryes_count >= 5 {
                        return Err(err)?; // lol 
                    }

                    self.client_set_addr()?

                },
            };
            self.addr = add;
            Ok((size, add))
    }

    pub fn trasmision_download(&mut self) -> Result<()> {
        // we asume the buffers are already set up
        self.packet = pack_ack(0);

        'block: loop {

            let (size, _add) = match self.socket.recv_from(&mut self.response) {
                Ok(it) => it,
                Err(err) =>{
                    log!("recv fail {err}");

                    self.failed_tryes_count += 1;
                    if self.failed_tryes_count >= 5 {
                        Err(err)?; // lol 
                    }

                    continue 'block;
                },
            };
            // resize buffer if last call
            self.response.truncate(size);


            // self.addr = add;

            // check if not err ei, 5
            if self.response.opcode() == 5 {
                unsafe {
                    // 2 for opcode 2 for errtype rest errmsg
                    let start_str = self.response.as_ptr().add(4);
                    let code = self.response.block();
                    log!("resived: {} Error: {}", code ,*start_str);

                }

                return Err(TftpError::InvalidData);
            }

            // check if block num increased
            let block = self.packet.block();
            // log!("block {block}");

            if self.response.block() != block +1 {
                log!("block {block}: already recved");

                self.failed_tryes_count += 1;
                if self.failed_tryes_count == 5 {
                    return Err(TftpError::TooManyFailTryes);
                }

                // do nothing just resend the block
            } else {
                self.pack_data_store()?;
                self.packet.block_pp();

                if size < self.packlen +4 {
                    break 'block;
                }
            }


            // expect data
            self.socket.send_to(&self.packet, self.addr)?;
        }

        self.file.flush()?;

        Ok(())
    }

    pub fn trasmision_upload(&mut self) -> Result<()> {
        // we asume the buffers are already set up
        self.packet = pack_data(1, self.packlen);
        self.pack_data_read()?;

        'block: loop {
            // expect data
            self.socket.send_to(&self.packet, self.addr)?;

            if self.packet.len() < self.packlen + 4 {
                break 'block;
            }


            // expect ack
            let (_size, _add) = match self.socket.recv_from(&mut self.response) {
                Ok(it) => it,
                Err(err) =>{
                    log!("recv fail {err}");

                    self.failed_tryes_count += 1;
                    if self.failed_tryes_count >= 5 {
                        Err(err)?; // lol 
                    }

                    continue 'block;
                },
            };
            // self.addr = add;

            // check if not err ei, 5
            if self.response.opcode() == 5 {
                unsafe {
                    // 2 for opcode 2 for errtype rest errmsg
                    let start_str = self.response.as_ptr().add(4);
                    let code = self.response.block();
                    eprintln!("resived: {} Error: {}", code ,*start_str);

                }

                return Err(TftpError::InvalidData);
            }

            // check if block num increased
            let block : u16 = self.packet.block();
            // log!("block {block}");

            if self.response.block() != block {
                eprintln!("block {block}: already recved");

                self.failed_tryes_count += 1;
                if self.failed_tryes_count == 5 {
                    return Err(TftpError::TooManyFailTryes);
                }

                // do nothing just resend the block
            } else {
                self.packet.block_pp();
                self.pack_data_read()?;
            }
        }

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
    #[inline]
    fn pack_data_read(&mut self) -> Result<()> {
        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = self.file.read(match self.packet.get_mut(4..) {
            None => return Err(TftpError::PackError),
            Some(o) => o,
        })?;

        self.packet.truncate(red_b + 4);

        Ok(())
    }

    #[inline]
    fn pack_data_store(&mut self) -> Result<()> {
        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = self.file.write(match self.response.get(4..) {
            None => return Err(TftpError::PackError),
            Some(o) => o,
        })?;

        self.response.truncate(red_b + 4);

        Ok(())
    }
}

//  =============================================================================
//  TEST
//  =============================================================================

#[test]
fn writepack() -> Result<()> {

    let filename = "/tmp/test";
    let payload = "hello word";
    {
        let mut file = File::create(filename)?;
        file.write(payload.as_bytes())?;
    }

    let pack = {
        let mut aget = Agent::client(&"[::1]:69", RwMod::Write, &filename)?;
        aget.pack_data_read()?;
        aget.packet
    };
    {
        let mut aget = Agent::client(&"[::1]:69", RwMod::Read, filename)?;
        aget.response = pack;
        aget.pack_data_store()?;
    }

    let buff = {
        let mut buff = String::new(); 
        let mut file = File::open(filename)?;
        file.read_to_string(&mut buff)?;
        buff
    };

    println!("expected: {:#?}", payload);
    println!("recieved: {:#?}", buff);

    assert_eq!(payload, buff);

    Ok(())
}

#[test]
fn readtopack() -> Result<()> {
    let mut packetare = Agent::client(&"[::1]:69", RwMod::Read, "/tmp/test")?;

    packetare.rw_mod = RwMod::Read;

    packetare.packet = packet::pack_data(4, 10);
    println!("init packet {:?}", packetare);
    println!("pack size {}", packetare.packet.len());

    packetare.file = File::open("src/agent.rs")?;

    packetare.pack_data_read()?;

    println!("filled packet {:?}", packetare);
    println!("pack size {}", packetare.packet.len());

    Ok(())
}

#[test]
fn pack_write_test() -> Result<()> {
    let mut agent = Agent::client(&"[::1]:69", RwMod::Read, "/tmp/test")?;

    agent.response = vec![0, 0, 0, 0, 72, 69, 76, 76, 79];

    agent.file = File::create("/tmp/pack_write.log")?;

    agent.pack_data_store()?;

    let data = {
        let mut file = File::open("/tmp/pack_write.log")?;
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        buff
    };

    assert_eq!(data, [72, 69, 76, 76, 79,]);

    Ok(())
}

#[test]
fn client_test() -> Result<()> {
    Agent::client(&"[::1]:69", RwMod::Read, "/tmp/test")?;

    Ok(())
}
