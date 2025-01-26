use sysinfo::System;

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