use sysinfo::{System, Disks};
use system::render_memory_stats;
mod system;
use crate::system::render_sys_stats;
use crate::system::render_cpu_stats;
use crate::system::render_disk_stats;

fn main() {
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    
    // Update all the information in the System struct
    sys.refresh_all();

    render_sys_stats();
    render_cpu_stats(&sys);
    render_memory_stats(&sys);
    render_disk_stats(&disks);
    // let mut usage = get_cpu_usage(&sys);
    // println!("CPU Usage: {}%", usage)

//     println!("=> system:");
// // RAM and swap information:
// println!("total memory: {} bytes", sys.total_memory());
// println!("used memory : {} bytes", sys.used_memory());
// println!("total swap  : {} bytes", sys.total_swap());
// println!("used swap   : {} bytes", sys.used_swap());

// // Display system information:
// println!("System name:             {:?}", System::name());
// println!("System kernel version:   {:?}", System::kernel_version());
// println!("System OS version:       {:?}", System::os_version());
// println!("System host name:        {:?}", System::host_name());

// // Number of CPUs:
// println!("NB CPUs: {}", sys.cpus().len());

// // Display processes ID, name na disk usage:
// for (pid, process) in sys.processes() {
//     println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
// }

// // We display all disks' information:
// println!("=> disks:");
// let disks = Disks::new_with_refreshed_list();
// for disk in &disks {
//     println!("{disk:?}");
// }

// // Network interfaces name, total data received and total data transmitted:
// let networks = Networks::new_with_refreshed_list();
// println!("=> networks:");
// for (interface_name, data) in &networks {
//     println!(
//         "{interface_name}: {} B (down) / {} B (up)",
//         data.total_received(),
//         data.total_transmitted(),
//     );
//     // If you want the amount of data received/transmitted since last call
//     // to `Networks::refresh`, use `received`/`transmitted`.
// }

// // Components temperature:
// let components = Components::new_with_refreshed_list();
// println!("=> components:");
// for component in &components {
//     println!("{component:?}");
// }


}
