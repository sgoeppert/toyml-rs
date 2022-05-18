use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Splitmix64 {
    state: u64,
}

impl Splitmix64 {
    pub fn new() -> Splitmix64 {
        Splitmix64 {
            state: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }

    pub fn from_seed(seed: u64) -> Splitmix64 {
        Splitmix64 {
            state: seed
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}


#[derive(Debug)]
struct RngBase {
    s: [u64; 2],
}

impl RngBase {
    pub fn new() -> RngBase {
        let mut mix = Splitmix64::new();

        RngBase {
            s: [mix.next(), mix.next()]
        }
    }

    pub fn from_seed(seed: u64) -> RngBase {
        let mut mix = Splitmix64::from_seed(seed);

        RngBase {
            s: [mix.next(), mix.next()]
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut s0 = self.s[0];
        let mut s1 = self.s[1];

        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14);
        s1 = s1.rotate_left(36);
        self.s[0] = s0;
        self.s[1] = s1;

        result
    }
}

#[derive(Debug)]
pub struct Rng {
    gen: RngBase
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            gen: RngBase::new()
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        let n = self.gen.next();
        n as f64 / u64::MAX as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_float() {
        let mut rng = Rng::new();

        let values: Vec<f64> = (0..100).into_iter().map(|_| rng.next_f64()).collect();

        println!("{:?}", values);
    }
}