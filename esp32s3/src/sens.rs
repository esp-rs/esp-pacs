#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    sar_reader1_ctrl: SAR_READER1_CTRL,
    sar_reader1_status: SAR_READER1_STATUS,
    sar_meas1_ctrl1: SAR_MEAS1_CTRL1,
    sar_meas1_ctrl2: SAR_MEAS1_CTRL2,
    sar_meas1_mux: SAR_MEAS1_MUX,
    sar_atten1: SAR_ATTEN1,
    sar_amp_ctrl1: SAR_AMP_CTRL1,
    sar_amp_ctrl2: SAR_AMP_CTRL2,
    sar_amp_ctrl3: SAR_AMP_CTRL3,
    sar_reader2_ctrl: SAR_READER2_CTRL,
    sar_reader2_status: SAR_READER2_STATUS,
    sar_meas2_ctrl1: SAR_MEAS2_CTRL1,
    sar_meas2_ctrl2: SAR_MEAS2_CTRL2,
    sar_meas2_mux: SAR_MEAS2_MUX,
    sar_atten2: SAR_ATTEN2,
    sar_power_xpd_sar: SAR_POWER_XPD_SAR,
    sar_slave_addr1: SAR_SLAVE_ADDR1,
    sar_slave_addr2: SAR_SLAVE_ADDR2,
    sar_slave_addr3: SAR_SLAVE_ADDR3,
    sar_slave_addr4: SAR_SLAVE_ADDR4,
    sar_tsens_ctrl: SAR_TSENS_CTRL,
    sar_tsens_ctrl2: SAR_TSENS_CTRL2,
    sar_i2c_ctrl: SAR_I2C_CTRL,
    sar_touch_conf: SAR_TOUCH_CONF,
    sar_touch_denoise: SAR_TOUCH_DENOISE,
    sar_touch_thres1: SAR_TOUCH_THRES1,
    sar_touch_thres2: SAR_TOUCH_THRES2,
    sar_touch_thres3: SAR_TOUCH_THRES3,
    sar_touch_thres4: SAR_TOUCH_THRES4,
    sar_touch_thres5: SAR_TOUCH_THRES5,
    sar_touch_thres6: SAR_TOUCH_THRES6,
    sar_touch_thres7: SAR_TOUCH_THRES7,
    sar_touch_thres8: SAR_TOUCH_THRES8,
    sar_touch_thres9: SAR_TOUCH_THRES9,
    sar_touch_thres10: SAR_TOUCH_THRES10,
    sar_touch_thres11: SAR_TOUCH_THRES11,
    sar_touch_thres12: SAR_TOUCH_THRES12,
    sar_touch_thres13: SAR_TOUCH_THRES13,
    sar_touch_thres14: SAR_TOUCH_THRES14,
    sar_touch_chn_st: SAR_TOUCH_CHN_ST,
    sar_touch_status0: SAR_TOUCH_STATUS0,
    sar_touch_status1: SAR_TOUCH_STATUS1,
    sar_touch_status2: SAR_TOUCH_STATUS2,
    sar_touch_status3: SAR_TOUCH_STATUS3,
    sar_touch_status4: SAR_TOUCH_STATUS4,
    sar_touch_status5: SAR_TOUCH_STATUS5,
    sar_touch_status6: SAR_TOUCH_STATUS6,
    sar_touch_status7: SAR_TOUCH_STATUS7,
    sar_touch_status8: SAR_TOUCH_STATUS8,
    sar_touch_status9: SAR_TOUCH_STATUS9,
    sar_touch_status10: SAR_TOUCH_STATUS10,
    sar_touch_status11: SAR_TOUCH_STATUS11,
    sar_touch_status12: SAR_TOUCH_STATUS12,
    sar_touch_status13: SAR_TOUCH_STATUS13,
    sar_touch_status14: SAR_TOUCH_STATUS14,
    sar_touch_status15: SAR_TOUCH_STATUS15,
    sar_touch_status16: SAR_TOUCH_STATUS16,
    sar_cocpu_state: SAR_COCPU_STATE,
    sar_cocpu_int_raw: SAR_COCPU_INT_RAW,
    sar_cocpu_int_ena: SAR_COCPU_INT_ENA,
    sar_cocpu_int_st: SAR_COCPU_INT_ST,
    sar_cocpu_int_clr: SAR_COCPU_INT_CLR,
    sar_cocpu_debug: SAR_COCPU_DEBUG,
    sar_hall_ctrl: SAR_HALL_CTRL,
    sar_nouse: SAR_NOUSE,
    sar_peri_clk_gate_conf: SAR_PERI_CLK_GATE_CONF,
    sar_peri_reset_conf: SAR_PERI_RESET_CONF,
    sar_cocpu_int_ena_w1ts: SAR_COCPU_INT_ENA_W1TS,
    sar_cocpu_int_ena_w1tc: SAR_COCPU_INT_ENA_W1TC,
    sar_debug_conf: SAR_DEBUG_CONF,
    _reserved70: [u8; 0xe4],
    sar_sardate: SAR_SARDATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure saradc1 reader"]
    #[inline(always)]
    pub const fn sar_reader1_ctrl(&self) -> &SAR_READER1_CTRL {
        &self.sar_reader1_ctrl
    }
    #[doc = "0x04 - get saradc1 reader controller status"]
    #[inline(always)]
    pub const fn sar_reader1_status(&self) -> &SAR_READER1_STATUS {
        &self.sar_reader1_status
    }
    #[doc = "0x08 - no public"]
    #[inline(always)]
    pub const fn sar_meas1_ctrl1(&self) -> &SAR_MEAS1_CTRL1 {
        &self.sar_meas1_ctrl1
    }
    #[doc = "0x0c - configure saradc1 controller"]
    #[inline(always)]
    pub const fn sar_meas1_ctrl2(&self) -> &SAR_MEAS1_CTRL2 {
        &self.sar_meas1_ctrl2
    }
    #[doc = "0x10 - configure saradc1 controller"]
    #[inline(always)]
    pub const fn sar_meas1_mux(&self) -> &SAR_MEAS1_MUX {
        &self.sar_meas1_mux
    }
    #[doc = "0x14 - configure saradc1 controller"]
    #[inline(always)]
    pub const fn sar_atten1(&self) -> &SAR_ATTEN1 {
        &self.sar_atten1
    }
    #[doc = "0x18 - no public"]
    #[inline(always)]
    pub const fn sar_amp_ctrl1(&self) -> &SAR_AMP_CTRL1 {
        &self.sar_amp_ctrl1
    }
    #[doc = "0x1c - no public"]
    #[inline(always)]
    pub const fn sar_amp_ctrl2(&self) -> &SAR_AMP_CTRL2 {
        &self.sar_amp_ctrl2
    }
    #[doc = "0x20 - no public"]
    #[inline(always)]
    pub const fn sar_amp_ctrl3(&self) -> &SAR_AMP_CTRL3 {
        &self.sar_amp_ctrl3
    }
    #[doc = "0x24 - configure saradc2 reader"]
    #[inline(always)]
    pub const fn sar_reader2_ctrl(&self) -> &SAR_READER2_CTRL {
        &self.sar_reader2_ctrl
    }
    #[doc = "0x28 - get saradc1 reader controller status"]
    #[inline(always)]
    pub const fn sar_reader2_status(&self) -> &SAR_READER2_STATUS {
        &self.sar_reader2_status
    }
    #[doc = "0x2c - configure saradc2 controller"]
    #[inline(always)]
    pub const fn sar_meas2_ctrl1(&self) -> &SAR_MEAS2_CTRL1 {
        &self.sar_meas2_ctrl1
    }
    #[doc = "0x30 - configure saradc2 controller"]
    #[inline(always)]
    pub const fn sar_meas2_ctrl2(&self) -> &SAR_MEAS2_CTRL2 {
        &self.sar_meas2_ctrl2
    }
    #[doc = "0x34 - configure saradc2 controller"]
    #[inline(always)]
    pub const fn sar_meas2_mux(&self) -> &SAR_MEAS2_MUX {
        &self.sar_meas2_mux
    }
    #[doc = "0x38 - configure saradc2 controller"]
    #[inline(always)]
    pub const fn sar_atten2(&self) -> &SAR_ATTEN2 {
        &self.sar_atten2
    }
    #[doc = "0x3c - configure power of saradc"]
    #[inline(always)]
    pub const fn sar_power_xpd_sar(&self) -> &SAR_POWER_XPD_SAR {
        &self.sar_power_xpd_sar
    }
    #[doc = "0x40 - configure i2c slave address"]
    #[inline(always)]
    pub const fn sar_slave_addr1(&self) -> &SAR_SLAVE_ADDR1 {
        &self.sar_slave_addr1
    }
    #[doc = "0x44 - configure i2c slave address"]
    #[inline(always)]
    pub const fn sar_slave_addr2(&self) -> &SAR_SLAVE_ADDR2 {
        &self.sar_slave_addr2
    }
    #[doc = "0x48 - configure i2c slave address"]
    #[inline(always)]
    pub const fn sar_slave_addr3(&self) -> &SAR_SLAVE_ADDR3 {
        &self.sar_slave_addr3
    }
    #[doc = "0x4c - configure i2c slave address"]
    #[inline(always)]
    pub const fn sar_slave_addr4(&self) -> &SAR_SLAVE_ADDR4 {
        &self.sar_slave_addr4
    }
    #[doc = "0x50 - configure tsens controller"]
    #[inline(always)]
    pub const fn sar_tsens_ctrl(&self) -> &SAR_TSENS_CTRL {
        &self.sar_tsens_ctrl
    }
    #[doc = "0x54 - configure tsens controller"]
    #[inline(always)]
    pub const fn sar_tsens_ctrl2(&self) -> &SAR_TSENS_CTRL2 {
        &self.sar_tsens_ctrl2
    }
    #[doc = "0x58 - configure rtc i2c controller by sw"]
    #[inline(always)]
    pub const fn sar_i2c_ctrl(&self) -> &SAR_I2C_CTRL {
        &self.sar_i2c_ctrl
    }
    #[doc = "0x5c - configure touch controller"]
    #[inline(always)]
    pub const fn sar_touch_conf(&self) -> &SAR_TOUCH_CONF {
        &self.sar_touch_conf
    }
    #[doc = "0x60 - configure touch controller"]
    #[inline(always)]
    pub const fn sar_touch_denoise(&self) -> &SAR_TOUCH_DENOISE {
        &self.sar_touch_denoise
    }
    #[doc = "0x64 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres1(&self) -> &SAR_TOUCH_THRES1 {
        &self.sar_touch_thres1
    }
    #[doc = "0x68 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres2(&self) -> &SAR_TOUCH_THRES2 {
        &self.sar_touch_thres2
    }
    #[doc = "0x6c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres3(&self) -> &SAR_TOUCH_THRES3 {
        &self.sar_touch_thres3
    }
    #[doc = "0x70 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres4(&self) -> &SAR_TOUCH_THRES4 {
        &self.sar_touch_thres4
    }
    #[doc = "0x74 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres5(&self) -> &SAR_TOUCH_THRES5 {
        &self.sar_touch_thres5
    }
    #[doc = "0x78 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres6(&self) -> &SAR_TOUCH_THRES6 {
        &self.sar_touch_thres6
    }
    #[doc = "0x7c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres7(&self) -> &SAR_TOUCH_THRES7 {
        &self.sar_touch_thres7
    }
    #[doc = "0x80 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres8(&self) -> &SAR_TOUCH_THRES8 {
        &self.sar_touch_thres8
    }
    #[doc = "0x84 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres9(&self) -> &SAR_TOUCH_THRES9 {
        &self.sar_touch_thres9
    }
    #[doc = "0x88 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres10(&self) -> &SAR_TOUCH_THRES10 {
        &self.sar_touch_thres10
    }
    #[doc = "0x8c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres11(&self) -> &SAR_TOUCH_THRES11 {
        &self.sar_touch_thres11
    }
    #[doc = "0x90 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres12(&self) -> &SAR_TOUCH_THRES12 {
        &self.sar_touch_thres12
    }
    #[doc = "0x94 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres13(&self) -> &SAR_TOUCH_THRES13 {
        &self.sar_touch_thres13
    }
    #[doc = "0x98 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres14(&self) -> &SAR_TOUCH_THRES14 {
        &self.sar_touch_thres14
    }
    #[doc = "0x9c - Get touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_chn_st(&self) -> &SAR_TOUCH_CHN_ST {
        &self.sar_touch_chn_st
    }
    #[doc = "0xa0 - get touch scan status"]
    #[inline(always)]
    pub const fn sar_touch_status0(&self) -> &SAR_TOUCH_STATUS0 {
        &self.sar_touch_status0
    }
    #[doc = "0xa4 - touch channel status of touch pad 1"]
    #[inline(always)]
    pub const fn sar_touch_status1(&self) -> &SAR_TOUCH_STATUS1 {
        &self.sar_touch_status1
    }
    #[doc = "0xa8 - touch channel status of touch pad 2"]
    #[inline(always)]
    pub const fn sar_touch_status2(&self) -> &SAR_TOUCH_STATUS2 {
        &self.sar_touch_status2
    }
    #[doc = "0xac - touch channel status of touch pad 3"]
    #[inline(always)]
    pub const fn sar_touch_status3(&self) -> &SAR_TOUCH_STATUS3 {
        &self.sar_touch_status3
    }
    #[doc = "0xb0 - touch channel status of touch pad 4"]
    #[inline(always)]
    pub const fn sar_touch_status4(&self) -> &SAR_TOUCH_STATUS4 {
        &self.sar_touch_status4
    }
    #[doc = "0xb4 - touch channel status of touch pad 5"]
    #[inline(always)]
    pub const fn sar_touch_status5(&self) -> &SAR_TOUCH_STATUS5 {
        &self.sar_touch_status5
    }
    #[doc = "0xb8 - touch channel status of touch pad 6"]
    #[inline(always)]
    pub const fn sar_touch_status6(&self) -> &SAR_TOUCH_STATUS6 {
        &self.sar_touch_status6
    }
    #[doc = "0xbc - touch channel status of touch pad 7"]
    #[inline(always)]
    pub const fn sar_touch_status7(&self) -> &SAR_TOUCH_STATUS7 {
        &self.sar_touch_status7
    }
    #[doc = "0xc0 - touch channel status of touch pad 8"]
    #[inline(always)]
    pub const fn sar_touch_status8(&self) -> &SAR_TOUCH_STATUS8 {
        &self.sar_touch_status8
    }
    #[doc = "0xc4 - touch channel status of touch pad 9"]
    #[inline(always)]
    pub const fn sar_touch_status9(&self) -> &SAR_TOUCH_STATUS9 {
        &self.sar_touch_status9
    }
    #[doc = "0xc8 - touch channel status of touch pad 10"]
    #[inline(always)]
    pub const fn sar_touch_status10(&self) -> &SAR_TOUCH_STATUS10 {
        &self.sar_touch_status10
    }
    #[doc = "0xcc - touch channel status of touch pad 11"]
    #[inline(always)]
    pub const fn sar_touch_status11(&self) -> &SAR_TOUCH_STATUS11 {
        &self.sar_touch_status11
    }
    #[doc = "0xd0 - touch channel status of touch pad 12"]
    #[inline(always)]
    pub const fn sar_touch_status12(&self) -> &SAR_TOUCH_STATUS12 {
        &self.sar_touch_status12
    }
    #[doc = "0xd4 - touch channel status of touch pad 13"]
    #[inline(always)]
    pub const fn sar_touch_status13(&self) -> &SAR_TOUCH_STATUS13 {
        &self.sar_touch_status13
    }
    #[doc = "0xd8 - touch channel status of touch pad 14"]
    #[inline(always)]
    pub const fn sar_touch_status14(&self) -> &SAR_TOUCH_STATUS14 {
        &self.sar_touch_status14
    }
    #[doc = "0xdc - touch channel status of sleep pad"]
    #[inline(always)]
    pub const fn sar_touch_status15(&self) -> &SAR_TOUCH_STATUS15 {
        &self.sar_touch_status15
    }
    #[doc = "0xe0 - touch channel status of approach mode"]
    #[inline(always)]
    pub const fn sar_touch_status16(&self) -> &SAR_TOUCH_STATUS16 {
        &self.sar_touch_status16
    }
    #[doc = "0xe4 - get cocpu status"]
    #[inline(always)]
    pub const fn sar_cocpu_state(&self) -> &SAR_COCPU_STATE {
        &self.sar_cocpu_state
    }
    #[doc = "0xe8 - the interrupt raw of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_raw(&self) -> &SAR_COCPU_INT_RAW {
        &self.sar_cocpu_int_raw
    }
    #[doc = "0xec - the interrupt enable of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_ena(&self) -> &SAR_COCPU_INT_ENA {
        &self.sar_cocpu_int_ena
    }
    #[doc = "0xf0 - the interrupt state of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_st(&self) -> &SAR_COCPU_INT_ST {
        &self.sar_cocpu_int_st
    }
    #[doc = "0xf4 - the interrupt clear of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_clr(&self) -> &SAR_COCPU_INT_CLR {
        &self.sar_cocpu_int_clr
    }
    #[doc = "0xf8 - Ulp-riscv debug signal"]
    #[inline(always)]
    pub const fn sar_cocpu_debug(&self) -> &SAR_COCPU_DEBUG {
        &self.sar_cocpu_debug
    }
    #[doc = "0xfc - no public"]
    #[inline(always)]
    pub const fn sar_hall_ctrl(&self) -> &SAR_HALL_CTRL {
        &self.sar_hall_ctrl
    }
    #[doc = "0x100 - no public"]
    #[inline(always)]
    pub const fn sar_nouse(&self) -> &SAR_NOUSE {
        &self.sar_nouse
    }
    #[doc = "0x104 - the peri clock gate of rtc peri"]
    #[inline(always)]
    pub const fn sar_peri_clk_gate_conf(&self) -> &SAR_PERI_CLK_GATE_CONF {
        &self.sar_peri_clk_gate_conf
    }
    #[doc = "0x108 - the peri reset of rtc peri"]
    #[inline(always)]
    pub const fn sar_peri_reset_conf(&self) -> &SAR_PERI_RESET_CONF {
        &self.sar_peri_reset_conf
    }
    #[doc = "0x10c - the interrupt enable of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_ena_w1ts(&self) -> &SAR_COCPU_INT_ENA_W1TS {
        &self.sar_cocpu_int_ena_w1ts
    }
    #[doc = "0x110 - the interrupt enable clear of ulp"]
    #[inline(always)]
    pub const fn sar_cocpu_int_ena_w1tc(&self) -> &SAR_COCPU_INT_ENA_W1TC {
        &self.sar_cocpu_int_ena_w1tc
    }
    #[doc = "0x114 - rtc peri debug configure"]
    #[inline(always)]
    pub const fn sar_debug_conf(&self) -> &SAR_DEBUG_CONF {
        &self.sar_debug_conf
    }
    #[doc = "0x1fc - version"]
    #[inline(always)]
    pub const fn sar_sardate(&self) -> &SAR_SARDATE {
        &self.sar_sardate
    }
}
#[doc = "SAR_READER1_CTRL (rw) register accessor: configure saradc1 reader\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_reader1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_ctrl`] module"]
pub type SAR_READER1_CTRL = crate::Reg<sar_reader1_ctrl::SAR_READER1_CTRL_SPEC>;
#[doc = "configure saradc1 reader"]
pub mod sar_reader1_ctrl;
#[doc = "SAR_READER1_STATUS (r) register accessor: get saradc1 reader controller status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_status`] module"]
pub type SAR_READER1_STATUS = crate::Reg<sar_reader1_status::SAR_READER1_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader1_status;
#[doc = "SAR_MEAS1_CTRL1 (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl1`] module"]
pub type SAR_MEAS1_CTRL1 = crate::Reg<sar_meas1_ctrl1::SAR_MEAS1_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_meas1_ctrl1;
#[doc = "SAR_MEAS1_CTRL2 (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl2`] module"]
pub type SAR_MEAS1_CTRL2 = crate::Reg<sar_meas1_ctrl2::SAR_MEAS1_CTRL2_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_ctrl2;
#[doc = "SAR_MEAS1_MUX (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_mux`] module"]
pub type SAR_MEAS1_MUX = crate::Reg<sar_meas1_mux::SAR_MEAS1_MUX_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_mux;
#[doc = "SAR_ATTEN1 (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_atten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_atten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten1`] module"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_atten1;
#[doc = "SAR_AMP_CTRL1 (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_amp_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_amp_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl1`] module"]
pub type SAR_AMP_CTRL1 = crate::Reg<sar_amp_ctrl1::SAR_AMP_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl1;
#[doc = "SAR_AMP_CTRL2 (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_amp_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_amp_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl2`] module"]
pub type SAR_AMP_CTRL2 = crate::Reg<sar_amp_ctrl2::SAR_AMP_CTRL2_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl2;
#[doc = "SAR_AMP_CTRL3 (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_amp_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_amp_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl3`] module"]
pub type SAR_AMP_CTRL3 = crate::Reg<sar_amp_ctrl3::SAR_AMP_CTRL3_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl3;
#[doc = "SAR_READER2_CTRL (rw) register accessor: configure saradc2 reader\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_reader2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_ctrl`] module"]
pub type SAR_READER2_CTRL = crate::Reg<sar_reader2_ctrl::SAR_READER2_CTRL_SPEC>;
#[doc = "configure saradc2 reader"]
pub mod sar_reader2_ctrl;
#[doc = "SAR_READER2_STATUS (r) register accessor: get saradc1 reader controller status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_status`] module"]
pub type SAR_READER2_STATUS = crate::Reg<sar_reader2_status::SAR_READER2_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader2_status;
#[doc = "SAR_MEAS2_CTRL1 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl1`] module"]
pub type SAR_MEAS2_CTRL1 = crate::Reg<sar_meas2_ctrl1::SAR_MEAS2_CTRL1_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl1;
#[doc = "SAR_MEAS2_CTRL2 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl2`] module"]
pub type SAR_MEAS2_CTRL2 = crate::Reg<sar_meas2_ctrl2::SAR_MEAS2_CTRL2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl2;
#[doc = "SAR_MEAS2_MUX (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_mux`] module"]
pub type SAR_MEAS2_MUX = crate::Reg<sar_meas2_mux::SAR_MEAS2_MUX_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_mux;
#[doc = "SAR_ATTEN2 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_atten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_atten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten2`] module"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_atten2;
#[doc = "SAR_POWER_XPD_SAR (rw) register accessor: configure power of saradc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_power_xpd_sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_power_xpd_sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_power_xpd_sar`] module"]
pub type SAR_POWER_XPD_SAR = crate::Reg<sar_power_xpd_sar::SAR_POWER_XPD_SAR_SPEC>;
#[doc = "configure power of saradc"]
pub mod sar_power_xpd_sar;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr1`] module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr2`] module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr3`] module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr4`] module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: configure tsens controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl`] module"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl;
#[doc = "SAR_TSENS_CTRL2 (rw) register accessor: configure tsens controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl2`] module"]
pub type SAR_TSENS_CTRL2 = crate::Reg<sar_tsens_ctrl2::SAR_TSENS_CTRL2_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl2;
#[doc = "SAR_I2C_CTRL (rw) register accessor: configure rtc i2c controller by sw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_ctrl`] module"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = "configure rtc i2c controller by sw"]
pub mod sar_i2c_ctrl;
#[doc = "SAR_TOUCH_CONF (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_conf`] module"]
pub type SAR_TOUCH_CONF = crate::Reg<sar_touch_conf::SAR_TOUCH_CONF_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_conf;
#[doc = "SAR_TOUCH_DENOISE (r) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_denoise::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_denoise`] module"]
pub type SAR_TOUCH_DENOISE = crate::Reg<sar_touch_denoise::SAR_TOUCH_DENOISE_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_denoise;
#[doc = "SAR_TOUCH_THRES1 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres1`] module"]
pub type SAR_TOUCH_THRES1 = crate::Reg<sar_touch_thres1::SAR_TOUCH_THRES1_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres1;
#[doc = "SAR_TOUCH_THRES2 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres2`] module"]
pub type SAR_TOUCH_THRES2 = crate::Reg<sar_touch_thres2::SAR_TOUCH_THRES2_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres2;
#[doc = "SAR_TOUCH_THRES3 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres3`] module"]
pub type SAR_TOUCH_THRES3 = crate::Reg<sar_touch_thres3::SAR_TOUCH_THRES3_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres3;
#[doc = "SAR_TOUCH_THRES4 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres4`] module"]
pub type SAR_TOUCH_THRES4 = crate::Reg<sar_touch_thres4::SAR_TOUCH_THRES4_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres4;
#[doc = "SAR_TOUCH_THRES5 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres5`] module"]
pub type SAR_TOUCH_THRES5 = crate::Reg<sar_touch_thres5::SAR_TOUCH_THRES5_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres5;
#[doc = "SAR_TOUCH_THRES6 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres6`] module"]
pub type SAR_TOUCH_THRES6 = crate::Reg<sar_touch_thres6::SAR_TOUCH_THRES6_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres6;
#[doc = "SAR_TOUCH_THRES7 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres7`] module"]
pub type SAR_TOUCH_THRES7 = crate::Reg<sar_touch_thres7::SAR_TOUCH_THRES7_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres7;
#[doc = "SAR_TOUCH_THRES8 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres8`] module"]
pub type SAR_TOUCH_THRES8 = crate::Reg<sar_touch_thres8::SAR_TOUCH_THRES8_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres8;
#[doc = "SAR_TOUCH_THRES9 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres9`] module"]
pub type SAR_TOUCH_THRES9 = crate::Reg<sar_touch_thres9::SAR_TOUCH_THRES9_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres9;
#[doc = "SAR_TOUCH_THRES10 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres10`] module"]
pub type SAR_TOUCH_THRES10 = crate::Reg<sar_touch_thres10::SAR_TOUCH_THRES10_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres10;
#[doc = "SAR_TOUCH_THRES11 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres11`] module"]
pub type SAR_TOUCH_THRES11 = crate::Reg<sar_touch_thres11::SAR_TOUCH_THRES11_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres11;
#[doc = "SAR_TOUCH_THRES12 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres12`] module"]
pub type SAR_TOUCH_THRES12 = crate::Reg<sar_touch_thres12::SAR_TOUCH_THRES12_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres12;
#[doc = "SAR_TOUCH_THRES13 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres13`] module"]
pub type SAR_TOUCH_THRES13 = crate::Reg<sar_touch_thres13::SAR_TOUCH_THRES13_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres13;
#[doc = "SAR_TOUCH_THRES14 (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres14`] module"]
pub type SAR_TOUCH_THRES14 = crate::Reg<sar_touch_thres14::SAR_TOUCH_THRES14_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres14;
#[doc = "SAR_TOUCH_CHN_ST (rw) register accessor: Get touch channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_chn_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_chn_st`] module"]
pub type SAR_TOUCH_CHN_ST = crate::Reg<sar_touch_chn_st::SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Get touch channel status"]
pub mod sar_touch_chn_st;
#[doc = "SAR_TOUCH_STATUS0 (r) register accessor: get touch scan status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status0`] module"]
pub type SAR_TOUCH_STATUS0 = crate::Reg<sar_touch_status0::SAR_TOUCH_STATUS0_SPEC>;
#[doc = "get touch scan status"]
pub mod sar_touch_status0;
#[doc = "SAR_TOUCH_STATUS1 (r) register accessor: touch channel status of touch pad 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status1`] module"]
pub type SAR_TOUCH_STATUS1 = crate::Reg<sar_touch_status1::SAR_TOUCH_STATUS1_SPEC>;
#[doc = "touch channel status of touch pad 1"]
pub mod sar_touch_status1;
#[doc = "SAR_TOUCH_STATUS2 (r) register accessor: touch channel status of touch pad 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status2`] module"]
pub type SAR_TOUCH_STATUS2 = crate::Reg<sar_touch_status2::SAR_TOUCH_STATUS2_SPEC>;
#[doc = "touch channel status of touch pad 2"]
pub mod sar_touch_status2;
#[doc = "SAR_TOUCH_STATUS3 (r) register accessor: touch channel status of touch pad 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status3`] module"]
pub type SAR_TOUCH_STATUS3 = crate::Reg<sar_touch_status3::SAR_TOUCH_STATUS3_SPEC>;
#[doc = "touch channel status of touch pad 3"]
pub mod sar_touch_status3;
#[doc = "SAR_TOUCH_STATUS4 (r) register accessor: touch channel status of touch pad 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status4`] module"]
pub type SAR_TOUCH_STATUS4 = crate::Reg<sar_touch_status4::SAR_TOUCH_STATUS4_SPEC>;
#[doc = "touch channel status of touch pad 4"]
pub mod sar_touch_status4;
#[doc = "SAR_TOUCH_STATUS5 (r) register accessor: touch channel status of touch pad 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status5`] module"]
pub type SAR_TOUCH_STATUS5 = crate::Reg<sar_touch_status5::SAR_TOUCH_STATUS5_SPEC>;
#[doc = "touch channel status of touch pad 5"]
pub mod sar_touch_status5;
#[doc = "SAR_TOUCH_STATUS6 (r) register accessor: touch channel status of touch pad 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status6`] module"]
pub type SAR_TOUCH_STATUS6 = crate::Reg<sar_touch_status6::SAR_TOUCH_STATUS6_SPEC>;
#[doc = "touch channel status of touch pad 6"]
pub mod sar_touch_status6;
#[doc = "SAR_TOUCH_STATUS7 (r) register accessor: touch channel status of touch pad 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status7`] module"]
pub type SAR_TOUCH_STATUS7 = crate::Reg<sar_touch_status7::SAR_TOUCH_STATUS7_SPEC>;
#[doc = "touch channel status of touch pad 7"]
pub mod sar_touch_status7;
#[doc = "SAR_TOUCH_STATUS8 (r) register accessor: touch channel status of touch pad 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status8`] module"]
pub type SAR_TOUCH_STATUS8 = crate::Reg<sar_touch_status8::SAR_TOUCH_STATUS8_SPEC>;
#[doc = "touch channel status of touch pad 8"]
pub mod sar_touch_status8;
#[doc = "SAR_TOUCH_STATUS9 (r) register accessor: touch channel status of touch pad 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status9`] module"]
pub type SAR_TOUCH_STATUS9 = crate::Reg<sar_touch_status9::SAR_TOUCH_STATUS9_SPEC>;
#[doc = "touch channel status of touch pad 9"]
pub mod sar_touch_status9;
#[doc = "SAR_TOUCH_STATUS10 (r) register accessor: touch channel status of touch pad 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status10`] module"]
pub type SAR_TOUCH_STATUS10 = crate::Reg<sar_touch_status10::SAR_TOUCH_STATUS10_SPEC>;
#[doc = "touch channel status of touch pad 10"]
pub mod sar_touch_status10;
#[doc = "SAR_TOUCH_STATUS11 (r) register accessor: touch channel status of touch pad 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status11`] module"]
pub type SAR_TOUCH_STATUS11 = crate::Reg<sar_touch_status11::SAR_TOUCH_STATUS11_SPEC>;
#[doc = "touch channel status of touch pad 11"]
pub mod sar_touch_status11;
#[doc = "SAR_TOUCH_STATUS12 (r) register accessor: touch channel status of touch pad 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status12`] module"]
pub type SAR_TOUCH_STATUS12 = crate::Reg<sar_touch_status12::SAR_TOUCH_STATUS12_SPEC>;
#[doc = "touch channel status of touch pad 12"]
pub mod sar_touch_status12;
#[doc = "SAR_TOUCH_STATUS13 (r) register accessor: touch channel status of touch pad 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status13`] module"]
pub type SAR_TOUCH_STATUS13 = crate::Reg<sar_touch_status13::SAR_TOUCH_STATUS13_SPEC>;
#[doc = "touch channel status of touch pad 13"]
pub mod sar_touch_status13;
#[doc = "SAR_TOUCH_STATUS14 (r) register accessor: touch channel status of touch pad 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status14`] module"]
pub type SAR_TOUCH_STATUS14 = crate::Reg<sar_touch_status14::SAR_TOUCH_STATUS14_SPEC>;
#[doc = "touch channel status of touch pad 14"]
pub mod sar_touch_status14;
#[doc = "SAR_TOUCH_STATUS15 (r) register accessor: touch channel status of sleep pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status15`] module"]
pub type SAR_TOUCH_STATUS15 = crate::Reg<sar_touch_status15::SAR_TOUCH_STATUS15_SPEC>;
#[doc = "touch channel status of sleep pad"]
pub mod sar_touch_status15;
#[doc = "SAR_TOUCH_STATUS16 (r) register accessor: touch channel status of approach mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status16`] module"]
pub type SAR_TOUCH_STATUS16 = crate::Reg<sar_touch_status16::SAR_TOUCH_STATUS16_SPEC>;
#[doc = "touch channel status of approach mode"]
pub mod sar_touch_status16;
#[doc = "SAR_COCPU_STATE (rw) register accessor: get cocpu status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_state`] module"]
pub type SAR_COCPU_STATE = crate::Reg<sar_cocpu_state::SAR_COCPU_STATE_SPEC>;
#[doc = "get cocpu status"]
pub mod sar_cocpu_state;
#[doc = "SAR_COCPU_INT_RAW (r) register accessor: the interrupt raw of ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_raw`] module"]
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
#[doc = "the interrupt raw of ulp"]
pub mod sar_cocpu_int_raw;
#[doc = "SAR_COCPU_INT_ENA (rw) register accessor: the interrupt enable of ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena`] module"]
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena;
#[doc = "SAR_COCPU_INT_ST (r) register accessor: the interrupt state of ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_st`] module"]
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
#[doc = "the interrupt state of ulp"]
pub mod sar_cocpu_int_st;
#[doc = "SAR_COCPU_INT_CLR (w) register accessor: the interrupt clear of ulp\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_clr`] module"]
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
#[doc = "the interrupt clear of ulp"]
pub mod sar_cocpu_int_clr;
#[doc = "SAR_COCPU_DEBUG (r) register accessor: Ulp-riscv debug signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_debug::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_debug`] module"]
pub type SAR_COCPU_DEBUG = crate::Reg<sar_cocpu_debug::SAR_COCPU_DEBUG_SPEC>;
#[doc = "Ulp-riscv debug signal"]
pub mod sar_cocpu_debug;
#[doc = "SAR_HALL_CTRL (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_hall_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_hall_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_hall_ctrl`] module"]
pub type SAR_HALL_CTRL = crate::Reg<sar_hall_ctrl::SAR_HALL_CTRL_SPEC>;
#[doc = "no public"]
pub mod sar_hall_ctrl;
#[doc = "SAR_NOUSE (rw) register accessor: no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_nouse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_nouse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_nouse`] module"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = "no public"]
pub mod sar_nouse;
#[doc = "SAR_PERI_CLK_GATE_CONF (rw) register accessor: the peri clock gate of rtc peri\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_peri_clk_gate_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_peri_clk_gate_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_peri_clk_gate_conf`] module"]
pub type SAR_PERI_CLK_GATE_CONF = crate::Reg<sar_peri_clk_gate_conf::SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "the peri clock gate of rtc peri"]
pub mod sar_peri_clk_gate_conf;
#[doc = "SAR_PERI_RESET_CONF (rw) register accessor: the peri reset of rtc peri\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_peri_reset_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_peri_reset_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_peri_reset_conf`] module"]
pub type SAR_PERI_RESET_CONF = crate::Reg<sar_peri_reset_conf::SAR_PERI_RESET_CONF_SPEC>;
#[doc = "the peri reset of rtc peri"]
pub mod sar_peri_reset_conf;
#[doc = "SAR_COCPU_INT_ENA_W1TS (w) register accessor: the interrupt enable of ulp\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena_w1ts`] module"]
pub type SAR_COCPU_INT_ENA_W1TS = crate::Reg<sar_cocpu_int_ena_w1ts::SAR_COCPU_INT_ENA_W1TS_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena_w1ts;
#[doc = "SAR_COCPU_INT_ENA_W1TC (w) register accessor: the interrupt enable clear of ulp\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena_w1tc`] module"]
pub type SAR_COCPU_INT_ENA_W1TC = crate::Reg<sar_cocpu_int_ena_w1tc::SAR_COCPU_INT_ENA_W1TC_SPEC>;
#[doc = "the interrupt enable clear of ulp"]
pub mod sar_cocpu_int_ena_w1tc;
#[doc = "SAR_DEBUG_CONF (rw) register accessor: rtc peri debug configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_debug_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_debug_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_debug_conf`] module"]
pub type SAR_DEBUG_CONF = crate::Reg<sar_debug_conf::SAR_DEBUG_CONF_SPEC>;
#[doc = "rtc peri debug configure"]
pub mod sar_debug_conf;
#[doc = "SAR_SARDATE (rw) register accessor: version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_sardate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_sardate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_sardate`] module"]
pub type SAR_SARDATE = crate::Reg<sar_sardate::SAR_SARDATE_SPEC>;
#[doc = "version"]
pub mod sar_sardate;
