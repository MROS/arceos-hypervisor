#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate libax;

#[cfg(target_arch = "aarch64")]
use aarch64_config::GUEST_KERNEL_BASE_VADDR;
#[cfg(target_arch = "aarch64")]
use dtb_aarch64::MachineMeta;
#[cfg(target_arch = "riscv64")]
use dtb_riscv64::MachineMeta;
#[cfg(not(target_arch = "aarch64"))]
use libax::{
    hv::{
        self, phys_to_virt, GuestPageTable, GuestPageTableTrait, HyperCallMsg, HyperCraftHalImpl,
        PerCpu, Result, VCpu, VmCpus, VmExitInfo, VM, VMM,
    },
    info,
};
#[cfg(target_arch = "aarch64")]
use libax::{
    hv::{
        self, GuestPageTable, GuestPageTableTrait, HyperCraftHalImpl, PerCpu, Result, VCpu, VmCpus,
        VM,
    },
    info,
};

use page_table_entry::MappingFlags;

#[cfg(target_arch = "aarch64")]
mod aarch64_config;
#[cfg(target_arch = "aarch64")]
mod dtb_aarch64;
#[cfg(target_arch = "riscv64")]
mod dtb_riscv64;

#[cfg(target_arch = "x86_64")]
mod x64;

#[no_mangle]
fn main(hart_id: usize) {
    println!("Hello, hv!");

    #[cfg(target_arch = "riscv64")]
    {
        // boot cpu
        PerCpu::<HyperCraftHalImpl>::init(0, 0x4000);

        // get current percpu
        let pcpu = PerCpu::<HyperCraftHalImpl>::this_cpu();

        let mut vmm: VMM<HyperCraftHalImpl, GuestPageTable> = VMM::new();

        let base_dtb_address: usize = 0x9000_0000;
        let base_bin_address: usize = 0x9020_0000;
        let diff: usize = 0x1000_0000;

        // 偵測各個物理地址上是否被載入虛擬機
        for i in 0..3 {
            let dtb_address = base_dtb_address + i * diff;
            let bin_address = base_bin_address + i * diff;

            let meta = match MachineMeta::parse(dtb_address) {
                Some(meta) => meta,
                None => {
                    // 無法解析 dtb ，代表該區段不存在虛擬機
                    break;
                }
            };

            println!("發現虛擬機 {}", i);

            let gpt = setup_gpm(meta).unwrap();

            println!("創建 vcpu");
            let vcpu = pcpu.create_vcpu(0, bin_address).unwrap();

            println!("創建 vm");
            let mut vm: VM<HyperCraftHalImpl, GuestPageTable> = {
                let mut vcpus = VmCpus::new();

                // add vcpu 1 into vm 1
                vcpus.add_vcpu(vcpu).unwrap();
                VM::new(vcpus, gpt).unwrap()
            };

            println!("初始化 vm 的 vcpu");
            vm.init_vcpu(0);

            println!("將 vm 加入 vmm");
            vmm.add_vm(vm);

            println!("加入虛擬機 {} 完成", i);
        }

        println!("執行 hypervisor");
        vmm.run(hart_id);
    }
    #[cfg(target_arch = "aarch64")]
    {
        // boot cpu
        PerCpu::<HyperCraftHalImpl>::init(0, 0x4000); // change to pub const CPU_STACK_SIZE: usize = PAGE_SIZE * 128?

        // get current percpu
        let pcpu = PerCpu::<HyperCraftHalImpl>::this_cpu();

        // create vcpu, need to change addr for aarch64!
        let gpt = setup_gpm(0x7000_0000, 0x7020_0000).unwrap();
        let vcpu = pcpu.create_vcpu(0).unwrap();
        let mut vcpus = VmCpus::new();

        // add vcpu into vm
        vcpus.add_vcpu(vcpu).unwrap();
        let mut vm: VM<HyperCraftHalImpl, GuestPageTable> = VM::new(vcpus, gpt, 0).unwrap();
        vm.init_vm_vcpu(0, 0x7020_0000, 0x7000_0000);

        info!("vm run cpu{}", hart_id);
        // suppose hart_id to be 0
        vm.run(0);
    }
    #[cfg(target_arch = "x86_64")]
    {
        println!("into main {}", hart_id);

        let mut p = PerCpu::<HyperCraftHalImpl>::new(hart_id);
        p.hardware_enable().unwrap();

        let gpm = x64::setup_gpm().unwrap();
        info!("{:#x?}", gpm);

        let mut vcpu = p
            .create_vcpu(x64::BIOS_ENTRY, gpm.nest_page_table_root())
            .unwrap();

        println!("Running guest...");
        vcpu.run();

        p.hardware_disable().unwrap();

        return;
    }
    #[cfg(not(any(
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "aarch64"
    )))]
    {
        panic!("Other arch is not supported yet!")
    }
}

