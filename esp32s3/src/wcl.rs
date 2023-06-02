#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Core_0 Entry 1 address configuration Register"]
    pub core_0_entry_1_addr: CORE_0_ENTRY_1_ADDR,
    #[doc = "0x04 - Core_0 Entry 2 address configuration Register"]
    pub core_0_entry_2_addr: CORE_0_ENTRY_2_ADDR,
    #[doc = "0x08 - Core_0 Entry 3 address configuration Register"]
    pub core_0_entry_3_addr: CORE_0_ENTRY_3_ADDR,
    #[doc = "0x0c - Core_0 Entry 4 address configuration Register"]
    pub core_0_entry_4_addr: CORE_0_ENTRY_4_ADDR,
    #[doc = "0x10 - Core_0 Entry 5 address configuration Register"]
    pub core_0_entry_5_addr: CORE_0_ENTRY_5_ADDR,
    #[doc = "0x14 - Core_0 Entry 6 address configuration Register"]
    pub core_0_entry_6_addr: CORE_0_ENTRY_6_ADDR,
    #[doc = "0x18 - Core_0 Entry 7 address configuration Register"]
    pub core_0_entry_7_addr: CORE_0_ENTRY_7_ADDR,
    #[doc = "0x1c - Core_0 Entry 8 address configuration Register"]
    pub core_0_entry_8_addr: CORE_0_ENTRY_8_ADDR,
    #[doc = "0x20 - Core_0 Entry 9 address configuration Register"]
    pub core_0_entry_9_addr: CORE_0_ENTRY_9_ADDR,
    #[doc = "0x24 - Core_0 Entry 10 address configuration Register"]
    pub core_0_entry_10_addr: CORE_0_ENTRY_10_ADDR,
    #[doc = "0x28 - Core_0 Entry 11 address configuration Register"]
    pub core_0_entry_11_addr: CORE_0_ENTRY_11_ADDR,
    #[doc = "0x2c - Core_0 Entry 12 address configuration Register"]
    pub core_0_entry_12_addr: CORE_0_ENTRY_12_ADDR,
    #[doc = "0x30 - Core_0 Entry 13 address configuration Register"]
    pub core_0_entry_13_addr: CORE_0_ENTRY_13_ADDR,
    _reserved13: [u8; 0x48],
    #[doc = "0x7c - Core_0 Entry check configuration Register"]
    pub core_0_entry_check: CORE_0_ENTRY_CHECK,
    #[doc = "0x80 - Status register of world switch of entry 1"]
    pub core_0_statustable1: CORE_0_STATUSTABLE1,
    #[doc = "0x84 - Status register of world switch of entry 2"]
    pub core_0_statustable2: CORE_0_STATUSTABLE2,
    #[doc = "0x88 - Status register of world switch of entry 3"]
    pub core_0_statustable3: CORE_0_STATUSTABLE3,
    #[doc = "0x8c - Status register of world switch of entry 4"]
    pub core_0_statustable4: CORE_0_STATUSTABLE4,
    #[doc = "0x90 - Status register of world switch of entry 5"]
    pub core_0_statustable5: CORE_0_STATUSTABLE5,
    #[doc = "0x94 - Status register of world switch of entry 6"]
    pub core_0_statustable6: CORE_0_STATUSTABLE6,
    #[doc = "0x98 - Status register of world switch of entry 7"]
    pub core_0_statustable7: CORE_0_STATUSTABLE7,
    #[doc = "0x9c - Status register of world switch of entry 8"]
    pub core_0_statustable8: CORE_0_STATUSTABLE8,
    #[doc = "0xa0 - Status register of world switch of entry 9"]
    pub core_0_statustable9: CORE_0_STATUSTABLE9,
    #[doc = "0xa4 - Status register of world switch of entry 10"]
    pub core_0_statustable10: CORE_0_STATUSTABLE10,
    #[doc = "0xa8 - Status register of world switch of entry 11"]
    pub core_0_statustable11: CORE_0_STATUSTABLE11,
    #[doc = "0xac - Status register of world switch of entry 12"]
    pub core_0_statustable12: CORE_0_STATUSTABLE12,
    #[doc = "0xb0 - Status register of world switch of entry 13"]
    pub core_0_statustable13: CORE_0_STATUSTABLE13,
    _reserved27: [u8; 0x48],
    #[doc = "0xfc - Status register of statustable current"]
    pub core_0_statustable_current: CORE_0_STATUSTABLE_CURRENT,
    #[doc = "0x100 - Clear writer_buffer write address configuration register"]
    pub core_0_message_addr: CORE_0_MESSAGE_ADDR,
    #[doc = "0x104 - Clear writer_buffer write number configuration register"]
    pub core_0_message_max: CORE_0_MESSAGE_MAX,
    #[doc = "0x108 - Clear writer_buffer status register"]
    pub core_0_message_phase: CORE_0_MESSAGE_PHASE,
    _reserved31: [u8; 0x34],
    #[doc = "0x140 - Core_0 trigger address configuration Register"]
    pub core_0_world_trigger_addr: CORE_0_WORLD_TRIGGER_ADDR,
    #[doc = "0x144 - Core_0 prepare world configuration Register"]
    pub core_0_world_prepare: CORE_0_WORLD_PREPARE,
    #[doc = "0x148 - Core_0 configuration update register"]
    pub core_0_world_update: CORE_0_WORLD_UPDATE,
    #[doc = "0x14c - Core_0 configuration cancel register"]
    pub core_0_world_cancel: CORE_0_WORLD_CANCEL,
    #[doc = "0x150 - Core_0 Iram0 world register"]
    pub core_0_world_iram0: CORE_0_WORLD_IRAM0,
    #[doc = "0x154 - Core_0 dram0 and PIF world register"]
    pub core_0_world_dram0_pif: CORE_0_WORLD_DRAM0_PIF,
    #[doc = "0x158 - Core_0 world status register"]
    pub core_0_world_phase: CORE_0_WORLD_PHASE,
    _reserved38: [u8; 0x24],
    #[doc = "0x180 - Core_0 NMI mask enable register"]
    pub core_0_nmi_mask_enable: CORE_0_NMI_MASK_ENABLE,
    #[doc = "0x184 - Core_0 NMI mask trigger address register"]
    pub core_0_nmi_mask_trigger_addr: CORE_0_NMI_MASK_TRIGGER_ADDR,
    #[doc = "0x188 - Core_0 NMI mask disable register"]
    pub core_0_nmi_mask_disable: CORE_0_NMI_MASK_DISABLE,
    #[doc = "0x18c - Core_0 NMI mask disable register"]
    pub core_0_nmi_mask_cancle: CORE_0_NMI_MASK_CANCLE,
    #[doc = "0x190 - Core_0 NMI mask register"]
    pub core_0_nmi_mask: CORE_0_NMI_MASK,
    #[doc = "0x194 - Core_0 NMI mask phase register"]
    pub core_0_nmi_mask_phase: CORE_0_NMI_MASK_PHASE,
    _reserved44: [u8; 0x0268],
    #[doc = "0x400 - Core_1 Entry 1 address configuration Register"]
    pub core_1_entry_1_addr: CORE_1_ENTRY_1_ADDR,
    #[doc = "0x404 - Core_1 Entry 2 address configuration Register"]
    pub core_1_entry_2_addr: CORE_1_ENTRY_2_ADDR,
    #[doc = "0x408 - Core_1 Entry 3 address configuration Register"]
    pub core_1_entry_3_addr: CORE_1_ENTRY_3_ADDR,
    #[doc = "0x40c - Core_1 Entry 4 address configuration Register"]
    pub core_1_entry_4_addr: CORE_1_ENTRY_4_ADDR,
    #[doc = "0x410 - Core_1 Entry 5 address configuration Register"]
    pub core_1_entry_5_addr: CORE_1_ENTRY_5_ADDR,
    #[doc = "0x414 - Core_1 Entry 6 address configuration Register"]
    pub core_1_entry_6_addr: CORE_1_ENTRY_6_ADDR,
    #[doc = "0x418 - Core_1 Entry 7 address configuration Register"]
    pub core_1_entry_7_addr: CORE_1_ENTRY_7_ADDR,
    #[doc = "0x41c - Core_1 Entry 8 address configuration Register"]
    pub core_1_entry_8_addr: CORE_1_ENTRY_8_ADDR,
    #[doc = "0x420 - Core_1 Entry 9 address configuration Register"]
    pub core_1_entry_9_addr: CORE_1_ENTRY_9_ADDR,
    #[doc = "0x424 - Core_1 Entry 10 address configuration Register"]
    pub core_1_entry_10_addr: CORE_1_ENTRY_10_ADDR,
    #[doc = "0x428 - Core_1 Entry 11 address configuration Register"]
    pub core_1_entry_11_addr: CORE_1_ENTRY_11_ADDR,
    #[doc = "0x42c - Core_1 Entry 12 address configuration Register"]
    pub core_1_entry_12_addr: CORE_1_ENTRY_12_ADDR,
    #[doc = "0x430 - Core_1 Entry 13 address configuration Register"]
    pub core_1_entry_13_addr: CORE_1_ENTRY_13_ADDR,
    _reserved57: [u8; 0x48],
    #[doc = "0x47c - Core_1 Entry check configuration Register"]
    pub core_1_entry_check: CORE_1_ENTRY_CHECK,
    #[doc = "0x480 - Status register of world switch of entry 1"]
    pub core_1_statustable1: CORE_1_STATUSTABLE1,
    #[doc = "0x484 - Status register of world switch of entry 2"]
    pub core_1_statustable2: CORE_1_STATUSTABLE2,
    #[doc = "0x488 - Status register of world switch of entry 3"]
    pub core_1_statustable3: CORE_1_STATUSTABLE3,
    #[doc = "0x48c - Status register of world switch of entry 4"]
    pub core_1_statustable4: CORE_1_STATUSTABLE4,
    #[doc = "0x490 - Status register of world switch of entry 5"]
    pub core_1_statustable5: CORE_1_STATUSTABLE5,
    #[doc = "0x494 - Status register of world switch of entry 6"]
    pub core_1_statustable6: CORE_1_STATUSTABLE6,
    #[doc = "0x498 - Status register of world switch of entry 7"]
    pub core_1_statustable7: CORE_1_STATUSTABLE7,
    #[doc = "0x49c - Status register of world switch of entry 8"]
    pub core_1_statustable8: CORE_1_STATUSTABLE8,
    #[doc = "0x4a0 - Status register of world switch of entry 9"]
    pub core_1_statustable9: CORE_1_STATUSTABLE9,
    #[doc = "0x4a4 - Status register of world switch of entry 10"]
    pub core_1_statustable10: CORE_1_STATUSTABLE10,
    #[doc = "0x4a8 - Status register of world switch of entry 11"]
    pub core_1_statustable11: CORE_1_STATUSTABLE11,
    #[doc = "0x4ac - Status register of world switch of entry 12"]
    pub core_1_statustable12: CORE_1_STATUSTABLE12,
    #[doc = "0x4b0 - Status register of world switch of entry 13"]
    pub core_1_statustable13: CORE_1_STATUSTABLE13,
    _reserved71: [u8; 0x48],
    #[doc = "0x4fc - Status register of statustable current"]
    pub core_1_statustable_current: CORE_1_STATUSTABLE_CURRENT,
    #[doc = "0x500 - Clear writer_buffer write address configuration register"]
    pub core_1_message_addr: CORE_1_MESSAGE_ADDR,
    #[doc = "0x504 - Clear writer_buffer write number configuration register"]
    pub core_1_message_max: CORE_1_MESSAGE_MAX,
    #[doc = "0x508 - Clear writer_buffer status register"]
    pub core_1_message_phase: CORE_1_MESSAGE_PHASE,
    _reserved75: [u8; 0x34],
    #[doc = "0x540 - Core_1 trigger address configuration Register"]
    pub core_1_world_trigger_addr: CORE_1_WORLD_TRIGGER_ADDR,
    #[doc = "0x544 - Core_1 prepare world configuration Register"]
    pub core_1_world_prepare: CORE_1_WORLD_PREPARE,
    #[doc = "0x548 - Core_1 configuration update register"]
    pub core_1_world_update: CORE_1_WORLD_UPDATE,
    #[doc = "0x54c - Core_1 configuration cancel register"]
    pub core_1_world_cancel: CORE_1_WORLD_CANCEL,
    #[doc = "0x550 - Core_1 Iram0 world register"]
    pub core_1_world_iram0: CORE_1_WORLD_IRAM0,
    #[doc = "0x554 - Core_1 dram0 and PIF world register"]
    pub core_1_world_dram0_pif: CORE_1_WORLD_DRAM0_PIF,
    #[doc = "0x558 - Core_0 world status register"]
    pub core_1_world_phase: CORE_1_WORLD_PHASE,
    _reserved82: [u8; 0x24],
    #[doc = "0x580 - Core_1 NMI mask enable register"]
    pub core_1_nmi_mask_enable: CORE_1_NMI_MASK_ENABLE,
    #[doc = "0x584 - Core_1 NMI mask trigger addr register"]
    pub core_1_nmi_mask_trigger_addr: CORE_1_NMI_MASK_TRIGGER_ADDR,
    #[doc = "0x588 - Core_1 NMI mask disable register"]
    pub core_1_nmi_mask_disable: CORE_1_NMI_MASK_DISABLE,
    #[doc = "0x58c - Core_1 NMI mask disable register"]
    pub core_1_nmi_mask_cancle: CORE_1_NMI_MASK_CANCLE,
    #[doc = "0x590 - Core_1 NMI mask register"]
    pub core_1_nmi_mask: CORE_1_NMI_MASK,
    #[doc = "0x594 - Core_1 NMI mask phase register"]
    pub core_1_nmi_mask_phase: CORE_1_NMI_MASK_PHASE,
}
#[doc = "Core_0_ENTRY_1_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_1_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_1_ADDR = crate::Reg<core_0_entry_1_addr::CORE_0_ENTRY_1_ADDR_SPEC>;
#[doc = "Core_0 Entry 1 address configuration Register"]
pub mod core_0_entry_1_addr;
#[doc = "Core_0_ENTRY_2_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_2_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_2_ADDR = crate::Reg<core_0_entry_2_addr::CORE_0_ENTRY_2_ADDR_SPEC>;
#[doc = "Core_0 Entry 2 address configuration Register"]
pub mod core_0_entry_2_addr;
#[doc = "Core_0_ENTRY_3_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_3_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_3_ADDR = crate::Reg<core_0_entry_3_addr::CORE_0_ENTRY_3_ADDR_SPEC>;
#[doc = "Core_0 Entry 3 address configuration Register"]
pub mod core_0_entry_3_addr;
#[doc = "Core_0_ENTRY_4_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_4_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_4_ADDR = crate::Reg<core_0_entry_4_addr::CORE_0_ENTRY_4_ADDR_SPEC>;
#[doc = "Core_0 Entry 4 address configuration Register"]
pub mod core_0_entry_4_addr;
#[doc = "Core_0_ENTRY_5_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_5_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_5_ADDR = crate::Reg<core_0_entry_5_addr::CORE_0_ENTRY_5_ADDR_SPEC>;
#[doc = "Core_0 Entry 5 address configuration Register"]
pub mod core_0_entry_5_addr;
#[doc = "Core_0_ENTRY_6_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_6_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_6_ADDR = crate::Reg<core_0_entry_6_addr::CORE_0_ENTRY_6_ADDR_SPEC>;
#[doc = "Core_0 Entry 6 address configuration Register"]
pub mod core_0_entry_6_addr;
#[doc = "Core_0_ENTRY_7_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_7_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_7_ADDR = crate::Reg<core_0_entry_7_addr::CORE_0_ENTRY_7_ADDR_SPEC>;
#[doc = "Core_0 Entry 7 address configuration Register"]
pub mod core_0_entry_7_addr;
#[doc = "Core_0_ENTRY_8_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_8_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_8_ADDR = crate::Reg<core_0_entry_8_addr::CORE_0_ENTRY_8_ADDR_SPEC>;
#[doc = "Core_0 Entry 8 address configuration Register"]
pub mod core_0_entry_8_addr;
#[doc = "Core_0_ENTRY_9_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_9_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_9_ADDR = crate::Reg<core_0_entry_9_addr::CORE_0_ENTRY_9_ADDR_SPEC>;
#[doc = "Core_0 Entry 9 address configuration Register"]
pub mod core_0_entry_9_addr;
#[doc = "Core_0_ENTRY_10_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_10_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_10_ADDR = crate::Reg<core_0_entry_10_addr::CORE_0_ENTRY_10_ADDR_SPEC>;
#[doc = "Core_0 Entry 10 address configuration Register"]
pub mod core_0_entry_10_addr;
#[doc = "Core_0_ENTRY_11_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_11_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_11_ADDR = crate::Reg<core_0_entry_11_addr::CORE_0_ENTRY_11_ADDR_SPEC>;
#[doc = "Core_0 Entry 11 address configuration Register"]
pub mod core_0_entry_11_addr;
#[doc = "Core_0_ENTRY_12_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_12_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_12_ADDR = crate::Reg<core_0_entry_12_addr::CORE_0_ENTRY_12_ADDR_SPEC>;
#[doc = "Core_0 Entry 12 address configuration Register"]
pub mod core_0_entry_12_addr;
#[doc = "Core_0_ENTRY_13_ADDR (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_13_ADDR_SPEC>`"]
pub type CORE_0_ENTRY_13_ADDR = crate::Reg<core_0_entry_13_addr::CORE_0_ENTRY_13_ADDR_SPEC>;
#[doc = "Core_0 Entry 13 address configuration Register"]
pub mod core_0_entry_13_addr;
#[doc = "Core_0_ENTRY_CHECK (rw) register accessor: an alias for `Reg<CORE_0_ENTRY_CHECK_SPEC>`"]
pub type CORE_0_ENTRY_CHECK = crate::Reg<core_0_entry_check::CORE_0_ENTRY_CHECK_SPEC>;
#[doc = "Core_0 Entry check configuration Register"]
pub mod core_0_entry_check;
#[doc = "Core_0_STATUSTABLE1 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE1_SPEC>`"]
pub type CORE_0_STATUSTABLE1 = crate::Reg<core_0_statustable1::CORE_0_STATUSTABLE1_SPEC>;
#[doc = "Status register of world switch of entry 1"]
pub mod core_0_statustable1;
#[doc = "Core_0_STATUSTABLE2 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE2_SPEC>`"]
pub type CORE_0_STATUSTABLE2 = crate::Reg<core_0_statustable2::CORE_0_STATUSTABLE2_SPEC>;
#[doc = "Status register of world switch of entry 2"]
pub mod core_0_statustable2;
#[doc = "Core_0_STATUSTABLE3 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE3_SPEC>`"]
pub type CORE_0_STATUSTABLE3 = crate::Reg<core_0_statustable3::CORE_0_STATUSTABLE3_SPEC>;
#[doc = "Status register of world switch of entry 3"]
pub mod core_0_statustable3;
#[doc = "Core_0_STATUSTABLE4 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE4_SPEC>`"]
pub type CORE_0_STATUSTABLE4 = crate::Reg<core_0_statustable4::CORE_0_STATUSTABLE4_SPEC>;
#[doc = "Status register of world switch of entry 4"]
pub mod core_0_statustable4;
#[doc = "Core_0_STATUSTABLE5 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE5_SPEC>`"]
pub type CORE_0_STATUSTABLE5 = crate::Reg<core_0_statustable5::CORE_0_STATUSTABLE5_SPEC>;
#[doc = "Status register of world switch of entry 5"]
pub mod core_0_statustable5;
#[doc = "Core_0_STATUSTABLE6 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE6_SPEC>`"]
pub type CORE_0_STATUSTABLE6 = crate::Reg<core_0_statustable6::CORE_0_STATUSTABLE6_SPEC>;
#[doc = "Status register of world switch of entry 6"]
pub mod core_0_statustable6;
#[doc = "Core_0_STATUSTABLE7 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE7_SPEC>`"]
pub type CORE_0_STATUSTABLE7 = crate::Reg<core_0_statustable7::CORE_0_STATUSTABLE7_SPEC>;
#[doc = "Status register of world switch of entry 7"]
pub mod core_0_statustable7;
#[doc = "Core_0_STATUSTABLE8 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE8_SPEC>`"]
pub type CORE_0_STATUSTABLE8 = crate::Reg<core_0_statustable8::CORE_0_STATUSTABLE8_SPEC>;
#[doc = "Status register of world switch of entry 8"]
pub mod core_0_statustable8;
#[doc = "Core_0_STATUSTABLE9 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE9_SPEC>`"]
pub type CORE_0_STATUSTABLE9 = crate::Reg<core_0_statustable9::CORE_0_STATUSTABLE9_SPEC>;
#[doc = "Status register of world switch of entry 9"]
pub mod core_0_statustable9;
#[doc = "Core_0_STATUSTABLE10 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE10_SPEC>`"]
pub type CORE_0_STATUSTABLE10 = crate::Reg<core_0_statustable10::CORE_0_STATUSTABLE10_SPEC>;
#[doc = "Status register of world switch of entry 10"]
pub mod core_0_statustable10;
#[doc = "Core_0_STATUSTABLE11 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE11_SPEC>`"]
pub type CORE_0_STATUSTABLE11 = crate::Reg<core_0_statustable11::CORE_0_STATUSTABLE11_SPEC>;
#[doc = "Status register of world switch of entry 11"]
pub mod core_0_statustable11;
#[doc = "Core_0_STATUSTABLE12 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE12_SPEC>`"]
pub type CORE_0_STATUSTABLE12 = crate::Reg<core_0_statustable12::CORE_0_STATUSTABLE12_SPEC>;
#[doc = "Status register of world switch of entry 12"]
pub mod core_0_statustable12;
#[doc = "Core_0_STATUSTABLE13 (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE13_SPEC>`"]
pub type CORE_0_STATUSTABLE13 = crate::Reg<core_0_statustable13::CORE_0_STATUSTABLE13_SPEC>;
#[doc = "Status register of world switch of entry 13"]
pub mod core_0_statustable13;
#[doc = "Core_0_STATUSTABLE_CURRENT (rw) register accessor: an alias for `Reg<CORE_0_STATUSTABLE_CURRENT_SPEC>`"]
pub type CORE_0_STATUSTABLE_CURRENT =
    crate::Reg<core_0_statustable_current::CORE_0_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Status register of statustable current"]
