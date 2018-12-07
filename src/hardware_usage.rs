use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use sysinfo;
use sysinfo::{DiskExt, SystemExt};
use systemstat;
use systemstat::{CPULoad, DelayedMeasurement, Platform};

/**
 * We use both sysinfo and systemstat
 * sysinfo is slow on single core machine when calculating cpu usage.
 * But sysinfo gives disk usage by disk(not by mount)
 *
 * systemstat is fast when calculating cpu usage.
 *
 * cpu : systemstat
 * memory : systemstat
 * disk : sysinfo
 */

#[derive(Clone)]
pub struct HardwareService {
    quit: Sender<()>,
    hardware_info: Arc<Mutex<HardwareInfo>>,
}

type CpuMeasurement = DelayedMeasurement<Vec<CPULoad>>;

impl HardwareService {
    pub fn create() -> (Self, Receiver<()>) {
        let (tx, rx) = channel::unbounded();
        (
            Self {
                quit: tx,
                hardware_info: Arc::new(Mutex::new(HardwareInfo::default())),
            },
            rx,
        )
    }

    pub fn run_thread() -> HardwareService {
        let (mut hardware_service, quit_rx) = HardwareService::create();
        let hardware_service_ret = hardware_service.clone();

        thread::Builder::new()
            .name("hardware".to_string())
            .spawn(move || {
                let mut sysinfo_sys = sysinfo::System::new();

                loop {
                    let measurement = match hardware_service.prepare_cpu_usage() {
                        Ok(measurement) => Some(measurement),
                        Err(_) => {
                            // Do not print error.
                            // There will be too many error if cpu usage is not supported
                            None
                        }
                    };

                    let timeout = Duration::new(1, 0);
                    select! {
                        recv(quit_rx, _msg) => {
                            cinfo!(HARDWARE, "Close hardware thread");
                            return
                        },
                        recv(channel::after(timeout)) => {}
                    }

                    let _ = hardware_service.update(measurement, &mut sysinfo_sys);
                    // Do not print error.
                    // There will be too many error if cpu usage is not supported
                }
            })
            .expect("Should success running process thread");

        hardware_service_ret
    }

    fn prepare_cpu_usage(&self) -> Result<CpuMeasurement, String> {
        let sys = systemstat::System::new();
        Ok(sys.cpu_load().map_err(|err| err.description().to_string())?)
    }

    fn update(&mut self, cpu_measure: Option<CpuMeasurement>, sysinfo_sys: &mut sysinfo::System) -> Result<(), String> {
        let cpu_usage = if let Some(measure) = cpu_measure {
            let cpu = measure.done().map_err(|err| err.description().to_string())?;
            cpu.iter().map(|core| f64::from(core.user + core.system)).collect()
        } else {
            Vec::new()
        };

        let disk_usage = get_disk_usage(sysinfo_sys);
        let mut systemstat_sys = systemstat::System::new();
        let memory_usage = get_memory_usage(&mut systemstat_sys);

        match self.hardware_info.try_lock() {
            Ok(mut hardware_info) => {
                *hardware_info = HardwareInfo {
                    cpu_usage,
                    disk_usage,
                    memory_usage,
                };
            }
            Err(err) => cdebug!(HARDWARE, "Cannot acquire hardware_info lock : {}", err),
        }
        Ok(())
    }

    pub fn get(&self) -> HardwareInfo {
        if let Ok(hardware_info) = self.hardware_info.try_lock() {
            hardware_info.clone()
        } else {
            Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn quit(&self) {
        self.quit.send(());
    }
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize, Default, Clone)]
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
    let percentage_used = if total == 0 {
        0f64
    } else {
        (total - available) as f64 / total as f64
    };
    HardwareUsage {
        total,
        available,
        percentage_used,
    }
}

fn get_memory_usage(sys: &mut systemstat::System) -> HardwareUsage {
    let mem = match sys.memory() {
        Ok(mem) => mem,
        Err(_) => return HardwareUsage::default(),
    };

    let total = mem.total.as_usize() as i64;
    let available = mem.free.as_usize() as i64;
    let used = total - available;
    let percentage_used = if total == 0 {
        0f64
    } else {
        used as f64 / total as f64
    };

    HardwareUsage {
        total,
        available,
        percentage_used,
    }
}
