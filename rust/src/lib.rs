extern crate rand;

use rand::{thread_rng, Rng};
use std::mem;

const EXP: i64 = 65537;

#[derive(Debug)]
pub struct KeySet {
    e: i64,
    d: i64,
    n: i64,
}

fn gen_prime(p: i64) -> i64 {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let mut rng = thread_rng();
    let mut x: u64 = rng.gen_range(0..p as u64 - 1);
    while !is_prime(x) {
        x = rng.gen_range(0..p as u64 - 1);
    }
    x as i64
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    (2..n).all(|a| n % a != 0)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        mem::swap(&mut a, &mut b)
    }
    a
}

fn ext_euclid(a: i64, b: i64) -> i64 {
    let (mut sj, mut sj_last) = (0, 1);
    let (mut tj, mut tj_last) = (1, 0);
    let (mut rj, mut rj_last) = (b, a);
    while rj != 0 {
        let quotient = rj_last / rj;
        rj_last -= quotient * rj;
        sj_last -= quotient * sj;
        tj_last -= quotient * tj;
        mem::swap(&mut rj, &mut rj_last);
        mem::swap(&mut sj, &mut sj_last);
        mem::swap(&mut tj, &mut tj_last);
    }
    tj_last
}

pub fn gen_keys() -> KeySet {
    let (n, phi) = {
        loop {
            let p = gen_prime(EXP);
            let q = gen_prime(EXP);
            let n = p * q;
            let phi = (p - 1) * (q - 1);
            if p != 0 && q != 0 && p != q && gcd(phi, EXP) == 1 {
                break (n, phi);
            }
        }
    };
    let mut d = ext_euclid(phi, EXP);
    while d < 0 {
        d += phi;
    }
    KeySet { e: EXP, d, n }
}

// binary-exponentiation-and-recursive-function
fn modpow(b: i64, e: i64, m: i64) -> Result<i64, &'static str> {
    if b < 0 || e < 0 || m <= 0 {
        return Err("Invalid");
    }
    let n = match e {
        0 => 1,
        1 => b,
        _ => match e % 2 {
            0 => modpow(b * b % m, e / 2, m)? % m,
            _ => b * modpow(b, e - 1, m)? % m,
        },
    };
    Ok(n)
}

pub fn encrypt(m: &[u8], k: &KeySet) -> Vec<i64> {
    m.iter()
        .map(|x| modpow((*x).into(), k.e, k.n).unwrap())
        .collect()
}

pub fn decrypt(s: &[i64], k: &KeySet) -> Vec<u8> {
    s.iter()
        .map(|x| modpow(*x as i64, k.d, k.n).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(3));
        assert!(is_prime(187963));
    }

    #[test]
    fn test_rand_prime() {
        let rand1 = gen_prime(EXP);
        assert!(is_prime(rand1 as u64));

        let rand2 = gen_prime(EXP);
        assert!(is_prime(rand2 as u64));
        assert!(rand1 != rand2);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 4), 4);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(8, 6), 2);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_whole_scheme() {
        let m = b"Be water my friend....";
        let k = gen_keys();
        let enc = encrypt(m, &k);
        let dec = decrypt(&enc, &k);
        assert_eq!(dec, m);
    }
}
