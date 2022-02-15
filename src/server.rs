/**
 * File: server.rs
 * Author: alukard <alukard6942@github>
 * Date: 10.02.2022
 * Last Modified Date: 10.02.2022
 */

mod packet;
mod agent;

use std::{error::Error};

use agent::Agent;


fn main() -> Result<(), Box<dyn Error>> {
    
    let server = Agent::server()?;
    
    println!("server: {:?}", server);
    
    Ok(())
    
}

