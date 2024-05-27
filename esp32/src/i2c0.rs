#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    scl_low_period: SCL_LOW_PERIOD,
    ctr: CTR,
    sr: SR,
    to: TO,
    slave_addr: SLAVE_ADDR,
    rxfifo_st: RXFIFO_ST,
    fifo_conf: FIFO_CONF,
    data: DATA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_ena: INT_ENA,
    int_st: INT_ST,
    sda_hold: SDA_HOLD,
    sda_sample: SDA_SAMPLE,
    scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 0x04],
    scl_start_hold: SCL_START_HOLD,
    scl_rstart_setup: SCL_RSTART_SETUP,
    scl_stop_hold: SCL_STOP_HOLD,
    scl_stop_setup: SCL_STOP_SETUP,
    scl_filter_cfg: SCL_FILTER_CFG,
    sda_filter_cfg: SDA_FILTER_CFG,
    comd: [COMD; 16],
    _reserved22: [u8; 0x60],
    date: DATE,
    _reserved23: [u8; 0x04],
    fifo_start_addr: FIFO_START_ADDR,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn scl_low_period(&self) -> &SCL_LOW_PERIOD {
        &self.scl_low_period
    }
    ///0x04 -
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    ///0x08 -
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c -
    #[inline(always)]
    pub const fn to(&self) -> &TO {
        &self.to
    }
    ///0x10 -
    #[inline(always)]
    pub const fn slave_addr(&self) -> &SLAVE_ADDR {
        &self.slave_addr
    }
    ///0x14 -
    #[inline(always)]
    pub const fn rxfifo_st(&self) -> &RXFIFO_ST {
        &self.rxfifo_st
    }
    ///0x18 -
    #[inline(always)]
    pub const fn fifo_conf(&self) -> &FIFO_CONF {
        &self.fifo_conf
    }
    ///0x1c -
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    ///0x20 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x24 -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x28 -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x2c -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x30 -
    #[inline(always)]
    pub const fn sda_hold(&self) -> &SDA_HOLD {
        &self.sda_hold
    }
    ///0x34 -
    #[inline(always)]
    pub const fn sda_sample(&self) -> &SDA_SAMPLE {
        &self.sda_sample
    }
    ///0x38 -
    #[inline(always)]
    pub const fn scl_high_period(&self) -> &SCL_HIGH_PERIOD {
        &self.scl_high_period
    }
    ///0x40 -
    #[inline(always)]
    pub const fn scl_start_hold(&self) -> &SCL_START_HOLD {
        &self.scl_start_hold
    }
    ///0x44 -
    #[inline(always)]
    pub const fn scl_rstart_setup(&self) -> &SCL_RSTART_SETUP {
        &self.scl_rstart_setup
    }
    ///0x48 -
    #[inline(always)]
    pub const fn scl_stop_hold(&self) -> &SCL_STOP_HOLD {
        &self.scl_stop_hold
    }
    ///0x4c -
    #[inline(always)]
    pub const fn scl_stop_setup(&self) -> &SCL_STOP_SETUP {
        &self.scl_stop_setup
    }
    ///0x50 -
    #[inline(always)]
    pub const fn scl_filter_cfg(&self) -> &SCL_FILTER_CFG {
        &self.scl_filter_cfg
    }
    ///0x54 -
    #[inline(always)]
    pub const fn sda_filter_cfg(&self) -> &SDA_FILTER_CFG {
        &self.sda_filter_cfg
    }
    ///0x58..0x98 -
    #[inline(always)]
    pub const fn comd(&self, n: usize) -> &COMD {
        &self.comd[n]
    }
    ///Iterator for array of:
    ///0x58..0x98 -
    #[inline(always)]
    pub fn comd_iter(&self) -> impl Iterator<Item = &COMD> {
        self.comd.iter()
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x100 -
    #[inline(always)]
    pub const fn fifo_start_addr(&self) -> &FIFO_START_ADDR {
        &self.fifo_start_addr
    }
}
/**SCL_LOW_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_low_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_low_period`] module*/
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
///
pub mod scl_low_period;
/**CTR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctr`] module*/
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
///
pub mod ctr;
/**SR (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SR_SPEC>;
///
pub mod sr;
/**TO (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@to`] module*/
pub type TO = crate::Reg<to::TO_SPEC>;
///
pub mod to;
/**SLAVE_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave_addr`] module*/
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
///
pub mod slave_addr;
/**RXFIFO_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rxfifo_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxfifo_st`] module*/
pub type RXFIFO_ST = crate::Reg<rxfifo_st::RXFIFO_ST_SPEC>;
///
pub mod rxfifo_st;
/**FIFO_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_conf`] module*/
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
///
pub mod fifo_conf;
/**DATA (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type DATA = crate::Reg<data::DATA_SPEC>;
///
pub mod data;
/**INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**SDA_HOLD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_hold`] module*/
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
///
pub mod sda_hold;
/**SDA_SAMPLE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sda_sample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_sample`] module*/
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
///
pub mod sda_sample;
/**SCL_HIGH_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_high_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_high_period`] module*/
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
///
pub mod scl_high_period;
/**SCL_START_HOLD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_start_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_start_hold`] module*/
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
///
pub mod scl_start_hold;
/**SCL_RSTART_SETUP (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_rstart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_rstart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_rstart_setup`] module*/
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
///
pub mod scl_rstart_setup;
/**SCL_STOP_HOLD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stop_hold`] module*/
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
///
pub mod scl_stop_hold;
/**SCL_STOP_SETUP (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stop_setup`] module*/
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
///
pub mod scl_stop_setup;
/**SCL_FILTER_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_filter_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_filter_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_filter_cfg`] module*/
pub type SCL_FILTER_CFG = crate::Reg<scl_filter_cfg::SCL_FILTER_CFG_SPEC>;
///
pub mod scl_filter_cfg;
/**SDA_FILTER_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sda_filter_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_filter_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_filter_cfg`] module*/
pub type SDA_FILTER_CFG = crate::Reg<sda_filter_cfg::SDA_FILTER_CFG_SPEC>;
///
pub mod sda_filter_cfg;
/**COMD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`comd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comd`] module*/
pub type COMD = crate::Reg<comd::COMD_SPEC>;
///
pub mod comd;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
/**FIFO_START_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_start_addr`] module*/
pub type FIFO_START_ADDR = crate::Reg<fifo_start_addr::FIFO_START_ADDR_SPEC>;
///
pub mod fifo_start_addr;
