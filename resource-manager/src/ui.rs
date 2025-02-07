use std::{
    io::{self, Result},
    time,
};
use std::{thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use sysinfo::{System};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::system::render_cpu_stats;

pub fn test_render_cpu_stats() -> Result<()> {
    // 1) Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2) Create a System to pass in
    let mut sys = System::new_all();
    sys.refresh_all(); // Just to ensure we have fresh CPU usage data

    // 3) Draw once
    terminal.draw(|frame| {
        let size = frame.size();
        // Call your function to render CPU stats in the entire terminal area
        render_cpu_stats(frame, &mut sys, size);
    })?;

    // 4) Pause for a few seconds so you can see it
    thread::sleep(Duration::from_secs(5));

    // 5) Exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn generate_terminal() -> Result<()> {
    

    enable_raw_mode()?;
   let mut stdout = io::stdout();
   execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
         .direction(Direction::Horizontal)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(40),
                 Constraint::Percentage(60),
             ].as_ref()
         )
         .split(f.size());
     let block = Block::default()
          .title("Stats")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[0]);
     let block = Block::default()
          .title("Processes")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[1]);
    })?; 

    thread::sleep(Duration::from_millis(5000));
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
