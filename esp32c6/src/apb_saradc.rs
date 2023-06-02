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
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "digital saradc configure register"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: an alias for `Reg<FILTER_CTRL1_SPEC>`"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "digital saradc configure register"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: an alias for `Reg<FSM_WAIT_SPEC>`"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "digital saradc configure register"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: an alias for `Reg<SAR1_STATUS_SPEC>`"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: an alias for `Reg<SAR2_STATUS_SPEC>`"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar2_status;
#[doc = "SAR_PATT_TAB1 (rw) register accessor: an alias for `Reg<SAR_PATT_TAB1_SPEC>`"]
pub type SAR_PATT_TAB1 = crate::Reg<sar_patt_tab1::SAR_PATT_TAB1_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar_patt_tab1;
#[doc = "SAR_PATT_TAB2 (rw) register accessor: an alias for `Reg<SAR_PATT_TAB2_SPEC>`"]
pub type SAR_PATT_TAB2 = crate::Reg<sar_patt_tab2::SAR_PATT_TAB2_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar_patt_tab2;
#[doc = "ONETIME_SAMPLE (rw) register accessor: an alias for `Reg<ONETIME_SAMPLE_SPEC>`"]
pub type ONETIME_SAMPLE = crate::Reg<onetime_sample::ONETIME_SAMPLE_SPEC>;
#[doc = "digital saradc configure register"]
pub mod onetime_sample;
#[doc = "ARB_CTRL (rw) register accessor: an alias for `Reg<ARB_CTRL_SPEC>`"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: an alias for `Reg<FILTER_CTRL0_SPEC>`"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "digital saradc configure register"]
pub mod filter_ctrl0;
#[doc = "SAR1DATA_STATUS (r) register accessor: an alias for `Reg<SAR1DATA_STATUS_SPEC>`"]
pub type SAR1DATA_STATUS = crate::Reg<sar1data_status::SAR1DATA_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar1data_status;
#[doc = "SAR2DATA_STATUS (r) register accessor: an alias for `Reg<SAR2DATA_STATUS_SPEC>`"]
pub type SAR2DATA_STATUS = crate::Reg<sar2data_status::SAR2DATA_STATUS_SPEC>;
#[doc = "digital saradc configure register"]
pub mod sar2data_status;
#[doc = "THRES0_CTRL (rw) register accessor: an alias for `Reg<THRES0_CTRL_SPEC>`"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: an alias for `Reg<THRES1_CTRL_SPEC>`"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: an alias for `Reg<THRES_CTRL_SPEC>`"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "digital saradc configure register"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "digital saradc int register"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "digital saradc configure register"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: an alias for `Reg<CLKM_CONF_SPEC>`"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "digital saradc configure register"]
pub mod clkm_conf;
#[doc = "APB_TSENS_CTRL (rw) register accessor: an alias for `Reg<APB_TSENS_CTRL_SPEC>`"]
pub type APB_TSENS_CTRL = crate::Reg<apb_tsens_ctrl::APB_TSENS_CTRL_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_ctrl;
#[doc = "TSENS_CTRL2 (rw) register accessor: an alias for `Reg<TSENS_CTRL2_SPEC>`"]
pub type TSENS_CTRL2 = crate::Reg<tsens_ctrl2::TSENS_CTRL2_SPEC>;
#[doc = "digital tsens configure register"]
pub mod tsens_ctrl2;
#[doc = "CALI (rw) register accessor: an alias for `Reg<CALI_SPEC>`"]
pub type CALI = crate::Reg<cali::CALI_SPEC>;
#[doc = "digital saradc configure register"]
pub mod cali;
#[doc = "APB_TSENS_WAKE (rw) register accessor: an alias for `Reg<APB_TSENS_WAKE_SPEC>`"]
pub type APB_TSENS_WAKE = crate::Reg<apb_tsens_wake::APB_TSENS_WAKE_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_wake;
#[doc = "APB_TSENS_SAMPLE (rw) register accessor: an alias for `Reg<APB_TSENS_SAMPLE_SPEC>`"]
pub type APB_TSENS_SAMPLE = crate::Reg<apb_tsens_sample::APB_TSENS_SAMPLE_SPEC>;
#[doc = "digital tsens configure register"]
pub mod apb_tsens_sample;
#[doc = "CTRL_DATE (rw) register accessor: an alias for `Reg<CTRL_DATE_SPEC>`"]
pub type CTRL_DATE = crate::Reg<ctrl_date::CTRL_DATE_SPEC>;
#[doc = "version"]
pub mod ctrl_date;
