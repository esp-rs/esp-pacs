#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu: [CPU; 2],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - Cluster CPU%s, containing CORE_?_ENTRY_ADDR%s, Core_?_ENTRY_CHECK, Core_?_MESSAGE_ADDR, Core_?_MESSAGE_MAX, Core_?_MESSAGE_PHASE, Core_?_NMI_MASK, Core_?_NMI_MASK_CANCLE, Core_?_NMI_MASK_DISABLE, Core_?_NMI_MASK_ENABLE, Core_?_NMI_MASK_PHASE, Core_?_NMI_MASK_TRIGGER_ADDR, CORE_?_STATUSTABLE%s, CORE_?_STATUSTABLE_CURRENT, Core_?_World_Cancel, Core_?_World_DRam0_PIF, Core_?_World_IRam0, Core_?_World_Phase, Core_?_World_PREPARE, Core_?_World_TRIGGER_ADDR, Core_?_World_UPDATE"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        &self.cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - Cluster CPU%s, containing CORE_?_ENTRY_ADDR%s, Core_?_ENTRY_CHECK, Core_?_MESSAGE_ADDR, Core_?_MESSAGE_MAX, Core_?_MESSAGE_PHASE, Core_?_NMI_MASK, Core_?_NMI_MASK_CANCLE, Core_?_NMI_MASK_DISABLE, Core_?_NMI_MASK_ENABLE, Core_?_NMI_MASK_PHASE, Core_?_NMI_MASK_TRIGGER_ADDR, CORE_?_STATUSTABLE%s, CORE_?_STATUSTABLE_CURRENT, Core_?_World_Cancel, Core_?_World_DRam0_PIF, Core_?_World_IRam0, Core_?_World_Phase, Core_?_World_PREPARE, Core_?_World_TRIGGER_ADDR, Core_?_World_UPDATE"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        self.cpu.iter()
    }
}
#[doc = "Cluster CPU%s, containing CORE_?_ENTRY_ADDR%s, Core_?_ENTRY_CHECK, Core_?_MESSAGE_ADDR, Core_?_MESSAGE_MAX, Core_?_MESSAGE_PHASE, Core_?_NMI_MASK, Core_?_NMI_MASK_CANCLE, Core_?_NMI_MASK_DISABLE, Core_?_NMI_MASK_ENABLE, Core_?_NMI_MASK_PHASE, Core_?_NMI_MASK_TRIGGER_ADDR, CORE_?_STATUSTABLE%s, CORE_?_STATUSTABLE_CURRENT, Core_?_World_Cancel, Core_?_World_DRam0_PIF, Core_?_World_IRam0, Core_?_World_Phase, Core_?_World_PREPARE, Core_?_World_TRIGGER_ADDR, Core_?_World_UPDATE"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_ENTRY_ADDR%s, Core_?_ENTRY_CHECK, Core_?_MESSAGE_ADDR, Core_?_MESSAGE_MAX, Core_?_MESSAGE_PHASE, Core_?_NMI_MASK, Core_?_NMI_MASK_CANCLE, Core_?_NMI_MASK_DISABLE, Core_?_NMI_MASK_ENABLE, Core_?_NMI_MASK_PHASE, Core_?_NMI_MASK_TRIGGER_ADDR, CORE_?_STATUSTABLE%s, CORE_?_STATUSTABLE_CURRENT, Core_?_World_Cancel, Core_?_World_DRam0_PIF, Core_?_World_IRam0, Core_?_World_Phase, Core_?_World_PREPARE, Core_?_World_TRIGGER_ADDR, Core_?_World_UPDATE"]
pub mod cpu;
