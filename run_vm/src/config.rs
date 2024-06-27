use derive_more::Display;

pub enum Device {
    Net,
    Disk,
}

#[derive(Display)]
pub enum Log {
    #[display(fmt = "off")]
    Off,
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "warn")]
    Warn,
    #[display(fmt = "info")]
    Info,
    #[display(fmt = "debug")]
    Debug,
    #[display(fmt = "trace")]
    Trace,
}

#[derive(Display)]
pub enum Mode {
    #[display(fmt = "debug")]
    Debug,
    #[display(fmt = "release")]
    Release,
}

pub struct VirtualMachine {
    pub os: &'static str,
    pub devices: Vec<Device>,
    pub log: Log,
    pub mode: Mode,
}

pub fn get_vm_list() -> Vec<VirtualMachine> {
    vec![
        VirtualMachine {
            os: "apps/fs/shell",
            devices: vec![Device::Disk],
            log: Log::Info,
            mode: Mode::Debug,
        },
        // VirtualMachine {
        //     os: "apps/fs/shell",
        //     devices: vec![Device::Disk],
        //     log: Log::Info,
        //     mode: Mode::Debug,
        // },
        // VirtualMachine {
        //     os: "apps/net/httpserver",
        //     devices: vec![Device::Net],
        //     log: Log::Info,
        //     mode: Mode::Debug,
        // },
    ]
}
