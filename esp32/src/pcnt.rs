#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    unit: [UNIT; 8],
    u_cnt: [U_CNT; 8],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    u_status: [U_STATUS; 8],
    ctrl: CTRL,
    _reserved8: [u8; 0x48],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x60 - Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
    #[inline(always)]
    pub const fn unit(&self, n: usize) -> &UNIT {
        &self.unit[n]
    }
    ///Iterator for array of:
    ///0x00..0x60 - Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2
    #[inline(always)]
    pub fn unit_iter(&self) -> impl Iterator<Item = &UNIT> {
        self.unit.iter()
    }
    ///0x60..0x80 -
    #[inline(always)]
    pub const fn u_cnt(&self, n: usize) -> &U_CNT {
        &self.u_cnt[n]
    }
    ///Iterator for array of:
    ///0x60..0x80 -
    #[inline(always)]
    pub fn u_cnt_iter(&self) -> impl Iterator<Item = &U_CNT> {
        self.u_cnt.iter()
    }
    ///0x60 - U0_CNT
    #[inline(always)]
    pub const fn u0_cnt(&self) -> &U_CNT {
        self.u_cnt(0)
    }
    ///0x64 - U1_CNT
    #[inline(always)]
    pub const fn u1_cnt(&self) -> &U_CNT {
        self.u_cnt(1)
    }
    ///0x68 - U2_CNT
    #[inline(always)]
    pub const fn u2_cnt(&self) -> &U_CNT {
        self.u_cnt(2)
    }
    ///0x6c - U3_CNT
    #[inline(always)]
    pub const fn u3_cnt(&self) -> &U_CNT {
        self.u_cnt(3)
    }
    ///0x70 - U4_CNT
    #[inline(always)]
    pub const fn u4_cnt(&self) -> &U_CNT {
        self.u_cnt(4)
    }
    ///0x74 - U5_CNT
    #[inline(always)]
    pub const fn u5_cnt(&self) -> &U_CNT {
        self.u_cnt(5)
    }
    ///0x78 - U6_CNT
    #[inline(always)]
    pub const fn u6_cnt(&self) -> &U_CNT {
        self.u_cnt(6)
    }
    ///0x7c - U7_CNT
    #[inline(always)]
    pub const fn u7_cnt(&self) -> &U_CNT {
        self.u_cnt(7)
    }
    ///0x80 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x84 -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x88 -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x8c -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x90..0xb0 -
    #[inline(always)]
    pub const fn u_status(&self, n: usize) -> &U_STATUS {
        &self.u_status[n]
    }
    ///Iterator for array of:
    ///0x90..0xb0 -
    #[inline(always)]
    pub fn u_status_iter(&self) -> impl Iterator<Item = &U_STATUS> {
        self.u_status.iter()
    }
    ///0x90 - U0_STATUS
    #[inline(always)]
    pub const fn u0_status(&self) -> &U_STATUS {
        self.u_status(0)
    }
    ///0x94 - U1_STATUS
    #[inline(always)]
    pub const fn u1_status(&self) -> &U_STATUS {
        self.u_status(1)
    }
    ///0x98 - U2_STATUS
    #[inline(always)]
    pub const fn u2_status(&self) -> &U_STATUS {
        self.u_status(2)
    }
    ///0x9c - U3_STATUS
    #[inline(always)]
    pub const fn u3_status(&self) -> &U_STATUS {
        self.u_status(3)
    }
    ///0xa0 - U4_STATUS
    #[inline(always)]
    pub const fn u4_status(&self) -> &U_STATUS {
        self.u_status(4)
    }
    ///0xa4 - U5_STATUS
    #[inline(always)]
    pub const fn u5_status(&self) -> &U_STATUS {
        self.u_status(5)
    }
    ///0xa8 - U6_STATUS
    #[inline(always)]
    pub const fn u6_status(&self) -> &U_STATUS {
        self.u_status(6)
    }
    ///0xac - U7_STATUS
    #[inline(always)]
    pub const fn u7_status(&self) -> &U_STATUS {
        self.u_status(7)
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0xfc -
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
/**U_CNT (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`u_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@u_cnt`] module*/
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
///
pub mod u_cnt;
/**INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**U_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@u_status`] module*/
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
///
pub mod u_status;
/**CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///
pub mod ctrl;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
