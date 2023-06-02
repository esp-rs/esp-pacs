#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - DIG ADC common configuration"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DIG ADC common configuration"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - digital adc control register"]
    pub fsm: FSM,
    #[doc = "0x0c - configure saradc fsm internal parameter base on test"]
    pub fsm_wait: FSM_WAIT,
    #[doc = "0x10 - digital adc1 status"]
    pub sar1_status: SAR1_STATUS,
    #[doc = "0x14 - digital adc2 status"]
    pub sar2_status: SAR2_STATUS,
    #[doc = "0x18 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    pub sar1_patt_tab1: SAR1_PATT_TAB1,
    #[doc = "0x1c - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    pub sar1_patt_tab2: SAR1_PATT_TAB2,
    #[doc = "0x20 - Item 8 ~ 11 for pattern table 1 (each item one byte)"]
    pub sar1_patt_tab3: SAR1_PATT_TAB3,
    #[doc = "0x24 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    pub sar1_patt_tab4: SAR1_PATT_TAB4,
    #[doc = "0x28 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    pub sar2_patt_tab1: SAR2_PATT_TAB1,
    #[doc = "0x2c - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    pub sar2_patt_tab2: SAR2_PATT_TAB2,
    #[doc = "0x30 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    pub sar2_patt_tab3: SAR2_PATT_TAB3,
    #[doc = "0x34 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    pub sar2_patt_tab4: SAR2_PATT_TAB4,
    #[doc = "0x38 - Configure the settings of DIG ADC2 arbiter"]
    pub arb_ctrl: ARB_CTRL,
    #[doc = "0x3c - Configure the settings of DIG ADC2 filter"]
    pub filter_ctrl: FILTER_CTRL,
    #[doc = "0x40 - Data status of DIG ADC2 filter"]
    pub filter_status: FILTER_STATUS,
    #[doc = "0x44 - Configure monitor threshold for DIG ADC2"]
    pub thres_ctrl: THRES_CTRL,
    #[doc = "0x48 - Enable DIG ADC interrupts"]
    pub int_ena: INT_ENA,
    #[doc = "0x4c - DIG ADC interrupt raw bits"]
    pub int_raw: INT_RAW,
    #[doc = "0x50 - DIG ADC interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x54 - Clear DIG ADC interrupts"]
    pub int_clr: INT_CLR,
    #[doc = "0x58 - Configure digital ADC DMA path"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x5c - Configure DIG ADC clock"]
    pub clkm_conf: CLKM_CONF,
    #[doc = "0x60 - Configure DAC settings"]
    pub apb_dac_ctrl: APB_DAC_CTRL,
    _reserved25: [u8; 0x0398],
    #[doc = "0x3fc - Version control register"]
    pub apb_ctrl_date: APB_CTRL_DATE,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DIG ADC common configuration"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "DIG ADC common configuration"]
pub mod ctrl2;
#[doc = "FSM (rw) register accessor: an alias for `Reg<FSM_SPEC>`"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "digital adc control register"]
pub mod fsm;
#[doc = "FSM_WAIT (rw) register accessor: an alias for `Reg<FSM_WAIT_SPEC>`"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "configure saradc fsm internal parameter base on test"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: an alias for `Reg<SAR1_STATUS_SPEC>`"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "digital adc1 status"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: an alias for `Reg<SAR2_STATUS_SPEC>`"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "digital adc2 status"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: an alias for `Reg<SAR1_PATT_TAB1_SPEC>`"]
pub type SAR1_PATT_TAB1 = crate::Reg<sar1_patt_tab1::SAR1_PATT_TAB1_SPEC>;
#[doc = "item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: an alias for `Reg<SAR1_PATT_TAB2_SPEC>`"]
pub type SAR1_PATT_TAB2 = crate::Reg<sar1_patt_tab2::SAR1_PATT_TAB2_SPEC>;
#[doc = "Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: an alias for `Reg<SAR1_PATT_TAB3_SPEC>`"]
pub type SAR1_PATT_TAB3 = crate::Reg<sar1_patt_tab3::SAR1_PATT_TAB3_SPEC>;
#[doc = "Item 8 ~ 11 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: an alias for `Reg<SAR1_PATT_TAB4_SPEC>`"]
pub type SAR1_PATT_TAB4 = crate::Reg<sar1_patt_tab4::SAR1_PATT_TAB4_SPEC>;
#[doc = "Item 12 ~ 15 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: an alias for `Reg<SAR2_PATT_TAB1_SPEC>`"]
pub type SAR2_PATT_TAB1 = crate::Reg<sar2_patt_tab1::SAR2_PATT_TAB1_SPEC>;
#[doc = "item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: an alias for `Reg<SAR2_PATT_TAB2_SPEC>`"]
pub type SAR2_PATT_TAB2 = crate::Reg<sar2_patt_tab2::SAR2_PATT_TAB2_SPEC>;
#[doc = "Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: an alias for `Reg<SAR2_PATT_TAB3_SPEC>`"]
pub type SAR2_PATT_TAB3 = crate::Reg<sar2_patt_tab3::SAR2_PATT_TAB3_SPEC>;
#[doc = "Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: an alias for `Reg<SAR2_PATT_TAB4_SPEC>`"]
pub type SAR2_PATT_TAB4 = crate::Reg<sar2_patt_tab4::SAR2_PATT_TAB4_SPEC>;
#[doc = "Item 12 ~ 15 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: an alias for `Reg<ARB_CTRL_SPEC>`"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "Configure the settings of DIG ADC2 arbiter"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL (rw) register accessor: an alias for `Reg<FILTER_CTRL_SPEC>`"]
pub type FILTER_CTRL = crate::Reg<filter_ctrl::FILTER_CTRL_SPEC>;
#[doc = "Configure the settings of DIG ADC2 filter"]
pub mod filter_ctrl;
#[doc = "FILTER_STATUS (r) register accessor: an alias for `Reg<FILTER_STATUS_SPEC>`"]
pub type FILTER_STATUS = crate::Reg<filter_status::FILTER_STATUS_SPEC>;
#[doc = "Data status of DIG ADC2 filter"]
pub mod filter_status;
#[doc = "THRES_CTRL (rw) register accessor: an alias for `Reg<THRES_CTRL_SPEC>`"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "Configure monitor threshold for DIG ADC2"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enable DIG ADC interrupts"]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "DIG ADC interrupt raw bits"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "DIG ADC interrupt status"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clear DIG ADC interrupts"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "Configure digital ADC DMA path"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: an alias for `Reg<CLKM_CONF_SPEC>`"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "Configure DIG ADC clock"]
pub mod clkm_conf;
#[doc = "APB_DAC_CTRL (rw) register accessor: an alias for `Reg<APB_DAC_CTRL_SPEC>`"]
pub type APB_DAC_CTRL = crate::Reg<apb_dac_ctrl::APB_DAC_CTRL_SPEC>;
#[doc = "Configure DAC settings"]
pub mod apb_dac_ctrl;
#[doc = "APB_CTRL_DATE (rw) register accessor: an alias for `Reg<APB_CTRL_DATE_SPEC>`"]
pub type APB_CTRL_DATE = crate::Reg<apb_ctrl_date::APB_CTRL_DATE_SPEC>;
#[doc = "Version control register"]
pub mod apb_ctrl_date;
