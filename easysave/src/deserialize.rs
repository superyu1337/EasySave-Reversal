use std::{string::FromUtf8Error};

pub fn deserialize(data: &[u8]) {
    let blocks = split_blocks(data);
    println!("{:#x?}", blocks);

    for block in blocks {
        let key = get_key(block).expect("error when getting the key");
        println!("key: {:x?}", key);
    }

    // todo: deserialization of the value
}

fn split_blocks(data: &[u8]) -> Vec<Vec<u8>> { // this is probably pretty inefficient. Find a better way to do this.
    let mut blocks = Vec::<Vec<u8>>::new();
    let mut current_block = Vec::<u8>::new();

    for value in data {
        if *value == 0x7B {
            current_block.push(value.clone());
            blocks.push(current_block.clone());
            current_block.clear();
        } else {
            current_block.push(value.clone());
        }
    }

    blocks
}

fn get_key(datablock: Vec<u8>) -> Result<String, FromUtf8Error> {
    let keylength = datablock[1] as usize;
    let key = datablock.get(2 .. 2+keylength).unwrap();
    String::from_utf8(key.to_vec())
}