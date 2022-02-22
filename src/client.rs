

mod packet;
mod agent;

use std::error::Error;

use agent::{Agent, RW_mod};



fn send_file(addr: &str, path : &str) -> Result<(), Box<dyn Error>> {

    let mut client = Agent::new()?;

    client.set_addr(addr)?;
    client.send_file(path);
    

    
    Ok(())
}




fn main() -> Result<(), Box<dyn Error>>{    
    


    // todo 
    

    Ok(())

}
