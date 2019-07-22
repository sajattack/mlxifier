extern crate itertools;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::SeekFrom;
use itertools::Itertools;

fn mlx_checksum(address: usize, data: &[u8]) -> u8 {
    let mut checksum = (address as f64 / 256 as f64) as isize;
    checksum = address as isize - 254 * checksum + 255 * (if checksum > 127 { -1 } else { 0 });
    checksum = checksum + 255 * (if checksum > 255 { -1 } else { 0 });
    for i in 0..8 {
        checksum = checksum * 2 + 255 * (if checksum > 127 { -1 } else { 0 }) + data[i] as isize;
        checksum = checksum + 255 * (if checksum > 255 { -1 } else { 0 });
    }
    checksum as u8
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let starting_address = *&args[2].parse::<usize>().unwrap();
    let mut file = File::open(filename).unwrap();
    file.seek(SeekFrom::Start(2));
    let mut address = starting_address; 
    for chunk in &file.bytes().chunks(8) {
        let mut chunk = chunk.map(|r: Result<u8, _>| r.unwrap()).collect::<Vec<u8>>();
        while chunk.len() < 8 {
            chunk.push(0);
        }
        let checksum = mlx_checksum(address, &chunk);
        let mut output = format!("{:04X}: {:02X?} {:02X}", address, chunk, checksum);
        output = output.replace("[","");
        output = output.replace("]","");
        output = output.replace(",","");
        println!("{}",output);
        address += 8;
    }
}
