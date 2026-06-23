use super::block_time_estimator::BlockTimeEstimator;

pub struct PacketTimeoutCalculator {
    estimator: BlockTimeEstimator,
}

pub struct TimeoutResult {
    pub timeout_height: u64,
    pub estimated_blocks: u64,
    pub used_p95: u64,
}

impl PacketTimeoutCalculator {
    pub fn new() -> Self {
        Self { estimator: BlockTimeEstimator::new() }
    }

    pub fn record_block(&mut self, block_time_ms: u64) {
        self.estimator.record_block_time(block_time_ms);
    }

    pub fn compute_timeout(&self, source_height: u64, timeout_delta_secs: u64) -> TimeoutResult {
        let p95 = self.estimator.p95_block_time_ms();
        let estimated_blocks = self.estimator.estimated_blocks_for_timeout(timeout_delta_secs);
        TimeoutResult {
            timeout_height: source_height.saturating_add(estimated_blocks),
            estimated_blocks,
            used_p95: if p95 == 0 { 2000 } else { p95 },
        }
    }

    pub fn estimator(&self) -> &BlockTimeEstimator { &self.estimator }
}
