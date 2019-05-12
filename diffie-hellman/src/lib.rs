extern crate rand;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2,p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a) % p
}
