pub struct PacketTimeoutMisestimation {
    pub packet_id: u64,
    pub expected_timeout_height: u64,
    pub actual_destination_height: u64,
}

pub struct PacketRelayer {
    misestimations: Vec<PacketTimeoutMisestimation>,
}

impl PacketRelayer {
    pub fn new() -> Self {
        Self {
            misestimations: Vec::new(),
        }
    }

    pub fn check_timeout_accuracy(
        &mut self,
        packet_id: u64,
        expected_timeout_height: u64,
        destination_height: u64,
    ) -> bool {
        if destination_height >= expected_timeout_height {
            return true;
        }
        self.misestimations.push(PacketTimeoutMisestimation {
            packet_id,
            expected_timeout_height,
            actual_destination_height: destination_height,
        });
        false
    }

    pub fn misestimation_count(&self) -> usize {
        self.misestimations.len()
    }

    pub fn misestimation_rate(&self, total_packets: usize) -> f64 {
        if total_packets == 0 {
            return 0.0;
        }
        self.misestimations.len() as f64 / total_packets as f64 * 100.0
    }

    pub fn needs_recalibration(&self, total_packets: usize) -> bool {
        self.misestimation_rate(total_packets) > 5.0
    }
}
