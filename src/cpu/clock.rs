struct Clock {
#[derive(Debug, Default)]
    m_cycles: u32,
    t_cycles: u64,
}

impl Clock {
    fn inc_cycle(&mut self, count: u8) {
        self.m_cycles += count as u32;
        self.t_cycles += count as u64 * 4;
    }
}
