#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    u_conf0: (),
    _reserved1: [u8; 0x04],
    u_conf1: (),
    _reserved2: [u8; 0x04],
    u_conf2: (),
    _reserved3: [u8; 0x58],
    u_cnt: [U_CNT; 8],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    u_status: [U_STATUS; 8],
    ctrl: CTRL,
    _reserved10: [u8; 0x48],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub const fn u_conf0(&self, n: usize) -> &U_CONF0 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub fn u_conf0_iter(&self) -> impl Iterator<Item = &U_CONF0> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(12 * n).cast() })
    }
    #[doc = "0x00 - U0_CONF0"]
    #[inline(always)]
    pub const fn u0_conf0(&self) -> &U_CONF0 {
        self.u_conf0(0)
    }
    #[doc = "0x0c - U1_CONF0"]
    #[inline(always)]
    pub const fn u1_conf0(&self) -> &U_CONF0 {
        self.u_conf0(1)
    }
    #[doc = "0x18 - U2_CONF0"]
    #[inline(always)]
    pub const fn u2_conf0(&self) -> &U_CONF0 {
        self.u_conf0(2)
    }
    #[doc = "0x24 - U3_CONF0"]
    #[inline(always)]
    pub const fn u3_conf0(&self) -> &U_CONF0 {
        self.u_conf0(3)
    }
    #[doc = "0x30 - U4_CONF0"]
    #[inline(always)]
    pub const fn u4_conf0(&self) -> &U_CONF0 {
        self.u_conf0(4)
    }
    #[doc = "0x3c - U5_CONF0"]
    #[inline(always)]
    pub const fn u5_conf0(&self) -> &U_CONF0 {
        self.u_conf0(5)
    }
    #[doc = "0x48 - U6_CONF0"]
    #[inline(always)]
    pub const fn u6_conf0(&self) -> &U_CONF0 {
        self.u_conf0(6)
    }
    #[doc = "0x54 - U7_CONF0"]
    #[inline(always)]
    pub const fn u7_conf0(&self) -> &U_CONF0 {
        self.u_conf0(7)
    }
    #[doc = "0x04..0x24 - "]
    #[inline(always)]
    pub const fn u_conf1(&self, n: usize) -> &U_CONF1 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x24 - "]
    #[inline(always)]
    pub fn u_conf1_iter(&self) -> impl Iterator<Item = &U_CONF1> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(12 * n).cast() })
    }
    #[doc = "0x04 - U0_CONF1"]
    #[inline(always)]
    pub const fn u0_conf1(&self) -> &U_CONF1 {
        self.u_conf1(0)
    }
    #[doc = "0x10 - U1_CONF1"]
    #[inline(always)]
    pub const fn u1_conf1(&self) -> &U_CONF1 {
        self.u_conf1(1)
    }
    #[doc = "0x1c - U2_CONF1"]
    #[inline(always)]
    pub const fn u2_conf1(&self) -> &U_CONF1 {
        self.u_conf1(2)
    }
    #[doc = "0x28 - U3_CONF1"]
    #[inline(always)]
    pub const fn u3_conf1(&self) -> &U_CONF1 {
        self.u_conf1(3)
    }
    #[doc = "0x34 - U4_CONF1"]
    #[inline(always)]
    pub const fn u4_conf1(&self) -> &U_CONF1 {
        self.u_conf1(4)
    }
    #[doc = "0x40 - U5_CONF1"]
    #[inline(always)]
    pub const fn u5_conf1(&self) -> &U_CONF1 {
        self.u_conf1(5)
    }
    #[doc = "0x4c - U6_CONF1"]
    #[inline(always)]
    pub const fn u6_conf1(&self) -> &U_CONF1 {
        self.u_conf1(6)
    }
    #[doc = "0x58 - U7_CONF1"]
    #[inline(always)]
    pub const fn u7_conf1(&self) -> &U_CONF1 {
        self.u_conf1(7)
    }
    #[doc = "0x08..0x28 - "]
    #[inline(always)]
    pub const fn u_conf2(&self, n: usize) -> &U_CONF2 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(12 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x28 - "]
    #[inline(always)]
    pub fn u_conf2_iter(&self) -> impl Iterator<Item = &U_CONF2> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(12 * n).cast() })
    }
    #[doc = "0x08 - U0_CONF2"]
    #[inline(always)]
    pub const fn u0_conf2(&self) -> &U_CONF2 {
        self.u_conf2(0)
    }
    #[doc = "0x14 - U1_CONF2"]
    #[inline(always)]
    pub const fn u1_conf2(&self) -> &U_CONF2 {
        self.u_conf2(1)
    }
    #[doc = "0x20 - U2_CONF2"]
    #[inline(always)]
    pub const fn u2_conf2(&self) -> &U_CONF2 {
        self.u_conf2(2)
    }
    #[doc = "0x2c - U3_CONF2"]
    #[inline(always)]
    pub const fn u3_conf2(&self) -> &U_CONF2 {
        self.u_conf2(3)
    }
    #[doc = "0x38 - U4_CONF2"]
    #[inline(always)]
    pub const fn u4_conf2(&self) -> &U_CONF2 {
        self.u_conf2(4)
    }
    #[doc = "0x44 - U5_CONF2"]
    #[inline(always)]
    pub const fn u5_conf2(&self) -> &U_CONF2 {
        self.u_conf2(5)
    }
    #[doc = "0x50 - U6_CONF2"]
    #[inline(always)]
    pub const fn u6_conf2(&self) -> &U_CONF2 {
        self.u_conf2(6)
    }
    #[doc = "0x5c - U7_CONF2"]
    #[inline(always)]
    pub const fn u7_conf2(&self) -> &U_CONF2 {
        self.u_conf2(7)
    }
    #[doc = "0x60..0x80 - "]
    #[inline(always)]
    pub const fn u_cnt(&self, n: usize) -> &U_CNT {
        &self.u_cnt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - "]
    #[inline(always)]
    pub fn u_cnt_iter(&self) -> impl Iterator<Item = &U_CNT> {
        self.u_cnt.iter()
    }
    #[doc = "0x60 - U0_CNT"]
    #[inline(always)]
    pub const fn u0_cnt(&self) -> &U_CNT {
        self.u_cnt(0)
    }
    #[doc = "0x64 - U1_CNT"]
    #[inline(always)]
    pub const fn u1_cnt(&self) -> &U_CNT {
        self.u_cnt(1)
    }
    #[doc = "0x68 - U2_CNT"]
    #[inline(always)]
    pub const fn u2_cnt(&self) -> &U_CNT {
        self.u_cnt(2)
    }
    #[doc = "0x6c - U3_CNT"]
    #[inline(always)]
    pub const fn u3_cnt(&self) -> &U_CNT {
        self.u_cnt(3)
    }
    #[doc = "0x70 - U4_CNT"]
    #[inline(always)]
    pub const fn u4_cnt(&self) -> &U_CNT {
        self.u_cnt(4)
    }
    #[doc = "0x74 - U5_CNT"]
    #[inline(always)]
    pub const fn u5_cnt(&self) -> &U_CNT {
        self.u_cnt(5)
    }
    #[doc = "0x78 - U6_CNT"]
    #[inline(always)]
    pub const fn u6_cnt(&self) -> &U_CNT {
        self.u_cnt(6)
    }
    #[doc = "0x7c - U7_CNT"]
    #[inline(always)]
    pub const fn u7_cnt(&self) -> &U_CNT {
        self.u_cnt(7)
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x90..0xb0 - "]
    #[inline(always)]
    pub const fn u_status(&self, n: usize) -> &U_STATUS {
        &self.u_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xb0 - "]
    #[inline(always)]
    pub fn u_status_iter(&self) -> impl Iterator<Item = &U_STATUS> {
        self.u_status.iter()
    }
    #[doc = "0x90 - U0_STATUS"]
    #[inline(always)]
    pub const fn u0_status(&self) -> &U_STATUS {
        self.u_status(0)
    }
    #[doc = "0x94 - U1_STATUS"]
    #[inline(always)]
    pub const fn u1_status(&self) -> &U_STATUS {
        self.u_status(1)
    }
    #[doc = "0x98 - U2_STATUS"]
    #[inline(always)]
    pub const fn u2_status(&self) -> &U_STATUS {
        self.u_status(2)
    }
    #[doc = "0x9c - U3_STATUS"]
    #[inline(always)]
    pub const fn u3_status(&self) -> &U_STATUS {
        self.u_status(3)
    }
    #[doc = "0xa0 - U4_STATUS"]
    #[inline(always)]
    pub const fn u4_status(&self) -> &U_STATUS {
        self.u_status(4)
    }
    #[doc = "0xa4 - U5_STATUS"]
    #[inline(always)]
    pub const fn u5_status(&self) -> &U_STATUS {
        self.u_status(5)
    }
    #[doc = "0xa8 - U6_STATUS"]
    #[inline(always)]
    pub const fn u6_status(&self) -> &U_STATUS {
        self.u_status(6)
    }
    #[doc = "0xac - U7_STATUS"]
    #[inline(always)]
    pub const fn u7_status(&self) -> &U_STATUS {
        self.u_status(7)
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "U_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf0`] module"]
pub type U_CONF0 = crate::Reg<u_conf0::U_CONF0_SPEC>;
#[doc = ""]
pub mod u_conf0;
#[doc = "U_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf1`] module"]
pub type U_CONF1 = crate::Reg<u_conf1::U_CONF1_SPEC>;
#[doc = ""]
pub mod u_conf1;
#[doc = "U_CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_conf2`] module"]
pub type U_CONF2 = crate::Reg<u_conf2::U_CONF2_SPEC>;
#[doc = ""]
pub mod u_conf2;
#[doc = "U_CNT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_cnt`] module"]
pub type U_CNT = crate::Reg<u_cnt::U_CNT_SPEC>;
#[doc = ""]
pub mod u_cnt;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "U_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_status`] module"]
pub type U_STATUS = crate::Reg<u_status::U_STATUS_SPEC>;
#[doc = ""]
pub mod u_status;
#[doc = "CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
