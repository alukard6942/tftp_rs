

mod packet;
mod agent;

use std::error::Error;

use agent::{Agent, rw_mod};
use packet::pack_write;



fn send_file(addr: &str, path : &str) -> Result<(), Box<dyn Error>> {

    let mut client = Agent::client(addr, rw_mod::Write, path )?;

    Ok(())
}


#[test]
fn send_file_test() -> Result<(), Box<dyn Error>>{
    
    send_file("127.0.0.1:69", "/tmp/test")?;

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>>{    
    
    let mut client = Agent::client("127.0.0.1:69", rw_mod::Write , "/tmp/test")?;
    client.transmit();


    // todo 
    

    Ok(())

}
