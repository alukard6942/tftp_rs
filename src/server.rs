/**
 * File: server.rs
 * Author: alukard <alukard6942@github>
 * Date: 10.02.2022
 * Last Modified Date: 10.02.2022
 */

mod packet;
mod agent;

use std::{error::Error};

use crate::packet::Packet;
use agent::{Agent, RW_mod::Read};


fn main() -> Result<(), Box<dyn Error>> {
    
    let server = Agent::new()?;

    


    Ok(())
}

