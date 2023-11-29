#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    u_conf0: (),
    _reserved1: [u8; 0x04],
    u_conf1: (),
    _reserved2: [u8; 0x04],
    u_conf2: (),
    _reserved3: [u8; 0x28],
    u_cnt: [U_CNT; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    u_status: [U_STATUS; 4],
    ctrl: CTRL,
    _reserved10: [u8; 0x98],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Configuration register 0 for unit %s"]
    #[inline(always)]
    pub const fn u_conf0(&self, n: usize) -> &U_CONF0 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Configuration register 0 for unit %s"]
    #[inline(always)]
    pub fn u_conf0_iter(&self) -> impl Iterator<Item = &U_CONF0> {
        (0..4).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(12 * n).cast() })
    }
    #[doc = "0x00 - Configuration register 0 for unit 0"]
    #[inline(always)]
    pub const fn u0_conf0(&self) -> &U_CONF0 {
        self.u_conf0(0)
    }
    #[doc = "0x0c - Configuration register 0 for unit 1"]
    #[inline(always)]
    pub const fn u1_conf0(&self) -> &U_CONF0 {
        self.u_conf0(1)
    }
    #[doc = "0x18 - Configuration register 0 for unit 2"]
    #[inline(always)]
    pub const fn u2_conf0(&self) -> &U_CONF0 {
        self.u_conf0(2)
    }
    #[doc = "0x24 - Configuration register 0 for unit 3"]
    #[inline(always)]
    pub const fn u3_conf0(&self) -> &U_CONF0 {
        self.u_conf0(3)
    }
    #[doc = "0x04..0x14 - Configuration register 1 for unit %s"]
    #[inline(always)]
    pub const fn u_conf1(&self, n: usize) -> &U_CONF1 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - Configuration register 1 for unit %s"]
    #[inline(always)]
    pub fn u_conf1_iter(&self) -> impl Iterator<Item = &U_CONF1> {
        (0..4).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(12 * n).cast() })
    }
    #[doc = "0x04 - Configuration register 1 for unit 0"]
    #[inline(always)]
    pub const fn u0_conf1(&self) -> &U_CONF1 {
        self.u_conf1(0)
    }
    #[doc = "0x10 - Configuration register 1 for unit 1"]
    #[inline(always)]
    pub const fn u1_conf1(&self) -> &U_CONF1 {
        self.u_conf1(1)
    }
    #[doc = "0x1c - Configuration register 1 for unit 2"]
    #[inline(always)]
    pub const fn u2_conf1(&self) -> &U_CONF1 {
        self.u_conf1(2)
    }
    #[doc = "0x28 - Configuration register 1 for unit 3"]
    #[inline(always)]
    pub const fn u3_conf1(&self) -> &U_CONF1 {
        self.u_conf1(3)
    }
    #[doc = "0x08..0x18 - Configuration register 2 for unit %s"]
    #[inline(always)]
    pub const fn u_conf2(&self, n: usize) -> &U_CONF2 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - Configuration register 2 for unit %s"]
    #[inline(always)]
    pub fn u_conf2_iter(&self) -> impl Iterator<Item = &U_CONF2> {
        (0..4).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(12 * n).cast() })
    }
    #[doc = "0x08 - Configuration register 2 for unit 0"]
    #[inline(always)]
    pub const fn u0_conf2(&self) -> &U_CONF2 {
        self.u_conf2(0)
    }
    #[doc = "0x14 - Configuration register 2 for unit 1"]
    #[inline(always)]
    pub const fn u1_conf2(&self) -> &U_CONF2 {
        self.u_conf2(1)
    }
    #[doc = "0x20 - Configuration register 2 for unit 2"]
    #[inline(always)]
    pub const fn u2_conf2(&self) -> &U_CONF2 {
        self.u_conf2(2)
    }
    #[doc = "0x2c - Configuration register 2 for unit 3"]
    #[inline(always)]
    pub const fn u3_conf2(&self) -> &U_CONF2 {
        self.u_conf2(3)
    }
    #[doc = "0x30..0x40 - Counter value for unit %s"]
    #[inline(always)]
    pub const fn u_cnt(&self, n: usize) -> &U_CNT {
        &self.u_cnt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Counter value for unit %s"]
    #[inline(always)]
    pub fn u_cnt_iter(&self) -> impl Iterator<Item = &U_CNT> {
        self.u_cnt.iter()
    }
    #[doc = "0x30 - Counter value for unit 0"]
    #[inline(always)]
    pub const fn u0_cnt(&self) -> &U_CNT {
        self.u_cnt(0)
    }
    #[doc = "0x34 - Counter value for unit 1"]
    #[inline(always)]
    pub const fn u1_cnt(&self) -> &U_CNT {
        self.u_cnt(1)
    }
    #[doc = "0x38 - Counter value for unit 2"]
    #[inline(always)]
    pub const fn u2_cnt(&self) -> &U_CNT {
        self.u_cnt(2)
    }
    #[doc = "0x3c - Counter value for unit 3"]
    #[inline(always)]
    pub const fn u3_cnt(&self) -> &U_CNT {
        self.u_cnt(3)
    }
    #[doc = "0x40 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x44 - Interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x48 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x4c - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x50..0x60 - PNCT UNIT%s status register"]
    #[inline(always)]
    pub const fn u_status(&self, n: usize) -> &U_STATUS {
        &self.u_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - PNCT UNIT%s status register"]
    #[inline(always)]
    pub fn u_status_iter(&self) -> impl Iterator<Item = &U_STATUS> {
        self.u_status.iter()
    }
    #[doc = "0x50 - PNCT UNIT0 status register"]
    #[inline(always)]
    pub const fn u0_status(&self) -> &U_STATUS {
        self.u_status(0)
    }
    #[doc = "0x54 - PNCT UNIT1 status register"]
    #[inline(always)]
    pub const fn u1_status(&self) -> &U_STATUS {
        self.u_status(1)
    }
    #[doc = "0x58 - PNCT UNIT2 status register"]
    #[inline(always)]
    pub const fn u2_status(&self) -> &U_STATUS {
        self.u_status(2)
    }
    #[doc = "0x5c - PNCT UNIT3 status register"]
    #[inline(always)]
    pub const fn u3_status(&self) -> &U_STATUS {
        self.u_status(3)
    }
    #[doc = "0x60 - Control register for all counters"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xfc - PCNT version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "U_CONF0 (rw) register accessor: Configuration register 0 for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf0`] module"]
pub type U_CONF0 = crate::Reg<u_conf0::U_CONF0_SPEC>;
#[doc = "Configuration register 0 for unit %s"]
pub mod u_conf0;
#[doc = "U_CONF1 (rw) register accessor: Configuration register 1 for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf1`] module"]
pub type U_CONF1 = crate::Reg<u_conf1::U_CONF1_SPEC>;
#[doc = "Configuration register 1 for unit %s"]
pub mod u_conf1;
#[doc = "U_CONF2 (rw) register accessor: Configuration register 2 for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf2`] module"]
pub type U_CONF2 = crate::Reg<u_conf2::U_CONF2_SPEC>;
#[doc = "Configuration register 2 for unit %s"]
pub mod u_conf2;
#[doc = "U_CNT (r) register accessor: Counter value for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_cnt`] module"]
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
#[doc = "Counter value for unit %s"]
pub mod u_cnt;
#[doc = "INT_RAW (r) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "U_STATUS (r) register accessor: PNCT UNIT%s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_status`] module"]
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
#[doc = "PNCT UNIT%s status register"]
pub mod u_status;
#[doc = "CTRL (rw) register accessor: Control register for all counters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register for all counters"]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: PCNT version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "PCNT version control register"]
pub mod date;
