extern crate rand;
use rand::Rng;

fn mod_pow(b: u64, mut e: u64, m: u64) -> u64 {
    
    if m == 1 {
        return 0;
    }
    let mut c: u64 = 1;
    while e > 0 {
        c = (c*b) % m;
        e-=1;
    }
    c
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2,p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub,a,p)
}
