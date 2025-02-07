use std::io;
use sysinfo::{System, Disks};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ui::test_render_cpu_stats;
// use system::render_memory_stats;
mod system;
mod processes;
mod ui;
use crate::ui::generate_terminal;
// use crate::system::render_sys_stats;
// use crate::system::render_cpu_stats;
// use crate::system::render_disk_stats;
// use crate::processes::add_processes;
use crate::system::collect_system_stats;
use crate::system::print_stats;
// use crate::processes::collect_processes;
use crate::processes::print_processes;

fn main() {
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    
    // Update all the information in the System struct
    sys.refresh_all();

    println!("-------");

    generate_terminal();
    test_render_cpu_stats();

}
