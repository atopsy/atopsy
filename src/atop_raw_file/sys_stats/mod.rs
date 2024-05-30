pub mod container_stats;
pub mod cpu_stats;
pub mod disk_stats;
pub mod gpu_stats;
pub mod interface_stats;
pub mod last_level_cache_stats;
pub mod mem_stats;
pub mod net_stats;
pub mod nfs_stats;
pub mod numa_stats;
pub mod pressure;
pub mod www_stats;

use std::io::Read;

use crate::constants::SYS_STATS_SIZE;
use container_stats::ContStat;
use cpu_stats::CpuStats;
use disk_stats::DskStat;
use flate2::read::ZlibDecoder;
use gpu_stats::GpuStat;
use interface_stats::{IfbStat, IntfStat};
use last_level_cache_stats::LlcStat;
use mem_stats::MemStats;
use net_stats::NetStats;
use nfs_stats::NfsStat;
use numa_stats::{CpuNuma, MemNuma};
use pressure::Pressure;
use www_stats::WwwStat;

#[derive(Debug)]
#[repr(C)]
pub struct SysStats {
    cpu_stats: CpuStats,
    mem_stats: MemStats,
    net_stats: NetStats,
    interface_stats: IntfStat,
    memnuma: MemNuma,
    cpunuma: CpuNuma,
    dsk: DskStat,
    nfs: NfsStat,
    cfs: ContStat,
    psi: Pressure,
    gpu: GpuStat,
    ifb: IfbStat,
    llc: LlcStat,
    www: WwwStat,
}

impl SysStats {
    pub unsafe fn from(compressed: Vec<u8>) -> Self {
        let mut sys_stats_buffer = [0u8; SYS_STATS_SIZE];
        let mut decompresser = ZlibDecoder::new(compressed.as_slice());
        decompresser.read_exact(&mut sys_stats_buffer).unwrap();
        std::mem::transmute(sys_stats_buffer)
    }
}