pub mod core_0_statustable_current;
#[doc = "Core_0_MESSAGE_ADDR (rw) register accessor: an alias for `Reg<CORE_0_MESSAGE_ADDR_SPEC>`"]
pub type CORE_0_MESSAGE_ADDR = crate::Reg<core_0_message_addr::CORE_0_MESSAGE_ADDR_SPEC>;
#[doc = "Clear writer_buffer write address configuration register"]
pub mod core_0_message_addr;
#[doc = "Core_0_MESSAGE_MAX (rw) register accessor: an alias for `Reg<CORE_0_MESSAGE_MAX_SPEC>`"]
pub type CORE_0_MESSAGE_MAX = crate::Reg<core_0_message_max::CORE_0_MESSAGE_MAX_SPEC>;
#[doc = "Clear writer_buffer write number configuration register"]
pub mod core_0_message_max;
#[doc = "Core_0_MESSAGE_PHASE (r) register accessor: an alias for `Reg<CORE_0_MESSAGE_PHASE_SPEC>`"]
pub type CORE_0_MESSAGE_PHASE = crate::Reg<core_0_message_phase::CORE_0_MESSAGE_PHASE_SPEC>;
#[doc = "Clear writer_buffer status register"]
pub mod core_0_message_phase;
#[doc = "Core_0_World_TRIGGER_ADDR (rw) register accessor: an alias for `Reg<CORE_0_WORLD_TRIGGER_ADDR_SPEC>`"]
pub type CORE_0_WORLD_TRIGGER_ADDR =
    crate::Reg<core_0_world_trigger_addr::CORE_0_WORLD_TRIGGER_ADDR_SPEC>;
