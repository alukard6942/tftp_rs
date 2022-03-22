/**
 * File: agent.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */

use std::{net::{UdpSocket, SocketAddr, ToSocketAddrs}, io::{self, ErrorKind, Read, Write}, time::Duration};

use thiserror::Error;

use std::fs::File;

use crate::packet::{self, pack_data, Packet, pack_ack};


#[derive(Debug, Error)]
pub enum TftpError {
    
    #[error("unenable to connect to socket")]
    SockError,

    #[error("data makes no sence in this context")]
    InvalidData,

    #[error("packet has not enough size for data")]
    PackError,

    #[error("Nonspecific Error, should not be used !!!Temporary!!!")]
    TODO,
    
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
type Result<T> = std::result::Result<T, TftpError>;



#[derive(Debug, Clone)]
pub enum rw_mod {Read, Write, none}

#[derive(Debug)]
pub struct Agent {
    feiled_tryes_count: u16,
    socket: UdpSocket,
    addr: SocketAddr,
    pub file: Option<File>,
    pub rw_mod: rw_mod,
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
    pub fn client(addr: impl ToSocketAddrs, rw_mod: rw_mod, filename: &str) -> Result<Agent>{
        
        let len = 512;
        
        use packet::{pack_ack, pack_data, pack_read, pack_write};
        use std::net::{SocketAddr, ToSocketAddrs};


        // assuming 'localhost' resolves to 127.0.0.1
        let mut addrs = addr.to_socket_addrs()?.next().unwrap();

        Ok(Agent{
            feiled_tryes_count: 0,
            
            // vildcard bind to localhost random port
            socket: UdpSocket::bind("[::]:0")?,

            addr: addrs,

            file: Some(File::create(filename)?),

            packet: match rw_mod.clone(){
                rw_mod::Read  => pack_read(filename, "octed"),
                rw_mod::Write => pack_write(filename, "octed"),
                rw_mod::none  => return Err( TftpError:: InvalidData ),
            },
            response: match rw_mod.clone(){
                rw_mod::Read  => pack_ack(1),
                rw_mod::Write => pack_data(1, len),
                rw_mod::none  => return Err( TftpError:: InvalidData ),
            },

            rw_mod,

        })
    }
    
    pub fn get_soc69() -> Result<UdpSocket> {

        Ok( UdpSocket::bind("[::]:69")?)
    }

    pub fn server(socrcvr: UdpSocket) -> Result<Agent>{
        
        let mut response = Vec::new();
        
        socrcvr.recv_from(&mut response);


        
        
        Ok(Agent{
            feiled_tryes_count: 0,
            file: None,
            //bind
            socket: UdpSocket::bind("[::]:0")?,
            addr: SocketAddr::from(([127,0,0,1], 69)),
            rw_mod: rw_mod::none,
            packet: Vec::new(),
            response: response,
        })
    }
    
    /**
     * @brief hanles the logic of trasmision is same for client and server
     *
     * @param self
     *
     * @return 
     */
    pub fn transmit(&mut self) -> Result<()>{
        
        match self.rw_mod {
            rw_mod::Read => self.trasmision_read(),
            rw_mod::Write => self.trasmision_write(),
            rw_mod::none => Err(TftpError::InvalidData),
        }
        
    }
    
    #[inline]
    fn trasmision_read(&mut self) -> Result<()> {
        let pack_len = 512;
        
        // we asume the buffers are already set up
        
        // read request
        self.socket.send_to(&self.packet, &self.addr)?;

        self.packet = pack_ack(0);
        
        
        'block : loop  {
            
            // expect data
            let (size, add ) = self.socket.recv_from(&mut self.response)?;
            self.addr = add;
            
            // check if not err ei, 5
            if self.response.opcode() == 5 {
                unsafe {
                    // 2 for opcode 2 for errtype rest errmsg
                    let start_str = self.response.as_ptr().add(4);
                    eprintln!("resived Error: {}", *start_str);
                }

                return Err(TftpError::InvalidData);
            }
            
            // check if block num increased 
            let block = self.packet.block();
            if self.response.block() != block {
                
                eprintln!("block {block}: already recved");
                

                // do nothing just resend the block

            } else {
                self.packet.block_pp();
                self.pack_data_store();
            }
                
            if size < pack_len {
                break;
            }

            self.socket.send_to(&self.packet, self.addr)?;
        }
        
        Ok(())
    }


    #[inline]
    fn trasmision_write(&mut self) -> Result<()> {
        let pack_len = 512;
        
        // we asume the buffers are already set up
        
        // read request
        self.socket.send_to(&self.packet, &self.addr)?;

        self.packet = pack_data(1, pack_len);
        self.pack_data_read();
        
        
        'block : loop  {
            
            // expect ack
            let (size, add ) = self.socket.recv_from(&mut self.response)?;
            self.addr = add;
            
            // check if not err ei, 5
            if self.response.opcode() == 5 {
                unsafe {
                    // 2 for opcode 2 for errtype rest errmsg
                    let start_str = self.response.as_ptr().add(4);
                    eprintln!("resived Error: {}", *start_str);
                }

                return Err(TftpError::InvalidData);
            }
            
            // check if block num increased 
            let block = self.packet.block();
            if self.response.block() != block {
                
                eprintln!("block {block}: already recved");
                

                // do nothing just resend the block

            } else {
                self.packet.block_pp();
                self.pack_data_read();
            }
                
            if self.packet.len() < pack_len {
                break;
            }

            self.socket.send_to(&self.packet, self.addr)?;
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
    fn pack_data_read(&mut self) -> Result<()>{

        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = &mut self.file.as_ref().unwrap().read( match self.packet.get_mut(4..){
                None => return Err(TftpError::PackError),
                Some(o) => o,
            }
        )?;
        
        self.packet.truncate(*red_b + 4 + 1);

        Ok(())
    }
    
    #[inline]
    fn pack_data_store(&mut self) -> Result<()>{
        
        // read from file to the packet buffer skips the first 4 B  the rest is for data viz. specefication
        let red_b = &mut self.file.as_ref().unwrap().write( match self.response.get_mut(4..){
                None => return Err(TftpError::PackError),
                Some(o) => o,
            }
        )?;
        
        self.response.truncate(*red_b + 4 +1);

        Ok(())
    }

    

    
}




//  =============================================================================
//  TEST
//  =============================================================================
#[test]
fn readtopack() -> Result<()> {
    
    let mut packetare = Agent::client("[::1]:69", rw_mod::Read, "/tmp/test")?;
    
    packetare.rw_mod = rw_mod::Read;
    
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
fn pack_write() -> Result<()> {
    
    let mut agent = Agent::client("[::1]:69", rw_mod::Read, "/tmp/test")?;
    
    agent.response = vec![0,0,0,0,   72, 69, 76, 76, 79,];
    
    agent.file = Some(File::create("/tmp/pack_write.log")?);
    
    agent.pack_data_store()?;
    
    let data ={
        
        let mut file = File::open("/tmp/pack_write.log")?;
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;
        
        buff
    };
    
    assert_eq!(data, [72, 69, 76, 76, 79,]);
    
    
    Ok(())
}

#[test]
fn client_test()-> Result<()> {
    
    Agent::client("[::1]:69", rw_mod::Read, "/tmp/test")?;

    Ok(())
    
}