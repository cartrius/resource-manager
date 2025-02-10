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
