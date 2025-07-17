#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    ctrl2: CTRL2,
    filter_ctrl1: FILTER_CTRL1,
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
    filter_ctrl0: FILTER_CTRL0,
    apb_saradc1_data_status: APB_SARADC1_DATA_STATUS,
    thres0_ctrl: THRES0_CTRL,
    thres1_ctrl: THRES1_CTRL,
    _reserved19: [u8; 0x0c],
    thres_ctrl: THRES_CTRL,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    dma_conf: DMA_CONF,
    clkm_conf: CLKM_CONF,
    _reserved26: [u8; 0x04],
    apb_saradc2_data_status: APB_SARADC2_DATA_STATUS,
    _reserved27: [u8; 0x0380],
    ctrl_date: CTRL_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure apb saradc controller"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - configure apb saradc controller"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x08 - configure saradc filter"]
    #[inline(always)]
    pub const fn filter_ctrl1(&self) -> &FILTER_CTRL1 {
        &self.filter_ctrl1
    }
    #[doc = "0x0c - configure apb saradc fsm"]
    #[inline(always)]
    pub const fn fsm_wait(&self) -> &FSM_WAIT {
        &self.fsm_wait
    }
    #[doc = "0x10 - saradc1 status for debug"]
    #[inline(always)]
    pub const fn sar1_status(&self) -> &SAR1_STATUS {
        &self.sar1_status
    }
    #[doc = "0x14 - saradc2 status for debug"]
    #[inline(always)]
    pub const fn sar2_status(&self) -> &SAR2_STATUS {
        &self.sar2_status
    }
    #[doc = "0x18 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar1_patt_tab1(&self) -> &SAR1_PATT_TAB1 {
        &self.sar1_patt_tab1
    }
    #[doc = "0x1c - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar1_patt_tab2(&self) -> &SAR1_PATT_TAB2 {
        &self.sar1_patt_tab2
    }
    #[doc = "0x20 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar1_patt_tab3(&self) -> &SAR1_PATT_TAB3 {
        &self.sar1_patt_tab3
    }
    #[doc = "0x24 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar1_patt_tab4(&self) -> &SAR1_PATT_TAB4 {
        &self.sar1_patt_tab4
    }
    #[doc = "0x28 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar2_patt_tab1(&self) -> &SAR2_PATT_TAB1 {
        &self.sar2_patt_tab1
    }
    #[doc = "0x2c - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar2_patt_tab2(&self) -> &SAR2_PATT_TAB2 {
        &self.sar2_patt_tab2
    }
    #[doc = "0x30 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar2_patt_tab3(&self) -> &SAR2_PATT_TAB3 {
        &self.sar2_patt_tab3
    }
    #[doc = "0x34 - configure apb saradc pattern table"]
    #[inline(always)]
    pub const fn sar2_patt_tab4(&self) -> &SAR2_PATT_TAB4 {
        &self.sar2_patt_tab4
    }
    #[doc = "0x38 - configure apb saradc arbit"]
    #[inline(always)]
    pub const fn arb_ctrl(&self) -> &ARB_CTRL {
        &self.arb_ctrl
    }
    #[doc = "0x3c - configure apb saradc arbit"]
    #[inline(always)]
    pub const fn filter_ctrl0(&self) -> &FILTER_CTRL0 {
        &self.filter_ctrl0
    }
    #[doc = "0x40 - get apb saradc sample data"]
    #[inline(always)]
    pub const fn apb_saradc1_data_status(&self) -> &APB_SARADC1_DATA_STATUS {
        &self.apb_saradc1_data_status
    }
    #[doc = "0x44 - configure apb saradc thres monitor"]
    #[inline(always)]
    pub const fn thres0_ctrl(&self) -> &THRES0_CTRL {
        &self.thres0_ctrl
    }
    #[doc = "0x48 - configure apb saradc thres monitor"]
    #[inline(always)]
    pub const fn thres1_ctrl(&self) -> &THRES1_CTRL {
        &self.thres1_ctrl
    }
    #[doc = "0x58 - configure thres monitor enable"]
    #[inline(always)]
    pub const fn thres_ctrl(&self) -> &THRES_CTRL {
        &self.thres_ctrl
    }
    #[doc = "0x5c - enable interrupt"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x60 - raw of interrupt"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x64 - state of interrupt"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x68 - clear interrupt"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x6c - configure apb saradc dma"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x70 - configure apb saradc clock"]
    #[inline(always)]
    pub const fn clkm_conf(&self) -> &CLKM_CONF {
        &self.clkm_conf
    }
    #[doc = "0x78 - get apb saradc2 sample data"]
    #[inline(always)]
    pub const fn apb_saradc2_data_status(&self) -> &APB_SARADC2_DATA_STATUS {
        &self.apb_saradc2_data_status
    }
    #[doc = "0x3fc - version"]
    #[inline(always)]
    pub const fn ctrl_date(&self) -> &CTRL_DATE {
        &self.ctrl_date
    }
}
#[doc = "CTRL (rw) register accessor: configure apb saradc controller\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "configure apb saradc controller"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: configure apb saradc controller\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "configure apb saradc controller"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: configure saradc filter\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl1`] module"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "configure saradc filter"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: configure apb saradc fsm\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_wait`] module"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "configure apb saradc fsm"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: saradc1 status for debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_status`] module"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "saradc1 status for debug"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: saradc2 status for debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_status`] module"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "saradc2 status for debug"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab1`] module"]
pub type SAR1_PATT_TAB1 = crate::Reg<sar1_patt_tab1::SAR1_PATT_TAB1_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab2`] module"]
pub type SAR1_PATT_TAB2 = crate::Reg<sar1_patt_tab2::SAR1_PATT_TAB2_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab3`] module"]
pub type SAR1_PATT_TAB3 = crate::Reg<sar1_patt_tab3::SAR1_PATT_TAB3_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab4`] module"]
pub type SAR1_PATT_TAB4 = crate::Reg<sar1_patt_tab4::SAR1_PATT_TAB4_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab1`] module"]
pub type SAR2_PATT_TAB1 = crate::Reg<sar2_patt_tab1::SAR2_PATT_TAB1_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab2`] module"]
pub type SAR2_PATT_TAB2 = crate::Reg<sar2_patt_tab2::SAR2_PATT_TAB2_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab3`] module"]
pub type SAR2_PATT_TAB3 = crate::Reg<sar2_patt_tab3::SAR2_PATT_TAB3_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab4`] module"]
pub type SAR2_PATT_TAB4 = crate::Reg<sar2_patt_tab4::SAR2_PATT_TAB4_SPEC>;
#[doc = "configure apb saradc pattern table"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: configure apb saradc arbit\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ctrl`] module"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "configure apb saradc arbit"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: configure apb saradc arbit\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl0`] module"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "configure apb saradc arbit"]
pub mod filter_ctrl0;
#[doc = "APB_SARADC1_DATA_STATUS (r) register accessor: get apb saradc sample data\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_saradc1_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc1_data_status`] module"]
pub type APB_SARADC1_DATA_STATUS =
    crate::Reg<apb_saradc1_data_status::APB_SARADC1_DATA_STATUS_SPEC>;
