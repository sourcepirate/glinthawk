///! The file is read from /proc/diskstats

#[derive(Debug, Copy, Clone)]
pub struct DiskStat {
    pub major: usize,
    pub minor: usize,
    pub read_completed: usize,
    pub read_merged: usize,
    pub sector_read: usize,
    pub read_spent: usize,
    pub write_completed: usize,
    pub sector_write: usize,
    pub write_time: usize,
    pub io_progress: usize,
    pub io_spent: usize,
    pub io_wait: usize
}