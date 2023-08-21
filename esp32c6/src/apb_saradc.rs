#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - digital saradc configure register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - digital saradc configure register"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - digital saradc configure register"]
    pub filter_ctrl1: FILTER_CTRL1,
    #[doc = "0x0c - digital saradc configure register"]
    pub fsm_wait: FSM_WAIT,
    #[doc = "0x10 - digital saradc configure register"]
    pub sar1_status: SAR1_STATUS,
    #[doc = "0x14 - digital saradc configure register"]
    pub sar2_status: SAR2_STATUS,
    #[doc = "0x18 - digital saradc configure register"]
    pub sar_patt_tab1: SAR_PATT_TAB1,
    #[doc = "0x1c - digital saradc configure register"]
    pub sar_patt_tab2: SAR_PATT_TAB2,
    #[doc = "0x20 - digital saradc configure register"]
    pub onetime_sample: ONETIME_SAMPLE,
    #[doc = "0x24 - digital saradc configure register"]
    pub arb_ctrl: ARB_CTRL,
    #[doc = "0x28 - digital saradc configure register"]
    pub filter_ctrl0: FILTER_CTRL0,
    #[doc = "0x2c - digital saradc configure register"]
    pub sar1data_status: SAR1DATA_STATUS,
    #[doc = "0x30 - digital saradc configure register"]
    pub sar2data_status: SAR2DATA_STATUS,
    #[doc = "0x34 - digital saradc configure register"]
    pub thres0_ctrl: THRES0_CTRL,
    #[doc = "0x38 - digital saradc configure register"]
    pub thres1_ctrl: THRES1_CTRL,
    #[doc = "0x3c - digital saradc configure register"]
    pub thres_ctrl: THRES_CTRL,
    #[doc = "0x40 - digital saradc int register"]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - digital saradc int register"]
    pub int_raw: INT_RAW,
    #[doc = "0x48 - digital saradc int register"]
    pub int_st: INT_ST,
    #[doc = "0x4c - digital saradc int register"]
    pub int_clr: INT_CLR,
    #[doc = "0x50 - digital saradc configure register"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x54 - digital saradc configure register"]
    pub clkm_conf: CLKM_CONF,
    #[doc = "0x58 - digital tsens configure register"]
    pub apb_tsens_ctrl: APB_TSENS_CTRL,
    #[doc = "0x5c - digital tsens configure register"]
    pub tsens_ctrl2: TSENS_CTRL2,
    #[doc = "0x60 - digital saradc configure register"]
    pub cali: CALI,
    #[doc = "0x64 - digital tsens configure register"]
    pub apb_tsens_wake: APB_TSENS_WAKE,
    #[doc = "0x68 - digital tsens configure register"]
    pub apb_tsens_sample: APB_TSENS_SAMPLE,
    _reserved27: [u8; 0x0390],
    #[doc = "0x3fc - version"]
    pub ctrl_date: CTRL_DATE,
}
#[doc = "CTRL (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "digital saradc configure register"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`filter_ctrl1`] module"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "digital saradc configure register"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_wait::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm_wait`] module"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "digital saradc configure register"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_status`] module"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_status`] module"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar2_status;
#[doc = "SAR_PATT_TAB1 (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_patt_tab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_patt_tab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_patt_tab1`] module"]
pub type SAR_PATT_TAB1 = crate::Reg<sar_patt_tab1::SAR_PATT_TAB1_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar_patt_tab1;
#[doc = "SAR_PATT_TAB2 (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_patt_tab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_patt_tab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_patt_tab2`] module"]
pub type SAR_PATT_TAB2 = crate::Reg<sar_patt_tab2::SAR_PATT_TAB2_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar_patt_tab2;
#[doc = "ONETIME_SAMPLE (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`onetime_sample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`onetime_sample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`onetime_sample`] module"]
pub type ONETIME_SAMPLE = crate::Reg<onetime_sample::ONETIME_SAMPLE_SPEC>;
#[doc = "digital saradc configure register"]
pub mod onetime_sample;
#[doc = "ARB_CTRL (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`arb_ctrl`] module"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`filter_ctrl0`] module"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "digital saradc configure register"]
pub mod filter_ctrl0;
#[doc = "SAR1DATA_STATUS (r) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1data_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1data_status`] module"]
pub type SAR1DATA_STATUS = crate::Reg<sar1data_status::SAR1DATA_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar1data_status;
#[doc = "SAR2DATA_STATUS (r) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2data_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2data_status`] module"]
pub type SAR2DATA_STATUS = crate::Reg<sar2data_status::SAR2DATA_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar2data_status;
#[doc = "THRES0_CTRL (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres0_ctrl`] module"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres1_ctrl`] module"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres_ctrl`] module"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: digital saradc int register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "digital saradc configure register"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkm_conf`] module"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "digital saradc configure register"]
pub mod clkm_conf;
#[doc = "APB_TSENS_CTRL (rw) register accessor: digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_tsens_ctrl`] module"]
pub type APB_TSENS_CTRL = crate::Reg<apb_tsens_ctrl::APB_TSENS_CTRL_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_ctrl;
#[doc = "TSENS_CTRL2 (rw) register accessor: digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsens_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsens_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tsens_ctrl2`] module"]
pub type TSENS_CTRL2 = crate::Reg<tsens_ctrl2::TSENS_CTRL2_SPEC>;
#[doc = "digital tsens configure register"]
pub mod tsens_ctrl2;
#[doc = "CALI (rw) register accessor: digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cali`] module"]
pub type CALI = crate::Reg<cali::CALI_SPEC>;
#[doc = "digital saradc configure register"]
pub mod cali;
#[doc = "APB_TSENS_WAKE (rw) register accessor: digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_wake::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_wake::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_tsens_wake`] module"]
pub type APB_TSENS_WAKE = crate::Reg<apb_tsens_wake::APB_TSENS_WAKE_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_wake;
#[doc = "APB_TSENS_SAMPLE (rw) register accessor: digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_sample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_sample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_tsens_sample`] module"]
pub type APB_TSENS_SAMPLE = crate::Reg<apb_tsens_sample::APB_TSENS_SAMPLE_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_sample;
#[doc = "CTRL_DATE (rw) register accessor: version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl_date`] module"]
pub type CTRL_DATE = crate::Reg<ctrl_date::CTRL_DATE_SPEC>;
#[doc = "version"]
pub mod ctrl_date;
