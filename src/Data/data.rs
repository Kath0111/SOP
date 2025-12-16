
pub struct PrivateKeys {
    p: i128,
    q: i128,
}

impl PrivateKeys{
    pub fn new(p: i128, q: i128) -> Self {
        PrivateKeys { p, q }
    }
    pub fn m(&self) -> i128 {
        self.p * self.q
    }
    pub fn øm(&self) -> i128 {
        (self.p -1) * (self.q - 1)
    }
}

pub struct PublicKeys {
    pub k: i128,
    pub m: i128,
}