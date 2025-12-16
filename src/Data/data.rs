
pub struct MyKeys {
    pub p: i128,
    pub q: i128,
    pub k: i128,

}

impl MyKeys{
    pub fn m(&self) -> i128 {
        self.p * self.q
    }
    pub fn øm(&self) -> i128 {
        (self.p -1) * (self.q - 1)
    }
}