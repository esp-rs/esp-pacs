#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - configure apb saradc controller"]
    pub ctrl: CTRL,
    #[doc = "0x04 - configure apb saradc controller"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - configure saradc filter"]
    pub filter_ctrl1: FILTER_CTRL1,
    #[doc = "0x0c - configure apb saradc fsm"]
    pub fsm_wait: FSM_WAIT,
    #[doc = "0x10 - saradc1 status for debug"]
    pub sar1_status: SAR1_STATUS,
    #[doc = "0x14 - saradc2 status for debug"]
    pub sar2_status: SAR2_STATUS,
    #[doc = "0x18 - configure apb saradc pattern table"]
    pub sar1_patt_tab1: SAR1_PATT_TAB1,
    #[doc = "0x1c - configure apb saradc pattern table"]
    pub sar1_patt_tab2: SAR1_PATT_TAB2,
    #[doc = "0x20 - configure apb saradc pattern table"]
    pub sar1_patt_tab3: SAR1_PATT_TAB3,
    #[doc = "0x24 - configure apb saradc pattern table"]
    pub sar1_patt_tab4: SAR1_PATT_TAB4,
    #[doc = "0x28 - configure apb saradc pattern table"]
    pub sar2_patt_tab1: SAR2_PATT_TAB1,
    #[doc = "0x2c - configure apb saradc pattern table"]
    pub sar2_patt_tab2: SAR2_PATT_TAB2,
    #[doc = "0x30 - configure apb saradc pattern table"]
    pub sar2_patt_tab3: SAR2_PATT_TAB3,
    #[doc = "0x34 - configure apb saradc pattern table"]
    pub sar2_patt_tab4: SAR2_PATT_TAB4,
    #[doc = "0x38 - configure apb saradc arbit"]
    pub arb_ctrl: ARB_CTRL,
    #[doc = "0x3c - configure apb saradc arbit"]
    pub filter_ctrl0: FILTER_CTRL0,
    #[doc = "0x40 - get apb saradc sample data"]
    pub apb_saradc1_data_status: APB_SARADC1_DATA_STATUS,
    #[doc = "0x44 - configure apb saradc thres monitor"]
    pub thres0_ctrl: THRES0_CTRL,
    #[doc = "0x48 - configure apb saradc thres monitor"]
    pub thres1_ctrl: THRES1_CTRL,
    _reserved19: [u8; 0x0c],
    #[doc = "0x58 - configure thres monitor enable"]
    pub thres_ctrl: THRES_CTRL,
    #[doc = "0x5c - enable interrupt"]
    pub int_ena: INT_ENA,
    #[doc = "0x60 - raw of interrupt"]
    pub int_raw: INT_RAW,
    #[doc = "0x64 - state of interrupt"]
    pub int_st: INT_ST,
    #[doc = "0x68 - clear interrupt"]
    pub int_clr: INT_CLR,
    #[doc = "0x6c - configure apb saradc dma"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x70 - configure apb saradc clock"]
    pub clkm_conf: CLKM_CONF,
    _reserved26: [u8; 0x04],
    #[doc = "0x78 - get apb saradc2 sample data"]
    pub apb_saradc2_data_status: APB_SARADC2_DATA_STATUS,
    _reserved27: [u8; 0x0380],
    #[doc = "0x3fc - version"]
    pub apb_ctrl_date: APB_CTRL_DATE,
}
#[doc = "CTRL (rw) register accessor: configure apb saradc controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "configure apb saradc controller"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: configure apb saradc controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "configure apb saradc controller"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: configure saradc filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`filter_ctrl1`] module"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "configure saradc filter"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: configure apb saradc fsm\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_wait::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm_wait`] module"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "configure apb saradc fsm"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: saradc1 status for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_status`] module"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "saradc1 status for debug"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: saradc2 status for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_status`] module"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "saradc2 status for debug"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_patt_tab1`] module"]
pub type SAR1_PATT_TAB1 = crate::Reg<sar1_patt_tab1::SAR1_PATT_TAB1_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_patt_tab2`] module"]
pub type SAR1_PATT_TAB2 = crate::Reg<sar1_patt_tab2::SAR1_PATT_TAB2_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_patt_tab3`] module"]
pub type SAR1_PATT_TAB3 = crate::Reg<sar1_patt_tab3::SAR1_PATT_TAB3_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar1_patt_tab4`] module"]
pub type SAR1_PATT_TAB4 = crate::Reg<sar1_patt_tab4::SAR1_PATT_TAB4_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_patt_tab1`] module"]
pub type SAR2_PATT_TAB1 = crate::Reg<sar2_patt_tab1::SAR2_PATT_TAB1_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_patt_tab2`] module"]
pub type SAR2_PATT_TAB2 = crate::Reg<sar2_patt_tab2::SAR2_PATT_TAB2_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_patt_tab3`] module"]
pub type SAR2_PATT_TAB3 = crate::Reg<sar2_patt_tab3::SAR2_PATT_TAB3_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar2_patt_tab4`] module"]
pub type SAR2_PATT_TAB4 = crate::Reg<sar2_patt_tab4::SAR2_PATT_TAB4_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: configure apb saradc arbit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`arb_ctrl`] module"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "configure apb saradc arbit"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: configure apb saradc arbit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`filter_ctrl0`] module"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "configure apb saradc arbit"]
pub mod filter_ctrl0;
#[doc = "APB_SARADC1_DATA_STATUS (r) register accessor: get apb saradc sample data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc1_data_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_saradc1_data_status`] module"]
pub type APB_SARADC1_DATA_STATUS =
    crate::Reg<apb_saradc1_data_status::APB_SARADC1_DATA_STATUS_SPEC>;
#[doc = "get apb saradc sample data"]
pub mod apb_saradc1_data_status;
#[doc = "THRES0_CTRL (rw) register accessor: configure apb saradc thres monitor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres0_ctrl`] module"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "configure apb saradc thres monitor"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: configure apb saradc thres monitor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres1_ctrl`] module"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "configure apb saradc thres monitor"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: configure thres monitor enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres_ctrl`] module"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "configure thres monitor enable"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "enable interrupt"]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: raw of interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "raw of interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: state of interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "state of interrupt"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: clear interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "clear interrupt"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: configure apb saradc dma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "configure apb saradc dma"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: configure apb saradc clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkm_conf`] module"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "configure apb saradc clock"]
pub mod clkm_conf;
#[doc = "APB_SARADC2_DATA_STATUS (r) register accessor: get apb saradc2 sample data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc2_data_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_saradc2_data_status`] module"]
pub type APB_SARADC2_DATA_STATUS =
    crate::Reg<apb_saradc2_data_status::APB_SARADC2_DATA_STATUS_SPEC>;
#[doc = "get apb saradc2 sample data"]
pub mod apb_saradc2_data_status;
#[doc = "APB_CTRL_DATE (rw) register accessor: version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_ctrl_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_ctrl_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_ctrl_date`] module"]
pub type APB_CTRL_DATE = crate::Reg<apb_ctrl_date::APB_CTRL_DATE_SPEC>;
#[doc = "version"]
pub mod apb_ctrl_date;
