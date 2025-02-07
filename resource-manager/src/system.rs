use sysinfo::{Cpu, Disk, Disks, System};

pub struct SystemStats {
    pub host_name: Option<String>,
    pub os_version: Option<String>,
    pub uptime: u64,
    pub arch: String,
    pub os_name: Option<String>,
    pub cpu_global_usage: f32,
    pub cpu_cores: Vec<f32>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

pub fn collect_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_all();
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
        cpu_global_usage: sys.global_cpu_usage(),
        cpu_cores: cpu_cores_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
    }
}

pub fn print_stats(stats: &SystemStats) {
    println!("Host Name: {:?}", stats.host_name);
    println!("OS Version: {:?}", stats.os_version);
    println!("Uptime: {}", stats.uptime);
    println!("Arch: {:?}", stats.arch);
    println!("OS Name: {:?}", stats.os_name);
    println!("CPU Usage (global): {:.2}%", stats.cpu_global_usage);
    println!("CPU usage per core: {:?}", stats.cpu_cores);
    println!("Total memory: {}", stats.total_memory);
    println!("Used memory: {}", stats.used_memory);
    println!("Free memory: {}", stats.free_memory);
}

pub fn get_cpu_usage(sys: &System) -> f32 {
    sys.global_cpu_usage()
}

fn get_individual_cpus(sys: &System) -> Vec<&Cpu> {
    let mut cpu_vec = Vec::new();
    for cpu in sys.cpus() {
        cpu_vec.push(cpu);
    }
    cpu_vec
}

fn get_individual_disks(disks: &Disks) -> Vec<&Disk> {
    let mut disk_vec = Vec::new();
    for disk in disks.list() {
        disk_vec.push(disk)
    }
    disk_vec
}

pub fn render_sys_stats() {
    let host_name = System::host_name();
    let version = System::os_version();
    let uptime = System::uptime();
    let arch = System::cpu_arch();
    let os = System::name();

    println!("System host name: {:?}", host_name);
    println!("System os version: {:?}", version);
    println!("System uptime: {:?}", uptime);
    println!("System arch: {:?}", arch);
    println!("System operating system {:?}", os);
}

pub fn render_cpu_stats(sys: &System) {
    println!("CPU Usage: {:?}", get_cpu_usage(sys));
    println!("Individual CPS: {:?}", get_individual_cpus(sys));
}

pub fn render_memory_stats(sys: &System) {
    let total_memory = sys.total_memory();
    let avail_memory = sys.available_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();
    println!("Total memory: {}", total_memory);
    println!("Available memory: {}", avail_memory);
    println!("Used memory: {}", used_memory);
    println!("Free memory: {}", free_memory);
}

pub fn render_disk_stats(disks: &Disks) {
    println!("DISKS: {:?}", get_individual_disks(disks));
}