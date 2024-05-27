#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    tar0_low: TAR0_LOW,
    tar0_high: TAR0_HIGH,
    tar1_low: TAR1_LOW,
    tar1_high: TAR1_HIGH,
    update: UPDATE,
    main_buf0_low: MAIN_BUF0_LOW,
    main_buf0_high: MAIN_BUF0_HIGH,
    main_buf1_low: MAIN_BUF1_LOW,
    main_buf1_high: MAIN_BUF1_HIGH,
    main_overflow: MAIN_OVERFLOW,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    lp_int_raw: LP_INT_RAW,
    lp_int_st: LP_INT_ST,
    lp_int_ena: LP_INT_ENA,
    lp_int_clr: LP_INT_CLR,
    _reserved18: [u8; 0x03b4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need_des
    #[inline(always)]
    pub const fn tar0_low(&self) -> &TAR0_LOW {
        &self.tar0_low
    }
    ///0x04 - need_des
    #[inline(always)]
    pub const fn tar0_high(&self) -> &TAR0_HIGH {
        &self.tar0_high
    }
    ///0x08 - need_des
    #[inline(always)]
    pub const fn tar1_low(&self) -> &TAR1_LOW {
        &self.tar1_low
    }
    ///0x0c - need_des
    #[inline(always)]
    pub const fn tar1_high(&self) -> &TAR1_HIGH {
        &self.tar1_high
    }
    ///0x10 - need_des
    #[inline(always)]
    pub const fn update(&self) -> &UPDATE {
        &self.update
    }
    ///0x14 - need_des
    #[inline(always)]
    pub const fn main_buf0_low(&self) -> &MAIN_BUF0_LOW {
        &self.main_buf0_low
    }
    ///0x18 - need_des
    #[inline(always)]
    pub const fn main_buf0_high(&self) -> &MAIN_BUF0_HIGH {
        &self.main_buf0_high
    }
    ///0x1c - need_des
    #[inline(always)]
    pub const fn main_buf1_low(&self) -> &MAIN_BUF1_LOW {
        &self.main_buf1_low
    }
    ///0x20 - need_des
    #[inline(always)]
    pub const fn main_buf1_high(&self) -> &MAIN_BUF1_HIGH {
        &self.main_buf1_high
    }
    ///0x24 - need_des
    #[inline(always)]
    pub const fn main_overflow(&self) -> &MAIN_OVERFLOW {
        &self.main_overflow
    }
    ///0x28 - need_des
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x2c - need_des
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x30 - need_des
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x34 - need_des
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x38 - need_des
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LP_INT_RAW {
        &self.lp_int_raw
    }
    ///0x3c - need_des
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LP_INT_ST {
        &self.lp_int_st
    }
    ///0x40 - need_des
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LP_INT_ENA {
        &self.lp_int_ena
    }
    ///0x44 - need_des
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LP_INT_CLR {
        &self.lp_int_clr
    }
    ///0x3fc - need_des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**TAR0_LOW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`tar0_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar0_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tar0_low`] module*/
pub type TAR0_LOW = crate::Reg<tar0_low::TAR0_LOW_SPEC>;
///need_des
pub mod tar0_low;
/**TAR0_HIGH (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`tar0_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar0_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tar0_high`] module*/
pub type TAR0_HIGH = crate::Reg<tar0_high::TAR0_HIGH_SPEC>;
///need_des
pub mod tar0_high;
/**TAR1_LOW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`tar1_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar1_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tar1_low`] module*/
pub type TAR1_LOW = crate::Reg<tar1_low::TAR1_LOW_SPEC>;
///need_des
pub mod tar1_low;
/**TAR1_HIGH (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`tar1_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar1_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tar1_high`] module*/
pub type TAR1_HIGH = crate::Reg<tar1_high::TAR1_HIGH_SPEC>;
///need_des
pub mod tar1_high;
/**UPDATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@update`] module*/
pub type UPDATE = crate::Reg<update::UPDATE_SPEC>;
///need_des
pub mod update;
/**MAIN_BUF0_LOW (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_buf0_low::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_buf0_low`] module*/
pub type MAIN_BUF0_LOW = crate::Reg<main_buf0_low::MAIN_BUF0_LOW_SPEC>;
///need_des
pub mod main_buf0_low;
/**MAIN_BUF0_HIGH (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_buf0_high::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_buf0_high`] module*/
pub type MAIN_BUF0_HIGH = crate::Reg<main_buf0_high::MAIN_BUF0_HIGH_SPEC>;
///need_des
pub mod main_buf0_high;
/**MAIN_BUF1_LOW (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_buf1_low::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_buf1_low`] module*/
pub type MAIN_BUF1_LOW = crate::Reg<main_buf1_low::MAIN_BUF1_LOW_SPEC>;
///need_des
pub mod main_buf1_low;
/**MAIN_BUF1_HIGH (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_buf1_high::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_buf1_high`] module*/
pub type MAIN_BUF1_HIGH = crate::Reg<main_buf1_high::MAIN_BUF1_HIGH_SPEC>;
///need_des
pub mod main_buf1_high;
/**MAIN_OVERFLOW (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_overflow::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_overflow`] module*/
pub type MAIN_OVERFLOW = crate::Reg<main_overflow::MAIN_OVERFLOW_SPEC>;
///need_des
pub mod main_overflow;
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
/**LP_INT_RAW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_raw`] module*/
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
///need_des
pub mod lp_int_raw;
/**LP_INT_ST (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_st`] module*/
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
///need_des
pub mod lp_int_st;
/**LP_INT_ENA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_ena`] module*/
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
///need_des
pub mod lp_int_ena;
/**LP_INT_CLR (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_clr`] module*/
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
///need_des
pub mod lp_int_clr;
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
