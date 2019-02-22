///! The file consists of accumulated information
///! from the /proc/meminfo

#[derive(Debug, Copy, Clone)]
pub struct MemStat {
    pub total: usize,
    pub free: usize,
    pub cached: usize,
    pub swapcached: usize,
    pub active: usize,
    pub inactive: usize,
    pub swaptotal: usize,
    pub swapfree: usize,
    pub unevitable: usize,
    pub vm_total: usize,
    pub vm_used: usize,
    pub vm_chunk: usize
}