#[doc = "get apb saradc sample data"]
pub mod apb_saradc1_data_status;
#[doc = "THRES0_CTRL (rw) register accessor: configure apb saradc thres monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`thres0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres0_ctrl`] module"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "configure apb saradc thres monitor"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: configure apb saradc thres monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`thres1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres1_ctrl`] module"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "configure apb saradc thres monitor"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: configure thres monitor enable\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_ctrl`] module"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "configure thres monitor enable"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "enable interrupt"]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: raw of interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "raw of interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: state of interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "state of interrupt"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: clear interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "clear interrupt"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: configure apb saradc dma\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "configure apb saradc dma"]
pub mod dma_conf;
#[doc = "CLKM_CONF (rw) register accessor: configure apb saradc clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkm_conf`] module"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "configure apb saradc clock"]
pub mod clkm_conf;
#[doc = "APB_SARADC2_DATA_STATUS (r) register accessor: get apb saradc2 sample data\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_saradc2_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_saradc2_data_status`] module"]
pub type APB_SARADC2_DATA_STATUS =
    crate::Reg<apb_saradc2_data_status::APB_SARADC2_DATA_STATUS_SPEC>;
#[doc = "get apb saradc2 sample data"]
pub mod apb_saradc2_data_status;
pub use crate::aes::date as ctrl_date;
pub use crate::aes::DATE as CTRL_DATE;
