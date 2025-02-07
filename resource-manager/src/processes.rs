use sysinfo::{Pid, Uid, Gid, Process, System};

pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub memory: u64,
    pub cpu: f32,
    pub uptime: u64,
    pub euid: Option<Uid>,
    pub egid: Option<Gid>,
}

pub fn collect_processes(sys: &System) -> Vec<ProcessInfo> {
    let mut process_info_vec: Vec<ProcessInfo> = Vec::new();
    for (_, process) in sys.processes() {
        process_info_vec.push(ProcessInfo {
            pid: process.pid(),
            name: process.name().to_string_lossy().to_string(),
            memory: process.memory(),
            cpu: process.cpu_usage(),
            uptime: process.run_time(),
            euid: process.effective_user_id().cloned(),
            egid: process.effective_group_id(),
        })
    }
    process_info_vec
}

pub fn print_processes(sys: &System) {
    let procs = collect_processes(sys);
    for p in procs {
        println!("PID: {:?}", p.pid);
        println!("Name: {:?}", p.name);
        println!("Memory: {:?}", p.memory);
        println!("CPU: {:?}", p.cpu);
        println!("Uptime: {:?}", p.uptime);
        // println!("EUID: {:?}", p.euid.unwrap());
        // println!("EGID: {:?}", p.egid.unwrap());
        println!("-------------------");
    }
}

