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

fn color_severity(s: String, num: f32) -> Span<'static> {
    if num > 75.5 {
        return Span::styled(s, Style::default().fg(Color::LightRed));
    } else if num > 50.0 {
        return Span::styled(s, Style::default().fg(Color::LightYellow));
    }
    return Span::styled(s, Style::default().fg(Color::LightGreen));
}

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
            Constraint::Percentage(28),
            Constraint::Percentage(72),
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
        .borders(Borders::NONE);

    f.render_widget(block, area);

    // Divide to split Global Usage and Individual CPU Usages
    let cpu_sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    // GLOBAL CPU USAGE
    let usage_val = stats.cpu_global_usage;
    let usage_str = format!("{:.2}%", usage_val);
    let usage_span = color_severity(usage_str, usage_val as f32);
    let global_cpu_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cpu_sub_chunks[0]);
    let label_paragraph = Paragraph::new("Global CPU Usage: ")
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    f.render_widget(label_paragraph, global_cpu_chunk[0]);
    let usage_paragraph = Paragraph::new(usage_span)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right);
    f.render_widget(usage_paragraph, global_cpu_chunk[1]);

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
        .borders(Borders::NONE);

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
    let mem_percentage_val = (stats.used_memory as f64 / stats.total_memory as f64) * 100.0;
    let mem_percentage_str = format!("{:.2}%", mem_percentage_val);
    let colored_span = color_severity(mem_percentage_str, mem_percentage_val as f32);

    // Render the “Memory"
    let label_paragraph = Paragraph::new("Memory: ")
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    f.render_widget(label_paragraph, mem_label_subchunks[0]);
    let value_paragraph = Paragraph::new(colored_span)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right);
    f.render_widget(value_paragraph, mem_num_subchunks[0]);
    render_label_value(f, "Total Memory: ", format!("{:.2} GB", (stats.total_memory as f64 / 1000000000.0)), mem_label_subchunks[2], mem_num_subchunks[2]);
    render_label_value(f, "Avail Memory: ", format!("{:.2} GB", (avail_mem as f64 / 1000000000.0)), mem_label_subchunks[3], mem_num_subchunks[3]);
    render_label_value(f, "Used Memory: ", format!("{:.2} GB", (stats.used_memory as f64 / 1000000000.0)), mem_label_subchunks[4], mem_num_subchunks[4]);
    render_label_value(f, "Free Memory: ", format!("{:.2} MB", (stats.free_memory as f64 / 1000000.0)), mem_label_subchunks[5], mem_num_subchunks[5]);
}

fn draw_disk_section<B: Backend>(f: &mut Frame<B>, disk_stats: &DisksStats, area: Rect) {
    let num_disks = disk_stats.disk_names.len();
    let constraints = vec![Constraint::Percentage(100 / (num_disks as u16)); num_disks];
    let disk_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(area);

    for i in 0..num_disks {
        let block = Block::default()
            .title(format!("Disk {i}"))
            .borders(Borders::ALL);
        f.render_widget(block.clone(), disk_chunks[i]);

        // Inner area for the disk block
        let inner_area = block.inner(disk_chunks[i]);
        let disk_sub_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(inner_area);

        // Color-code disk usage
        let usage_val = disk_stats.disk_usages[i].parse::<f32>().unwrap();
        let usage_str = format!("{:.2}%", usage_val);
        let usage_span = color_severity(usage_str, usage_val);

        // Left column label chunk
        let label_col = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Mount point
                Constraint::Length(1), // Name
                Constraint::Length(1), // Usage label
                Constraint::Length(1), // Filesystem
                Constraint::Length(1), // Kind
            ])
            .split(disk_sub_chunks[0]);

        // Right column value chunk
        let value_col = Layout::default()
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

        render_label_value(
            f,
            "Mount Point: ",
            disk_stats.disk_mnt_pts[i].clone(),
            label_col[0],
            value_col[0],
        );

        render_label_value(
            f,
            "Name: ",
            disk_stats.disk_names[i].clone(),
            label_col[1],
            value_col[1],
        );

        // Color-code Usage
        let label_usage = Paragraph::new("Usage: ")
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(label_usage, label_col[2]);

        let usage_par = Paragraph::new(usage_span)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Right);
        f.render_widget(usage_par, value_col[2]);

        render_label_value(
            f,
            "Filesystem: ",
            disk_stats.disk_filesystems[i].clone(),
            label_col[3],
            value_col[3],
        );

        render_label_value(
            f,
            "Kind: ",
            disk_stats.disk_kinds[i].clone(),
            label_col[4],
            value_col[4],
        );
    }

}

fn draw_system_section<B: Backend>(f: &mut Frame<B>, stats: &SystemStats, area: Rect) {
    let block = Block::default()
        .borders(Borders::NONE);
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