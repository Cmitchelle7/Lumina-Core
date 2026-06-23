pub struct HeaderSync {
    latest_height: u64,
    latest_block_time_ms: u64,
}

impl HeaderSync {
    pub fn new() -> Self { Self { latest_height: 0, latest_block_time_ms: 2000 } }
    pub fn update_header(&mut self, height: u64, block_time_ms: u64) {
        self.latest_height = height;
        self.latest_block_time_ms = block_time_ms;
    }
    pub fn latest_height(&self) -> u64 { self.latest_height }
    pub fn latest_block_time_ms(&self) -> u64 { self.latest_block_time_ms }
}
