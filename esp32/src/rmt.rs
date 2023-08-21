#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - "]
    pub chdata: [CHDATA; 8],
    #[doc = "0x20 - "]
    pub ch0conf0: CHCONF0,
    #[doc = "0x24 - "]
    pub ch0conf1: CHCONF1,
    #[doc = "0x28 - "]
    pub ch1conf0: CHCONF0,
    #[doc = "0x2c - "]
    pub ch1conf1: CHCONF1,
    #[doc = "0x30 - "]
    pub ch2conf0: CHCONF0,
    #[doc = "0x34 - "]
    pub ch2conf1: CHCONF1,
    #[doc = "0x38 - "]
    pub ch3conf0: CHCONF0,
    #[doc = "0x3c - "]
    pub ch3conf1: CHCONF1,
    #[doc = "0x40 - "]
    pub ch4conf0: CHCONF0,
    #[doc = "0x44 - "]
    pub ch4conf1: CHCONF1,
    #[doc = "0x48 - "]
    pub ch5conf0: CHCONF0,
    #[doc = "0x4c - "]
    pub ch5conf1: CHCONF1,
    #[doc = "0x50 - "]
    pub ch6conf0: CHCONF0,
    #[doc = "0x54 - "]
    pub ch6conf1: CHCONF1,
    #[doc = "0x58 - "]
    pub ch7conf0: CHCONF0,
    #[doc = "0x5c - "]
    pub ch7conf1: CHCONF1,
    #[doc = "0x60..0x80 - "]
    pub chstatus: [CHSTATUS; 8],
    #[doc = "0x80..0xa0 - "]
    pub chaddr: [CHADDR; 8],
    #[doc = "0xa0 - "]
    pub int_raw: INT_RAW,
    #[doc = "0xa4 - "]
    pub int_st: INT_ST,
    #[doc = "0xa8 - "]
    pub int_ena: INT_ENA,
    #[doc = "0xac - "]
    pub int_clr: INT_CLR,
    #[doc = "0xb0..0xd0 - "]
    pub chcarrier_duty: [CHCARRIER_DUTY; 8],
    #[doc = "0xd0..0xf0 - "]
    pub ch_tx_lim: [CH_TX_LIM; 8],
    #[doc = "0xf0 - "]
    pub apb_conf: APB_CONF,
    _reserved26: [u8; 0x08],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CHDATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chdata`] module"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = ""]
pub mod chdata;
#[doc = "CHCONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chconf0`] module"]
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
#[doc = ""]
pub mod chconf0;
#[doc = "CHCONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chconf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chconf1`] module"]
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
#[doc = ""]
pub mod chconf1;
#[doc = "CHSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chstatus`] module"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = ""]
pub mod chstatus;
#[doc = "CHADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chaddr`] module"]
pub type CHADDR = crate::Reg<chaddr::CHADDR_SPEC>;
#[doc = ""]
pub mod chaddr;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chcarrier_duty`] module"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod chcarrier_duty;
#[doc = "CH_TX_LIM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_lim`] module"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = ""]
pub mod ch_tx_lim;
#[doc = "APB_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_conf`] module"]
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
#[doc = ""]
pub mod apb_conf;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
