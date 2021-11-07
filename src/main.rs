#![allow(dead_code)]
mod proc;

use proc::cpuinfo::CPUInfo;
use proc::meminfo::MemInfo;

fn main() {
    let _meminfo = MemInfo::new();
    let _cpuinfo = CPUInfo::new();
}
