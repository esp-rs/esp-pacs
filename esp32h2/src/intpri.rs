#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu_int_enable: CPU_INT_ENABLE,
    cpu_int_type: CPU_INT_TYPE,
    cpu_int_eip_status: CPU_INT_EIP_STATUS,
    cpu_int_pri: [CPU_INT_PRI; 32],
    cpu_int_thresh: CPU_INT_THRESH,
    cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    date: DATE,
    clock_gate: CLOCK_GATE,
    cpu_int_clear: CPU_INT_CLEAR,
    rnd_eco: RND_ECO,
    rnd_eco_low: RND_ECO_LOW,
    _reserved14: [u8; 0x0348],
    rnd_eco_high: RND_ECO_HIGH,
}
impl RegisterBlock {
    #[doc = "0x00 - register description"]
    #[inline(always)]
    pub const fn cpu_int_enable(&self) -> &CPU_INT_ENABLE {
        &self.cpu_int_enable
    }
    #[doc = "0x04 - register description"]
    #[inline(always)]
    pub const fn cpu_int_type(&self) -> &CPU_INT_TYPE {
        &self.cpu_int_type
    }
    #[doc = "0x08 - register description"]
    #[inline(always)]
    pub const fn cpu_int_eip_status(&self) -> &CPU_INT_EIP_STATUS {
        &self.cpu_int_eip_status
    }
    #[doc = "0x0c..0x8c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri(&self, n: usize) -> &CPU_INT_PRI {
        &self.cpu_int_pri[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x8c - register description"]
    #[inline(always)]
    pub fn cpu_int_pri_iter(&self) -> impl Iterator<Item = &CPU_INT_PRI> {
        self.cpu_int_pri.iter()
    }
    #[doc = "0x8c - register description"]
    #[inline(always)]
    pub const fn cpu_int_thresh(&self) -> &CPU_INT_THRESH {
        &self.cpu_int_thresh
    }
    #[doc = "0x90 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> &CPU_INTR_FROM_CPU_0 {
        &self.cpu_intr_from_cpu_0
    }
    #[doc = "0x94 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> &CPU_INTR_FROM_CPU_1 {
        &self.cpu_intr_from_cpu_1
    }
    #[doc = "0x98 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> &CPU_INTR_FROM_CPU_2 {
        &self.cpu_intr_from_cpu_2
    }
    #[doc = "0x9c - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> &CPU_INTR_FROM_CPU_3 {
        &self.cpu_intr_from_cpu_3
    }
    #[doc = "0xa0 - register description"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0xa4 - register description"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xa8 - register description"]
    #[inline(always)]
    pub const fn cpu_int_clear(&self) -> &CPU_INT_CLEAR {
        &self.cpu_int_clear
    }
    #[doc = "0xac - redcy eco register."]
    #[inline(always)]
    pub const fn rnd_eco(&self) -> &RND_ECO {
        &self.rnd_eco
    }
    #[doc = "0xb0 - redcy eco low register."]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    #[doc = "0x3fc - redcy eco high register."]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
}
#[doc = "CPU_INT_ENABLE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_enable`] module"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_type`] module"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_type;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_eip_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_eip_status`] module"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "register description"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_pri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_pri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri`] module"]
pub type CPU_INT_PRI = crate::Reg<cpu_int_pri::CPU_INT_PRI_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri;
#[doc = "CPU_INT_THRESH (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_thresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_thresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_thresh`] module"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "register description"]
pub mod cpu_int_thresh;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0`] module"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1`] module"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2`] module"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3`] module"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3;
#[doc = "DATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "register description"]
pub mod date;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "CPU_INT_CLEAR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_clear`] module"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "register description"]
pub mod cpu_int_clear;
#[doc = "RND_ECO (rw) register accessor: redcy eco register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco`] module"]
pub type RND_ECO = crate::Reg<rnd_eco::RND_ECO_SPEC>;
#[doc = "redcy eco register."]
pub mod rnd_eco;
#[doc = "RND_ECO_LOW (rw) register accessor: redcy eco low register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "redcy eco low register."]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: redcy eco high register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "redcy eco high register."]
pub mod rnd_eco_high;
