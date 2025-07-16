#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    ctrl2: CTRL2,
    fsm: FSM,
    fsm_wait: FSM_WAIT,
    sar1_status: SAR1_STATUS,
    sar2_status: SAR2_STATUS,
    sar1_patt_tab1: SAR1_PATT_TAB1,
    sar1_patt_tab2: SAR1_PATT_TAB2,
    sar1_patt_tab3: SAR1_PATT_TAB3,
    sar1_patt_tab4: SAR1_PATT_TAB4,
    sar2_patt_tab1: SAR2_PATT_TAB1,
    sar2_patt_tab2: SAR2_PATT_TAB2,
    sar2_patt_tab3: SAR2_PATT_TAB3,
    sar2_patt_tab4: SAR2_PATT_TAB4,
    arb_ctrl: ARB_CTRL,
    filter_ctrl: FILTER_CTRL,
    filter_status: FILTER_STATUS,
    thres_ctrl: THRES_CTRL,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    dma_conf: DMA_CONF,
    clkm_conf: CLKM_CONF,
    apb_dac_ctrl: APB_DAC_CTRL,
    _reserved25: [u8; 0x0398],
    ctrl_date: CTRL_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - DIG ADC common configuration"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - DIG ADC common configuration"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x08 - digital adc control register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0x0c - configure saradc fsm internal parameter base on test"]
    #[inline(always)]
    pub const fn fsm_wait(&self) -> &FSM_WAIT {
        &self.fsm_wait
    }
    #[doc = "0x10 - digital adc1 status"]
    #[inline(always)]
    pub const fn sar1_status(&self) -> &SAR1_STATUS {
        &self.sar1_status
    }
    #[doc = "0x14 - digital adc2 status"]
    #[inline(always)]
    pub const fn sar2_status(&self) -> &SAR2_STATUS {
        &self.sar2_status
    }
    #[doc = "0x18 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub const fn sar1_patt_tab1(&self) -> &SAR1_PATT_TAB1 {
        &self.sar1_patt_tab1
    }
    #[doc = "0x1c - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub const fn sar1_patt_tab2(&self) -> &SAR1_PATT_TAB2 {
        &self.sar1_patt_tab2
    }
    #[doc = "0x20 - Item 8 ~ 11 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub const fn sar1_patt_tab3(&self) -> &SAR1_PATT_TAB3 {
        &self.sar1_patt_tab3
    }
    #[doc = "0x24 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub const fn sar1_patt_tab4(&self) -> &SAR1_PATT_TAB4 {
        &self.sar1_patt_tab4
    }
    #[doc = "0x28 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub const fn sar2_patt_tab1(&self) -> &SAR2_PATT_TAB1 {
        &self.sar2_patt_tab1
    }
    #[doc = "0x2c - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub const fn sar2_patt_tab2(&self) -> &SAR2_PATT_TAB2 {
        &self.sar2_patt_tab2
    }
    #[doc = "0x30 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub const fn sar2_patt_tab3(&self) -> &SAR2_PATT_TAB3 {
        &self.sar2_patt_tab3
    }
    #[doc = "0x34 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub const fn sar2_patt_tab4(&self) -> &SAR2_PATT_TAB4 {
        &self.sar2_patt_tab4
    }
    #[doc = "0x38 - Configure the settings of DIG ADC2 arbiter"]
    #[inline(always)]
    pub const fn arb_ctrl(&self) -> &ARB_CTRL {
        &self.arb_ctrl
    }
    #[doc = "0x3c - Configure the settings of DIG ADC2 filter"]
    #[inline(always)]
    pub const fn filter_ctrl(&self) -> &FILTER_CTRL {
        &self.filter_ctrl
    }
    #[doc = "0x40 - Data status of DIG ADC2 filter"]
    #[inline(always)]
    pub const fn filter_status(&self) -> &FILTER_STATUS {
        &self.filter_status
    }
    #[doc = "0x44 - Configure monitor threshold for DIG ADC2"]
    #[inline(always)]
    pub const fn thres_ctrl(&self) -> &THRES_CTRL {
        &self.thres_ctrl
    }
    #[doc = "0x48 - Enable DIG ADC interrupts"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x4c - DIG ADC interrupt raw bits"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x50 - DIG ADC interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x54 - Clear DIG ADC interrupts"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x58 - Configure digital ADC DMA path"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x5c - Configure DIG ADC clock"]
    #[inline(always)]
    pub const fn clkm_conf(&self) -> &CLKM_CONF {
        &self.clkm_conf
    }
    #[doc = "0x60 - Configure DAC settings"]
    #[inline(always)]
    pub const fn apb_dac_ctrl(&self) -> &APB_DAC_CTRL {
        &self.apb_dac_ctrl
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn ctrl_date(&self) -> &CTRL_DATE {
        &self.ctrl_date
    }
}
#[doc = "CTRL (rw) register accessor: DIG ADC common configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DIG ADC common configuration"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: DIG ADC common configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "DIG ADC common configuration"]
pub mod ctrl2;
#[doc = "FSM (rw) register accessor: digital adc control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "digital adc control register"]
pub mod fsm;
#[doc = "FSM_WAIT (rw) register accessor: configure saradc fsm internal parameter base on test\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_wait`] module"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "configure saradc fsm internal parameter base on test"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: digital adc1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_status`] module"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "digital adc1 status"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: digital adc2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_status`] module"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "digital adc2 status"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: item 0 ~ 3 for pattern table 1 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab1`] module"]
pub type SAR1_PATT_TAB1 = crate::Reg<sar1_patt_tab1::SAR1_PATT_TAB1_SPEC>;
#[doc = "item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: Item 4 ~ 7 for pattern table 1 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab2`] module"]
pub type SAR1_PATT_TAB2 = crate::Reg<sar1_patt_tab2::SAR1_PATT_TAB2_SPEC>;
#[doc = "Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: Item 8 ~ 11 for pattern table 1 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab3`] module"]
pub type SAR1_PATT_TAB3 = crate::Reg<sar1_patt_tab3::SAR1_PATT_TAB3_SPEC>;
#[doc = "Item 8 ~ 11 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: Item 12 ~ 15 for pattern table 1 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab4`] module"]
pub type SAR1_PATT_TAB4 = crate::Reg<sar1_patt_tab4::SAR1_PATT_TAB4_SPEC>;
#[doc = "Item 12 ~ 15 for pattern table 1 (each item one byte)"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: item 0 ~ 3 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab1`] module"]
pub type SAR2_PATT_TAB1 = crate::Reg<sar2_patt_tab1::SAR2_PATT_TAB1_SPEC>;
#[doc = "item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: Item 4 ~ 7 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab2`] module"]
pub type SAR2_PATT_TAB2 = crate::Reg<sar2_patt_tab2::SAR2_PATT_TAB2_SPEC>;
#[doc = "Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: Item 8 ~ 11 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab3`] module"]
pub type SAR2_PATT_TAB3 = crate::Reg<sar2_patt_tab3::SAR2_PATT_TAB3_SPEC>;
#[doc = "Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: Item 12 ~ 15 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab4`] module"]
pub type SAR2_PATT_TAB4 = crate::Reg<sar2_patt_tab4::SAR2_PATT_TAB4_SPEC>;
#[doc = "Item 12 ~ 15 for pattern table 2 (each item one byte)"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: Configure the settings of DIG ADC2 arbiter\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ctrl`] module"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "Configure the settings of DIG ADC2 arbiter"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL (rw) register accessor: Configure the settings of DIG ADC2 filter\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl`] module"]
pub type FILTER_CTRL = crate::Reg<filter_ctrl::FILTER_CTRL_SPEC>;
#[doc = "Configure the settings of DIG ADC2 filter"]
pub mod filter_ctrl;
#[doc = "FILTER_STATUS (r) register accessor: Data status of DIG ADC2 filter\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_status`] module"]
pub type FILTER_STATUS = crate::Reg<filter_status::FILTER_STATUS_SPEC>;
#[doc = "Data status of DIG ADC2 filter"]
pub mod filter_status;
#[doc = "THRES_CTRL (rw) register accessor: Configure monitor threshold for DIG ADC2\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_ctrl`] module"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "Configure monitor threshold for DIG ADC2"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: Enable DIG ADC interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enable DIG ADC interrupts"]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: DIG ADC interrupt raw bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "DIG ADC interrupt raw bits"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: DIG ADC interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "DIG ADC interrupt status"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Clear DIG ADC interrupts\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clear DIG ADC interrupts"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: Configure digital ADC DMA path\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "Configure digital ADC DMA path"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: Configure DIG ADC clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkm_conf`] module"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "Configure DIG ADC clock"]
pub mod clkm_conf;
#[doc = "APB_DAC_CTRL (rw) register accessor: Configure DAC settings\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_dac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_dac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_dac_ctrl`] module"]
pub type APB_DAC_CTRL = crate::Reg<apb_dac_ctrl::APB_DAC_CTRL_SPEC>;
#[doc = "Configure DAC settings"]
pub mod apb_dac_ctrl;
pub use crate::aes::date as ctrl_date;
pub use crate::aes::DATE as CTRL_DATE;
