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
    ///0x00 - Timer 0 configuration register
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    ///0x04 - Timer 0 current value, low 32 bits
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
    ///0x08 - Timer 0 current value, high 32 bits
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x0c - Write to copy current timer value to TIMG_T0LO_REG or TIMGn_T0HI_REG
    #[inline(always)]
    pub const fn update(&self) -> &UPDATE {
        &self.update
    }
    ///0x10 - Timer 0 alarm value, low 32 bits
    #[inline(always)]
    pub const fn alarmlo(&self) -> &ALARMLO {
        &self.alarmlo
    }
    ///0x14 - Timer 0 alarm value, high bits
    #[inline(always)]
    pub const fn alarmhi(&self) -> &ALARMHI {
        &self.alarmhi
    }
    ///0x18 - Timer 0 reload value, low 32 bits
    #[inline(always)]
    pub const fn loadlo(&self) -> &LOADLO {
        &self.loadlo
    }
    ///0x1c - Timer 0 reload value, high 32 bits
    #[inline(always)]
    pub const fn loadhi(&self) -> &LOADHI {
        &self.loadhi
    }
    ///0x20 - Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG
    #[inline(always)]
    pub const fn load(&self) -> &LOAD {
        &self.load
    }
}
/**CONFIG (rw) register accessor: Timer 0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
///Timer 0 configuration register
pub mod config;
/**LO (r) register accessor: Timer 0 current value, low 32 bits

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///Timer 0 current value, low 32 bits
pub mod lo;
/**HI (r) register accessor: Timer 0 current value, high 32 bits

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///Timer 0 current value, high 32 bits
pub mod hi;
/**UPDATE (rw) register accessor: Write to copy current timer value to TIMG_T0LO_REG or TIMGn_T0HI_REG

You can [`read`](crate::generic::Reg::read) this register and get [`update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@update`] module*/
pub type UPDATE = crate::Reg<update::UPDATE_SPEC>;
///Write to copy current timer value to TIMG_T0LO_REG or TIMGn_T0HI_REG
pub mod update;
/**ALARMLO (rw) register accessor: Timer 0 alarm value, low 32 bits

You can [`read`](crate::generic::Reg::read) this register and get [`alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@alarmlo`] module*/
pub type ALARMLO = crate::Reg<alarmlo::ALARMLO_SPEC>;
///Timer 0 alarm value, low 32 bits
pub mod alarmlo;
/**ALARMHI (rw) register accessor: Timer 0 alarm value, high bits

You can [`read`](crate::generic::Reg::read) this register and get [`alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@alarmhi`] module*/
pub type ALARMHI = crate::Reg<alarmhi::ALARMHI_SPEC>;
///Timer 0 alarm value, high bits
pub mod alarmhi;
/**LOADLO (rw) register accessor: Timer 0 reload value, low 32 bits

You can [`read`](crate::generic::Reg::read) this register and get [`loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@loadlo`] module*/
pub type LOADLO = crate::Reg<loadlo::LOADLO_SPEC>;
///Timer 0 reload value, low 32 bits
pub mod loadlo;
/**LOADHI (rw) register accessor: Timer 0 reload value, high 32 bits

You can [`read`](crate::generic::Reg::read) this register and get [`loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@loadhi`] module*/
pub type LOADHI = crate::Reg<loadhi::LOADHI_SPEC>;
///Timer 0 reload value, high 32 bits
pub mod loadhi;
/**LOAD (w) register accessor: Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load`] module*/
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
///Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG
pub mod load;
