use std::{
    io::{self, Result},
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
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame, Terminal,
};
use sysinfo::System;

use crate::system::{collect_system_stats, collect_disks_stats, DisksStats, SystemStats};
use crate::processes::{ProcessInfo, collect_processes};

pub fn render_label_value<B: Backend>(f: &mut Frame<B>, label: &str, value: String, label_chunk: Rect, value_chunk: Rect) {
    let label_paragraph = Paragraph::new(Span::styled(label, Style::default()))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    let value_paragraph = Paragraph::new(Span::styled(value, Style::default()))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right);
    f.render_widget(label_paragraph, label_chunk);
    f.render_widget(value_paragraph, value_chunk);
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, disks: &DisksStats, processes: &[ProcessInfo]) {
    // Main terminal frame
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(32),
            Constraint::Percentage(68),
        ].as_ref())
        .split(f.size());

    create_stats_block(f, stats, disks, main_chunks[0]);
    create_processes_block(f, processes, main_chunks[1]);
}

pub fn create_processes_block<B: Backend>(f: &mut Frame<B>, processes: &[ProcessInfo], chunk: Rect) {
     let processes_block = Block::default()
     .title("Processes")
     .borders(Borders::ALL);
 f.render_widget(processes_block.clone(), chunk);

 let inner_area = processes_block.inner(chunk);
 let process_margined_chunk = Layout::default()
 .direction(Direction::Horizontal)
 .horizontal_margin(3)
 .vertical_margin(2)
 .constraints([Constraint::Percentage(100)].as_ref())
 .split(inner_area)[0];
 let mut rows = Vec::new();
 for p in processes {
     let mem_mb = (p.memory as f64) / 1000000.0;
     let euid_egid = match (p.euid.clone(), p.egid) {
        (Some(_), Some(_)) => format!("{:?}/{:?}", *p.euid.clone().unwrap(), *p.egid.unwrap()),
        (Some(_), None)       => format!("{:?} / N/A", *p.euid.clone().unwrap()),
        (None, Some(_))       => format!("N/A / {:?}", *p.egid.unwrap()),
        (None, None)            => String::from("N/A"),
    };

     let row = Row::new(vec![
         p.pid.to_string(),
         p.name.clone(),
         format!("{:.2}", mem_mb),
         format!("{:.2}%", p.cpu * 100.0), 
         p.uptime.to_string(),
         euid_egid,
     ]);

     rows.push(row);
 }

 // Column Names
 let header = Row::new(vec![
     "PID", "Name", "Mem (MB)", "CPU", "Uptime (s)", "EUID/EGID"
 ])
 .style(Style::default().fg(Color::Yellow))
 .bottom_margin(1);

 let table = Table::new(rows)
     .header(header)
     .block(
         Block::default().borders(Borders::NONE)
     )
     .widths(&[
        Constraint::Percentage(8),  // PID
        Constraint::Percentage(32), // NAME
        Constraint::Percentage(13), // MEM (MB)
        Constraint::Percentage(10), // CPU
        Constraint::Percentage(16), // UPTIME (s)
        Constraint::Percentage(19), // EUID/EGID
     ])
     .column_spacing(2) // extra space between columns
     .highlight_style(Style::default().add_modifier(Modifier::BOLD))
     .highlight_symbol(">>");

 f.render_widget(table, process_margined_chunk);
}

pub fn create_stats_block<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, disks: &DisksStats, chunk: Rect) {
    let block = Block::default()
        .title("Stats")
        .borders(Borders::ALL);
    f.render_widget(block, chunk);

    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints(
            [
                Constraint::Percentage(10 + (stats.cpu_names.len() as u16 * 2)), // cpu
                Constraint::Percentage(15),                              // mem
                Constraint::Percentage(50),                              // disks
                Constraint::Percentage(10),                               // system stats
            ]
            .as_ref(),
        )
        .split(chunk);

    draw_cpu_section(f, stats, sub_chunks[0]);
    draw_memory_section(f, stats, sub_chunks[1]);
    draw_disk_section(f, &disks, sub_chunks[2]);
    draw_system_section(f, stats, sub_chunks[3]);
}

