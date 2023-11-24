#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    frc1_load: FRC1_LOAD,
    frc1_count: FRC1_COUNT,
    frc1_ctrl: FRC1_CTRL,
    frc1_int: FRC1_INT,
    _reserved4: [u8; 0x10],
    frc2_load: FRC2_LOAD,
    frc2_count: FRC2_COUNT,
    frc2_ctrl: FRC2_CTRL,
    frc2_int: FRC2_INT,
    frc2_alarm: FRC2_ALARM,
}
impl RegisterBlock {
    #[doc = "0x00 - the load value into the counter"]
    #[inline(always)]
    pub const fn frc1_load(&self) -> &FRC1_LOAD {
        &self.frc1_load
    }
    #[doc = "0x04 - the current value of the counter. It is a decreasingcounter."]
    #[inline(always)]
    pub const fn frc1_count(&self) -> &FRC1_COUNT {
        &self.frc1_count
    }
    #[doc = "0x08 - FRC1_CTRL"]
    #[inline(always)]
    pub const fn frc1_ctrl(&self) -> &FRC1_CTRL {
        &self.frc1_ctrl
    }
    #[doc = "0x0c - FRC1_INT"]
    #[inline(always)]
    pub const fn frc1_int(&self) -> &FRC1_INT {
        &self.frc1_int
    }
    #[doc = "0x20 - the load value into the counter"]
    #[inline(always)]
    pub const fn frc2_load(&self) -> &FRC2_LOAD {
        &self.frc2_load
    }
    #[doc = "0x24 - the current value of the counter. It is a increasingcounter."]
    #[inline(always)]
    pub const fn frc2_count(&self) -> &FRC2_COUNT {
        &self.frc2_count
    }
    #[doc = "0x28 - FRC2_CTRL"]
    #[inline(always)]
    pub const fn frc2_ctrl(&self) -> &FRC2_CTRL {
        &self.frc2_ctrl
    }
    #[doc = "0x2c - FRC2_INT"]
    #[inline(always)]
    pub const fn frc2_int(&self) -> &FRC2_INT {
        &self.frc2_int
    }
    #[doc = "0x30 - the alarm value for the counter"]
    #[inline(always)]
    pub const fn frc2_alarm(&self) -> &FRC2_ALARM {
        &self.frc2_alarm
    }
}
#[doc = "FRC1_LOAD (rw) register accessor: the load value into the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc1_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc1_load`] module"]
pub type FRC1_LOAD = crate::Reg<frc1_load::FRC1_LOAD_SPEC>;
#[doc = "the load value into the counter"]
pub mod frc1_load;
#[doc = "FRC1_COUNT (r) register accessor: the current value of the counter. It is a decreasingcounter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc1_count`] module"]
pub type FRC1_COUNT = crate::Reg<frc1_count::FRC1_COUNT_SPEC>;
#[doc = "the current value of the counter. It is a decreasingcounter."]
pub mod frc1_count;
#[doc = "FRC1_CTRL (rw) register accessor: FRC1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc1_ctrl`] module"]
pub type FRC1_CTRL = crate::Reg<frc1_ctrl::FRC1_CTRL_SPEC>;
#[doc = "FRC1_CTRL"]
pub mod frc1_ctrl;
#[doc = "FRC1_INT (rw) register accessor: FRC1_INT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc1_int`] module"]
pub type FRC1_INT = crate::Reg<frc1_int::FRC1_INT_SPEC>;
#[doc = "FRC1_INT"]
pub mod frc1_int;
#[doc = "FRC2_LOAD (rw) register accessor: the load value into the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc2_load`] module"]
pub type FRC2_LOAD = crate::Reg<frc2_load::FRC2_LOAD_SPEC>;
#[doc = "the load value into the counter"]
pub mod frc2_load;
#[doc = "FRC2_COUNT (r) register accessor: the current value of the counter. It is a increasingcounter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc2_count`] module"]
pub type FRC2_COUNT = crate::Reg<frc2_count::FRC2_COUNT_SPEC>;
#[doc = "the current value of the counter. It is a increasingcounter."]
pub mod frc2_count;
#[doc = "FRC2_CTRL (rw) register accessor: FRC2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc2_ctrl`] module"]
pub type FRC2_CTRL = crate::Reg<frc2_ctrl::FRC2_CTRL_SPEC>;
#[doc = "FRC2_CTRL"]
pub mod frc2_ctrl;
#[doc = "FRC2_INT (rw) register accessor: FRC2_INT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc2_int`] module"]
pub type FRC2_INT = crate::Reg<frc2_int::FRC2_INT_SPEC>;
#[doc = "FRC2_INT"]
pub mod frc2_int;
#[doc = "FRC2_ALARM (rw) register accessor: the alarm value for the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_alarm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_alarm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc2_alarm`] module"]
pub type FRC2_ALARM = crate::Reg<frc2_alarm::FRC2_ALARM_SPEC>;
#[doc = "the alarm value for the counter"]
pub mod frc2_alarm;
