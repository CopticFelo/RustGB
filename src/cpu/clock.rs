#[derive(Debug, Default)]
pub struct Clock {
    m_cycles: u32,
    t_cycles: u64,
}

// HACK: Incomplete understanding of how clocks work

impl Clock {
    pub fn tick(&mut self) {
        self.m_cycles += 1_u32;
        self.t_cycles += 4_u64;
    }
}
