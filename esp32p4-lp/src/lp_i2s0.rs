#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    vad_conf: VAD_CONF,
    vad_result: VAD_RESULT,
    rx_mem_conf: RX_MEM_CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved7: [u8; 0x04],
    rx_conf: RX_CONF,
    _reserved8: [u8; 0x04],
    rx_conf1: RX_CONF1,
    _reserved9: [u8; 0x24],
    rx_tdm_ctrl: RX_TDM_CTRL,
    _reserved10: [u8; 0x04],
    rx_timing: RX_TIMING,
    _reserved11: [u8; 0x04],
    lc_hung_conf: LC_HUNG_CONF,
    rxeof_num: RXEOF_NUM,
    conf_sigle_data: CONF_SIGLE_DATA,
    _reserved14: [u8; 0x04],
    rx_pdm_conf: RX_PDM_CONF,
    eco_low: ECO_LOW,
    eco_high: ECO_HIGH,
    eco_conf: ECO_CONF,
    vad_param0: VAD_PARAM0,
    vad_param1: VAD_PARAM1,
    vad_param2: VAD_PARAM2,
    vad_param3: VAD_PARAM3,
    vad_param4: VAD_PARAM4,
    vad_param5: VAD_PARAM5,
    vad_param6: VAD_PARAM6,
    vad_param7: VAD_PARAM7,
    vad_param8: VAD_PARAM8,
    _reserved27: [u8; 0x0c],
    vad_ob0: VAD_OB0,
    vad_ob1: VAD_OB1,
    vad_ob2: VAD_OB2,
    vad_ob3: VAD_OB3,
    vad_ob4: VAD_OB4,
    vad_ob5: VAD_OB5,
    vad_ob6: VAD_OB6,
    vad_ob7: VAD_OB7,
    vad_ob8: VAD_OB8,
    _reserved36: [u8; 0x24],
    clk_gate: CLK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - I2S VAD Configure register"]
    #[inline(always)]
    pub const fn vad_conf(&self) -> &VAD_CONF {
        &self.vad_conf
    }
    #[doc = "0x04 - I2S VAD Result register"]
    #[inline(always)]
    pub const fn vad_result(&self) -> &VAD_RESULT {
        &self.vad_result
    }
    #[doc = "0x08 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn rx_mem_conf(&self) -> &RX_MEM_CONF {
        &self.rx_mem_conf
    }
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10 - I2S interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x14 - I2S interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - I2S interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x20 - I2S RX configure register"]
    #[inline(always)]
    pub const fn rx_conf(&self) -> &RX_CONF {
        &self.rx_conf
    }
    #[doc = "0x28 - I2S RX configure register 1"]
    #[inline(always)]
    pub const fn rx_conf1(&self) -> &RX_CONF1 {
        &self.rx_conf1
    }
    #[doc = "0x50 - I2S TX TDM mode control register"]
    #[inline(always)]
    pub const fn rx_tdm_ctrl(&self) -> &RX_TDM_CTRL {
        &self.rx_tdm_ctrl
    }
    #[doc = "0x58 - I2S RX timing control register"]
    #[inline(always)]
    pub const fn rx_timing(&self) -> &RX_TIMING {
        &self.rx_timing
    }
    #[doc = "0x60 - I2S HUNG configure register."]
    #[inline(always)]
    pub const fn lc_hung_conf(&self) -> &LC_HUNG_CONF {
        &self.lc_hung_conf
    }
    #[doc = "0x64 - I2S RX data number control register."]
    #[inline(always)]
    pub const fn rxeof_num(&self) -> &RXEOF_NUM {
        &self.rxeof_num
    }
    #[doc = "0x68 - I2S signal data register"]
    #[inline(always)]
    pub const fn conf_sigle_data(&self) -> &CONF_SIGLE_DATA {
        &self.conf_sigle_data
    }
    #[doc = "0x70 - I2S RX configure register"]
    #[inline(always)]
    pub const fn rx_pdm_conf(&self) -> &RX_PDM_CONF {
        &self.rx_pdm_conf
    }
    #[doc = "0x74 - I2S ECO register"]
    #[inline(always)]
    pub const fn eco_low(&self) -> &ECO_LOW {
        &self.eco_low
    }
    #[doc = "0x78 - I2S ECO register"]
    #[inline(always)]
    pub const fn eco_high(&self) -> &ECO_HIGH {
        &self.eco_high
    }
    #[doc = "0x7c - I2S ECO register"]
    #[inline(always)]
    pub const fn eco_conf(&self) -> &ECO_CONF {
        &self.eco_conf
    }
    #[doc = "0x80 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param0(&self) -> &VAD_PARAM0 {
        &self.vad_param0
    }
    #[doc = "0x84 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param1(&self) -> &VAD_PARAM1 {
        &self.vad_param1
    }
    #[doc = "0x88 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param2(&self) -> &VAD_PARAM2 {
        &self.vad_param2
    }
    #[doc = "0x8c - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param3(&self) -> &VAD_PARAM3 {
        &self.vad_param3
    }
    #[doc = "0x90 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param4(&self) -> &VAD_PARAM4 {
        &self.vad_param4
    }
    #[doc = "0x94 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param5(&self) -> &VAD_PARAM5 {
        &self.vad_param5
    }
    #[doc = "0x98 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param6(&self) -> &VAD_PARAM6 {
        &self.vad_param6
    }
    #[doc = "0x9c - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param7(&self) -> &VAD_PARAM7 {
        &self.vad_param7
    }
    #[doc = "0xa0 - I2S VAD Parameter register"]
    #[inline(always)]
    pub const fn vad_param8(&self) -> &VAD_PARAM8 {
        &self.vad_param8
    }
    #[doc = "0xb0 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob0(&self) -> &VAD_OB0 {
        &self.vad_ob0
    }
    #[doc = "0xb4 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob1(&self) -> &VAD_OB1 {
        &self.vad_ob1
    }
    #[doc = "0xb8 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob2(&self) -> &VAD_OB2 {
        &self.vad_ob2
    }
    #[doc = "0xbc - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob3(&self) -> &VAD_OB3 {
        &self.vad_ob3
    }
    #[doc = "0xc0 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob4(&self) -> &VAD_OB4 {
        &self.vad_ob4
    }
    #[doc = "0xc4 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob5(&self) -> &VAD_OB5 {
        &self.vad_ob5
    }
    #[doc = "0xc8 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob6(&self) -> &VAD_OB6 {
        &self.vad_ob6
    }
    #[doc = "0xcc - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob7(&self) -> &VAD_OB7 {
        &self.vad_ob7
    }
    #[doc = "0xd0 - I2S VAD Observe register"]
    #[inline(always)]
    pub const fn vad_ob8(&self) -> &VAD_OB8 {
        &self.vad_ob8
    }
    #[doc = "0xf8 - Clock gate register"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &CLK_GATE {
        &self.clk_gate
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "VAD_CONF (rw) register accessor: I2S VAD Configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_conf`] module"]
pub type VAD_CONF = crate::Reg<vad_conf::VAD_CONF_SPEC>;
#[doc = "I2S VAD Configure register"]
pub mod vad_conf;
#[doc = "VAD_RESULT (r) register accessor: I2S VAD Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_result`] module"]
pub type VAD_RESULT = crate::Reg<vad_result::VAD_RESULT_SPEC>;
#[doc = "I2S VAD Result register"]
pub mod vad_result;
#[doc = "RX_MEM_CONF (rw) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_mem_conf`] module"]
pub type RX_MEM_CONF = crate::Reg<rx_mem_conf::RX_MEM_CONF_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod rx_mem_conf;
#[doc = "INT_RAW (r) register accessor: I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: I2S interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: I2S interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: I2S interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod int_clr;
#[doc = "RX_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_conf`] module"]
pub type RX_CONF = crate::Reg<rx_conf::RX_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod rx_conf;
#[doc = "RX_CONF1 (rw) register accessor: I2S RX configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_conf1`] module"]
pub type RX_CONF1 = crate::Reg<rx_conf1::RX_CONF1_SPEC>;
#[doc = "I2S RX configure register 1"]
pub mod rx_conf1;
#[doc = "RX_TDM_CTRL (rw) register accessor: I2S TX TDM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tdm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tdm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_tdm_ctrl`] module"]
pub type RX_TDM_CTRL = crate::Reg<rx_tdm_ctrl::RX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod rx_tdm_ctrl;
#[doc = "RX_TIMING (rw) register accessor: I2S RX timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_timing`] module"]
pub type RX_TIMING = crate::Reg<rx_timing::RX_TIMING_SPEC>;
#[doc = "I2S RX timing control register"]
pub mod rx_timing;
#[doc = "LC_HUNG_CONF (rw) register accessor: I2S HUNG configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_hung_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_hung_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_hung_conf`] module"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = "I2S HUNG configure register."]
pub mod lc_hung_conf;
#[doc = "RXEOF_NUM (rw) register accessor: I2S RX data number control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxeof_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxeof_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxeof_num`] module"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = "I2S RX data number control register."]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: I2S signal data register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_sigle_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf_sigle_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_sigle_data`] module"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = "I2S signal data register"]
pub mod conf_sigle_data;
#[doc = "RX_PDM_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pdm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_pdm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_pdm_conf`] module"]
pub type RX_PDM_CONF = crate::Reg<rx_pdm_conf::RX_PDM_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod rx_pdm_conf;
#[doc = "ECO_LOW (rw) register accessor: I2S ECO register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_low`] module"]
pub type ECO_LOW = crate::Reg<eco_low::ECO_LOW_SPEC>;
#[doc = "I2S ECO register"]
pub mod eco_low;
#[doc = "ECO_HIGH (rw) register accessor: I2S ECO register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_high`] module"]
pub type ECO_HIGH = crate::Reg<eco_high::ECO_HIGH_SPEC>;
#[doc = "I2S ECO register"]
pub mod eco_high;
#[doc = "ECO_CONF (rw) register accessor: I2S ECO register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_conf`] module"]
pub type ECO_CONF = crate::Reg<eco_conf::ECO_CONF_SPEC>;
#[doc = "I2S ECO register"]
pub mod eco_conf;
#[doc = "VAD_PARAM0 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param0`] module"]
pub type VAD_PARAM0 = crate::Reg<vad_param0::VAD_PARAM0_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param0;
#[doc = "VAD_PARAM1 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param1`] module"]
pub type VAD_PARAM1 = crate::Reg<vad_param1::VAD_PARAM1_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param1;
#[doc = "VAD_PARAM2 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param2`] module"]
pub type VAD_PARAM2 = crate::Reg<vad_param2::VAD_PARAM2_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param2;
#[doc = "VAD_PARAM3 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param3`] module"]
pub type VAD_PARAM3 = crate::Reg<vad_param3::VAD_PARAM3_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param3;
#[doc = "VAD_PARAM4 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param4`] module"]
pub type VAD_PARAM4 = crate::Reg<vad_param4::VAD_PARAM4_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param4;
#[doc = "VAD_PARAM5 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param5`] module"]
pub type VAD_PARAM5 = crate::Reg<vad_param5::VAD_PARAM5_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param5;
#[doc = "VAD_PARAM6 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param6`] module"]
pub type VAD_PARAM6 = crate::Reg<vad_param6::VAD_PARAM6_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param6;
#[doc = "VAD_PARAM7 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param7`] module"]
pub type VAD_PARAM7 = crate::Reg<vad_param7::VAD_PARAM7_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param7;
#[doc = "VAD_PARAM8 (rw) register accessor: I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_param8`] module"]
pub type VAD_PARAM8 = crate::Reg<vad_param8::VAD_PARAM8_SPEC>;
#[doc = "I2S VAD Parameter register"]
pub mod vad_param8;
#[doc = "VAD_OB0 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob0`] module"]
pub type VAD_OB0 = crate::Reg<vad_ob0::VAD_OB0_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob0;
#[doc = "VAD_OB1 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob1`] module"]
pub type VAD_OB1 = crate::Reg<vad_ob1::VAD_OB1_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob1;
#[doc = "VAD_OB2 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob2`] module"]
pub type VAD_OB2 = crate::Reg<vad_ob2::VAD_OB2_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob2;
#[doc = "VAD_OB3 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob3`] module"]
pub type VAD_OB3 = crate::Reg<vad_ob3::VAD_OB3_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob3;
#[doc = "VAD_OB4 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob4`] module"]
pub type VAD_OB4 = crate::Reg<vad_ob4::VAD_OB4_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob4;
#[doc = "VAD_OB5 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob5`] module"]
pub type VAD_OB5 = crate::Reg<vad_ob5::VAD_OB5_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob5;
#[doc = "VAD_OB6 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob6`] module"]
pub type VAD_OB6 = crate::Reg<vad_ob6::VAD_OB6_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob6;
#[doc = "VAD_OB7 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob7`] module"]
pub type VAD_OB7 = crate::Reg<vad_ob7::VAD_OB7_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob7;
#[doc = "VAD_OB8 (r) register accessor: I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ob8`] module"]
pub type VAD_OB8 = crate::Reg<vad_ob8::VAD_OB8_SPEC>;
#[doc = "I2S VAD Observe register"]
pub mod vad_ob8;
#[doc = "CLK_GATE (rw) register accessor: Clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
pub type CLK_GATE = crate::Reg<clk_gate::CLK_GATE_SPEC>;
#[doc = "Clock gate register"]
pub mod clk_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