#[doc = "Core_0 trigger address configuration Register"]
pub mod core_0_world_trigger_addr;
#[doc = "Core_0_World_PREPARE (rw) register accessor: an alias for `Reg<CORE_0_WORLD_PREPARE_SPEC>`"]
pub type CORE_0_WORLD_PREPARE = crate::Reg<core_0_world_prepare::CORE_0_WORLD_PREPARE_SPEC>;
#[doc = "Core_0 prepare world configuration Register"]
pub mod core_0_world_prepare;
#[doc = "Core_0_World_UPDATE (w) register accessor: an alias for `Reg<CORE_0_WORLD_UPDATE_SPEC>`"]
pub type CORE_0_WORLD_UPDATE = crate::Reg<core_0_world_update::CORE_0_WORLD_UPDATE_SPEC>;
#[doc = "Core_0 configuration update register"]
pub mod core_0_world_update;
#[doc = "Core_0_World_Cancel (w) register accessor: an alias for `Reg<CORE_0_WORLD_CANCEL_SPEC>`"]
pub type CORE_0_WORLD_CANCEL = crate::Reg<core_0_world_cancel::CORE_0_WORLD_CANCEL_SPEC>;
#[doc = "Core_0 configuration cancel register"]
pub mod core_0_world_cancel;
#[doc = "Core_0_World_IRam0 (rw) register accessor: an alias for `Reg<CORE_0_WORLD_IRAM0_SPEC>`"]
pub type CORE_0_WORLD_IRAM0 = crate::Reg<core_0_world_iram0::CORE_0_WORLD_IRAM0_SPEC>;
#[doc = "Core_0 Iram0 world register"]
pub mod core_0_world_iram0;
#[doc = "Core_0_World_DRam0_PIF (rw) register accessor: an alias for `Reg<CORE_0_WORLD_DRAM0_PIF_SPEC>`"]
pub type CORE_0_WORLD_DRAM0_PIF = crate::Reg<core_0_world_dram0_pif::CORE_0_WORLD_DRAM0_PIF_SPEC>;
#[doc = "Core_0 dram0 and PIF world register"]
pub mod core_0_world_dram0_pif;
#[doc = "Core_0_World_Phase (r) register accessor: an alias for `Reg<CORE_0_WORLD_PHASE_SPEC>`"]
pub type CORE_0_WORLD_PHASE = crate::Reg<core_0_world_phase::CORE_0_WORLD_PHASE_SPEC>;
#[doc = "Core_0 world status register"]
pub mod core_0_world_phase;
#[doc = "Core_0_NMI_MASK_ENABLE (w) register accessor: an alias for `Reg<CORE_0_NMI_MASK_ENABLE_SPEC>`"]
pub type CORE_0_NMI_MASK_ENABLE = crate::Reg<core_0_nmi_mask_enable::CORE_0_NMI_MASK_ENABLE_SPEC>;
#[doc = "Core_0 NMI mask enable register"]
pub mod core_0_nmi_mask_enable;
#[doc = "Core_0_NMI_MASK_TRIGGER_ADDR (rw) register accessor: an alias for `Reg<CORE_0_NMI_MASK_TRIGGER_ADDR_SPEC>`"]
pub type CORE_0_NMI_MASK_TRIGGER_ADDR =
    crate::Reg<core_0_nmi_mask_trigger_addr::CORE_0_NMI_MASK_TRIGGER_ADDR_SPEC>;
