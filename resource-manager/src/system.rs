use sysinfo::{Disks, System};

pub struct SystemStats {
    pub host_name: Option<String>,
    pub os_version: Option<String>,
    pub uptime: u64,
    pub arch: String,
    pub os_name: Option<String>,
    pub cpu_names: Vec<String>,
    pub cpu_global_usage: f32,
    pub cpu_cores: Vec<f32>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

pub struct DisksStats {
    pub disk_names: Vec<String>,
    pub disk_mnt_pts: Vec<String>,
    pub disk_usages: Vec<String>,
    pub disk_filesystems: Vec<String>,
    pub disk_kinds: Vec<String>,
}

pub fn collect_disks_stats() -> DisksStats {
    let disks = Disks::new_with_refreshed_list();
    let disk_list = disks.list();
    let disk_names = disk_list
            .iter()
            .map(|disk| disk.name().to_string_lossy().to_string())
            .collect::<Vec<String>>();
    let disk_mnts = disk_list
        .iter()
        .map(|disk| disk.mount_point().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    let disk_systems = disk_list
        .iter()
        .map(|disk| disk.file_system().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    let disk_kinds = disk_list
        .iter()
        .map(|disk| disk.kind().to_string())
        .collect::<Vec<String>>();
    let mut disk_usgs: Vec<String> = Vec::new();
    for disk in disks.list() {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let percentage_used = ((used_space as f64 / total_space as f64) * 100.0) as f32;
        disk_usgs.push(percentage_used.to_string());
    }
        
    DisksStats {
        disk_names: disk_names,
        disk_mnt_pts: disk_mnts,
        disk_usages: disk_usgs,
        disk_filesystems: disk_systems,
        disk_kinds: disk_kinds,
    }
}

pub fn collect_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_all();
    let cpu_names = sys
            .cpus()
            .iter()
            .map(|cpu| cpu.name().to_string())
            .collect::<Vec<String>>();
    let cpu_cores_usage = sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect::<Vec<f32>>();

    SystemStats {
        host_name: System::host_name(),
        os_version: System::os_version(),
        uptime: System::uptime(),
        arch: System::cpu_arch(),
        os_name: System::name(),
        cpu_names: cpu_names,
        cpu_global_usage: sys.global_cpu_usage(),
        cpu_cores: cpu_cores_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
    }
}
