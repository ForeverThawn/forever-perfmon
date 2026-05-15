#[derive(Clone, Copy, Default)]
pub struct IfTotals {
    pub received_bytes: u64,
    pub sent_bytes: u64,
}

#[derive(Clone, Copy)]
pub struct Sample {
    pub cpu_usage: f64,
    pub cpu_user_usage: f64,
    pub cpu_privileged_usage: f64,
    pub cpu_dpc_usage: f64,
    pub memory_committed_percentage: f64,
    pub memory_avail: f64,
    pub memory_committed: f64,
    pub memory_commit_limit: f64,
    pub disk_read: f64,
    pub disk_write: f64,
    pub hyperv_avail_bytes: Option<f64>,
    pub hyperv_total_bytes: Option<f64>,
}