#[doc = "Core_0 NMI mask trigger address register"]
pub mod core_0_nmi_mask_trigger_addr;
#[doc = "Core_0_NMI_MASK_DISABLE (w) register accessor: an alias for `Reg<CORE_0_NMI_MASK_DISABLE_SPEC>`"]
pub type CORE_0_NMI_MASK_DISABLE =
    crate::Reg<core_0_nmi_mask_disable::CORE_0_NMI_MASK_DISABLE_SPEC>;
#[doc = "Core_0 NMI mask disable register"]
pub mod core_0_nmi_mask_disable;
#[doc = "Core_0_NMI_MASK_CANCLE (w) register accessor: an alias for `Reg<CORE_0_NMI_MASK_CANCLE_SPEC>`"]
pub type CORE_0_NMI_MASK_CANCLE = crate::Reg<core_0_nmi_mask_cancle::CORE_0_NMI_MASK_CANCLE_SPEC>;
#[doc = "Core_0 NMI mask disable register"]
pub mod core_0_nmi_mask_cancle;
#[doc = "Core_0_NMI_MASK (rw) register accessor: an alias for `Reg<CORE_0_NMI_MASK_SPEC>`"]
pub type CORE_0_NMI_MASK = crate::Reg<core_0_nmi_mask::CORE_0_NMI_MASK_SPEC>;
#[doc = "Core_0 NMI mask register"]
pub mod core_0_nmi_mask;
#[doc = "Core_0_NMI_MASK_PHASE (r) register accessor: an alias for `Reg<CORE_0_NMI_MASK_PHASE_SPEC>`"]
pub type CORE_0_NMI_MASK_PHASE = crate::Reg<core_0_nmi_mask_phase::CORE_0_NMI_MASK_PHASE_SPEC>;
#[doc = "Core_0 NMI mask phase register"]
pub mod core_0_nmi_mask_phase;
#[doc = "Core_1_ENTRY_1_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_1_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_1_ADDR = crate::Reg<core_1_entry_1_addr::CORE_1_ENTRY_1_ADDR_SPEC>;
#[doc = "Core_1 Entry 1 address configuration Register"]
pub mod core_1_entry_1_addr;
#[doc = "Core_1_ENTRY_2_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_2_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_2_ADDR = crate::Reg<core_1_entry_2_addr::CORE_1_ENTRY_2_ADDR_SPEC>;
#[doc = "Core_1 Entry 2 address configuration Register"]
pub mod core_1_entry_2_addr;
#[doc = "Core_1_ENTRY_3_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_3_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_3_ADDR = crate::Reg<core_1_entry_3_addr::CORE_1_ENTRY_3_ADDR_SPEC>;
#[doc = "Core_1 Entry 3 address configuration Register"]
pub mod core_1_entry_3_addr;
#[doc = "Core_1_ENTRY_4_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_4_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_4_ADDR = crate::Reg<core_1_entry_4_addr::CORE_1_ENTRY_4_ADDR_SPEC>;
#[doc = "Core_1 Entry 4 address configuration Register"]
pub mod core_1_entry_4_addr;
#[doc = "Core_1_ENTRY_5_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_5_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_5_ADDR = crate::Reg<core_1_entry_5_addr::CORE_1_ENTRY_5_ADDR_SPEC>;
#[doc = "Core_1 Entry 5 address configuration Register"]
pub mod core_1_entry_5_addr;
#[doc = "Core_1_ENTRY_6_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_6_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_6_ADDR = crate::Reg<core_1_entry_6_addr::CORE_1_ENTRY_6_ADDR_SPEC>;
#[doc = "Core_1 Entry 6 address configuration Register"]
pub mod core_1_entry_6_addr;
#[doc = "Core_1_ENTRY_7_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_7_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_7_ADDR = crate::Reg<core_1_entry_7_addr::CORE_1_ENTRY_7_ADDR_SPEC>;
#[doc = "Core_1 Entry 7 address configuration Register"]
pub mod core_1_entry_7_addr;
#[doc = "Core_1_ENTRY_8_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_8_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_8_ADDR = crate::Reg<core_1_entry_8_addr::CORE_1_ENTRY_8_ADDR_SPEC>;
#[doc = "Core_1 Entry 8 address configuration Register"]
pub mod core_1_entry_8_addr;
#[doc = "Core_1_ENTRY_9_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_9_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_9_ADDR = crate::Reg<core_1_entry_9_addr::CORE_1_ENTRY_9_ADDR_SPEC>;
#[doc = "Core_1 Entry 9 address configuration Register"]
pub mod core_1_entry_9_addr;
#[doc = "Core_1_ENTRY_10_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_10_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_10_ADDR = crate::Reg<core_1_entry_10_addr::CORE_1_ENTRY_10_ADDR_SPEC>;
#[doc = "Core_1 Entry 10 address configuration Register"]
pub mod core_1_entry_10_addr;
#[doc = "Core_1_ENTRY_11_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_11_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_11_ADDR = crate::Reg<core_1_entry_11_addr::CORE_1_ENTRY_11_ADDR_SPEC>;
#[doc = "Core_1 Entry 11 address configuration Register"]
pub mod core_1_entry_11_addr;
#[doc = "Core_1_ENTRY_12_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_12_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_12_ADDR = crate::Reg<core_1_entry_12_addr::CORE_1_ENTRY_12_ADDR_SPEC>;
#[doc = "Core_1 Entry 12 address configuration Register"]
pub mod core_1_entry_12_addr;
#[doc = "Core_1_ENTRY_13_ADDR (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_13_ADDR_SPEC>`"]
pub type CORE_1_ENTRY_13_ADDR = crate::Reg<core_1_entry_13_addr::CORE_1_ENTRY_13_ADDR_SPEC>;
#[doc = "Core_1 Entry 13 address configuration Register"]
pub mod core_1_entry_13_addr;
#[doc = "Core_1_ENTRY_CHECK (rw) register accessor: an alias for `Reg<CORE_1_ENTRY_CHECK_SPEC>`"]
pub type CORE_1_ENTRY_CHECK = crate::Reg<core_1_entry_check::CORE_1_ENTRY_CHECK_SPEC>;
#[doc = "Core_1 Entry check configuration Register"]
pub mod core_1_entry_check;
#[doc = "Core_1_STATUSTABLE1 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE1_SPEC>`"]
pub type CORE_1_STATUSTABLE1 = crate::Reg<core_1_statustable1::CORE_1_STATUSTABLE1_SPEC>;
#[doc = "Status register of world switch of entry 1"]
pub mod core_1_statustable1;
#[doc = "Core_1_STATUSTABLE2 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE2_SPEC>`"]
pub type CORE_1_STATUSTABLE2 = crate::Reg<core_1_statustable2::CORE_1_STATUSTABLE2_SPEC>;
#[doc = "Status register of world switch of entry 2"]
pub mod core_1_statustable2;
#[doc = "Core_1_STATUSTABLE3 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE3_SPEC>`"]
pub type CORE_1_STATUSTABLE3 = crate::Reg<core_1_statustable3::CORE_1_STATUSTABLE3_SPEC>;
#[doc = "Status register of world switch of entry 3"]
pub mod core_1_statustable3;
#[doc = "Core_1_STATUSTABLE4 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE4_SPEC>`"]
pub type CORE_1_STATUSTABLE4 = crate::Reg<core_1_statustable4::CORE_1_STATUSTABLE4_SPEC>;
#[doc = "Status register of world switch of entry 4"]
pub mod core_1_statustable4;
#[doc = "Core_1_STATUSTABLE5 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE5_SPEC>`"]
pub type CORE_1_STATUSTABLE5 = crate::Reg<core_1_statustable5::CORE_1_STATUSTABLE5_SPEC>;
#[doc = "Status register of world switch of entry 5"]
pub mod core_1_statustable5;
#[doc = "Core_1_STATUSTABLE6 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE6_SPEC>`"]
pub type CORE_1_STATUSTABLE6 = crate::Reg<core_1_statustable6::CORE_1_STATUSTABLE6_SPEC>;
#[doc = "Status register of world switch of entry 6"]
pub mod core_1_statustable6;
#[doc = "Core_1_STATUSTABLE7 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE7_SPEC>`"]
pub type CORE_1_STATUSTABLE7 = crate::Reg<core_1_statustable7::CORE_1_STATUSTABLE7_SPEC>;
#[doc = "Status register of world switch of entry 7"]
pub mod core_1_statustable7;
#[doc = "Core_1_STATUSTABLE8 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE8_SPEC>`"]
pub type CORE_1_STATUSTABLE8 = crate::Reg<core_1_statustable8::CORE_1_STATUSTABLE8_SPEC>;
#[doc = "Status register of world switch of entry 8"]
pub mod core_1_statustable8;
#[doc = "Core_1_STATUSTABLE9 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE9_SPEC>`"]
pub type CORE_1_STATUSTABLE9 = crate::Reg<core_1_statustable9::CORE_1_STATUSTABLE9_SPEC>;
#[doc = "Status register of world switch of entry 9"]
pub mod core_1_statustable9;
#[doc = "Core_1_STATUSTABLE10 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE10_SPEC>`"]
pub type CORE_1_STATUSTABLE10 = crate::Reg<core_1_statustable10::CORE_1_STATUSTABLE10_SPEC>;
#[doc = "Status register of world switch of entry 10"]
pub mod core_1_statustable10;
#[doc = "Core_1_STATUSTABLE11 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE11_SPEC>`"]
pub type CORE_1_STATUSTABLE11 = crate::Reg<core_1_statustable11::CORE_1_STATUSTABLE11_SPEC>;
#[doc = "Status register of world switch of entry 11"]
pub mod core_1_statustable11;
#[doc = "Core_1_STATUSTABLE12 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE12_SPEC>`"]
pub type CORE_1_STATUSTABLE12 = crate::Reg<core_1_statustable12::CORE_1_STATUSTABLE12_SPEC>;
#[doc = "Status register of world switch of entry 12"]
pub mod core_1_statustable12;
#[doc = "Core_1_STATUSTABLE13 (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE13_SPEC>`"]
pub type CORE_1_STATUSTABLE13 = crate::Reg<core_1_statustable13::CORE_1_STATUSTABLE13_SPEC>;
#[doc = "Status register of world switch of entry 13"]
pub mod core_1_statustable13;
#[doc = "Core_1_STATUSTABLE_CURRENT (rw) register accessor: an alias for `Reg<CORE_1_STATUSTABLE_CURRENT_SPEC>`"]
pub type CORE_1_STATUSTABLE_CURRENT =
    crate::Reg<core_1_statustable_current::CORE_1_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Status register of statustable current"]
