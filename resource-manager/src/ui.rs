use std::{
    io::{self, Result},
    thread,
    time::Duration,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Layout, Constraint, Direction, Rect},
    text::Span,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use sysinfo::System;

use crate::system::{collect_system_stats, render_cpu_stats, SystemStats};
use crate::processes::{ProcessInfo, collect_processes};

pub fn render_label_value<B: Backend>(f: &mut Frame<B>, label: &str, value: String, label_chunk: Rect, value_chunk: Rect) {
    let label_paragraph = Paragraph::new(Span::styled(label, Style::default()))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    let value_paragraph = Paragraph::new(Span::styled(value, Style::default()))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    f.render_widget(label_paragraph, label_chunk);
    f.render_widget(value_paragraph, value_chunk);
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, processes: &[ProcessInfo]) {
    // Main terminal frame
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ].as_ref())
        .split(f.size());

    create_stats_block(f, stats, main_chunks[0]);
    create_processes_block(f, processes, main_chunks[1]);

    // let stats_area = main_chunks[0];
    // let processes_area = main_chunks[1];

    // let stats_chunks = Layout::default()
    //         .direction(Direction::Vertical)
    //         .constraints([
    //             Constraint::Length(7), // CPU
    //             Constraint::Length(7), // Memory
    //             Constraint::Length(7), // Disk - maybe System chunk too?
    //         ].as_ref())
    //         .split(stats_area);

    // // Draw chunks within respective chunks
    // draw_cpu_section(f, stats, stats_chunks[0]);
    // draw_memory_section(f, stats, stats_chunks[1]);
    // draw_disk_section(f, stats, stats_chunks[2]);
    // draw_processes_section(f, processes, processes_area);
}

pub fn create_processes_block<B: Backend>(f: &mut Frame<B>, processes: &[ProcessInfo], chunk: Rect) {
    let block = Block::default()
        .title("Processes")
        .borders(Borders::ALL);
    f.render_widget(block, chunk);
}

pub fn create_stats_block<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, chunk: Rect) {
    let block = Block::default()
        .title("Stats")
        .borders(Borders::ALL);
    f.render_widget(block, chunk);

    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .vertical_margin(1)
        .constraints(
            [
                // Constraint::Percentage(7 + (num_cpus * 2)), // cpu
                Constraint::Percentage(7),
                Constraint::Percentage(160),                 // mem
                Constraint::Percentage(31),                 // TBD
                Constraint::Percentage(20),                 // metadata
            ]
            .as_ref(),
        )
        .split(chunk);

    draw_cpu_section(f, stats, sub_chunks[0]);
    draw_memory_section(f, stats, sub_chunks[1]);
    draw_disk_section(f, stats, sub_chunks[2]);
    draw_system_section(f, stats, sub_chunks[3]);
}

fn draw_cpu_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("CPU Stats")
        .borders(Borders::ALL);

    f.render_widget(block, area);

    let cpu_sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .vertical_margin(1)
        .constraints(
            [
                // Constraint::Percentage(7 + (num_cpus * 2)), // cpu
                Constraint::Percentage(20),
                Constraint::Percentage(20),   
                Constraint::Percentage(20),      
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

}

fn draw_memory_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("Memory Stats")
        .borders(Borders::ALL);

    f.render_widget(block, area);

    // Memory block chunk
    let mem_sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        //                         Memory: XX.XX%    [|||       ]
        //                         Total Memory:         XX.XX GB
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Memory label subchunks
    let mem_label_subchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(mem_sub_chunks[0]);

    // Memory number subchunks
    let mem_num_subchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(mem_sub_chunks[1]);

    let avail_mem = stats.total_memory - stats.used_memory;
    render_label_value(f, "Total Memory: ", stats.total_memory.to_string(), mem_label_subchunks[2], mem_num_subchunks[2]);
    render_label_value(f, "Avail Memory: ", avail_mem.to_string(), mem_label_subchunks[3], mem_num_subchunks[3]);
    render_label_value(f, "Used Memory: ", stats.used_memory.to_string(), mem_label_subchunks[4], mem_num_subchunks[4]);
    render_label_value(f, "Free Memory: ", stats.free_memory.to_string(), mem_label_subchunks[5], mem_num_subchunks[5]);
}

fn draw_disk_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
    .title("Disk Stats")
    .borders(Borders::ALL);

    f.render_widget(block, area);
}

fn draw_system_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("System Stats")
        .borders(Borders::ALL);
    f.render_widget(block, area)
}

fn draw_processes_section<B: Backend>(f: &mut Frame<B>, processes: &[ProcessInfo], area: Rect) {
    let block = Block::default().title("Processes").borders(Borders::ALL);
    f.render_widget(block, area);

    // let inner_area = block.inner(area);
    // Maybe use a Table or Paragraph to show info
    // let table = Table::new(...)
    // f.render_widget(table, inner_area);
}


pub fn run_terminal_ui() -> Result<()> {
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

        // Draw terminal
        terminal.draw(|frame| {
            draw_ui(frame, &stats, &processes);
        })?;

        // Check if user pressed 'q' or `ESC` to quit
        if crossterm::event::poll(Duration::from_millis(200))? {
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