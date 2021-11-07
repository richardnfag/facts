use std::collections::HashMap;
use std::fs::read_to_string;

trait HashMapMember {
    fn get_member(&self, member: &str) -> Option<i64>;
}

impl HashMapMember for HashMap<String, i64> {
    fn get_member(self: &HashMap<String, i64>, member: &str) -> Option<i64> {
        match self.get(&member.to_string()) {
            Some(m) => Some(m.clone()),
            None => None,
        }
    }
}

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
            mem_total: meminfo.get_member("memtotal"),
            mem_free: meminfo.get_member("memfree"),
            mem_available: meminfo.get_member("memavailable"),
            buffers: meminfo.get_member("buffers"),
            cached: meminfo.get_member("cached"),
            swap_cached: meminfo.get_member("swapcached"),
            active: meminfo.get_member("active"),
            inactive: meminfo.get_member("inactive"),
            active_anon: meminfo.get_member("active_anon"),
            inactive_anon: meminfo.get_member("inactive_anon"),
            active_file: meminfo.get_member("active_file"),
            inactive_file: meminfo.get_member("inactive_file"),
            unevictable: meminfo.get_member("unevictable"),
            mlocked: meminfo.get_member("mlocked"),
            swap_total: meminfo.get_member("swaptotal"),
            swap_free: meminfo.get_member("swapfree"),
            dirty: meminfo.get_member("dirty"),
            writeback: meminfo.get_member("writeback"),
            anon_pages: meminfo.get_member("anonpages"),
            mapped: meminfo.get_member("mapped"),
            shmem: meminfo.get_member("shmem"),
            k_reclaimable: meminfo.get_member("kreclaimable"),
            slab: meminfo.get_member("slab"),
            s_reclaimable: meminfo.get_member("sreclaimable"),
            s_unreclaim: meminfo.get_member("sunreclaim"),
            kernel_stack: meminfo.get_member("kernelstack"),
            page_tables: meminfo.get_member("pagetables"),
            nfs_unstable: meminfo.get_member("nfs_unstable"),
            bounce: meminfo.get_member("bounce"),
            writeback_tmp: meminfo.get_member("writebacktmp"),
            commit_limit: meminfo.get_member("commitlimit"),
            committed_as: meminfo.get_member("committed_as"),
            vmalloc_total: meminfo.get_member("vmalloctotal"),
            vmalloc_used: meminfo.get_member("vmallocused"),
            vmalloc_chunk: meminfo.get_member("vmallocchunk"),
            percpu: meminfo.get_member("percpu"),
            hardware_corrupted: meminfo.get_member("hardwarecorrupted"),
            anon_huge_pages: meminfo.get_member("anonhugepages"),
            shmem_huge_pages: meminfo.get_member("shmemhugepages"),
            shmem_pmd_mapped: meminfo.get_member("shmempmdmapped"),
            file_huge_pages: meminfo.get_member("filehugepages"),
            file_pmd_mapped: meminfo.get_member("filepmdmapped"),
            cma_total: meminfo.get_member("cmatotal"),
            cma_free: meminfo.get_member("cmafree"),
            huge_pages_total: meminfo.get_member("hugepages_total"),
            huge_pages_free: meminfo.get_member("hugepages_free"),
            huge_pages_rsvd: meminfo.get_member("hugepages_rsvd"),
            huge_pages_surp: meminfo.get_member("hugepages_surp"),
            hugepagesize: meminfo.get_member("hugepagesize"),
            hugetlb: meminfo.get_member("hugetlb"),
            direct_map4k: meminfo.get_member("directmap4k"),
            direct_map2m: meminfo.get_member("directmap2m"),
            direct_map1g: meminfo.get_member("directmap1g"),
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
