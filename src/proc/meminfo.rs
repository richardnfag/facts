use std::collections::HashMap;
use std::fs::read_to_string;

use crate::utils::HashMapStr;

pub struct MemInfo {
    pub mem_total: Option<i64>,
    pub mem_free: Option<i64>,
    pub mem_available: Option<i64>,
    pub buffers: Option<i64>,
    pub cached: Option<i64>,
    pub swap_cached: Option<i64>,
    pub active: Option<i64>,
    pub inactive: Option<i64>,
    pub active_anon: Option<i64>,
    pub inactive_anon: Option<i64>,
    pub active_file: Option<i64>,
    pub inactive_file: Option<i64>,
    pub unevictable: Option<i64>,
    pub mlocked: Option<i64>,
    pub swap_total: Option<i64>,
    pub swap_free: Option<i64>,
    pub dirty: Option<i64>,
    pub writeback: Option<i64>,
    pub anon_pages: Option<i64>,
    pub mapped: Option<i64>,
    pub shmem: Option<i64>,
    pub k_reclaimable: Option<i64>,
    pub slab: Option<i64>,
    pub s_reclaimable: Option<i64>,
    pub s_unreclaim: Option<i64>,
    pub kernel_stack: Option<i64>,
    pub page_tables: Option<i64>,
    pub nfs_unstable: Option<i64>,
    pub bounce: Option<i64>,
    pub writeback_tmp: Option<i64>,
    pub commit_limit: Option<i64>,
    pub committed_as: Option<i64>,
    pub vmalloc_total: Option<i64>,
    pub vmalloc_used: Option<i64>,
    pub vmalloc_chunk: Option<i64>,
    pub percpu: Option<i64>,
    pub hardware_corrupted: Option<i64>,
    pub anon_huge_pages: Option<i64>,
    pub shmem_huge_pages: Option<i64>,
    pub shmem_pmd_mapped: Option<i64>,
    pub file_huge_pages: Option<i64>,
    pub file_pmd_mapped: Option<i64>,
    pub cma_total: Option<i64>,
    pub cma_free: Option<i64>,
    pub huge_pages_total: Option<i64>,
    pub huge_pages_free: Option<i64>,
    pub huge_pages_rsvd: Option<i64>,
    pub huge_pages_surp: Option<i64>,
    pub hugepagesize: Option<i64>,
    pub hugetlb: Option<i64>,
    pub direct_map4k: Option<i64>,
    pub direct_map2m: Option<i64>,
    pub direct_map1g: Option<i64>,
}

impl MemInfo {
    pub fn new() -> MemInfo {
        let meminfo = get_meminfo().unwrap();

        MemInfo {
            mem_total: meminfo.get_from_str("memtotal"),
            mem_free: meminfo.get_from_str("memfree"),
            mem_available: meminfo.get_from_str("memavailable"),
            buffers: meminfo.get_from_str("buffers"),
            cached: meminfo.get_from_str("cached"),
            swap_cached: meminfo.get_from_str("swapcached"),
            active: meminfo.get_from_str("active"),
            inactive: meminfo.get_from_str("inactive"),
            active_anon: meminfo.get_from_str("active_anon"),
            inactive_anon: meminfo.get_from_str("inactive_anon"),
            active_file: meminfo.get_from_str("active_file"),
            inactive_file: meminfo.get_from_str("inactive_file"),
            unevictable: meminfo.get_from_str("unevictable"),
            mlocked: meminfo.get_from_str("mlocked"),
            swap_total: meminfo.get_from_str("swaptotal"),
            swap_free: meminfo.get_from_str("swapfree"),
            dirty: meminfo.get_from_str("dirty"),
            writeback: meminfo.get_from_str("writeback"),
            anon_pages: meminfo.get_from_str("anonpages"),
            mapped: meminfo.get_from_str("mapped"),
            shmem: meminfo.get_from_str("shmem"),
            k_reclaimable: meminfo.get_from_str("kreclaimable"),
            slab: meminfo.get_from_str("slab"),
            s_reclaimable: meminfo.get_from_str("sreclaimable"),
            s_unreclaim: meminfo.get_from_str("sunreclaim"),
            kernel_stack: meminfo.get_from_str("kernelstack"),
            page_tables: meminfo.get_from_str("pagetables"),
            nfs_unstable: meminfo.get_from_str("nfs_unstable"),
            bounce: meminfo.get_from_str("bounce"),
            writeback_tmp: meminfo.get_from_str("writebacktmp"),
            commit_limit: meminfo.get_from_str("commitlimit"),
            committed_as: meminfo.get_from_str("committed_as"),
            vmalloc_total: meminfo.get_from_str("vmalloctotal"),
            vmalloc_used: meminfo.get_from_str("vmallocused"),
            vmalloc_chunk: meminfo.get_from_str("vmallocchunk"),
            percpu: meminfo.get_from_str("percpu"),
            hardware_corrupted: meminfo.get_from_str("hardwarecorrupted"),
            anon_huge_pages: meminfo.get_from_str("anonhugepages"),
            shmem_huge_pages: meminfo.get_from_str("shmemhugepages"),
            shmem_pmd_mapped: meminfo.get_from_str("shmempmdmapped"),
            file_huge_pages: meminfo.get_from_str("filehugepages"),
            file_pmd_mapped: meminfo.get_from_str("filepmdmapped"),
            cma_total: meminfo.get_from_str("cmatotal"),
            cma_free: meminfo.get_from_str("cmafree"),
            huge_pages_total: meminfo.get_from_str("hugepages_total"),
            huge_pages_free: meminfo.get_from_str("hugepages_free"),
            huge_pages_rsvd: meminfo.get_from_str("hugepages_rsvd"),
            huge_pages_surp: meminfo.get_from_str("hugepages_surp"),
            hugepagesize: meminfo.get_from_str("hugepagesize"),
            hugetlb: meminfo.get_from_str("hugetlb"),
            direct_map4k: meminfo.get_from_str("directmap4k"),
            direct_map2m: meminfo.get_from_str("directmap2m"),
            direct_map1g: meminfo.get_from_str("directmap1g"),
        }
    }
}

fn get_meminfo() -> Result<HashMap<String, i64>, std::io::Error> {
    let mut meminfo: HashMap<String, i64> = HashMap::new();
    let contents = read_to_string("/proc/meminfo")?;

    for line in contents.split("\n") {
        let mut line = line.split(":");
        let key = line.next();
        let value = line.next();

        match (key, value) {
            (Some(k), Some(v)) => {
                let k = k
                    .trim()
                    .replace(" ", "_")
                    .replace("(", "_")
                    .replace(")", "")
                    .to_lowercase();
                let v = v
                    .trim_start()
                    .to_lowercase()
                    .replace("kb", "")
                    .replace(" ", "")
                    .parse::<i64>()
                    .unwrap();
                meminfo.insert(k, v);
            }
            _ => {}
        }
    }

    Ok(meminfo)
}
