

mod packet;
mod agent;

use std::error::Error;

use agent::{Agent, RW_mod};
use packet::pack_write;



fn send_file(addr: &str, path : &str) -> Result<(), Box<dyn Error>> {

    let mut client = Agent::client(addr, RW_mod::Write, path )?;

    client.send_pack();
    
    Ok(())
}


#[test]
fn send_file_test() -> Result<(), Box<dyn Error>>{
    
    send_file("localhost", "/tmp/test")?;

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>>{    
    


    // todo 
    

    Ok(())

}
