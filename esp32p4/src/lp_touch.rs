#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    chn_status: CHN_STATUS,
    status_0: STATUS_0,
    status_1: STATUS_1,
    status_2: STATUS_2,
    status_3: STATUS_3,
    status_4: STATUS_4,
    status_5: STATUS_5,
    status_6: STATUS_6,
    status_7: STATUS_7,
    status_8: STATUS_8,
    status_9: STATUS_9,
    status_10: STATUS_10,
    status_11: STATUS_11,
    status_12: STATUS_12,
    status_13: STATUS_13,
    status_14: STATUS_14,
    status_15: STATUS_15,
    status_16: STATUS_16,
    status_17: STATUS_17,
    chn_tmp_status: CHN_TMP_STATUS,
    _reserved24: [u8; 0xa0],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need_des
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x04 - need_des
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x08 - need_des
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x0c - need_des
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x10 - need_des
    #[inline(always)]
    pub const fn chn_status(&self) -> &CHN_STATUS {
        &self.chn_status
    }
    ///0x14 - need_des
    #[inline(always)]
    pub const fn status_0(&self) -> &STATUS_0 {
        &self.status_0
    }
    ///0x18 - need_des
    #[inline(always)]
    pub const fn status_1(&self) -> &STATUS_1 {
        &self.status_1
    }
    ///0x1c - need_des
    #[inline(always)]
    pub const fn status_2(&self) -> &STATUS_2 {
        &self.status_2
    }
    ///0x20 - need_des
    #[inline(always)]
    pub const fn status_3(&self) -> &STATUS_3 {
        &self.status_3
    }
    ///0x24 - need_des
    #[inline(always)]
    pub const fn status_4(&self) -> &STATUS_4 {
        &self.status_4
    }
    ///0x28 - need_des
    #[inline(always)]
    pub const fn status_5(&self) -> &STATUS_5 {
        &self.status_5
    }
    ///0x2c - need_des
    #[inline(always)]
    pub const fn status_6(&self) -> &STATUS_6 {
        &self.status_6
    }
    ///0x30 - need_des
    #[inline(always)]
    pub const fn status_7(&self) -> &STATUS_7 {
        &self.status_7
    }
    ///0x34 - need_des
    #[inline(always)]
    pub const fn status_8(&self) -> &STATUS_8 {
        &self.status_8
    }
    ///0x38 - need_des
    #[inline(always)]
    pub const fn status_9(&self) -> &STATUS_9 {
        &self.status_9
    }
    ///0x3c - need_des
    #[inline(always)]
    pub const fn status_10(&self) -> &STATUS_10 {
        &self.status_10
    }
    ///0x40 - need_des
    #[inline(always)]
    pub const fn status_11(&self) -> &STATUS_11 {
        &self.status_11
    }
    ///0x44 - need_des
    #[inline(always)]
    pub const fn status_12(&self) -> &STATUS_12 {
        &self.status_12
    }
    ///0x48 - need_des
    #[inline(always)]
    pub const fn status_13(&self) -> &STATUS_13 {
        &self.status_13
    }
    ///0x4c - need_des
    #[inline(always)]
    pub const fn status_14(&self) -> &STATUS_14 {
        &self.status_14
    }
    ///0x50 - need_des
    #[inline(always)]
    pub const fn status_15(&self) -> &STATUS_15 {
        &self.status_15
    }
    ///0x54 - need_des
    #[inline(always)]
    pub const fn status_16(&self) -> &STATUS_16 {
        &self.status_16
    }
    ///0x58 - need_des
    #[inline(always)]
    pub const fn status_17(&self) -> &STATUS_17 {
        &self.status_17
    }
    ///0x5c - need_des
    #[inline(always)]
    pub const fn chn_tmp_status(&self) -> &CHN_TMP_STATUS {
        &self.chn_tmp_status
    }
    ///0x100 - need_des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**INT_RAW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///need_des
pub mod int_raw;
/**INT_ST (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///need_des
pub mod int_st;
/**INT_ENA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///need_des
pub mod int_ena;
/**INT_CLR (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///need_des
pub mod int_clr;
/**CHN_STATUS (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`chn_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chn_status`] module*/
pub type CHN_STATUS = crate::Reg<chn_status::CHN_STATUS_SPEC>;
///need_des
pub mod chn_status;
/**STATUS_0 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_0`] module*/
pub type STATUS_0 = crate::Reg<status_0::STATUS_0_SPEC>;
///need_des
pub mod status_0;
/**STATUS_1 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_1`] module*/
pub type STATUS_1 = crate::Reg<status_1::STATUS_1_SPEC>;
///need_des
pub mod status_1;
/**STATUS_2 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_2`] module*/
pub type STATUS_2 = crate::Reg<status_2::STATUS_2_SPEC>;
///need_des
pub mod status_2;
/**STATUS_3 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_3`] module*/
pub type STATUS_3 = crate::Reg<status_3::STATUS_3_SPEC>;
///need_des
pub mod status_3;
/**STATUS_4 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_4`] module*/
pub type STATUS_4 = crate::Reg<status_4::STATUS_4_SPEC>;
///need_des
pub mod status_4;
/**STATUS_5 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_5`] module*/
pub type STATUS_5 = crate::Reg<status_5::STATUS_5_SPEC>;
///need_des
pub mod status_5;
/**STATUS_6 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_6`] module*/
pub type STATUS_6 = crate::Reg<status_6::STATUS_6_SPEC>;
///need_des
pub mod status_6;
/**STATUS_7 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_7`] module*/
pub type STATUS_7 = crate::Reg<status_7::STATUS_7_SPEC>;
///need_des
pub mod status_7;
/**STATUS_8 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_8`] module*/
pub type STATUS_8 = crate::Reg<status_8::STATUS_8_SPEC>;
///need_des
pub mod status_8;
/**STATUS_9 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_9`] module*/
pub type STATUS_9 = crate::Reg<status_9::STATUS_9_SPEC>;
///need_des
pub mod status_9;
/**STATUS_10 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_10`] module*/
pub type STATUS_10 = crate::Reg<status_10::STATUS_10_SPEC>;
///need_des
pub mod status_10;
/**STATUS_11 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_11`] module*/
pub type STATUS_11 = crate::Reg<status_11::STATUS_11_SPEC>;
///need_des
pub mod status_11;
/**STATUS_12 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_12`] module*/
pub type STATUS_12 = crate::Reg<status_12::STATUS_12_SPEC>;
///need_des
pub mod status_12;
/**STATUS_13 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_13`] module*/
pub type STATUS_13 = crate::Reg<status_13::STATUS_13_SPEC>;
///need_des
pub mod status_13;
/**STATUS_14 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_14`] module*/
pub type STATUS_14 = crate::Reg<status_14::STATUS_14_SPEC>;
///need_des
pub mod status_14;
/**STATUS_15 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_15`] module*/
pub type STATUS_15 = crate::Reg<status_15::STATUS_15_SPEC>;
///need_des
pub mod status_15;
/**STATUS_16 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_16`] module*/
pub type STATUS_16 = crate::Reg<status_16::STATUS_16_SPEC>;
///need_des
pub mod status_16;
/**STATUS_17 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_17`] module*/
pub type STATUS_17 = crate::Reg<status_17::STATUS_17_SPEC>;
///need_des
pub mod status_17;
/**CHN_TMP_STATUS (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`chn_tmp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chn_tmp_status`] module*/
pub type CHN_TMP_STATUS = crate::Reg<chn_tmp_status::CHN_TMP_STATUS_SPEC>;
///need_des
pub mod chn_tmp_status;
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
