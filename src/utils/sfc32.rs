pub struct SFC32 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}
impl SFC32 {

    /// SFC32 (Small Fast Counter) is a pseudo-random number generator (PRNG) 
    /// which computes **deterministic random numbers** based on a given seed (a, b, c, d) 
    /// with a series of bitwise operations.
    pub fn new(seed: [u32; 4]) -> Self {
        let [a, b, c, d] = seed;
        Self { a, b, c, d }
    }

    /// Generates a pseudo-random f64 using SFC32
    pub fn rand_f64(&mut self) -> f64 {
        let t = self.a.wrapping_add(self.b).wrapping_add(self.d);
        self.d = self.d.wrapping_add(1);
        self.a ^= self.b >> 9;
        self.b = self.b.wrapping_add(self.c).wrapping_add(self.c << 3);
        self.c = (self.c << 21) | (self.c >> 11);
        self.c = self.c.wrapping_add(t);
        (t as f64) / 4294967296.0
    }
    /// Generates a random `usize` in range `[min, max)`.
    pub fn rand_between(&mut self, min: usize, max: usize) -> usize {
        let range = max - min;
        let rand_value = (self.rand_f64() * range as f64) as usize;
        min + rand_value
    }
}