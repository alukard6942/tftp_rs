/**
 * File: main.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */

mod client;
mod packet;

use client::Client;

fn main() {
    println!("Hello, world!");

    let tftp = Client::new("localhost:6969");

    let pack: Vec<u8> = packet::pack_read("aaa", "bbb");

    println!("{:?}", tftp);
    println!("{:?}", pack);
}