pub mod core_1_statustable_current;
#[doc = "Core_1_MESSAGE_ADDR (rw) register accessor: an alias for `Reg<CORE_1_MESSAGE_ADDR_SPEC>`"]
pub type CORE_1_MESSAGE_ADDR = crate::Reg<core_1_message_addr::CORE_1_MESSAGE_ADDR_SPEC>;
#[doc = "Clear writer_buffer write address configuration register"]
pub mod core_1_message_addr;
#[doc = "Core_1_MESSAGE_MAX (rw) register accessor: an alias for `Reg<CORE_1_MESSAGE_MAX_SPEC>`"]
pub type CORE_1_MESSAGE_MAX = crate::Reg<core_1_message_max::CORE_1_MESSAGE_MAX_SPEC>;
#[doc = "Clear writer_buffer write number configuration register"]
pub mod core_1_message_max;
#[doc = "Core_1_MESSAGE_PHASE (r) register accessor: an alias for `Reg<CORE_1_MESSAGE_PHASE_SPEC>`"]
pub type CORE_1_MESSAGE_PHASE = crate::Reg<core_1_message_phase::CORE_1_MESSAGE_PHASE_SPEC>;
#[doc = "Clear writer_buffer status register"]
pub mod core_1_message_phase;
#[doc = "Core_1_World_TRIGGER_ADDR (rw) register accessor: an alias for `Reg<CORE_1_WORLD_TRIGGER_ADDR_SPEC>`"]
pub type CORE_1_WORLD_TRIGGER_ADDR =
    crate::Reg<core_1_world_trigger_addr::CORE_1_WORLD_TRIGGER_ADDR_SPEC>;
