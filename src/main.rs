use blockchainlib::*;
fn main () {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Block".to_owned());
    let h = block.hash();
    block.hash = h;
    println!("{:?}", block)
}
