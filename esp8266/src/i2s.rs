#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    i2stxfifo: I2STXFIFO,
    i2srxfifo: I2SRXFIFO,
    i2sconf: I2SCONF,
    i2sint_raw: I2SINT_RAW,
    i2sint_st: I2SINT_ST,
    i2sint_ena: I2SINT_ENA,
    i2sint_clr: I2SINT_CLR,
    i2stiming: I2STIMING,
    i2s_fifo_conf: I2S_FIFO_CONF,
    i2srxeof_num: I2SRXEOF_NUM,
    i2sconf_sigle_data: I2SCONF_SIGLE_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - I2STXFIFO"]
    #[inline(always)]
    pub const fn i2stxfifo(&self) -> &I2STXFIFO {
        &self.i2stxfifo
    }
    #[doc = "0x04 - I2SRXFIFO"]
    #[inline(always)]
    pub const fn i2srxfifo(&self) -> &I2SRXFIFO {
        &self.i2srxfifo
    }
    #[doc = "0x08 - I2SCONF"]
    #[inline(always)]
    pub const fn i2sconf(&self) -> &I2SCONF {
        &self.i2sconf
    }
    #[doc = "0x0c - I2SINT_RAW"]
    #[inline(always)]
    pub const fn i2sint_raw(&self) -> &I2SINT_RAW {
        &self.i2sint_raw
    }
    #[doc = "0x10 - I2SINT_ST"]
    #[inline(always)]
    pub const fn i2sint_st(&self) -> &I2SINT_ST {
        &self.i2sint_st
    }
    #[doc = "0x14 - I2SINT_ENA"]
    #[inline(always)]
    pub const fn i2sint_ena(&self) -> &I2SINT_ENA {
        &self.i2sint_ena
    }
    #[doc = "0x18 - I2SINT_CLR"]
    #[inline(always)]
    pub const fn i2sint_clr(&self) -> &I2SINT_CLR {
        &self.i2sint_clr
    }
    #[doc = "0x1c - I2STIMING"]
    #[inline(always)]
    pub const fn i2stiming(&self) -> &I2STIMING {
        &self.i2stiming
    }
    #[doc = "0x20 - I2S_FIFO_CONF"]
    #[inline(always)]
    pub const fn i2s_fifo_conf(&self) -> &I2S_FIFO_CONF {
        &self.i2s_fifo_conf
    }
    #[doc = "0x24 - I2SRXEOF_NUM"]
    #[inline(always)]
    pub const fn i2srxeof_num(&self) -> &I2SRXEOF_NUM {
        &self.i2srxeof_num
    }
    #[doc = "0x28 - I2SCONF_SIGLE_DATA"]
    #[inline(always)]
    pub const fn i2sconf_sigle_data(&self) -> &I2SCONF_SIGLE_DATA {
        &self.i2sconf_sigle_data
    }
}
#[doc = "I2STXFIFO (rw) register accessor: I2STXFIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2stxfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2stxfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2stxfifo`] module"]
pub type I2STXFIFO = crate::Reg<i2stxfifo::I2STXFIFO_SPEC>;
#[doc = "I2STXFIFO"]
pub mod i2stxfifo;
#[doc = "I2SRXFIFO (rw) register accessor: I2SRXFIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2srxfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2srxfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2srxfifo`] module"]
pub type I2SRXFIFO = crate::Reg<i2srxfifo::I2SRXFIFO_SPEC>;
#[doc = "I2SRXFIFO"]
pub mod i2srxfifo;
#[doc = "I2SCONF (rw) register accessor: I2SCONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sconf`] module"]
pub type I2SCONF = crate::Reg<i2sconf::I2SCONF_SPEC>;
#[doc = "I2SCONF"]
pub mod i2sconf;
#[doc = "I2SINT_RAW (rw) register accessor: I2SINT_RAW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sint_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sint_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sint_raw`] module"]
pub type I2SINT_RAW = crate::Reg<i2sint_raw::I2SINT_RAW_SPEC>;
#[doc = "I2SINT_RAW"]
pub mod i2sint_raw;
#[doc = "I2SINT_ST (rw) register accessor: I2SINT_ST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sint_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sint_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sint_st`] module"]
pub type I2SINT_ST = crate::Reg<i2sint_st::I2SINT_ST_SPEC>;
#[doc = "I2SINT_ST"]
pub mod i2sint_st;
#[doc = "I2SINT_ENA (rw) register accessor: I2SINT_ENA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sint_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sint_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sint_ena`] module"]
pub type I2SINT_ENA = crate::Reg<i2sint_ena::I2SINT_ENA_SPEC>;
#[doc = "I2SINT_ENA"]
pub mod i2sint_ena;
#[doc = "I2SINT_CLR (rw) register accessor: I2SINT_CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sint_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sint_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sint_clr`] module"]
pub type I2SINT_CLR = crate::Reg<i2sint_clr::I2SINT_CLR_SPEC>;
#[doc = "I2SINT_CLR"]
pub mod i2sint_clr;
#[doc = "I2STIMING (rw) register accessor: I2STIMING\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2stiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2stiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2stiming`] module"]
pub type I2STIMING = crate::Reg<i2stiming::I2STIMING_SPEC>;
#[doc = "I2STIMING"]
pub mod i2stiming;
#[doc = "I2S_FIFO_CONF (rw) register accessor: I2S_FIFO_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_fifo_conf`] module"]
pub type I2S_FIFO_CONF = crate::Reg<i2s_fifo_conf::I2S_FIFO_CONF_SPEC>;
#[doc = "I2S_FIFO_CONF"]
pub mod i2s_fifo_conf;
#[doc = "I2SRXEOF_NUM (rw) register accessor: I2SRXEOF_NUM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2srxeof_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2srxeof_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2srxeof_num`] module"]
pub type I2SRXEOF_NUM = crate::Reg<i2srxeof_num::I2SRXEOF_NUM_SPEC>;
#[doc = "I2SRXEOF_NUM"]
pub mod i2srxeof_num;
#[doc = "I2SCONF_SIGLE_DATA (rw) register accessor: I2SCONF_SIGLE_DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sconf_sigle_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sconf_sigle_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sconf_sigle_data`] module"]
pub type I2SCONF_SIGLE_DATA = crate::Reg<i2sconf_sigle_data::I2SCONF_SIGLE_DATA_SPEC>;
#[doc = "I2SCONF_SIGLE_DATA"]
pub mod i2sconf_sigle_data;
