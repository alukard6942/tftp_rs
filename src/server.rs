/**
 * File: server.rs
 * Author: alukard <alukard6942@github>
 * Date: 10.02.2022
 * Last Modified Date: 10.02.2022
 */

extern crate chrono;

mod agent;
use agent::Agent;

use std::net::UdpSocket;

use std::thread;

mod log;
mod packet;

mod error;
use error::*;

fn main() -> Result<()> {
    let reqvests = UdpSocket::bind("[::1]:69")?;


    log!("{}","server started");

    loop {

        let size = 1024;
        let mut buffer = packet::pack_data(1, size);

        let (__, addr) = reqvests.recv_from(&mut buffer)?;

        log!("{} new reqvest {}", addr, chrono::Local::now());

        thread::spawn(move || {
            
            let (mut server, mode) = match Agent::server(&mut buffer, addr){
                Ok(o) => o,
                Err(e) => {
                    log!("{} Error while parsing reqvest: {}", addr, e);
                    return;
                }
            };

            let result = match mode {
                agent::RwMod::Read  => server.trasmision_upload(),
                agent::RwMod::Write => {
                    server.packet = packet::pack_ack(0);
                    match server.send_header() {
                        Ok(_) => server.trasmision_download(),
                        Err(err) => Err(err),
                    }
                },
                agent::RwMod::None  => todo!(),
            };

            // hadle result of transmision
            match result {
                Ok(_)  => log!("{addr} transmision sucseded"),
                Err(e) => log!("{addr} transmision failed: {e}"),
            };
        });
    }
}

