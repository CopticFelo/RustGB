#[derive(Debug)]
struct Clock {
    m_cycles: u32,
    c_cycles: u64,
}

impl Clock {
    fn inc_cycle(&self, count: u8) {
        self.m_cycles += count;
        self.c_cycles += count * 4;
    }
}
