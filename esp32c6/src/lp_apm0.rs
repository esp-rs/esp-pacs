#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    region_filter_en: REGION_FILTER_EN,
    region: [REGION; 4],
    _reserved2: [u8; 0x90],
    func_ctrl: FUNC_CTRL,
    m: [M; 1],
    int_en: INT_EN,
    clock_gate: CLOCK_GATE,
    _reserved6: [u8; 0x071c],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Region filter enable register
    #[inline(always)]
    pub const fn region_filter_en(&self) -> &REGION_FILTER_EN {
        &self.region_filter_en
    }
    ///0x04..0x34 - Cluster REGION%s, containing REGION*_ADDR_START, REGION*_ADDR_END, REGION*_PMS_ATTR
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &REGION {
        &self.region[n]
    }
    ///Iterator for array of:
    ///0x04..0x34 - Cluster REGION%s, containing REGION*_ADDR_START, REGION*_ADDR_END, REGION*_PMS_ATTR
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &REGION> {
        self.region.iter()
    }
    ///0xc4 - PMS function control register
    #[inline(always)]
    pub const fn func_ctrl(&self) -> &FUNC_CTRL {
        &self.func_ctrl
    }
    ///0xc8..0xd8 - Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1
    #[inline(always)]
    pub const fn m(&self, n: usize) -> &M {
        &self.m[n]
    }
    ///Iterator for array of:
    ///0xc8..0xd8 - Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1
    #[inline(always)]
    pub fn m_iter(&self) -> impl Iterator<Item = &M> {
        self.m.iter()
    }
    ///0xd8 - APM interrupt enable register
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    ///0xdc - clock gating register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x7fc - Version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**REGION_FILTER_EN (rw) register accessor: Region filter enable register

You can [`read`](crate::generic::Reg::read) this register and get [`region_filter_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_filter_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@region_filter_en`] module*/
pub type REGION_FILTER_EN = crate::Reg<region_filter_en::REGION_FILTER_EN_SPEC>;
///Region filter enable register
pub mod region_filter_en;
///Cluster REGION%s, containing REGION*_ADDR_START, REGION*_ADDR_END, REGION*_PMS_ATTR
pub use self::region::REGION;
///Cluster
///Cluster REGION%s, containing REGION*_ADDR_START, REGION*_ADDR_END, REGION*_PMS_ATTR
pub mod region;
/**FUNC_CTRL (rw) register accessor: PMS function control register

You can [`read`](crate::generic::Reg::read) this register and get [`func_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_ctrl`] module*/
pub type FUNC_CTRL = crate::Reg<func_ctrl::FUNC_CTRL_SPEC>;
///PMS function control register
pub mod func_ctrl;
///Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1
pub use self::m::M;
///Cluster
///Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1
pub mod m;
/**INT_EN (rw) register accessor: APM interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_en`] module*/
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
///APM interrupt enable register
pub mod int_en;
/**CLOCK_GATE (rw) register accessor: clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///clock gating register
pub mod clock_gate;
/**DATE (rw) register accessor: Version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version register
pub mod date;
