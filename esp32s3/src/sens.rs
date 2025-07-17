#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    sar_touch_thres: [SAR_TOUCH_THRES; 14],
    sar_touch_chn_st: SAR_TOUCH_CHN_ST,
    sar_touch_scan_status: SAR_TOUCH_SCAN_STATUS,
    sar_touch_status: [SAR_TOUCH_STATUS; 15],
    sar_touch_appr_status: SAR_TOUCH_APPR_STATUS,
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
    _reserved43: [u8; 0xe4],
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
    #[doc = "0x64..0x9c - configure touch thres of touch pad"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `SAR_TOUCH_THRES1` register.</div>"]
    #[inline(always)]
    pub const fn sar_touch_thres(&self, n: usize) -> &SAR_TOUCH_THRES {
        &self.sar_touch_thres[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x9c - configure touch thres of touch pad"]
    #[inline(always)]
    pub fn sar_touch_thres_iter(&self) -> impl Iterator<Item = &SAR_TOUCH_THRES> {
        self.sar_touch_thres.iter()
    }
    #[doc = "0x64 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres1(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(0)
    }
    #[doc = "0x68 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres2(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(1)
    }
    #[doc = "0x6c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres3(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(2)
    }
    #[doc = "0x70 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres4(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(3)
    }
    #[doc = "0x74 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres5(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(4)
    }
    #[doc = "0x78 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres6(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(5)
    }
    #[doc = "0x7c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres7(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(6)
    }
    #[doc = "0x80 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres8(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(7)
    }
    #[doc = "0x84 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres9(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(8)
    }
    #[doc = "0x88 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres10(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(9)
    }
    #[doc = "0x8c - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres11(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(10)
    }
    #[doc = "0x90 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres12(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(11)
    }
    #[doc = "0x94 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres13(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(12)
    }
    #[doc = "0x98 - configure touch thres of touch pad"]
    #[inline(always)]
    pub const fn sar_touch_thres14(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(13)
    }
    #[doc = "0x9c - Get touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_chn_st(&self) -> &SAR_TOUCH_CHN_ST {
        &self.sar_touch_chn_st
    }
    #[doc = "0xa0 - get touch scan status"]
    #[inline(always)]
    pub const fn sar_touch_scan_status(&self) -> &SAR_TOUCH_SCAN_STATUS {
        &self.sar_touch_scan_status
    }
    #[doc = "0xa4..0xe0 - Touch channel status"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `SAR_TOUCH_STATUS1` register.</div>"]
    #[inline(always)]
    pub const fn sar_touch_status(&self, n: usize) -> &SAR_TOUCH_STATUS {
        &self.sar_touch_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa4..0xe0 - Touch channel status"]
    #[inline(always)]
    pub fn sar_touch_status_iter(&self) -> impl Iterator<Item = &SAR_TOUCH_STATUS> {
        self.sar_touch_status.iter()
    }
    #[doc = "0xa4 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status1(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(0)
    }
    #[doc = "0xa8 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status2(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(1)
    }
    #[doc = "0xac - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status3(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(2)
    }
    #[doc = "0xb0 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status4(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(3)
    }
    #[doc = "0xb4 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status5(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(4)
    }
    #[doc = "0xb8 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status6(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(5)
    }
    #[doc = "0xbc - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status7(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(6)
    }
    #[doc = "0xc0 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status8(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(7)
    }
    #[doc = "0xc4 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status9(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(8)
    }
    #[doc = "0xc8 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status10(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(9)
    }
    #[doc = "0xcc - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status11(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(10)
    }
    #[doc = "0xd0 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status12(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(11)
    }
    #[doc = "0xd4 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status13(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(12)
    }
    #[doc = "0xd8 - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status14(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(13)
    }
    #[doc = "0xdc - Touch channel status"]
    #[inline(always)]
    pub const fn sar_touch_status15(&self) -> &SAR_TOUCH_STATUS {
        self.sar_touch_status(14)
    }
    #[doc = "0xe0 - touch channel status of approach mode"]
    #[inline(always)]
    pub const fn sar_touch_appr_status(&self) -> &SAR_TOUCH_APPR_STATUS {
        &self.sar_touch_appr_status
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
#[doc = "SAR_READER1_CTRL (rw) register accessor: configure saradc1 reader\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_ctrl`] module"]
pub type SAR_READER1_CTRL = crate::Reg<sar_reader1_ctrl::SAR_READER1_CTRL_SPEC>;
#[doc = "configure saradc1 reader"]
pub mod sar_reader1_ctrl;
#[doc = "SAR_READER1_STATUS (r) register accessor: get saradc1 reader controller status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_status`] module"]
pub type SAR_READER1_STATUS = crate::Reg<sar_reader1_status::SAR_READER1_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader1_status;
#[doc = "SAR_MEAS1_CTRL1 (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl1`] module"]
pub type SAR_MEAS1_CTRL1 = crate::Reg<sar_meas1_ctrl1::SAR_MEAS1_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_meas1_ctrl1;
#[doc = "SAR_MEAS1_CTRL2 (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl2`] module"]
pub type SAR_MEAS1_CTRL2 = crate::Reg<sar_meas1_ctrl2::SAR_MEAS1_CTRL2_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_ctrl2;
#[doc = "SAR_MEAS1_MUX (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_mux`] module"]
pub type SAR_MEAS1_MUX = crate::Reg<sar_meas1_mux::SAR_MEAS1_MUX_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_mux;
#[doc = "SAR_ATTEN1 (rw) register accessor: configure saradc1 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten1`] module"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_atten1;
#[doc = "SAR_AMP_CTRL1 (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl1`] module"]
pub type SAR_AMP_CTRL1 = crate::Reg<sar_amp_ctrl1::SAR_AMP_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl1;
#[doc = "SAR_AMP_CTRL2 (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl2`] module"]
pub type SAR_AMP_CTRL2 = crate::Reg<sar_amp_ctrl2::SAR_AMP_CTRL2_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl2;
#[doc = "SAR_AMP_CTRL3 (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl3`] module"]
pub type SAR_AMP_CTRL3 = crate::Reg<sar_amp_ctrl3::SAR_AMP_CTRL3_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl3;
#[doc = "SAR_READER2_CTRL (rw) register accessor: configure saradc2 reader\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_ctrl`] module"]
pub type SAR_READER2_CTRL = crate::Reg<sar_reader2_ctrl::SAR_READER2_CTRL_SPEC>;
#[doc = "configure saradc2 reader"]
pub mod sar_reader2_ctrl;
#[doc = "SAR_READER2_STATUS (r) register accessor: get saradc1 reader controller status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_status`] module"]
pub type SAR_READER2_STATUS = crate::Reg<sar_reader2_status::SAR_READER2_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader2_status;
#[doc = "SAR_MEAS2_CTRL1 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl1`] module"]
pub type SAR_MEAS2_CTRL1 = crate::Reg<sar_meas2_ctrl1::SAR_MEAS2_CTRL1_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl1;
#[doc = "SAR_MEAS2_CTRL2 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl2`] module"]
pub type SAR_MEAS2_CTRL2 = crate::Reg<sar_meas2_ctrl2::SAR_MEAS2_CTRL2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl2;
#[doc = "SAR_MEAS2_MUX (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_mux`] module"]
pub type SAR_MEAS2_MUX = crate::Reg<sar_meas2_mux::SAR_MEAS2_MUX_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_mux;
#[doc = "SAR_ATTEN2 (rw) register accessor: configure saradc2 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten2`] module"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_atten2;
#[doc = "SAR_POWER_XPD_SAR (rw) register accessor: configure power of saradc\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_power_xpd_sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_power_xpd_sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_power_xpd_sar`] module"]
pub type SAR_POWER_XPD_SAR = crate::Reg<sar_power_xpd_sar::SAR_POWER_XPD_SAR_SPEC>;
#[doc = "configure power of saradc"]
pub mod sar_power_xpd_sar;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr1`] module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr2`] module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr3`] module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr4`] module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: configure tsens controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl`] module"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl;
#[doc = "SAR_TSENS_CTRL2 (rw) register accessor: configure tsens controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl2`] module"]
pub type SAR_TSENS_CTRL2 = crate::Reg<sar_tsens_ctrl2::SAR_TSENS_CTRL2_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl2;
#[doc = "SAR_I2C_CTRL (rw) register accessor: configure rtc i2c controller by sw\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_ctrl`] module"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = "configure rtc i2c controller by sw"]
pub mod sar_i2c_ctrl;
#[doc = "SAR_TOUCH_CONF (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_conf`] module"]
pub type SAR_TOUCH_CONF = crate::Reg<sar_touch_conf::SAR_TOUCH_CONF_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_conf;
#[doc = "SAR_TOUCH_DENOISE (r) register accessor: configure touch controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_denoise::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_denoise`] module"]
pub type SAR_TOUCH_DENOISE = crate::Reg<sar_touch_denoise::SAR_TOUCH_DENOISE_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_denoise;
#[doc = "SAR_TOUCH_THRES (rw) register accessor: configure touch thres of touch pad\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres`] module"]
pub type SAR_TOUCH_THRES = crate::Reg<sar_touch_thres::SAR_TOUCH_THRES_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres;
#[doc = "SAR_TOUCH_CHN_ST (rw) register accessor: Get touch channel status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_chn_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_chn_st`] module"]
pub type SAR_TOUCH_CHN_ST = crate::Reg<sar_touch_chn_st::SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Get touch channel status"]
pub mod sar_touch_chn_st;
#[doc = "SAR_TOUCH_SCAN_STATUS (r) register accessor: get touch scan status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_scan_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_scan_status`] module"]
pub type SAR_TOUCH_SCAN_STATUS = crate::Reg<sar_touch_scan_status::SAR_TOUCH_SCAN_STATUS_SPEC>;
#[doc = "get touch scan status"]
pub mod sar_touch_scan_status;
#[doc = "SAR_TOUCH_STATUS (r) register accessor: Touch channel status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status`] module"]
pub type SAR_TOUCH_STATUS = crate::Reg<sar_touch_status::SAR_TOUCH_STATUS_SPEC>;
#[doc = "Touch channel status"]
pub mod sar_touch_status;
#[doc = "SAR_TOUCH_APPR_STATUS (r) register accessor: touch channel status of approach mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_appr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_appr_status`] module"]
pub type SAR_TOUCH_APPR_STATUS = crate::Reg<sar_touch_appr_status::SAR_TOUCH_APPR_STATUS_SPEC>;
#[doc = "touch channel status of approach mode"]
pub mod sar_touch_appr_status;
#[doc = "SAR_COCPU_STATE (rw) register accessor: get cocpu status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_state`] module"]
pub type SAR_COCPU_STATE = crate::Reg<sar_cocpu_state::SAR_COCPU_STATE_SPEC>;
#[doc = "get cocpu status"]
pub mod sar_cocpu_state;
#[doc = "SAR_COCPU_INT_RAW (r) register accessor: the interrupt raw of ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_raw`] module"]
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
#[doc = "the interrupt raw of ulp"]
pub mod sar_cocpu_int_raw;
#[doc = "SAR_COCPU_INT_ENA (rw) register accessor: the interrupt enable of ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena`] module"]
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena;
#[doc = "SAR_COCPU_INT_ST (r) register accessor: the interrupt state of ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_st`] module"]
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
#[doc = "the interrupt state of ulp"]
pub mod sar_cocpu_int_st;
#[doc = "SAR_COCPU_INT_CLR (w) register accessor: the interrupt clear of ulp\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_clr`] module"]
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
#[doc = "the interrupt clear of ulp"]
pub mod sar_cocpu_int_clr;
#[doc = "SAR_COCPU_DEBUG (r) register accessor: Ulp-riscv debug signal\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_debug`] module"]
pub type SAR_COCPU_DEBUG = crate::Reg<sar_cocpu_debug::SAR_COCPU_DEBUG_SPEC>;
#[doc = "Ulp-riscv debug signal"]
pub mod sar_cocpu_debug;
#[doc = "SAR_HALL_CTRL (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_hall_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_hall_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_hall_ctrl`] module"]
pub type SAR_HALL_CTRL = crate::Reg<sar_hall_ctrl::SAR_HALL_CTRL_SPEC>;
#[doc = "no public"]
pub mod sar_hall_ctrl;
#[doc = "SAR_NOUSE (rw) register accessor: no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_nouse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_nouse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_nouse`] module"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = "no public"]
pub mod sar_nouse;
#[doc = "SAR_PERI_CLK_GATE_CONF (rw) register accessor: the peri clock gate of rtc peri\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_peri_clk_gate_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_peri_clk_gate_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_peri_clk_gate_conf`] module"]
pub type SAR_PERI_CLK_GATE_CONF = crate::Reg<sar_peri_clk_gate_conf::SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "the peri clock gate of rtc peri"]
pub mod sar_peri_clk_gate_conf;
#[doc = "SAR_PERI_RESET_CONF (rw) register accessor: the peri reset of rtc peri\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_peri_reset_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_peri_reset_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_peri_reset_conf`] module"]
pub type SAR_PERI_RESET_CONF = crate::Reg<sar_peri_reset_conf::SAR_PERI_RESET_CONF_SPEC>;
#[doc = "the peri reset of rtc peri"]
pub mod sar_peri_reset_conf;
#[doc = "SAR_COCPU_INT_ENA_W1TS (w) register accessor: the interrupt enable of ulp\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena_w1ts`] module"]
pub type SAR_COCPU_INT_ENA_W1TS = crate::Reg<sar_cocpu_int_ena_w1ts::SAR_COCPU_INT_ENA_W1TS_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena_w1ts;
#[doc = "SAR_COCPU_INT_ENA_W1TC (w) register accessor: the interrupt enable clear of ulp\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena_w1tc`] module"]
pub type SAR_COCPU_INT_ENA_W1TC = crate::Reg<sar_cocpu_int_ena_w1tc::SAR_COCPU_INT_ENA_W1TC_SPEC>;
#[doc = "the interrupt enable clear of ulp"]
pub mod sar_cocpu_int_ena_w1tc;
#[doc = "SAR_DEBUG_CONF (rw) register accessor: rtc peri debug configure\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_debug_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_debug_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_debug_conf`] module"]
pub type SAR_DEBUG_CONF = crate::Reg<sar_debug_conf::SAR_DEBUG_CONF_SPEC>;
#[doc = "rtc peri debug configure"]
pub mod sar_debug_conf;
#[doc = "SAR_SARDATE (rw) register accessor: version\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_sardate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_sardate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_sardate`] module"]
pub type SAR_SARDATE = crate::Reg<sar_sardate::SAR_SARDATE_SPEC>;
#[doc = "version"]
pub mod sar_sardate;
