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
    sar1_data_status: SAR1_DATA_STATUS,
    thres0_ctrl: THRES0_CTRL,
    thres1_ctrl: THRES1_CTRL,
    thres_ctrl: THRES_CTRL,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    dma_conf: DMA_CONF,
    sar2_data_status: SAR2_DATA_STATUS,
    cali: CALI,
    rnd_eco_low: RND_ECO_LOW,
    rnd_eco_high: RND_ECO_HIGH,
    rnd_eco_cs: RND_ECO_CS,
    _reserved30: [u8; 0x0384],
    ctrl_date: CTRL_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Register"]
    #[inline(always)]
    pub const fn filter_ctrl1(&self) -> &FILTER_CTRL1 {
        &self.filter_ctrl1
    }
    #[doc = "0x0c - Register"]
    #[inline(always)]
    pub const fn fsm_wait(&self) -> &FSM_WAIT {
        &self.fsm_wait
    }
    #[doc = "0x10 - Register"]
    #[inline(always)]
    pub const fn sar1_status(&self) -> &SAR1_STATUS {
        &self.sar1_status
    }
    #[doc = "0x14 - Register"]
    #[inline(always)]
    pub const fn sar2_status(&self) -> &SAR2_STATUS {
        &self.sar2_status
    }
    #[doc = "0x18 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab1(&self) -> &SAR1_PATT_TAB1 {
        &self.sar1_patt_tab1
    }
    #[doc = "0x1c - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab2(&self) -> &SAR1_PATT_TAB2 {
        &self.sar1_patt_tab2
    }
    #[doc = "0x20 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab3(&self) -> &SAR1_PATT_TAB3 {
        &self.sar1_patt_tab3
    }
    #[doc = "0x24 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab4(&self) -> &SAR1_PATT_TAB4 {
        &self.sar1_patt_tab4
    }
    #[doc = "0x28 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab1(&self) -> &SAR2_PATT_TAB1 {
        &self.sar2_patt_tab1
    }
    #[doc = "0x2c - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab2(&self) -> &SAR2_PATT_TAB2 {
        &self.sar2_patt_tab2
    }
    #[doc = "0x30 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab3(&self) -> &SAR2_PATT_TAB3 {
        &self.sar2_patt_tab3
    }
    #[doc = "0x34 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab4(&self) -> &SAR2_PATT_TAB4 {
        &self.sar2_patt_tab4
    }
    #[doc = "0x38 - Register"]
    #[inline(always)]
    pub const fn arb_ctrl(&self) -> &ARB_CTRL {
        &self.arb_ctrl
    }
    #[doc = "0x3c - Register"]
    #[inline(always)]
    pub const fn filter_ctrl0(&self) -> &FILTER_CTRL0 {
        &self.filter_ctrl0
    }
    #[doc = "0x40 - Register"]
    #[inline(always)]
    pub const fn sar1_data_status(&self) -> &SAR1_DATA_STATUS {
        &self.sar1_data_status
    }
    #[doc = "0x44 - Register"]
    #[inline(always)]
    pub const fn thres0_ctrl(&self) -> &THRES0_CTRL {
        &self.thres0_ctrl
    }
    #[doc = "0x48 - Register"]
    #[inline(always)]
    pub const fn thres1_ctrl(&self) -> &THRES1_CTRL {
        &self.thres1_ctrl
    }
    #[doc = "0x4c - Register"]
    #[inline(always)]
    pub const fn thres_ctrl(&self) -> &THRES_CTRL {
        &self.thres_ctrl
    }
    #[doc = "0x50 - Register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x54 - Register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x58 - Register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x5c - Register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x60 - Register"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x64 - Register"]
    #[inline(always)]
    pub const fn sar2_data_status(&self) -> &SAR2_DATA_STATUS {
        &self.sar2_data_status
    }
    #[doc = "0x68 - Register"]
    #[inline(always)]
    pub const fn cali(&self) -> &CALI {
        &self.cali
    }
    #[doc = "0x6c - Register"]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    #[doc = "0x70 - Register"]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
    #[doc = "0x74 - Register"]
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RND_ECO_CS {
        &self.rnd_eco_cs
    }
    #[doc = "0x3fc - Register"]
    #[inline(always)]
    pub const fn ctrl_date(&self) -> &CTRL_DATE {
        &self.ctrl_date
    }
}
#[doc = "CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Register"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Register"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl1`] module"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "Register"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_wait`] module"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "Register"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_status`] module"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "Register"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_status`] module"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "Register"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab1`] module"]
pub type SAR1_PATT_TAB1 = crate::Reg<sar1_patt_tab1::SAR1_PATT_TAB1_SPEC>;
#[doc = "Register"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab2`] module"]
pub type SAR1_PATT_TAB2 = crate::Reg<sar1_patt_tab2::SAR1_PATT_TAB2_SPEC>;
#[doc = "Register"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab3`] module"]
pub type SAR1_PATT_TAB3 = crate::Reg<sar1_patt_tab3::SAR1_PATT_TAB3_SPEC>;
#[doc = "Register"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab4`] module"]
pub type SAR1_PATT_TAB4 = crate::Reg<sar1_patt_tab4::SAR1_PATT_TAB4_SPEC>;
#[doc = "Register"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab1`] module"]
pub type SAR2_PATT_TAB1 = crate::Reg<sar2_patt_tab1::SAR2_PATT_TAB1_SPEC>;
#[doc = "Register"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab2`] module"]
pub type SAR2_PATT_TAB2 = crate::Reg<sar2_patt_tab2::SAR2_PATT_TAB2_SPEC>;
#[doc = "Register"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab3`] module"]
pub type SAR2_PATT_TAB3 = crate::Reg<sar2_patt_tab3::SAR2_PATT_TAB3_SPEC>;
#[doc = "Register"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab4`] module"]
pub type SAR2_PATT_TAB4 = crate::Reg<sar2_patt_tab4::SAR2_PATT_TAB4_SPEC>;
#[doc = "Register"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ctrl`] module"]
pub type ARB_CTRL = crate::Reg<arb_ctrl::ARB_CTRL_SPEC>;
#[doc = "Register"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl0`] module"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "Register"]
pub mod filter_ctrl0;
#[doc = "SAR1_DATA_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_data_status`] module"]
pub type SAR1_DATA_STATUS = crate::Reg<sar1_data_status::SAR1_DATA_STATUS_SPEC>;
#[doc = "Register"]
pub mod sar1_data_status;
#[doc = "THRES0_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres0_ctrl`] module"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "Register"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres1_ctrl`] module"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "Register"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_ctrl`] module"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "Register"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Register"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "Register"]
pub mod dma_conf;
#[doc = "SAR2_DATA_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_data_status`] module"]
pub type SAR2_DATA_STATUS = crate::Reg<sar2_data_status::SAR2_DATA_STATUS_SPEC>;
#[doc = "Register"]
pub mod sar2_data_status;
#[doc = "CALI (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali`] module"]
pub type CALI = crate::Reg<cali::CALI_SPEC>;
#[doc = "Register"]
pub mod cali;
#[doc = "RND_ECO_LOW (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "Register"]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "Register"]
pub mod rnd_eco_high;
#[doc = "RND_ECO_CS (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_cs`] module"]
pub type RND_ECO_CS = crate::Reg<rnd_eco_cs::RND_ECO_CS_SPEC>;
#[doc = "Register"]
pub mod rnd_eco_cs;
#[doc = "CTRL_DATE (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_date`] module"]
pub type CTRL_DATE = crate::Reg<ctrl_date::CTRL_DATE_SPEC>;
#[doc = "Register"]
pub mod ctrl_date;
