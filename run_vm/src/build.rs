use crate::config::VirtualMachine;
use std::process::Command;

pub fn build(vm_list: &Vec<VirtualMachine>) {
    // qemu virtio mmio 預設 8 個，7 是最大的 index
    let mut device_index = 7;

    for (i, vm) in vm_list.iter().enumerate() {
        let mut command = Command::new("make");
        command.arg("ARCH=riscv64");
        command.arg(format!("A={}", vm.os));
        command.arg(format!("MODE={}", vm.mode));
        command.arg(format!("LOG={}", vm.log));
        command.arg(format!("VM={}", i + 1));

        let len = vm.devices.len();
        command.arg(format!(
            "DEVICES={}",
            ((device_index - len + 1)..=device_index)
                .rev()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ));
        device_index -= len;

        print!("執行：{}", command.get_program().to_str().unwrap());
        for arg in command.get_args() {
            print!(" {}", arg.to_str().unwrap())
        }
        println!("");
        command.arg("build").output();
    }
}
