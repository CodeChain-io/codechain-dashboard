use crossbeam::channel::{Receiver, Sender};
use crossbeam::{channel, select};
use parking_lot::Mutex;
use serde_derive::Serialize;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use sysinfo::{self, DiskExt, SystemExt};
use systemstat::{self, CPULoad, DelayedMeasurement, Platform};

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

        let disk_usages = get_disk_usages(sysinfo_sys);
        let disk_usage = merge_disk_usages(&disk_usages);
        let mut systemstat_sys = systemstat::System::new();
        let memory_usage = get_memory_usage(&mut systemstat_sys);

        if let Some(mut hardware_info) = self.hardware_info.try_lock() {
            *hardware_info = HardwareInfo {
                cpu_usage,
                disk_usage,
                disk_usages,
                memory_usage,
            };
        } else {
            cdebug!(HARDWARE, "Cannot acquire hardware_info lock");
        }
        Ok(())
    }

    pub fn get(&self) -> HardwareInfo {
        if let Some(hardware_info) = self.hardware_info.try_lock() {
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

impl HardwareUsage {
    fn new(total: i64, available: i64) -> HardwareUsage {
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
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    // disk_usage field is deprecated. The field will be removed later update
    pub disk_usage: HardwareUsage,
    pub disk_usages: Vec<HardwareUsage>,
    pub memory_usage: HardwareUsage,
}

fn get_disk_usages(sys: &mut sysinfo::System) -> Vec<HardwareUsage> {
    sys.refresh_disk_list();
    sys.refresh_disks();

    let mut result: Vec<HardwareUsage> = Vec::new();
    for disk in sys.get_disks() {
        let total = disk.get_total_space() as i64;
        let available = disk.get_available_space() as i64;

        result.push(HardwareUsage::new(total, available));
    }

    result
}

fn merge_disk_usages(usages: &[HardwareUsage]) -> HardwareUsage {
    let mut total = 0;
    let mut available = 0;

    for usage in usages {
        total += usage.total;
        available += usage.available;
    }

    HardwareUsage::new(total, available)
}

fn get_memory_usage(sys: &mut systemstat::System) -> HardwareUsage {
    let mem = match sys.memory() {
        Ok(mem) => mem,
        Err(_) => return HardwareUsage::default(),
    };

    let total = mem.total.as_usize() as i64;
    let available = mem.free.as_usize() as i64;

    HardwareUsage::new(total, available)
}
