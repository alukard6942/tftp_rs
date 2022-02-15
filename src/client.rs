

mod packet;
mod agent;

use agent::Agent;

use crate::agent::RW_mod::Read;

fn main(){    
    let mut client = Agent::client("127.0.0.1:6969", Read).expect("feaild to create client");
    
    println!("{:?}", client);

    let pack = packet::pack_read("hello", "netascci");
    println!("{:?}", pack);

    let size = client.send_pack().expect("faild to send");
    
    println!("packet of {}b was send", size);
}