#[cfg(target_arch = "riscv64")]
pub fn setup_gpm(meta: MachineMeta) -> Result<GuestPageTable> {
    let mut gpt = GuestPageTable::new()?;
    if let Some(test) = meta.test_finisher_address {
        gpt.map_region(
            test.base_address,
            test.base_address,
            test.size + 0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER | MappingFlags::EXECUTE,
        )?;
    }
    for virtio in meta.virtio.iter() {
        gpt.map_region(
            virtio.base_address,
            virtio.base_address,
            virtio.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(uart) = meta.uart {
        gpt.map_region(
            uart.base_address,
            uart.base_address,
            0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(clint) = meta.clint {
        gpt.map_region(
            clint.base_address,
            clint.base_address,
            clint.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(plic) = meta.plic {
        gpt.map_region(
            plic.base_address,
            plic.base_address,
            0x20_0000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pci) = meta.pci {
        gpt.map_region(
            pci.base_address,
            pci.base_address,
            pci.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    info!(
        "physical memory: [{:#x}: {:#x})",
        meta.physical_memory_offset,
        meta.physical_memory_offset + meta.physical_memory_size
    );

    gpt.map_region(
        meta.physical_memory_offset,
        meta.physical_memory_offset,
        meta.physical_memory_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;

    Ok(gpt)
}

#[cfg(target_arch = "aarch64")]
pub fn setup_gpm(dtb: usize, kernel_entry: usize) -> Result<GuestPageTable> {
    let mut gpt = GuestPageTable::new()?;
    let meta = MachineMeta::parse(dtb);
    /*
    for virtio in meta.virtio.iter() {
        gpt.map_region(
            virtio.base_address,
            virtio.base_address,
            0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
        debug!("finish one virtio");
    }
    */
    // hard code for virtio_mmio
    gpt.map_region(
        0xa000000,
        0xa000000,
        0x4000,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
    )?;

    if let Some(pl011) = meta.pl011 {
        gpt.map_region(
            pl011.base_address,
            pl011.base_address,
            pl011.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pl031) = meta.pl031 {
        gpt.map_region(
            pl031.base_address,
            pl031.base_address,
            pl031.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pl061) = meta.pl061 {
        gpt.map_region(
            pl061.base_address,
            pl061.base_address,
            pl061.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    for intc in meta.intc.iter() {
        gpt.map_region(
            intc.base_address,
            intc.base_address,
            intc.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pcie) = meta.pcie {
        gpt.map_region(
            pcie.base_address,
            pcie.base_address,
            pcie.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    for flash in meta.flash.iter() {
        gpt.map_region(
            flash.base_address,
            flash.base_address,
            flash.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    info!(
        "physical memory: [{:#x}: {:#x})",
        meta.physical_memory_offset,
        meta.physical_memory_offset + meta.physical_memory_size
    );

    gpt.map_region(
        meta.physical_memory_offset,
        meta.physical_memory_offset,
        meta.physical_memory_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;

    gpt.map_region(
        GUEST_KERNEL_BASE_VADDR,
        kernel_entry,
        meta.physical_memory_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;

    let gaddr: usize = 0x40_1000_0000;
    let paddr = gpt.translate(gaddr).unwrap();
    debug!("this is paddr for 0x{:X}: 0x{:X}", gaddr, paddr);
    Ok(gpt)
}
