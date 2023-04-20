use crate_interface::{call_interface, def_interface};

#[def_interface]
pub trait TrapHandler {
    fn handle_irq(irq_num: usize);

    #[cfg(feature = "user")]
    fn handle_syscall(id: usize, params: [usize; 6]) -> isize;
    // more e.g.: handle_page_fault();
}

/// Call the external IRQ handler.
#[allow(dead_code)]
pub(crate) fn handle_irq_extern(irq_num: usize) {
    call_interface!(TrapHandler::handle_irq, irq_num);
}

/// Call the external syscall handler.
#[cfg(feature = "user")]
#[allow(dead_code)]
pub(crate) fn handle_syscall_extern(syscall_num: usize, param: [usize; 6]) -> isize {
    call_interface!(TrapHandler::handle_syscall, syscall_num, param)
}

#[cfg(feature = "user-paging")]
use crate::arch::TrapFrame;
#[cfg(feature = "user-paging")]
#[def_interface]
pub trait CurrentTask {
    fn current_trap_frame() -> *mut TrapFrame;
    fn current_satp() -> usize;
    fn current_trap_frame_virt_addr() -> usize;
}

#[cfg(feature = "user-paging")]
pub(crate) fn get_current_trap_frame() -> *mut TrapFrame {
    call_interface!(CurrentTask::current_trap_frame)
}
#[cfg(feature = "user-paging")]
pub(crate) fn get_current_satp() -> usize {
    call_interface!(CurrentTask::current_satp)
}
#[cfg(feature = "user-paging")]
pub(crate) fn get_current_trap_frame_virt_addr() -> usize {
    call_interface!(CurrentTask::current_trap_frame_virt_addr)
}