#[doc = "Core_1 trigger address configuration Register"]
pub mod core_1_world_trigger_addr;
#[doc = "Core_1_World_PREPARE (rw) register accessor: an alias for `Reg<CORE_1_WORLD_PREPARE_SPEC>`"]
pub type CORE_1_WORLD_PREPARE = crate::Reg<core_1_world_prepare::CORE_1_WORLD_PREPARE_SPEC>;
#[doc = "Core_1 prepare world configuration Register"]
pub mod core_1_world_prepare;
#[doc = "Core_1_World_UPDATE (w) register accessor: an alias for `Reg<CORE_1_WORLD_UPDATE_SPEC>`"]
pub type CORE_1_WORLD_UPDATE = crate::Reg<core_1_world_update::CORE_1_WORLD_UPDATE_SPEC>;
#[doc = "Core_1 configuration update register"]
pub mod core_1_world_update;
#[doc = "Core_1_World_Cancel (w) register accessor: an alias for `Reg<CORE_1_WORLD_CANCEL_SPEC>`"]
pub type CORE_1_WORLD_CANCEL = crate::Reg<core_1_world_cancel::CORE_1_WORLD_CANCEL_SPEC>;
#[doc = "Core_1 configuration cancel register"]
pub mod core_1_world_cancel;
#[doc = "Core_1_World_IRam0 (rw) register accessor: an alias for `Reg<CORE_1_WORLD_IRAM0_SPEC>`"]
pub type CORE_1_WORLD_IRAM0 = crate::Reg<core_1_world_iram0::CORE_1_WORLD_IRAM0_SPEC>;
#[doc = "Core_1 Iram0 world register"]
pub mod core_1_world_iram0;
#[doc = "Core_1_World_DRam0_PIF (rw) register accessor: an alias for `Reg<CORE_1_WORLD_DRAM0_PIF_SPEC>`"]
pub type CORE_1_WORLD_DRAM0_PIF = crate::Reg<core_1_world_dram0_pif::CORE_1_WORLD_DRAM0_PIF_SPEC>;
#[doc = "Core_1 dram0 and PIF world register"]
pub mod core_1_world_dram0_pif;
#[doc = "Core_1_World_Phase (r) register accessor: an alias for `Reg<CORE_1_WORLD_PHASE_SPEC>`"]
pub type CORE_1_WORLD_PHASE = crate::Reg<core_1_world_phase::CORE_1_WORLD_PHASE_SPEC>;
#[doc = "Core_0 world status register"]
pub mod core_1_world_phase;
#[doc = "Core_1_NMI_MASK_ENABLE (w) register accessor: an alias for `Reg<CORE_1_NMI_MASK_ENABLE_SPEC>`"]
pub type CORE_1_NMI_MASK_ENABLE = crate::Reg<core_1_nmi_mask_enable::CORE_1_NMI_MASK_ENABLE_SPEC>;
#[doc = "Core_1 NMI mask enable register"]
pub mod core_1_nmi_mask_enable;
#[doc = "Core_1_NMI_MASK_TRIGGER_ADDR (rw) register accessor: an alias for `Reg<CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC>`"]
pub type CORE_1_NMI_MASK_TRIGGER_ADDR =
    crate::Reg<core_1_nmi_mask_trigger_addr::CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC>;
