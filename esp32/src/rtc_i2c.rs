#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    scl_low_period: SCL_LOW_PERIOD,
    ctrl: CTRL,
    debug_status: DEBUG_STATUS,
    timeout: TIMEOUT,
    slave_addr: SLAVE_ADDR,
    _reserved5: [u8; 0x08],
    data: DATA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_ena: INT_ENA,
    int_st: INT_ST,
    sda_duty: SDA_DUTY,
    _reserved11: [u8; 0x04],
    scl_high_period: SCL_HIGH_PERIOD,
    _reserved12: [u8; 0x04],
    scl_start_period: SCL_START_PERIOD,
    scl_stop_period: SCL_STOP_PERIOD,
    cmd: CMD,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn scl_low_period(&self) -> &SCL_LOW_PERIOD {
        &self.scl_low_period
    }
    ///0x04 -
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x08 -
    #[inline(always)]
    pub const fn debug_status(&self) -> &DEBUG_STATUS {
        &self.debug_status
    }
    ///0x0c -
    #[inline(always)]
    pub const fn timeout(&self) -> &TIMEOUT {
        &self.timeout
    }
    ///0x10 -
    #[inline(always)]
    pub const fn slave_addr(&self) -> &SLAVE_ADDR {
        &self.slave_addr
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
    pub const fn sda_duty(&self) -> &SDA_DUTY {
        &self.sda_duty
    }
    ///0x38 -
    #[inline(always)]
    pub const fn scl_high_period(&self) -> &SCL_HIGH_PERIOD {
        &self.scl_high_period
    }
    ///0x40 -
    #[inline(always)]
    pub const fn scl_start_period(&self) -> &SCL_START_PERIOD {
        &self.scl_start_period
    }
    ///0x44 -
    #[inline(always)]
    pub const fn scl_stop_period(&self) -> &SCL_STOP_PERIOD {
        &self.scl_stop_period
    }
    ///0x48 -
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
}
/**SCL_LOW_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_low_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_low_period`] module*/
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
///
pub mod scl_low_period;
/**CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///
pub mod ctrl;
/**DEBUG_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`debug_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@debug_status`] module*/
pub type DEBUG_STATUS = crate::Reg<debug_status::DEBUG_STATUS_SPEC>;
///
pub mod debug_status;
/**TIMEOUT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timeout`] module*/
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
///
pub mod timeout;
/**SLAVE_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave_addr`] module*/
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
///
pub mod slave_addr;
/**DATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type DATA = crate::Reg<data::DATA_SPEC>;
///
pub mod data;
/**INT_RAW (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_CLR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

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
/**INT_ST (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**SDA_DUTY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_duty`] module*/
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
///
pub mod sda_duty;
/**SCL_HIGH_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_high_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_high_period`] module*/
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
///
pub mod scl_high_period;
/**SCL_START_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_start_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_start_period`] module*/
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
///
pub mod scl_start_period;
/**SCL_STOP_PERIOD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stop_period`] module*/
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
///
pub mod scl_stop_period;
/**CMD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///
pub mod cmd;
