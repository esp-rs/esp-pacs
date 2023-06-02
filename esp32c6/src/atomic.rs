#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - hardware lock regsiter"]
    pub addr_lock: ADDR_LOCK,
    #[doc = "0x04 - gloable lr address regsiter"]
    pub lr_addr: LR_ADDR,
    #[doc = "0x08 - gloable lr value regsiter"]
    pub lr_value: LR_VALUE,
    #[doc = "0x0c - lock status regsiter"]
    pub lock_status: LOCK_STATUS,
    #[doc = "0x10 - wait counter register"]
    pub counter: COUNTER,
}
#[doc = "ADDR_LOCK (rw) register accessor: an alias for `Reg<ADDR_LOCK_SPEC>`"]
pub type ADDR_LOCK = crate::Reg<addr_lock::ADDR_LOCK_SPEC>;
#[doc = "hardware lock regsiter"]
pub mod addr_lock;
#[doc = "LR_ADDR (rw) register accessor: an alias for `Reg<LR_ADDR_SPEC>`"]
pub type LR_ADDR = crate::Reg<lr_addr::LR_ADDR_SPEC>;
#[doc = "gloable lr address regsiter"]
pub mod lr_addr;
#[doc = "LR_VALUE (rw) register accessor: an alias for `Reg<LR_VALUE_SPEC>`"]
pub type LR_VALUE = crate::Reg<lr_value::LR_VALUE_SPEC>;
#[doc = "gloable lr value regsiter"]
pub mod lr_value;
#[doc = "LOCK_STATUS (r) register accessor: an alias for `Reg<LOCK_STATUS_SPEC>`"]
pub type LOCK_STATUS = crate::Reg<lock_status::LOCK_STATUS_SPEC>;
#[doc = "lock status regsiter"]
pub mod lock_status;
#[doc = "COUNTER (rw) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "wait counter register"]
pub mod counter;
