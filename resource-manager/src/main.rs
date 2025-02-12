use sysinfo::{System, Disks};
mod system;
mod processes;
mod ui;


fn main() {
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    
    // Update all the information in the System struct
    sys.refresh_all();

    println!("-------");

    //generate_terminal();
    // test_render_cpu_stats();
    // Maybe we can create the system here and since it through there run_terminal_ui function?
    ui::run_terminal_ui();

}
