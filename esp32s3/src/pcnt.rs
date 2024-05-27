#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    unit: [UNIT; 4],
    u_cnt: [U_CNT; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    u_status: [U_STATUS; 4],
    ctrl: CTRL,
    _reserved8: [u8; 0x98],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x30 - Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
    #[inline(always)]
    pub const fn unit(&self, n: usize) -> &UNIT {
        &self.unit[n]
    }
    ///Iterator for array of:
    ///0x00..0x30 - Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
    #[inline(always)]
    pub fn unit_iter(&self) -> impl Iterator<Item = &UNIT> {
        self.unit.iter()
    }
    ///0x30..0x40 - Counter value for unit %s
    #[inline(always)]
    pub const fn u_cnt(&self, n: usize) -> &U_CNT {
        &self.u_cnt[n]
    }
    ///Iterator for array of:
    ///0x30..0x40 - Counter value for unit %s
    #[inline(always)]
    pub fn u_cnt_iter(&self) -> impl Iterator<Item = &U_CNT> {
        self.u_cnt.iter()
    }
    ///0x30 - Counter value for unit 0
    #[inline(always)]
    pub const fn u0_cnt(&self) -> &U_CNT {
        self.u_cnt(0)
    }
    ///0x34 - Counter value for unit 1
    #[inline(always)]
    pub const fn u1_cnt(&self) -> &U_CNT {
        self.u_cnt(1)
    }
    ///0x38 - Counter value for unit 2
    #[inline(always)]
    pub const fn u2_cnt(&self) -> &U_CNT {
        self.u_cnt(2)
    }
    ///0x3c - Counter value for unit 3
    #[inline(always)]
    pub const fn u3_cnt(&self) -> &U_CNT {
        self.u_cnt(3)
    }
    ///0x40 - Interrupt raw status register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x44 - Interrupt status register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x48 - Interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x4c - Interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x50..0x60 - PNCT UNIT%s status register
    #[inline(always)]
    pub const fn u_status(&self, n: usize) -> &U_STATUS {
        &self.u_status[n]
    }
    ///Iterator for array of:
    ///0x50..0x60 - PNCT UNIT%s status register
    #[inline(always)]
    pub fn u_status_iter(&self) -> impl Iterator<Item = &U_STATUS> {
        self.u_status.iter()
    }
    ///0x50 - PNCT UNIT0 status register
    #[inline(always)]
    pub const fn u0_status(&self) -> &U_STATUS {
        self.u_status(0)
    }
    ///0x54 - PNCT UNIT1 status register
    #[inline(always)]
    pub const fn u1_status(&self) -> &U_STATUS {
        self.u_status(1)
    }
    ///0x58 - PNCT UNIT2 status register
    #[inline(always)]
    pub const fn u2_status(&self) -> &U_STATUS {
        self.u_status(2)
    }
    ///0x5c - PNCT UNIT3 status register
    #[inline(always)]
    pub const fn u3_status(&self) -> &U_STATUS {
        self.u_status(3)
    }
    ///0x60 - Control register for all counters
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0xfc - PCNT version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
///Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
pub use self::unit::UNIT;
///Cluster
///Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
pub mod unit;
/**U_CNT (r) register accessor: Counter value for unit %s

You can [`read`](crate::generic::Reg::read) this register and get [`u_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@u_cnt`] module*/
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
///Counter value for unit %s
pub mod u_cnt;
/**INT_RAW (r) register accessor: Interrupt raw status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Interrupt raw status register
pub mod int_raw;
/**INT_ST (r) register accessor: Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Interrupt status register
pub mod int_st;
/**INT_ENA (rw) register accessor: Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable register
pub mod int_ena;
/**INT_CLR (w) register accessor: Interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear register
pub mod int_clr;
/**U_STATUS (r) register accessor: PNCT UNIT%s status register

You can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@u_status`] module*/
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
///PNCT UNIT%s status register
pub mod u_status;
/**CTRL (rw) register accessor: Control register for all counters

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///Control register for all counters
pub mod ctrl;
/**DATE (rw) register accessor: PCNT version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///PCNT version control register
pub mod date;
