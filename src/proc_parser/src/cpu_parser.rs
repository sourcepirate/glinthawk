///! Parses the cpu information from
///! /proc/cpuinfo file

#[derive(Copy, Clone, Debug)]
pub struct CpuInfo {
    // cpu vendor
    pub vendor: String,
    // number of cores in cpu
    pub num_cores: u32,
    // cpu clock in mhz
    pub cpu_mhz: f32,
    // Cache size in kbs
    pub cache_size: u32
}


// Information extracted from /proc/stat
#[derive(Copy, Clone, Debug)]
pub struct CpuStat {
    pub num_cs: usize,
    pub boot_time: usize,
    pub num_process: usize,
    pub num_running_process: usize,
    pub num_blocked_process: usize
}
