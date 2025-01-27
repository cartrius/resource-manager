use sysinfo::{Pid, Process, System};

pub fn get_processes(sys: &System) -> Vec<(&Process)> {
    let mut process_vec = Vec::new();
    for (_, process) in sys.processes() {
       process_vec.push(process);
    }
    process_vec
}

pub fn add_processes(sys: &System) {
    let processes = get_processes(sys);
    for process in processes {
        let pid = process.pid();
        let name = process.name();
        let memory = process.memory();
        let cpu = process.cpu_usage();
        let uptime = process.run_time();
        let euid = process.effective_user_id();
        let egid = process.effective_user_id();
        println!("PID: {}", pid.to_string());
        println!("Name: {:?}", name); 
        println!("Memory: {:?}", memory); 
        println!("CPU: {:?}", cpu); 
        println!("Uptime: {:?}", uptime); 
        //println!("EUID/EGID: {:?}/{:?}", euid.unwrap(), egid.unwrap()); 
    }
    
}