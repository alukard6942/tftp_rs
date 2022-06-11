/**
 * File: client.rs
 * Author: alukard <alukard6942@github>
 * Date: 11.06.2022
 * Last Modified Date: 11.06.2022
 */

use std::error::Error;

extern crate getopts;
use getopts::Options;
use std::env;

mod agent;
use agent::{Agent, RwMod};

mod log;


fn main() -> Result<(), Box<dyn Error>>{    
    
    let mut mode = RwMod::None;
    let mut addres = String::from("[::]:69");
    let mut file = String::from("/tmp/test");

    let mut opts = Options::new();
    opts.optopt("r", "read", "read form file", "FILE");
    opts.optopt("w", "write", "write to file", "FILE");
    opts.optopt("a", "addres", "specify addres", "ADRESS");


    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..])?;

    if let Some(write) = matches.opt_str("w"){
        file = write;
        mode = RwMod::Write;
    }

    else
    if let Some(read) = matches.opt_str("r"){
        file = read;
        mode = RwMod::Read;
    }

    else
    if let Some(addr) = matches.opt_str("a"){
        addres = addr;
    }


    let mut client = Agent::client(&addres, mode.clone(), &file)?;

    match client.send_header() {
        Ok(_)  => log!("header send"),
        Err(e) => log!("header failed: {e}"),
    };
    match client.client_set_addr() {
        Ok(_)  => log!("address set"),
        Err(e) => log!("address seting failed: {e}"),
    };

    log!("trasmiting {}", file);
    let result = match mode {
        agent::RwMod::Read  => client.trasmision_download(),
        agent::RwMod::Write => client.trasmision_upload(),
        agent::RwMod::None  => todo!(),
    };

    // hadle result of transmision
    match result {
        Ok(_)  => log!("transmision sucseded"),
        Err(e) => log!("transmision failed: {e}"),
    };

    Ok(())
}
