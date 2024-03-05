#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu_int_enable: CPU_INT_ENABLE,
    cpu_int_type: CPU_INT_TYPE,
    cpu_int_eip_status: CPU_INT_EIP_STATUS,
    cpu_int_pri_0: CPU_INT_PRI_0,
    cpu_int_pri_1: CPU_INT_PRI_1,
    cpu_int_pri_2: CPU_INT_PRI_2,
    cpu_int_pri_3: CPU_INT_PRI_3,
    cpu_int_pri_4: CPU_INT_PRI_4,
    cpu_int_pri_5: CPU_INT_PRI_5,
    cpu_int_pri_6: CPU_INT_PRI_6,
    cpu_int_pri_7: CPU_INT_PRI_7,
    cpu_int_pri_8: CPU_INT_PRI_8,
    cpu_int_pri_9: CPU_INT_PRI_9,
    cpu_int_pri_10: CPU_INT_PRI_10,
    cpu_int_pri_11: CPU_INT_PRI_11,
    cpu_int_pri_12: CPU_INT_PRI_12,
    cpu_int_pri_13: CPU_INT_PRI_13,
    cpu_int_pri_14: CPU_INT_PRI_14,
    cpu_int_pri_15: CPU_INT_PRI_15,
    cpu_int_pri_16: CPU_INT_PRI_16,
    cpu_int_pri_17: CPU_INT_PRI_17,
    cpu_int_pri_18: CPU_INT_PRI_18,
    cpu_int_pri_19: CPU_INT_PRI_19,
    cpu_int_pri_20: CPU_INT_PRI_20,
    cpu_int_pri_21: CPU_INT_PRI_21,
    cpu_int_pri_22: CPU_INT_PRI_22,
    cpu_int_pri_23: CPU_INT_PRI_23,
    cpu_int_pri_24: CPU_INT_PRI_24,
    cpu_int_pri_25: CPU_INT_PRI_25,
    cpu_int_pri_26: CPU_INT_PRI_26,
    cpu_int_pri_27: CPU_INT_PRI_27,
    cpu_int_pri_28: CPU_INT_PRI_28,
    cpu_int_pri_29: CPU_INT_PRI_29,
    cpu_int_pri_30: CPU_INT_PRI_30,
    cpu_int_pri_31: CPU_INT_PRI_31,
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
    _reserved45: [u8; 0x0348],
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
    #[doc = "0x0c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_0(&self) -> &CPU_INT_PRI_0 {
        &self.cpu_int_pri_0
    }
    #[doc = "0x10 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_1(&self) -> &CPU_INT_PRI_1 {
        &self.cpu_int_pri_1
    }
    #[doc = "0x14 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_2(&self) -> &CPU_INT_PRI_2 {
        &self.cpu_int_pri_2
    }
    #[doc = "0x18 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_3(&self) -> &CPU_INT_PRI_3 {
        &self.cpu_int_pri_3
    }
    #[doc = "0x1c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_4(&self) -> &CPU_INT_PRI_4 {
        &self.cpu_int_pri_4
    }
    #[doc = "0x20 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_5(&self) -> &CPU_INT_PRI_5 {
        &self.cpu_int_pri_5
    }
    #[doc = "0x24 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_6(&self) -> &CPU_INT_PRI_6 {
        &self.cpu_int_pri_6
    }
    #[doc = "0x28 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_7(&self) -> &CPU_INT_PRI_7 {
        &self.cpu_int_pri_7
    }
    #[doc = "0x2c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_8(&self) -> &CPU_INT_PRI_8 {
        &self.cpu_int_pri_8
    }
    #[doc = "0x30 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_9(&self) -> &CPU_INT_PRI_9 {
        &self.cpu_int_pri_9
    }
    #[doc = "0x34 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_10(&self) -> &CPU_INT_PRI_10 {
        &self.cpu_int_pri_10
    }
    #[doc = "0x38 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_11(&self) -> &CPU_INT_PRI_11 {
        &self.cpu_int_pri_11
    }
    #[doc = "0x3c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_12(&self) -> &CPU_INT_PRI_12 {
        &self.cpu_int_pri_12
    }
    #[doc = "0x40 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_13(&self) -> &CPU_INT_PRI_13 {
        &self.cpu_int_pri_13
    }
    #[doc = "0x44 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_14(&self) -> &CPU_INT_PRI_14 {
        &self.cpu_int_pri_14
    }
    #[doc = "0x48 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_15(&self) -> &CPU_INT_PRI_15 {
        &self.cpu_int_pri_15
    }
    #[doc = "0x4c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_16(&self) -> &CPU_INT_PRI_16 {
        &self.cpu_int_pri_16
    }
    #[doc = "0x50 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_17(&self) -> &CPU_INT_PRI_17 {
        &self.cpu_int_pri_17
    }
    #[doc = "0x54 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_18(&self) -> &CPU_INT_PRI_18 {
        &self.cpu_int_pri_18
    }
    #[doc = "0x58 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_19(&self) -> &CPU_INT_PRI_19 {
        &self.cpu_int_pri_19
    }
    #[doc = "0x5c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_20(&self) -> &CPU_INT_PRI_20 {
        &self.cpu_int_pri_20
    }
    #[doc = "0x60 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_21(&self) -> &CPU_INT_PRI_21 {
        &self.cpu_int_pri_21
    }
    #[doc = "0x64 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_22(&self) -> &CPU_INT_PRI_22 {
        &self.cpu_int_pri_22
    }
    #[doc = "0x68 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_23(&self) -> &CPU_INT_PRI_23 {
        &self.cpu_int_pri_23
    }
    #[doc = "0x6c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_24(&self) -> &CPU_INT_PRI_24 {
        &self.cpu_int_pri_24
    }
    #[doc = "0x70 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_25(&self) -> &CPU_INT_PRI_25 {
        &self.cpu_int_pri_25
    }
    #[doc = "0x74 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_26(&self) -> &CPU_INT_PRI_26 {
        &self.cpu_int_pri_26
    }
    #[doc = "0x78 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_27(&self) -> &CPU_INT_PRI_27 {
        &self.cpu_int_pri_27
    }
    #[doc = "0x7c - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_28(&self) -> &CPU_INT_PRI_28 {
        &self.cpu_int_pri_28
    }
    #[doc = "0x80 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_29(&self) -> &CPU_INT_PRI_29 {
        &self.cpu_int_pri_29
    }
    #[doc = "0x84 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_30(&self) -> &CPU_INT_PRI_30 {
        &self.cpu_int_pri_30
    }
    #[doc = "0x88 - register description"]
    #[inline(always)]
    pub const fn cpu_int_pri_31(&self) -> &CPU_INT_PRI_31 {
        &self.cpu_int_pri_31
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
#[doc = "CPU_INT_ENABLE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_enable`] module"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_type`] module"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_type;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_eip_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_eip_status`] module"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "register description"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_0`] module"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_1`] module"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_2`] module"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_3`] module"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_4`] module"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_5`] module"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_6`] module"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_7`] module"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_8`] module"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_9`] module"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_10`] module"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_11`] module"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_12`] module"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_13`] module"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_14`] module"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_15`] module"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_16`] module"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_17`] module"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_18`] module"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_19`] module"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_20`] module"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_21`] module"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_22`] module"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_23`] module"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_24`] module"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_25`] module"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_26`] module"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_27`] module"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_28`] module"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_29`] module"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_30`] module"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_31`] module"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_thresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_thresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_thresh`] module"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "register description"]
pub mod cpu_int_thresh;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0`] module"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1`] module"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2`] module"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3`] module"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3;
#[doc = "DATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "register description"]
pub mod date;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "CPU_INT_CLEAR (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_clear`] module"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "register description"]
pub mod cpu_int_clear;
#[doc = "RND_ECO (rw) register accessor: redcy eco register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco`] module"]
pub type RND_ECO = crate::Reg<rnd_eco::RND_ECO_SPEC>;
#[doc = "redcy eco register."]
pub mod rnd_eco;
#[doc = "RND_ECO_LOW (rw) register accessor: redcy eco low register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "redcy eco low register."]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: redcy eco high register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "redcy eco high register."]
pub mod rnd_eco_high;
