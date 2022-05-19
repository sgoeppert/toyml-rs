use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Splitmix64 {
    state: u64,
}

impl Splitmix64 {
    pub fn new() -> Splitmix64 {
        Splitmix64 {
            state: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64,
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

    pub fn next_f64(&mut self) -> f64 {
        let n = self.next();
        n as f64 / u64::MAX as f64
    }
}

pub struct NormalDistRng {
    gen: RngBase,
    buffer: Option<f64>,
    mean: f64,
    std_dev: f64,
}
impl NormalDistRng {
    pub fn new() -> Self {
        NormalDistRng {
            gen: RngBase::new(),
            buffer: None,
            mean: 0.,
            std_dev: 1.,
        }
    }
    pub fn from_mean_and_std(mean: f64, std_dev: f64) -> Self {
        NormalDistRng {
            gen: RngBase::new(),
            buffer: None,
            mean,
            std_dev,
        }
    }

    pub fn next(&mut self) -> f64 {
        self.mean + self.gauss_random() * self.std_dev
    }

    pub fn gauss_random(&mut self) -> f64 {
        match self.buffer {
            Some(val) => {
                self.buffer = None;
                val
            },
            None => {
                let mut u;
                let mut v;
                let mut r;

                loop {
                    u = self.gen.next_f64() * 2. - 1.;
                    v = self.gen.next_f64() * 2. - 1.;
                    r = u*u + v*v;

                    if r > 0. && r < 1. {
                        break;
                    }
                }
                let c = (-2. * r.ln()/r).sqrt();
                self.buffer = Some(c * v);
                u * v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_float() {
        let mut rng = RngBase::new();

        let values: Vec<f64> = (0..100).into_iter().map(|_| rng.next_f64()).collect();

        println!("{:?}", values);
    }

    #[test]
    fn next_normal() {
        let mut rng = NormalDistRng::from_mean_and_std(0., 1.);
        let values: Vec<f64> = (0..100).into_iter().map(|_| rng.next()).collect();
        println!("{:?}", values);
    }
}