fn draw_cpu_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("CPU")
        .borders(Borders::ALL);

    f.render_widget(block, area);

    // Divide to split Global Usage and Individual CPU Usages
    let cpu_sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);
    let global_usage = format!("{:.2}%", stats.cpu_global_usage);

    let global_cpu_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(cpu_sub_chunks[0]);
    render_label_value(f, "Global CPU Usage: ", global_usage, global_cpu_chunk[0], global_cpu_chunk[1]);
    let indiv_cpus_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(cpu_sub_chunks[1]);
    let num_cpus = stats.cpu_names.len();
    let constraints = vec![Constraint::Length(1); num_cpus];
    let indiv_cpus_label_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints.clone())
        .split(indiv_cpus_chunk[0]);

    let indiv_cpus_value_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(indiv_cpus_chunk[1]);

    for i in 0..num_cpus {
        let cpu_name = format!("CPU {}", stats.cpu_names[i]);
        render_label_value(f, &cpu_name, format!("{:.2}%", stats.cpu_cores[i]), indiv_cpus_label_chunk[i], indiv_cpus_value_chunk[i]);
    }

}

fn draw_memory_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("Memory")
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

    // EVERYTHING STILL NEEDS ROUNDING
    let mem_percentage = (stats.used_memory as f64 / stats.total_memory as f64 * 100.0).to_string();
    let avail_mem = stats.total_memory - stats.used_memory;
    render_label_value(f, "Memory: ", format!("{:.5}%", mem_percentage), mem_label_subchunks[0], mem_num_subchunks[0]);
    render_label_value(f, "Total Memory: ", format!("{:.2} GB", (stats.total_memory as f64 / 1000000000.0)), mem_label_subchunks[2], mem_num_subchunks[2]);
    render_label_value(f, "Avail Memory: ", format!("{:.2} GB", (avail_mem as f64 / 1000000000.0)), mem_label_subchunks[3], mem_num_subchunks[3]);
    render_label_value(f, "Used Memory: ", format!("{:.2} GB", (stats.used_memory as f64 / 1000000000.0)), mem_label_subchunks[4], mem_num_subchunks[4]);
    render_label_value(f, "Free Memory: ", format!("{:.2} MB", (stats.free_memory as f64 / 1000000.0)), mem_label_subchunks[5], mem_num_subchunks[5]);
}

fn draw_disk_section<B: Backend>(f: &mut Frame<B>, disk_stats: &DisksStats, area: Rect) {
    let num_disks = disk_stats.disk_names.len();
    // We can use a chunk to split the current area Rect/Chunk and then create blocks for each one
    let constraints =
        vec![Constraint::Percentage(100 / (u16::try_from(num_disks).unwrap())); num_disks];

    let disk_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(area);

    // For each disk
    for i in 0..num_disks {
        let block = Block::default()
            .title(format!("Disk {i}"))
            .borders(Borders::ALL);
        
        let disk_sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(disk_chunks[i]);

        let disk_label_subchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(disk_sub_chunks[0]);

        // System number subchunks
        let disk_num_subchunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(disk_sub_chunks[1]);
        
        render_label_value(f, "Mount Point: ", disk_stats.disk_mnt_pts[i].clone(), disk_label_subchunks[0], disk_num_subchunks[0]);
        render_label_value(f, "Name: ", disk_stats.disk_names[i].clone(), disk_label_subchunks[1], disk_num_subchunks[1]);
        render_label_value(f, "Usage: ", format!("{:.5}%", disk_stats.disk_usages[i].clone()), disk_label_subchunks[2], disk_num_subchunks[2]);
        render_label_value(f, "Filesystem: ", disk_stats.disk_filesystems[i].clone(), disk_label_subchunks[3], disk_num_subchunks[3]);
        render_label_value(f, "Kind: ", disk_stats.disk_kinds[i].clone(), disk_label_subchunks[4], disk_num_subchunks[4]);
        f.render_widget(block, disk_chunks[i]);
    }

}

fn draw_system_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .title("System")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    // System block chunk
    let sys_sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // System label subchunks
    let sys_label_subchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(sys_sub_chunks[0]);

    // System number subchunks
    let sys_num_subchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(sys_sub_chunks[1]);

    render_label_value(f, "Hostname: ", stats.host_name.clone().unwrap(), sys_label_subchunks[0], sys_num_subchunks[0]);
    render_label_value(f, "Version: ", stats.os_version.clone().unwrap(), sys_label_subchunks[1], sys_num_subchunks[1]);
    render_label_value(f, "Uptime: ", stats.uptime.to_string(), sys_label_subchunks[2], sys_num_subchunks[2]);
    render_label_value(f, "CPU_Arch: ", stats.arch.to_string(), sys_label_subchunks[3], sys_num_subchunks[3]);
    render_label_value(f, "OS: ", stats.os_name.clone().unwrap(), sys_label_subchunks[4], sys_num_subchunks[4]);
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
        let disks = collect_disks_stats();

        // Draw terminal
        terminal.draw(|frame| {
            draw_ui(frame, &stats, &disks, &processes);
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