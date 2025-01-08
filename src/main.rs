use eframe::egui::{self, IconData, ViewportBuilder};
use sysinfo::System;

struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }
}

impl eframe::App for SystemMonitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("System Monitor");
            ui.add_space(20.0);
            
            // Memory usage
            let memory_percentage = self.system.used_memory() as f32 / self.system.total_memory() as f32;
            ui.label("Memory Usage:");
            ui.add(egui::ProgressBar::new(memory_percentage)
                .text(format!("{:.1}%", memory_percentage * 100.0)));
            
            ui.add_space(20.0);
            
            // CPU usage
            let cpu_usage = self.system.global_cpu_usage();
            ui.label("CPU Usage:");
            ui.add(egui::ProgressBar::new(cpu_usage / 100.0)
                .text(format!("{:.1}%", cpu_usage)));
            
            // Request repaint every second
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([300.0, 150.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "System Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(SystemMonitor::new())))
    )
}
