fn prbs7(seed: &mut u8) -> Result<String, String> {
    if *seed == 0 {
        Err("Error. Result will always be zero".to_string()) 
    } else {
        let temp = ((*seed >> 6) ^ (*seed >> 5)) & 1;
        *seed = ((*seed << 1) | temp) & 0x7F;
        Ok("Succesfully".to_string())
    }
}

fn main() {

    // initial seed
    let mut res: u8 = 0;

    for _ in 0..127 {
        prbs7(&mut res).unwrap();
        println!("{:x}", res);
    }
}
