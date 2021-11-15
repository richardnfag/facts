use std::collections::HashMap;
use std::fs::read_to_string;

use crate::utils::HashMapStr;

pub struct CPUInfo {
    pub processors: Vec<Processor>,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            processors: get_processors().unwrap(),
        }
    }
}

pub struct Processor {
    processor: Option<String>,
    vendor_id: Option<String>,
    cpu_family: Option<String>,
    model: Option<String>,
    model_name: Option<String>,
    stepping: Option<String>,
    microcode: Option<String>,
    cpu_mhz: Option<String>,
    cache_size: Option<String>,
    physical_id: Option<String>,
    siblings: Option<String>,
    core_id: Option<String>,
    cpu_cores: Option<String>,
    apicid: Option<String>,
    initial_apicid: Option<String>,
    fpu: Option<String>,
    fpu_exception: Option<String>,
    cpuid_level: Option<String>,
    wp: Option<String>,
    flags: Option<String>,
    vmx_flags: Option<String>,
    bugs: Option<String>,
    bogomips: Option<String>,
    clflush_size: Option<String>,
    cache_alignment: Option<String>,
    address_sizes: Option<String>,
    power_management: Option<String>,
}

impl Processor {
    pub fn new(processor_map: &HashMap<String, String>) -> Processor {
        Processor {
            processor: processor_map.get_from_str("processor"),
            vendor_id: processor_map.get_from_str("vendor_id"),
            cpu_family: processor_map.get_from_str("cpu_family"),
            model: processor_map.get_from_str("model"),
            model_name: processor_map.get_from_str("model_name"),
            stepping: processor_map.get_from_str("stepping"),
            microcode: processor_map.get_from_str("microcode"),
            cpu_mhz: processor_map.get_from_str("cpu_mhz"),
            cache_size: processor_map.get_from_str("cache_size"),
            physical_id: processor_map.get_from_str("physical_id"),
            siblings: processor_map.get_from_str("siblings"),
            core_id: processor_map.get_from_str("core_id"),
            cpu_cores: processor_map.get_from_str("cpu_cores"),
            apicid: processor_map.get_from_str("apicid"),
            initial_apicid: processor_map.get_from_str("initial_apicid"),
            fpu: processor_map.get_from_str("fpu"),
            fpu_exception: processor_map.get_from_str("fpu_exception"),
            cpuid_level: processor_map.get_from_str("cpuid_level"),
            wp: processor_map.get_from_str("wp"),
            flags: processor_map.get_from_str("flags"),
            vmx_flags: processor_map.get_from_str("vmx_flags"),
            bugs: processor_map.get_from_str("bugs"),
            bogomips: processor_map.get_from_str("bogomips"),
            clflush_size: processor_map.get_from_str("clflush_size"),
            cache_alignment: processor_map.get_from_str("cache_alignment"),
            address_sizes: processor_map.get_from_str("address_sizes"),
            power_management: processor_map.get_from_str("power_management"),
        }
    }
}

fn get_processors() -> Result<Vec<Processor>, std::io::Error> {
    let mut processor: HashMap<String, String> = HashMap::new();
    let contents = read_to_string("/proc/cpuinfo")?;

    let mut processors: Vec<Processor> = Vec::new();

    for line in contents.split("\n") {
        let mut line = line.split(":");
        let key = line.next();
        let value = line.next();

        match (key, value) {
            (Some(k), Some(v)) => {
                let k = k.trim().replace(" ", "_").to_lowercase();
                processor.insert(k, v.trim_start().to_string());
            }
            _ => {
                if processor.len() > 0 {
                    processors.push(Processor::new(&processor));
                    processor.retain(|_, _| false);
                }
            }
        }
    }

    Ok(processors)
}
