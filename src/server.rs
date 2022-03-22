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
use agent::{Agent, rw_mod::Read};


fn main() -> Result<(), Box<dyn Error>> {
    
    let socket = Agent::get_soc69()?;

    let server = Agent::server(socket)?;


    Ok(())
}

