use std::error::Error;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use sysinfo;
use sysinfo::{DiskExt, SystemExt};
use systemstat;
use systemstat::{CPULoad, DelayedMeasurement, Platform};

#[derive(Clone)]
pub struct HardwareService {
    cpu_usage: Arc<RwLock<Vec<f64>>>,
    quit: Sender<()>,
}

type CpuMeasurement = DelayedMeasurement<Vec<CPULoad>>;

impl HardwareService {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel();
        (
            Self {
                cpu_usage: Arc::new(RwLock::new(Vec::new())),
                quit: tx,
            },
            rx,
        )
    }

    pub fn run_thread() -> HardwareService {
        let (mut hardware_service, quit_rx) = HardwareService::new();
        let hardware_service_ret = hardware_service.clone();

        thread::Builder::new()
            .name("hardware".to_string())
            .spawn(move || {
                let mut measurement = None;
                loop {
                    measurement = match hardware_service.update(measurement) {
                        Ok(measurement) => Some(measurement),
                        Err(err) => {
                            cwarn!(HARDWARE, "Error get cpu info {}", err);
                            None
                        }
                    };
                    match quit_rx.recv_timeout(Duration::new(1, 0)) {
                        Err(RecvTimeoutError::Timeout) => {}
                        Err(_) => panic!("Invalid error"),
                        Ok(_) => {
                            cinfo!(HARDWARE, "Close hardware thread");
                            return
                        }
                    }
                }
            })
            .expect("Should success running process thread");

        hardware_service_ret
    }

    fn update(&mut self, cpu_measure: Option<CpuMeasurement>) -> Result<CpuMeasurement, String> {
        if let Some(measure) = cpu_measure {
            let cpu = measure.done().map_err(|err| err.description().to_string())?;
            let mut usage = self.cpu_usage.write().map_err(|err| err.description().to_string())?;
            *usage = cpu.iter().map(|core| (core.user + core.system) as f64).collect();
        }

        let sys = systemstat::System::new();
        Ok(sys.cpu_load().map_err(|err| err.description().to_string())?)
    }

    pub fn get(&self) -> HardwareInfo {
        let mut sysinfo_sys = sysinfo::System::new();
        let disk_usage = get_disk_usage(&mut sysinfo_sys);
        let memory_usage = get_memory_usage(&mut sysinfo_sys);
        let cpu_usage = self.cpu_usage.read().map(|usage| usage.clone()).unwrap_or(Vec::new());
        HardwareInfo {
            cpu_usage,
            disk_usage,
            memory_usage,
        }
    }

    pub fn quit(&self) {
        if let Err(err) = self.quit.send(()) {
            cerror!(HARDWARE, "Error while quit hardware {}", err);
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

fn get_disk_usage(sys: &mut sysinfo::System) -> HardwareUsage {
    sys.refresh_disk_list();
    sys.refresh_disks();

    let mut total: i64 = 0;
    let mut available: i64 = 0;
    for disk in sys.get_disks() {
        total += disk.get_total_space() as i64;
        available += disk.get_available_space() as i64;
    }
    HardwareUsage {
        total,
        available,
        percentage_used: ((total - available) as f64 / total as f64),
    }
}

fn get_memory_usage(sys: &mut sysinfo::System) -> HardwareUsage {
    sys.refresh_system();

    // sysinfo library returns data in kB unit
    let total = (sys.get_total_memory() * 1024) as i64;
    let available = (sys.get_free_memory() * 1024) as i64;
    let used = sys.get_used_memory() as i64;
    HardwareUsage {
        total,
        available,
        percentage_used: (used as f64 / total as f64),
    }
}
