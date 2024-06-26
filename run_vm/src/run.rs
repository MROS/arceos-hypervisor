use crate::config::{Device, VirtualMachine};
use std::process::Command;

pub fn run(vm_list: &Vec<VirtualMachine>) {
    let mut command = Command::new("qemu-system-riscv64");
    command.args([
        "-m", "3G", "-smp", "1", "-machine", "virt", "-bios", "default",
    ]);
    command.arg("-nographic");
    command.args(["-kernel", "apps/hv/hv_qemu-virt-riscv.bin"]);

    let base_dtb_address: usize = 0x9000_0000;
    let base_bin_address: usize = 0x9020_0000;

    let diff: usize = 0x1000_0000;

    let mut block_device_count = 0;
    let mut net_device_count = 0;

    for (i, vm) in vm_list.iter().enumerate() {
        println!("i = {}", i);
        let os = vm.os.split("/").last().unwrap();
        let dtb_address = base_dtb_address + i * diff;
        let bin_address = base_bin_address + i * diff;

        command.arg("-device");
        command.arg(format!(
            "loader,file=apps/hv/guest/{}/{}-{}.dtb,addr=0x{:x},force-raw=on",
            os,
            os,
            i + 1,
            dtb_address
        ));
        command.arg("-device");
        command.arg(format!(
            "loader,file=apps/hv/guest/{}/{}-{}.bin,addr=0x{:x},force-raw=on",
            os,
            os,
            i + 1,
            bin_address
        ));
        for device in &vm.devices {
            match device {
                Device::Disk => {
                    let disk_file = format!("disk{}.img", block_device_count);
                    Command::new("make")
                        .arg(format!("DISK_IMG={}", disk_file))
                        .arg("disk_img")
                        .output();

                    command.arg("-drive");
                    command.arg(format!(
                        "id=disk{},if=none,format=raw,file={}",
                        block_device_count, disk_file
                    ));
                    command.arg("-device");
                    command.arg(format!(
                        "virtio-blk-device,drive=disk{}",
                        block_device_count
                    ));
                    block_device_count += 1;
                }
                Device::Net => {
                    command.arg("-device");
                    command.arg(format!("virtio-net-device,netdev=net{}", net_device_count));
                    command.arg("-netdev");
                    // TODO: 調整 port forward
                    command.arg(format!(
                        "user,id=net{},hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555",
                        net_device_count
                    ));
                    net_device_count += 1;
                }
            }
        }
    }
    print!("執行：{}", command.get_program().to_str().unwrap());
    for arg in command.get_args() {
        print!(" {}", arg.to_str().unwrap())
    }
    println!("");
    // command.spawn();
}
