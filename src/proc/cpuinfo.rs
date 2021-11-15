use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

use crate::utils::HashMapStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct CPUInfo {
    pub processors: Vec<Processor>,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            processors: get_processors().unwrap(),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Processor {
    pub processor: Option<String>,
    pub vendor_id: Option<String>,
    pub cpu_family: Option<String>,
    pub model: Option<String>,
    pub model_name: Option<String>,
    pub stepping: Option<String>,
    pub microcode: Option<String>,
    pub cpu_mhz: Option<String>,
    pub cache_size: Option<String>,
    pub physical_id: Option<String>,
    pub siblings: Option<String>,
    pub core_id: Option<String>,
    pub cpu_cores: Option<String>,
    pub apicid: Option<String>,
    pub initial_apicid: Option<String>,
    pub fpu: Option<String>,
    pub fpu_exception: Option<String>,
    pub cpuid_level: Option<String>,
    pub wp: Option<String>,
    pub flags: Option<String>,
    pub vmx_flags: Option<String>,
    pub bugs: Option<String>,
    pub bogomips: Option<String>,
    pub clflush_size: Option<String>,
    pub cache_alignment: Option<String>,
    pub address_sizes: Option<String>,
    pub power_management: Option<String>,
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
