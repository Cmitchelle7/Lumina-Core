use std::collections::VecDeque;

const ROLLING_WINDOW: usize = 100;
const EMA_ALPHA: f64 = 0.3;
const SAFETY_MARGIN: f64 = 1.5;
const MIN_SAMPLES_FOR_ACCURACY: usize = 10;

pub struct BlockTimeEstimator {
    block_times: VecDeque<u64>,
    ema: f64,
    p95: u64,
    samples_collected: usize,
}

impl BlockTimeEstimator {
    pub fn new() -> Self {
        Self {
            block_times: VecDeque::with_capacity(ROLLING_WINDOW),
            ema: 0.0,
            p95: 0,
            samples_collected: 0,
        }
    }

    pub fn record_block_time(&mut self, block_time_ms: u64) {
        self.block_times.push_back(block_time_ms);
        if self.block_times.len() > ROLLING_WINDOW {
            self.block_times.pop_front();
        }

        if self.samples_collected == 0 {
            self.ema = block_time_ms as f64;
        } else {
            self.ema = EMA_ALPHA * block_time_ms as f64 + (1.0 - EMA_ALPHA) * self.ema;
        }

        self.samples_collected += 1;
        self.recalculate_p95();
    }

    fn recalculate_p95(&mut self) {
        if self.block_times.is_empty() {
            self.p95 = 0;
            return;
        }
        let mut sorted: Vec<u64> = self.block_times.iter().cloned().collect();
        sorted.sort_unstable();
        let idx = (sorted.len() as f64 * 0.95).ceil() as usize - 1;
        let idx = idx.min(sorted.len() - 1);
        self.p95 = sorted[idx];
    }

    pub fn estimated_blocks_for_timeout(&self, timeout_delta_secs: u64) -> u64 {
        let timeout_ms = timeout_delta_secs * 1000;
        if self.p95 == 0 {
            return timeout_ms / 2000;
        }
        let raw_estimate = (timeout_ms as f64 / self.p95 as f64).ceil() as u64;
        if self.samples_collected < MIN_SAMPLES_FOR_ACCURACY {
            (raw_estimate as f64 * SAFETY_MARGIN).ceil() as u64
        } else {
            raw_estimate
        }
    }

    pub fn p95_block_time_ms(&self) -> u64 {
        self.p95
    }

    pub fn ema_block_time_ms(&self) -> f64 {
        self.ema
    }

    pub fn samples_collected(&self) -> usize {
        self.samples_collected
    }
}
