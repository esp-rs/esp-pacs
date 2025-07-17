#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CPU%s, containing CORE_?_ENTRY_ADDR%s, Core_?_ENTRY_CHECK, Core_?_MESSAGE_ADDR, Core_?_MESSAGE_MAX, Core_?_MESSAGE_PHASE, Core_?_NMI_MASK, Core_?_NMI_MASK_CANCLE, Core_?_NMI_MASK_DISABLE, Core_?_NMI_MASK_ENABLE, Core_?_NMI_MASK_PHASE, Core_?_NMI_MASK_TRIGGER_ADDR, CORE_?_STATUSTABLE%s, CORE_?_STATUSTABLE_CURRENT, Core_?_World_Cancel, Core_?_World_DRam0_PIF, Core_?_World_IRam0, Core_?_World_Phase, Core_?_World_PREPARE, Core_?_World_TRIGGER_ADDR, Core_?_World_UPDATE"]
pub struct CPU {
    entry_addr: [ENTRY_ADDR; 13],
    _reserved1: [u8; 0x48],
    entry_check: ENTRY_CHECK,
    statustable: [STATUSTABLE; 13],
    _reserved3: [u8; 0x48],
    statustable_current: STATUSTABLE_CURRENT,
    message_addr: MESSAGE_ADDR,
    message_max: MESSAGE_MAX,
    message_phase: MESSAGE_PHASE,
    _reserved7: [u8; 0x34],
    world_trigger_addr: WORLD_TRIGGER_ADDR,
    world_prepare: WORLD_PREPARE,
    world_update: WORLD_UPDATE,
    world_cancel: WORLD_CANCEL,
    world_iram0: WORLD_IRAM0,
    world_dram0_pif: WORLD_DRAM0_PIF,
    world_phase: WORLD_PHASE,
    _reserved14: [u8; 0x24],
    nmi_mask_enable: NMI_MASK_ENABLE,
    nmi_mask_trigger_addr: NMI_MASK_TRIGGER_ADDR,
    nmi_mask_disable: NMI_MASK_DISABLE,
    nmi_mask_cancel: NMI_MASK_CANCEL,
    nmi_mask: NMI_MASK,
    nmi_mask_phase: NMI_MASK_PHASE,
    _reserved_end: [u8; 0x0268],
}
impl CPU {
    #[doc = "0x00..0x34 - Core_0 Entry %s address configuration Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `ENTRY_ADDR1` register.</div>"]
    #[inline(always)]
    pub const fn entry_addr(&self, n: usize) -> &ENTRY_ADDR {
        &self.entry_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x34 - Core_0 Entry %s address configuration Register"]
    #[inline(always)]
    pub fn entry_addr_iter(&self) -> impl Iterator<Item = &ENTRY_ADDR> {
        self.entry_addr.iter()
    }
    #[doc = "0x00 - Core_0 Entry 1 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr1(&self) -> &ENTRY_ADDR {
        self.entry_addr(0)
    }
    #[doc = "0x04 - Core_0 Entry 2 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr2(&self) -> &ENTRY_ADDR {
        self.entry_addr(1)
    }
    #[doc = "0x08 - Core_0 Entry 3 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr3(&self) -> &ENTRY_ADDR {
        self.entry_addr(2)
    }
    #[doc = "0x0c - Core_0 Entry 4 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr4(&self) -> &ENTRY_ADDR {
        self.entry_addr(3)
    }
    #[doc = "0x10 - Core_0 Entry 5 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr5(&self) -> &ENTRY_ADDR {
        self.entry_addr(4)
    }
    #[doc = "0x14 - Core_0 Entry 6 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr6(&self) -> &ENTRY_ADDR {
        self.entry_addr(5)
    }
    #[doc = "0x18 - Core_0 Entry 7 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr7(&self) -> &ENTRY_ADDR {
        self.entry_addr(6)
    }
    #[doc = "0x1c - Core_0 Entry 8 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr8(&self) -> &ENTRY_ADDR {
        self.entry_addr(7)
    }
    #[doc = "0x20 - Core_0 Entry 9 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr9(&self) -> &ENTRY_ADDR {
        self.entry_addr(8)
    }
    #[doc = "0x24 - Core_0 Entry 10 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr10(&self) -> &ENTRY_ADDR {
        self.entry_addr(9)
    }
    #[doc = "0x28 - Core_0 Entry 11 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr11(&self) -> &ENTRY_ADDR {
        self.entry_addr(10)
    }
    #[doc = "0x2c - Core_0 Entry 12 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr12(&self) -> &ENTRY_ADDR {
        self.entry_addr(11)
    }
    #[doc = "0x30 - Core_0 Entry 13 address configuration Register"]
    #[inline(always)]
    pub const fn entry_addr13(&self) -> &ENTRY_ADDR {
        self.entry_addr(12)
    }
    #[doc = "0x7c - Core_0 Entry check configuration Register"]
    #[inline(always)]
    pub const fn entry_check(&self) -> &ENTRY_CHECK {
        &self.entry_check
    }
    #[doc = "0x80..0xb4 - Status register of world switch of entry %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `STATUSTABLE1` register.</div>"]
    #[inline(always)]
    pub const fn statustable(&self, n: usize) -> &STATUSTABLE {
        &self.statustable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xb4 - Status register of world switch of entry %s"]
    #[inline(always)]
    pub fn statustable_iter(&self) -> impl Iterator<Item = &STATUSTABLE> {
        self.statustable.iter()
    }
    #[doc = "0x80 - Status register of world switch of entry 1"]
    #[inline(always)]
    pub const fn statustable1(&self) -> &STATUSTABLE {
        self.statustable(0)
    }
    #[doc = "0x84 - Status register of world switch of entry 2"]
    #[inline(always)]
    pub const fn statustable2(&self) -> &STATUSTABLE {
        self.statustable(1)
    }
    #[doc = "0x88 - Status register of world switch of entry 3"]
    #[inline(always)]
    pub const fn statustable3(&self) -> &STATUSTABLE {
        self.statustable(2)
    }
    #[doc = "0x8c - Status register of world switch of entry 4"]
    #[inline(always)]
    pub const fn statustable4(&self) -> &STATUSTABLE {
        self.statustable(3)
    }
    #[doc = "0x90 - Status register of world switch of entry 5"]
    #[inline(always)]
    pub const fn statustable5(&self) -> &STATUSTABLE {
        self.statustable(4)
    }
    #[doc = "0x94 - Status register of world switch of entry 6"]
    #[inline(always)]
    pub const fn statustable6(&self) -> &STATUSTABLE {
        self.statustable(5)
    }
    #[doc = "0x98 - Status register of world switch of entry 7"]
    #[inline(always)]
    pub const fn statustable7(&self) -> &STATUSTABLE {
        self.statustable(6)
    }
    #[doc = "0x9c - Status register of world switch of entry 8"]
    #[inline(always)]
    pub const fn statustable8(&self) -> &STATUSTABLE {
        self.statustable(7)
    }
    #[doc = "0xa0 - Status register of world switch of entry 9"]
    #[inline(always)]
    pub const fn statustable9(&self) -> &STATUSTABLE {
        self.statustable(8)
    }
    #[doc = "0xa4 - Status register of world switch of entry 10"]
    #[inline(always)]
    pub const fn statustable10(&self) -> &STATUSTABLE {
        self.statustable(9)
    }
    #[doc = "0xa8 - Status register of world switch of entry 11"]
    #[inline(always)]
    pub const fn statustable11(&self) -> &STATUSTABLE {
        self.statustable(10)
    }
    #[doc = "0xac - Status register of world switch of entry 12"]
    #[inline(always)]
    pub const fn statustable12(&self) -> &STATUSTABLE {
        self.statustable(11)
    }
    #[doc = "0xb0 - Status register of world switch of entry 13"]
    #[inline(always)]
    pub const fn statustable13(&self) -> &STATUSTABLE {
        self.statustable(12)
    }
    #[doc = "0xfc - Status register of statustable current"]
    #[inline(always)]
    pub const fn statustable_current(&self) -> &STATUSTABLE_CURRENT {
        &self.statustable_current
    }
    #[doc = "0x100 - Clear writer_buffer write address configuration register"]
    #[inline(always)]
    pub const fn message_addr(&self) -> &MESSAGE_ADDR {
        &self.message_addr
    }
    #[doc = "0x104 - Clear writer_buffer write number configuration register"]
    #[inline(always)]
    pub const fn message_max(&self) -> &MESSAGE_MAX {
        &self.message_max
    }
    #[doc = "0x108 - Clear writer_buffer status register"]
    #[inline(always)]
    pub const fn message_phase(&self) -> &MESSAGE_PHASE {
        &self.message_phase
    }
    #[doc = "0x140 - Core_0 trigger address configuration Register"]
    #[inline(always)]
    pub const fn world_trigger_addr(&self) -> &WORLD_TRIGGER_ADDR {
        &self.world_trigger_addr
    }
    #[doc = "0x144 - Core_0 prepare world configuration Register"]
    #[inline(always)]
    pub const fn world_prepare(&self) -> &WORLD_PREPARE {
        &self.world_prepare
    }
    #[doc = "0x148 - Core_0 configuration update register"]
    #[inline(always)]
    pub const fn world_update(&self) -> &WORLD_UPDATE {
        &self.world_update
    }
    #[doc = "0x14c - Core_0 configuration cancel register"]
    #[inline(always)]
    pub const fn world_cancel(&self) -> &WORLD_CANCEL {
        &self.world_cancel
    }
    #[doc = "0x150 - Core_0 Iram0 world register"]
    #[inline(always)]
    pub const fn world_iram0(&self) -> &WORLD_IRAM0 {
        &self.world_iram0
    }
    #[doc = "0x154 - Core_0 dram0 and PIF world register"]
    #[inline(always)]
    pub const fn world_dram0_pif(&self) -> &WORLD_DRAM0_PIF {
        &self.world_dram0_pif
    }
    #[doc = "0x158 - Core_0 world status register"]
    #[inline(always)]
    pub const fn world_phase(&self) -> &WORLD_PHASE {
        &self.world_phase
    }
    #[doc = "0x180 - Core_0 NMI mask enable register"]
    #[inline(always)]
    pub const fn nmi_mask_enable(&self) -> &NMI_MASK_ENABLE {
        &self.nmi_mask_enable
    }
    #[doc = "0x184 - Core_0 NMI mask trigger address register"]
    #[inline(always)]
    pub const fn nmi_mask_trigger_addr(&self) -> &NMI_MASK_TRIGGER_ADDR {
        &self.nmi_mask_trigger_addr
    }
    #[doc = "0x188 - Core_0 NMI mask disable register"]
    #[inline(always)]
    pub const fn nmi_mask_disable(&self) -> &NMI_MASK_DISABLE {
        &self.nmi_mask_disable
    }
    #[doc = "0x18c - Core_0 NMI mask disable register"]
    #[inline(always)]
    pub const fn nmi_mask_cancel(&self) -> &NMI_MASK_CANCEL {
        &self.nmi_mask_cancel
    }
    #[doc = "0x190 - Core_0 NMI mask register"]
    #[inline(always)]
    pub const fn nmi_mask(&self) -> &NMI_MASK {
        &self.nmi_mask
    }
    #[doc = "0x194 - Core_0 NMI mask phase register"]
    #[inline(always)]
    pub const fn nmi_mask_phase(&self) -> &NMI_MASK_PHASE {
        &self.nmi_mask_phase
    }
}
#[doc = "ENTRY_ADDR (rw) register accessor: Core_0 Entry %s address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`entry_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_addr`] module"]
pub type ENTRY_ADDR = crate::Reg<entry_addr::ENTRY_ADDR_SPEC>;
#[doc = "Core_0 Entry %s address configuration Register"]
pub mod entry_addr;
#[doc = "ENTRY_CHECK (rw) register accessor: Core_0 Entry check configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`entry_check::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry_check::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_check`] module"]
pub type ENTRY_CHECK = crate::Reg<entry_check::ENTRY_CHECK_SPEC>;
#[doc = "Core_0 Entry check configuration Register"]
pub mod entry_check;
#[doc = "MESSAGE_ADDR (rw) register accessor: Clear writer_buffer write address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`message_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_addr`] module"]
pub type MESSAGE_ADDR = crate::Reg<message_addr::MESSAGE_ADDR_SPEC>;
#[doc = "Clear writer_buffer write address configuration register"]
pub mod message_addr;
#[doc = "MESSAGE_MAX (rw) register accessor: Clear writer_buffer write number configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`message_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_max`] module"]
pub type MESSAGE_MAX = crate::Reg<message_max::MESSAGE_MAX_SPEC>;
#[doc = "Clear writer_buffer write number configuration register"]
pub mod message_max;
#[doc = "MESSAGE_PHASE (r) register accessor: Clear writer_buffer status register\n\nYou can [`read`](crate::Reg::read) this register and get [`message_phase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_phase`] module"]
pub type MESSAGE_PHASE = crate::Reg<message_phase::MESSAGE_PHASE_SPEC>;
#[doc = "Clear writer_buffer status register"]
pub mod message_phase;
#[doc = "NMI_MASK (rw) register accessor: Core_0 NMI mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask`] module"]
pub type NMI_MASK = crate::Reg<nmi_mask::NMI_MASK_SPEC>;
#[doc = "Core_0 NMI mask register"]
pub mod nmi_mask;
#[doc = "NMI_MASK_CANCEL (w) register accessor: Core_0 NMI mask disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask_cancel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask_cancel`] module"]
pub type NMI_MASK_CANCEL = crate::Reg<nmi_mask_cancel::NMI_MASK_CANCEL_SPEC>;
#[doc = "Core_0 NMI mask disable register"]
pub mod nmi_mask_cancel;
#[doc = "NMI_MASK_DISABLE (w) register accessor: Core_0 NMI mask disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask_disable`] module"]
pub type NMI_MASK_DISABLE = crate::Reg<nmi_mask_disable::NMI_MASK_DISABLE_SPEC>;
#[doc = "Core_0 NMI mask disable register"]
pub mod nmi_mask_disable;
#[doc = "NMI_MASK_ENABLE (w) register accessor: Core_0 NMI mask enable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask_enable`] module"]
pub type NMI_MASK_ENABLE = crate::Reg<nmi_mask_enable::NMI_MASK_ENABLE_SPEC>;
#[doc = "Core_0 NMI mask enable register"]
pub mod nmi_mask_enable;
#[doc = "NMI_MASK_PHASE (r) register accessor: Core_0 NMI mask phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_mask_phase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask_phase`] module"]
pub type NMI_MASK_PHASE = crate::Reg<nmi_mask_phase::NMI_MASK_PHASE_SPEC>;
#[doc = "Core_0 NMI mask phase register"]
pub mod nmi_mask_phase;
#[doc = "NMI_MASK_TRIGGER_ADDR (rw) register accessor: Core_0 NMI mask trigger address register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_mask_trigger_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask_trigger_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_mask_trigger_addr`] module"]
pub type NMI_MASK_TRIGGER_ADDR = crate::Reg<nmi_mask_trigger_addr::NMI_MASK_TRIGGER_ADDR_SPEC>;
#[doc = "Core_0 NMI mask trigger address register"]
pub mod nmi_mask_trigger_addr;
#[doc = "STATUSTABLE (rw) register accessor: Status register of world switch of entry %s\n\nYou can [`read`](crate::Reg::read) this register and get [`statustable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statustable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statustable`] module"]
pub type STATUSTABLE = crate::Reg<statustable::STATUSTABLE_SPEC>;
#[doc = "Status register of world switch of entry %s"]
pub mod statustable;
#[doc = "STATUSTABLE_CURRENT (rw) register accessor: Status register of statustable current\n\nYou can [`read`](crate::Reg::read) this register and get [`statustable_current::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statustable_current::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statustable_current`] module"]
pub type STATUSTABLE_CURRENT = crate::Reg<statustable_current::STATUSTABLE_CURRENT_SPEC>;
#[doc = "Status register of statustable current"]
pub mod statustable_current;
#[doc = "WORLD_CANCEL (w) register accessor: Core_0 configuration cancel register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_cancel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_cancel`] module"]
pub type WORLD_CANCEL = crate::Reg<world_cancel::WORLD_CANCEL_SPEC>;
#[doc = "Core_0 configuration cancel register"]
pub mod world_cancel;
#[doc = "WORLD_DRAM0_PIF (rw) register accessor: Core_0 dram0 and PIF world register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_dram0_pif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_dram0_pif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_dram0_pif`] module"]
pub type WORLD_DRAM0_PIF = crate::Reg<world_dram0_pif::WORLD_DRAM0_PIF_SPEC>;
#[doc = "Core_0 dram0 and PIF world register"]
pub mod world_dram0_pif;
#[doc = "WORLD_IRAM0 (rw) register accessor: Core_0 Iram0 world register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_iram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_iram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_iram0`] module"]
pub type WORLD_IRAM0 = crate::Reg<world_iram0::WORLD_IRAM0_SPEC>;
#[doc = "Core_0 Iram0 world register"]
pub mod world_iram0;
#[doc = "WORLD_PHASE (r) register accessor: Core_0 world status register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_phase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_phase`] module"]
pub type WORLD_PHASE = crate::Reg<world_phase::WORLD_PHASE_SPEC>;
#[doc = "Core_0 world status register"]
pub mod world_phase;
#[doc = "WORLD_PREPARE (rw) register accessor: Core_0 prepare world configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_prepare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_prepare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_prepare`] module"]
pub type WORLD_PREPARE = crate::Reg<world_prepare::WORLD_PREPARE_SPEC>;
#[doc = "Core_0 prepare world configuration Register"]
pub mod world_prepare;
#[doc = "WORLD_TRIGGER_ADDR (rw) register accessor: Core_0 trigger address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_trigger_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_trigger_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_trigger_addr`] module"]
pub type WORLD_TRIGGER_ADDR = crate::Reg<world_trigger_addr::WORLD_TRIGGER_ADDR_SPEC>;
#[doc = "Core_0 trigger address configuration Register"]
pub mod world_trigger_addr;
#[doc = "WORLD_UPDATE (w) register accessor: Core_0 configuration update register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@world_update`] module"]
pub type WORLD_UPDATE = crate::Reg<world_update::WORLD_UPDATE_SPEC>;
#[doc = "Core_0 configuration update register"]
pub mod world_update;
