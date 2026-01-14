use sysinfo::{Networks, System};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use std::thread;

#[derive(Clone, Debug)]
pub struct CpuData {
    pub usage: f64,
    pub cores: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct MemoryData {
    pub used: u64,
    pub total: u64,
    pub used_percent: f64,
}

#[derive(Clone, Debug)]
pub struct NetworkData {
    pub received: u64,
    pub transmitted: u64,
    pub received_rate: f64,
    pub transmitted_rate: f64,
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub memory: u64,
    pub memory_percent: f64,
}

#[derive(Clone, Debug)]
pub struct SystemData {
    pub cpu: CpuData,
    pub memory: MemoryData,
    pub network: NetworkData,
    pub processes: Vec<ProcessInfo>,
    pub timestamp: Instant,
}

pub struct SystemMonitor {
    system: Arc<Mutex<System>>,
    networks: Arc<Mutex<Networks>>,
    sender: mpsc::Sender<SystemData>,
    last_network: Arc<Mutex<(u64, u64)>>,
    last_network_time: Arc<Mutex<Instant>>,
}

impl SystemMonitor {
    pub fn new(sender: mpsc::Sender<SystemData>) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let networks = Networks::new_with_refreshed_list();
        let mut total_received = 0u64;
        let mut total_transmitted = 0u64;
        
        for (_, network) in networks.iter() {
            total_received += network.received();
            total_transmitted += network.transmitted();
        }
        
        Self {
            system: Arc::new(Mutex::new(system)),
            networks: Arc::new(Mutex::new(networks)),
            sender,
            last_network: Arc::new(Mutex::new((total_received, total_transmitted))),
            last_network_time: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    pub fn start(self) {
        thread::spawn(move || {
            loop {
                let mut system = self.system.lock().unwrap();
                let mut networks = self.networks.lock().unwrap();
                
                system.refresh_cpu_all();
                system.refresh_memory();
                networks.refresh();
                system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                
                // CPU data
                let cpu_usage = system.cpus().iter().map(|c| c.cpu_usage() as f64).sum::<f64>() / system.cpus().len() as f64;
                let mut cores = Vec::new();
                for cpu in system.cpus() {
                    cores.push(cpu.cpu_usage() as f64);
                }
                
                // Memory data
                let total_memory = system.total_memory();
                let used_memory = system.used_memory();
                let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;
                
                // Network data
                let mut total_received = 0u64;
                let mut total_transmitted = 0u64;
                
                for (_, network) in networks.iter() {
                    total_received += network.received();
                    total_transmitted += network.transmitted();
                }
                
                let now = Instant::now();
                let mut last_network = self.last_network.lock().unwrap();
                let mut last_network_time = self.last_network_time.lock().unwrap();
                let elapsed = now.duration_since(*last_network_time).as_secs_f64();
                
                let received_rate = if elapsed > 0.0 {
                    ((total_received.saturating_sub(last_network.0)) as f64 / elapsed) / 1024.0 // KB/s
                } else {
                    0.0
                };
                
                let transmitted_rate = if elapsed > 0.0 {
                    ((total_transmitted.saturating_sub(last_network.1)) as f64 / elapsed) / 1024.0 // KB/s
                } else {
                    0.0
                };
                
                *last_network = (total_received, total_transmitted);
                *last_network_time = now;
                
                // Process data
                let total_memory_for_processes = total_memory;
                let mut processes: Vec<ProcessInfo> = system
                    .processes()
                    .iter()
                    .map(|(pid, process)| ProcessInfo {
                        pid: pid.as_u32(),
                        name: process.name().to_string_lossy().to_string(),
                        cpu_usage: process.cpu_usage() as f64,
                        memory: process.memory(),
                        memory_percent: (process.memory() as f64 / total_memory_for_processes as f64) * 100.0,
                    })
                    .collect();
                
                // Sort by CPU usage
                processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
                processes.truncate(50); // Limit to top 50
                
                drop(system);
                drop(networks);
                drop(last_network);
                drop(last_network_time);
                
                let data = SystemData {
                    cpu: CpuData {
                        usage: cpu_usage,
                        cores,
                    },
                    memory: MemoryData {
                        used: used_memory,
                        total: total_memory,
                        used_percent: memory_percent,
                    },
                    network: NetworkData {
                        received: total_received,
                        transmitted: total_transmitted,
                        received_rate,
                        transmitted_rate,
                    },
                    processes,
                    timestamp: now,
                };
                
                let _ = self.sender.send(data);
                
                thread::sleep(Duration::from_millis(500)); // Update every 500ms for 60 FPS
            }
        });
    }
}
