#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    tar0_low: TAR0_LOW,
    tar0_high: TAR0_HIGH,
    _reserved2: [u8; 0x08],
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
    _reserved12: [u8; 0x03c4],
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
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
