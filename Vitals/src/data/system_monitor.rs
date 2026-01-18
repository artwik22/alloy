use core::system::{SystemMonitor, CpuData, MemoryData, NetworkData, ProcessInfo};
use std::sync::{Arc, Mutex, mpsc};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct DiskData {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_percent: f64,
}

#[derive(Clone, Debug)]
pub struct GpuData {
    pub available: bool,
    pub name: String,
    pub usage: f64,
    pub temperature: f64,
}

#[derive(Clone, Debug)]
pub struct EnhancedSystemData {
    pub cpu: CpuData,
    pub memory: MemoryData,
    pub network: NetworkData,
    pub processes: Vec<ProcessInfo>,
    pub disks: Vec<DiskData>,
    pub gpu: GpuData,
}

pub struct EnhancedSystemMonitor {
    sender: async_channel::Sender<EnhancedSystemData>,
}

impl EnhancedSystemMonitor {
    pub fn new(sender: async_channel::Sender<EnhancedSystemData>) -> Self {
        Self { sender }
    }
    
    pub fn start(self) {
        // Create mpsc channel for core SystemMonitor
        let (tx, rx) = mpsc::channel();
        
        // Start the core system monitor
        let monitor = SystemMonitor::new(tx);
        monitor.start();
        
        // Spawn async task to forward data with disk info
        glib::spawn_future_local(async move {
            let rx = Arc::new(Mutex::new(rx));
            let sender = self.sender.clone();
            let cached_disks = Rc::new(RefCell::new(Vec::<DiskData>::new()));
            let last_disk_refresh = Rc::new(RefCell::new(std::time::Instant::now() - std::time::Duration::from_secs(60)));
            
            glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
                let rx_guard = rx.lock().unwrap();
                
                // Try to receive system data
                if let Ok(data) = rx_guard.try_recv() {
                    let mut disks = cached_disks.borrow_mut();
                    let mut last_refresh = last_disk_refresh.borrow_mut();
                    
                    // Gather disk information every 10 seconds (expensive operation)
                    if last_refresh.elapsed() > std::time::Duration::from_secs(10) {
                        disks.clear();
                        let sys_disks = sysinfo::Disks::new_with_refreshed_list();
                        
                        for disk in sys_disks.list() {
                            let total = disk.total_space();
                            let available = disk.available_space();
                            let used = total.saturating_sub(available);
                            let used_percent = if total > 0 {
                                (used as f64 / total as f64) * 100.0
                            } else {
                                0.0
                            };
                            
                            disks.push(DiskData {
                                name: disk.name().to_string_lossy().to_string(),
                                mount_point: disk.mount_point().to_string_lossy().to_string(),
                                total_space: total,
                                available_space: available,
                                used_percent,
                            });
                        }
                        *last_refresh = std::time::Instant::now();
                    }
                    
                    // GPU placeholder (no real GPU monitoring yet)
                    let gpu = GpuData {
                        available: false,
                        name: "Not Available".to_string(),
                        usage: 0.0,
                        temperature: 0.0,
                    };
                    
                    let enhanced_data = EnhancedSystemData {
                        cpu: data.cpu,
                        memory: data.memory,
                        network: data.network,
                        processes: data.processes,
                        disks: disks.clone(),
                        gpu,
                    };
                    
                    let _ = sender.send_blocking(enhanced_data);
                }
                
                glib::ControlFlow::Continue
            });
        });
    }
}
