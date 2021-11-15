use facts::proc::cpuinfo::*;
use facts::proc::meminfo::*;

#[test]
/// Checks if some attributes were obtained and if the number of processors is greater than 0
fn test_cpuinfo_new() {
    let cpuinfo = CPUInfo::new();

    assert!(cpuinfo.processors[0].model_name.is_some());
    assert!(cpuinfo.processors[0].cpu_cores.is_some());
    assert!(cpuinfo.processors.len() > 0);
}

#[test]
/// Check that the "to_json" function did not panic
fn test_cpuinfo_to_json() {
    let cpuinfo = CPUInfo::new();

    cpuinfo.to_json();
}

#[test]
/// Checks if some attributes were obtained
fn test_meminfo_new() {
    let meminfo = MemInfo::new();

    assert!(meminfo.mem_total.is_some());
    assert!(meminfo.mem_free.is_some());
    assert!(meminfo.mem_available.is_some());
}

#[test]
/// Check that the "to_json" function did not panic
fn test_meminfo_to_json() {
    let meminfo = MemInfo::new();

    meminfo.to_json();
}
