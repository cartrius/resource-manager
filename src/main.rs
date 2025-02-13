use sysinfo::System;
mod processes;
mod system;
mod ui;
use crate::processes::collect_processes;
use crate::system::{collect_disks_stats, collect_system_stats};
use crate::ui::draw_ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Result},
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Setup terminal in raw mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut sys = System::new_all();

    loop {
        // Refresh data
        sys.refresh_all();
        // Collect stats and processes
        let stats = collect_system_stats(&mut sys);
        let processes = collect_processes(&sys);
        let disks = collect_disks_stats();

        // Draw terminal
        terminal.draw(|frame| {
            draw_ui(frame, &stats, &disks, &processes);
        })?;

        // Check if user pressed 'q' or `ESC` to quit
        if crossterm::event::poll(Duration::from_millis(400))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
