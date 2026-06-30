#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pad_cfg: PAD_CFG,
    cali_result: CALI_RESULT,
    pdma_cfg: PDMA_CFG,
    pdma_timer_cfg: PDMA_TIMER_CFG,
    data_output_cfg: DATA_OUTPUT_CFG,
    sintx_cfg: SINTX_CFG,
    sintx_timer_cfg: SINTX_TIMER_CFG,
    sintx_data: SINTX_DATA,
    cali: CALI,
    sample_wait_cfg: SAMPLE_WAIT_CFG,
    hold_wait_cfg: HOLD_WAIT_CFG,
    refresh_wait_cfg: REFRESH_WAIT_CFG,
    pdma_int_raw: PDMA_INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved16: [u8; 0x03bc],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure dac pad register"]
    #[inline(always)]
    pub const fn pad_cfg(&self) -> &PAD_CFG {
        &self.pad_cfg
    }
    #[doc = "0x04 - DAC CALI result register"]
    #[inline(always)]
    pub const fn cali_result(&self) -> &CALI_RESULT {
        &self.cali_result
    }
    #[doc = "0x08 - dac cfg register"]
    #[inline(always)]
    pub const fn pdma_cfg(&self) -> &PDMA_CFG {
        &self.pdma_cfg
    }
    #[doc = "0x0c - PDMA path timer register"]
    #[inline(always)]
    pub const fn pdma_timer_cfg(&self) -> &PDMA_TIMER_CFG {
        &self.pdma_timer_cfg
    }
    #[doc = "0x10 - dac DATA OUTPUT cfg register"]
    #[inline(always)]
    pub const fn data_output_cfg(&self) -> &DATA_OUTPUT_CFG {
        &self.data_output_cfg
    }
    #[doc = "0x14 - dac rstn register"]
    #[inline(always)]
    pub const fn sintx_cfg(&self) -> &SINTX_CFG {
        &self.sintx_cfg
    }
    #[doc = "0x18 - SINTX path timer register"]
    #[inline(always)]
    pub const fn sintx_timer_cfg(&self) -> &SINTX_TIMER_CFG {
        &self.sintx_timer_cfg
    }
    #[doc = "0x1c - dac output register for sintx path"]
    #[inline(always)]
    pub const fn sintx_data(&self) -> &SINTX_DATA {
        &self.sintx_data
    }
    #[doc = "0x20 - cali algorithm register for DAC"]
    #[inline(always)]
    pub const fn cali(&self) -> &CALI {
        &self.cali
    }
    #[doc = "0x24 - cali sample phase duration register"]
    #[inline(always)]
    pub const fn sample_wait_cfg(&self) -> &SAMPLE_WAIT_CFG {
        &self.sample_wait_cfg
    }
    #[doc = "0x28 - cali hold phase duration register"]
    #[inline(always)]
    pub const fn hold_wait_cfg(&self) -> &HOLD_WAIT_CFG {
        &self.hold_wait_cfg
    }
    #[doc = "0x2c - cali refresh phase duration register"]
    #[inline(always)]
    pub const fn refresh_wait_cfg(&self) -> &REFRESH_WAIT_CFG {
        &self.refresh_wait_cfg
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn pdma_int_raw(&self) -> &PDMA_INT_RAW {
        &self.pdma_int_raw
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PAD_CFG (rw) register accessor: configure dac pad register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg`] module"]
pub type PAD_CFG = crate::Reg<pad_cfg::PAD_CFG_SPEC>;
#[doc = "configure dac pad register"]
pub mod pad_cfg;
#[doc = "CALI_RESULT (rw) register accessor: DAC CALI result register\n\nYou can [`read`](crate::Reg::read) this register and get [`cali_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali_result`] module"]
pub type CALI_RESULT = crate::Reg<cali_result::CALI_RESULT_SPEC>;
#[doc = "DAC CALI result register"]
pub mod cali_result;
#[doc = "PDMA_CFG (rw) register accessor: dac cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdma_cfg`] module"]
pub type PDMA_CFG = crate::Reg<pdma_cfg::PDMA_CFG_SPEC>;
#[doc = "dac cfg register"]
pub mod pdma_cfg;
#[doc = "PDMA_TIMER_CFG (rw) register accessor: PDMA path timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdma_timer_cfg`] module"]
pub type PDMA_TIMER_CFG = crate::Reg<pdma_timer_cfg::PDMA_TIMER_CFG_SPEC>;
#[doc = "PDMA path timer register"]
pub mod pdma_timer_cfg;
#[doc = "DATA_OUTPUT_CFG (rw) register accessor: dac DATA OUTPUT cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_output_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_output_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_output_cfg`] module"]
pub type DATA_OUTPUT_CFG = crate::Reg<data_output_cfg::DATA_OUTPUT_CFG_SPEC>;
#[doc = "dac DATA OUTPUT cfg register"]
pub mod data_output_cfg;
#[doc = "SINTX_CFG (rw) register accessor: dac rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintx_cfg`] module"]
pub type SINTX_CFG = crate::Reg<sintx_cfg::SINTX_CFG_SPEC>;
#[doc = "dac rstn register"]
pub mod sintx_cfg;
#[doc = "SINTX_TIMER_CFG (rw) register accessor: SINTX path timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintx_timer_cfg`] module"]
pub type SINTX_TIMER_CFG = crate::Reg<sintx_timer_cfg::SINTX_TIMER_CFG_SPEC>;
#[doc = "SINTX path timer register"]
pub mod sintx_timer_cfg;
#[doc = "SINTX_DATA (rw) register accessor: dac output register for sintx path\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintx_data`] module"]
pub type SINTX_DATA = crate::Reg<sintx_data::SINTX_DATA_SPEC>;
#[doc = "dac output register for sintx path"]
pub mod sintx_data;
#[doc = "CALI (r) register accessor: cali algorithm register for DAC\n\nYou can [`read`](crate::Reg::read) this register and get [`cali::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali`] module"]
pub type CALI = crate::Reg<cali::CALI_SPEC>;
#[doc = "cali algorithm register for DAC"]
pub mod cali;
#[doc = "SAMPLE_WAIT_CFG (rw) register accessor: cali sample phase duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_wait_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_wait_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_wait_cfg`] module"]
pub type SAMPLE_WAIT_CFG = crate::Reg<sample_wait_cfg::SAMPLE_WAIT_CFG_SPEC>;
#[doc = "cali sample phase duration register"]
pub mod sample_wait_cfg;
#[doc = "HOLD_WAIT_CFG (rw) register accessor: cali hold phase duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hold_wait_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hold_wait_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hold_wait_cfg`] module"]
pub type HOLD_WAIT_CFG = crate::Reg<hold_wait_cfg::HOLD_WAIT_CFG_SPEC>;
#[doc = "cali hold phase duration register"]
pub mod hold_wait_cfg;
#[doc = "REFRESH_WAIT_CFG (rw) register accessor: cali refresh phase duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_wait_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_wait_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh_wait_cfg`] module"]
pub type REFRESH_WAIT_CFG = crate::Reg<refresh_wait_cfg::REFRESH_WAIT_CFG_SPEC>;
#[doc = "cali refresh phase duration register"]
pub mod refresh_wait_cfg;
#[doc = "PDMA_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdma_int_raw`] module"]
pub type PDMA_INT_RAW = crate::Reg<pdma_int_raw::PDMA_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod pdma_int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
