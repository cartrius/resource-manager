use sysinfo::{Cpu, Disk, Disks, System};

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