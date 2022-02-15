/**
 * File: packet.rs
 * Author: alukard <alukard6942@github>
 * Date: 01.02.2022
 * Last Modified Date: 09.02.2022
 */


#[cfg(target_endian = "little")]
fn htons(val : u16) -> u16 {
    let o1 = (val >> 8)  as u8;
    let o0 =  val        as u8;
    (o0 as u16) << 8 | (o1 as u16)
}

#[cfg(target_endian = "big")]
fn htons(val : u16) -> u16 {
    val
}

#[cfg(target_endian = "little")]
#[test]
fn test_htonl() {
    assert_eq!(0x1234, htons(0x3412));
}

pub fn pack_read(filename: &str, mode: &str) -> Vec<u8>{
    let mut v = Vec::new();
    

    v.push(0);
    v.push(1);

    unsafe {
        v.append(filename.to_owned().as_mut_vec());
    }
    v.push(0);

    unsafe { 
        v.append(mode.to_owned().as_mut_vec());
    }
    v.push(0);

    return v;
}

pub fn pack_write(filename: &str, mode: &str) -> Vec<u8>{
    let mut v = Vec::new();

    v.push(0);
    v.push(1);

    unsafe {
        v.append(filename.to_owned().as_mut_vec());
    }
    v.push(0);

    unsafe { 
        v.append(mode.to_owned().as_mut_vec());
    }
    v.push(0);

    return v;
}

pub fn pack_data( block: u16,  data: &mut Vec<u8> ) -> Vec<u8>{
    let mut v = Vec::new();

    let hblock = htons(block);
    v.push(0);
    v.push(3);
    v.push(block as u8);
    v.push((block >> 8) as u8);
    v.append(data);

    return v;
}

pub fn pack_ack( block: u16) -> Vec<u8>{
    let mut v = Vec::new();

    let hblock = htons(block);
    v.push(0);
    v.push(4);
    v.push(block as u8);
    v.push((block >> 8) as u8);

    return v;
}


pub trait Packet {

    fn block(&self) -> u16;
    fn opcode(&self) -> u8;
}


impl Packet for Vec<u8> {

    fn block(&self) -> u16 {

        // there might not be space after
        // is not possible in corect pacckets
        unsafe {
            htons(*(self.as_ptr() as *const u16).add(2))
        }
    }

    fn opcode(&self) -> u8 {
        self[1]
    }

}

