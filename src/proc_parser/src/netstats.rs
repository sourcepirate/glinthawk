///! the file is being parsed from /proc/net/sockstat

#[derive(Copy, Clone, Debug)]
pub struct SocketStat {
    pub total: usize,
    pub tcp: usize,
    pub udp: usize,
    pub udpl: usize,
    pub raw: usize,
    pub frag: usize
}