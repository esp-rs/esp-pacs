#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub struct T {
    config: CONFIG,
    lo: LO,
    hi: HI,
    update: UPDATE,
    alarmlo: ALARMLO,
    alarmhi: ALARMHI,
    loadlo: LOADLO,
    loadhi: LOADHI,
    load: LOAD,
}
impl T {
    ///0x00 - TIMG_T0CONFIG_REG.
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    ///0x04 - TIMG_T0LO_REG.
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
    ///0x08 - TIMG_T0HI_REG.
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x0c - TIMG_T0UPDATE_REG.
    #[inline(always)]
    pub const fn update(&self) -> &UPDATE {
        &self.update
    }
    ///0x10 - TIMG_T0ALARMLO_REG.
    #[inline(always)]
    pub const fn alarmlo(&self) -> &ALARMLO {
        &self.alarmlo
    }
    ///0x14 - TIMG_T0ALARMHI_REG.
    #[inline(always)]
    pub const fn alarmhi(&self) -> &ALARMHI {
        &self.alarmhi
    }
    ///0x18 - TIMG_T0LOADLO_REG.
    #[inline(always)]
    pub const fn loadlo(&self) -> &LOADLO {
        &self.loadlo
    }
    ///0x1c - TIMG_T0LOADHI_REG.
    #[inline(always)]
    pub const fn loadhi(&self) -> &LOADHI {
        &self.loadhi
    }
    ///0x20 - TIMG_T0LOAD_REG.
    #[inline(always)]
    pub const fn load(&self) -> &LOAD {
        &self.load
    }
}
/**CONFIG (rw) register accessor: TIMG_T0CONFIG_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
///TIMG_T0CONFIG_REG.
pub mod config;
/**LO (r) register accessor: TIMG_T0LO_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///TIMG_T0LO_REG.
pub mod lo;
/**HI (r) register accessor: TIMG_T0HI_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///TIMG_T0HI_REG.
pub mod hi;
/**UPDATE (rw) register accessor: TIMG_T0UPDATE_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@update`] module*/
pub type UPDATE = crate::Reg<update::UPDATE_SPEC>;
///TIMG_T0UPDATE_REG.
pub mod update;
/**ALARMLO (rw) register accessor: TIMG_T0ALARMLO_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@alarmlo`] module*/
pub type ALARMLO = crate::Reg<alarmlo::ALARMLO_SPEC>;
///TIMG_T0ALARMLO_REG.
pub mod alarmlo;
/**ALARMHI (rw) register accessor: TIMG_T0ALARMHI_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@alarmhi`] module*/
pub type ALARMHI = crate::Reg<alarmhi::ALARMHI_SPEC>;
///TIMG_T0ALARMHI_REG.
pub mod alarmhi;
/**LOADLO (rw) register accessor: TIMG_T0LOADLO_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@loadlo`] module*/
pub type LOADLO = crate::Reg<loadlo::LOADLO_SPEC>;
///TIMG_T0LOADLO_REG.
pub mod loadlo;
/**LOADHI (rw) register accessor: TIMG_T0LOADHI_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@loadhi`] module*/
pub type LOADHI = crate::Reg<loadhi::LOADHI_SPEC>;
///TIMG_T0LOADHI_REG.
pub mod loadhi;
/**LOAD (w) register accessor: TIMG_T0LOAD_REG.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load`] module*/
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
///TIMG_T0LOAD_REG.
pub mod load;