#[doc = "Core_1 NMI mask trigger addr register"]
pub mod core_1_nmi_mask_trigger_addr;
#[doc = "Core_1_NMI_MASK_DISABLE (w) register accessor: an alias for `Reg<CORE_1_NMI_MASK_DISABLE_SPEC>`"]
pub type CORE_1_NMI_MASK_DISABLE =
    crate::Reg<core_1_nmi_mask_disable::CORE_1_NMI_MASK_DISABLE_SPEC>;
#[doc = "Core_1 NMI mask disable register"]
pub mod core_1_nmi_mask_disable;
#[doc = "Core_1_NMI_MASK_CANCLE (w) register accessor: an alias for `Reg<CORE_1_NMI_MASK_CANCLE_SPEC>`"]
pub type CORE_1_NMI_MASK_CANCLE = crate::Reg<core_1_nmi_mask_cancle::CORE_1_NMI_MASK_CANCLE_SPEC>;
#[doc = "Core_1 NMI mask disable register"]
pub mod core_1_nmi_mask_cancle;
#[doc = "Core_1_NMI_MASK (rw) register accessor: an alias for `Reg<CORE_1_NMI_MASK_SPEC>`"]
pub type CORE_1_NMI_MASK = crate::Reg<core_1_nmi_mask::CORE_1_NMI_MASK_SPEC>;
#[doc = "Core_1 NMI mask register"]
pub mod core_1_nmi_mask;
#[doc = "Core_1_NMI_MASK_PHASE (r) register accessor: an alias for `Reg<CORE_1_NMI_MASK_PHASE_SPEC>`"]
pub type CORE_1_NMI_MASK_PHASE = crate::Reg<core_1_nmi_mask_phase::CORE_1_NMI_MASK_PHASE_SPEC>;
#[doc = "Core_1 NMI mask phase register"]
pub mod core_1_nmi_mask_phase;
