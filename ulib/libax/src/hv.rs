//! Hypervisor related functions

pub use axhal::mem::{phys_to_virt, virt_to_phys, PhysAddr};
pub use axruntime::GuestPageTable;
pub use axruntime::HyperCraftHalImpl;
pub use hypercraft::GuestPageTableTrait;

pub use hypercraft::HyperCraftHal;
pub use hypercraft::HyperError as Error;
pub use hypercraft::HyperResult as Result;
#[cfg(target_arch = "riscv64")]
pub use hypercraft::VMM;
#[cfg(not(target_arch = "aarch64"))]
pub use hypercraft::{
    GuestPhysAddr, GuestVirtAddr, HostPhysAddr, HostVirtAddr, HyperCallMsg, VmExitInfo,
};
pub use hypercraft::{PerCpu, VCpu, VmCpus, VM};
