use sysinfo::{Cpu, Disks, System};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct SystemStats {
    pub host_name: Option<String>,
    pub os_version: Option<String>,
    pub uptime: u64,
    pub arch: String,
    pub os_name: Option<String>,
    pub cpu_names: Vec<String>,
    pub cpu_global_usage: f32,
    pub cpu_cores: Vec<f32>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

pub fn collect_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_all();
    let cpu_names = sys
            .cpus()
            .iter()
            .map(|cpu| cpu.name().to_string())
            .collect::<Vec<String>>();
    let cpu_cores_usage = sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect::<Vec<f32>>();

    SystemStats {
        host_name: System::host_name(),
        os_version: System::os_version(),
        uptime: System::uptime(),
        arch: System::cpu_arch(),
        os_name: System::name(),
        cpu_names: cpu_names,
        cpu_global_usage: sys.global_cpu_usage(),
        cpu_cores: cpu_cores_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
    }
}

pub fn print_stats(stats: &SystemStats) {
    println!("Host Name: {:?}", stats.host_name);
    println!("OS Version: {:?}", stats.os_version);
    println!("Uptime: {}", stats.uptime);
    println!("Arch: {:?}", stats.arch);
    println!("OS Name: {:?}", stats.os_name);
    println!("CPU Usage (global): {:.2}%", stats.cpu_global_usage);
    println!("CPU usage per core: {:?}", stats.cpu_cores);
    println!("Total memory: {}", stats.total_memory);
    println!("Used memory: {}", stats.used_memory);
    println!("Free memory: {}", stats.free_memory);
}

// Move in refactor later
pub fn render_cpu_stats<B: Backend>(f: &mut Frame<B>, sys: &mut System, chunk: Rect) {
    let cpu_stats = collect_system_stats(sys);
    let cpu_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Length(10)])
        .split(chunk);

    // CPU Global Stats
    let global_cpu_usage = cpu_stats.cpu_global_usage;
    let prefix = Span::styled("Global CPU Usage: ", Style::default());
    let percentage = Span::styled(format!("{:.2}%", global_cpu_usage), Style::default());
    let global_percentage = Spans::from(vec![prefix, percentage]);
    let global_percentage_text = Paragraph::new(global_percentage)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left);
    f.render_widget(global_percentage_text, cpu_chunk[0])


}
