use crate::deserialize::deserialize;

mod deserialize;

fn main() {
    let data = include_bytes!("../../TestData/Gold-Datablock.txt");
    deserialize(data);
}
