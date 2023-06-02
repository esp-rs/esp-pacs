#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - configure saradc1 reader"]
    pub sar_reader1_ctrl: SAR_READER1_CTRL,
    #[doc = "0x04 - get saradc1 reader controller status"]
    pub sar_reader1_status: SAR_READER1_STATUS,
    #[doc = "0x08 - no public"]
    pub sar_meas1_ctrl1: SAR_MEAS1_CTRL1,
    #[doc = "0x0c - configure saradc1 controller"]
    pub sar_meas1_ctrl2: SAR_MEAS1_CTRL2,
    #[doc = "0x10 - configure saradc1 controller"]
    pub sar_meas1_mux: SAR_MEAS1_MUX,
    #[doc = "0x14 - configure saradc1 controller"]
    pub sar_atten1: SAR_ATTEN1,
    #[doc = "0x18 - no public"]
    pub sar_amp_ctrl1: SAR_AMP_CTRL1,
    #[doc = "0x1c - no public"]
    pub sar_amp_ctrl2: SAR_AMP_CTRL2,
    #[doc = "0x20 - no public"]
    pub sar_amp_ctrl3: SAR_AMP_CTRL3,
    #[doc = "0x24 - configure saradc2 reader"]
    pub sar_reader2_ctrl: SAR_READER2_CTRL,
    #[doc = "0x28 - get saradc1 reader controller status"]
    pub sar_reader2_status: SAR_READER2_STATUS,
    #[doc = "0x2c - configure saradc2 controller"]
    pub sar_meas2_ctrl1: SAR_MEAS2_CTRL1,
    #[doc = "0x30 - configure saradc2 controller"]
    pub sar_meas2_ctrl2: SAR_MEAS2_CTRL2,
    #[doc = "0x34 - configure saradc2 controller"]
    pub sar_meas2_mux: SAR_MEAS2_MUX,
    #[doc = "0x38 - configure saradc2 controller"]
    pub sar_atten2: SAR_ATTEN2,
    #[doc = "0x3c - configure power of saradc"]
    pub sar_power_xpd_sar: SAR_POWER_XPD_SAR,
    #[doc = "0x40 - configure i2c slave address"]
    pub sar_slave_addr1: SAR_SLAVE_ADDR1,
    #[doc = "0x44 - configure i2c slave address"]
    pub sar_slave_addr2: SAR_SLAVE_ADDR2,
    #[doc = "0x48 - configure i2c slave address"]
    pub sar_slave_addr3: SAR_SLAVE_ADDR3,
    #[doc = "0x4c - configure i2c slave address"]
    pub sar_slave_addr4: SAR_SLAVE_ADDR4,
    #[doc = "0x50 - configure tsens controller"]
    pub sar_tsens_ctrl: SAR_TSENS_CTRL,
    #[doc = "0x54 - configure tsens controller"]
    pub sar_tsens_ctrl2: SAR_TSENS_CTRL2,
    #[doc = "0x58 - configure rtc i2c controller by sw"]
    pub sar_i2c_ctrl: SAR_I2C_CTRL,
    #[doc = "0x5c - configure touch controller"]
    pub sar_touch_conf: SAR_TOUCH_CONF,
    #[doc = "0x60 - configure touch controller"]
    pub sar_touch_denoise: SAR_TOUCH_DENOISE,
    #[doc = "0x64 - configure touch thres of touch pad"]
    pub sar_touch_thres1: SAR_TOUCH_THRES1,
    #[doc = "0x68 - configure touch thres of touch pad"]
    pub sar_touch_thres2: SAR_TOUCH_THRES2,
    #[doc = "0x6c - configure touch thres of touch pad"]
    pub sar_touch_thres3: SAR_TOUCH_THRES3,
    #[doc = "0x70 - configure touch thres of touch pad"]
    pub sar_touch_thres4: SAR_TOUCH_THRES4,
    #[doc = "0x74 - configure touch thres of touch pad"]
    pub sar_touch_thres5: SAR_TOUCH_THRES5,
    #[doc = "0x78 - configure touch thres of touch pad"]
    pub sar_touch_thres6: SAR_TOUCH_THRES6,
    #[doc = "0x7c - configure touch thres of touch pad"]
    pub sar_touch_thres7: SAR_TOUCH_THRES7,
    #[doc = "0x80 - configure touch thres of touch pad"]
    pub sar_touch_thres8: SAR_TOUCH_THRES8,
    #[doc = "0x84 - configure touch thres of touch pad"]
    pub sar_touch_thres9: SAR_TOUCH_THRES9,
    #[doc = "0x88 - configure touch thres of touch pad"]
    pub sar_touch_thres10: SAR_TOUCH_THRES10,
    #[doc = "0x8c - configure touch thres of touch pad"]
    pub sar_touch_thres11: SAR_TOUCH_THRES11,
    #[doc = "0x90 - configure touch thres of touch pad"]
    pub sar_touch_thres12: SAR_TOUCH_THRES12,
    #[doc = "0x94 - configure touch thres of touch pad"]
    pub sar_touch_thres13: SAR_TOUCH_THRES13,
    #[doc = "0x98 - configure touch thres of touch pad"]
    pub sar_touch_thres14: SAR_TOUCH_THRES14,
    #[doc = "0x9c - Get touch channel status"]
    pub sar_touch_chn_st: SAR_TOUCH_CHN_ST,
    #[doc = "0xa0 - get touch scan status"]
    pub sar_touch_status0: SAR_TOUCH_STATUS0,
    #[doc = "0xa4 - touch channel status of touch pad 1"]
    pub sar_touch_status1: SAR_TOUCH_STATUS1,
    #[doc = "0xa8 - touch channel status of touch pad 2"]
    pub sar_touch_status2: SAR_TOUCH_STATUS2,
    #[doc = "0xac - touch channel status of touch pad 3"]
    pub sar_touch_status3: SAR_TOUCH_STATUS3,
    #[doc = "0xb0 - touch channel status of touch pad 4"]
    pub sar_touch_status4: SAR_TOUCH_STATUS4,
    #[doc = "0xb4 - touch channel status of touch pad 5"]
    pub sar_touch_status5: SAR_TOUCH_STATUS5,
    #[doc = "0xb8 - touch channel status of touch pad 6"]
    pub sar_touch_status6: SAR_TOUCH_STATUS6,
    #[doc = "0xbc - touch channel status of touch pad 7"]
    pub sar_touch_status7: SAR_TOUCH_STATUS7,
    #[doc = "0xc0 - touch channel status of touch pad 8"]
    pub sar_touch_status8: SAR_TOUCH_STATUS8,
    #[doc = "0xc4 - touch channel status of touch pad 9"]
    pub sar_touch_status9: SAR_TOUCH_STATUS9,
    #[doc = "0xc8 - touch channel status of touch pad 10"]
    pub sar_touch_status10: SAR_TOUCH_STATUS10,
    #[doc = "0xcc - touch channel status of touch pad 11"]
    pub sar_touch_status11: SAR_TOUCH_STATUS11,
    #[doc = "0xd0 - touch channel status of touch pad 12"]
    pub sar_touch_status12: SAR_TOUCH_STATUS12,
    #[doc = "0xd4 - touch channel status of touch pad 13"]
    pub sar_touch_status13: SAR_TOUCH_STATUS13,
    #[doc = "0xd8 - touch channel status of touch pad 14"]
    pub sar_touch_status14: SAR_TOUCH_STATUS14,
    #[doc = "0xdc - touch channel status of sleep pad"]
    pub sar_touch_status15: SAR_TOUCH_STATUS15,
    #[doc = "0xe0 - touch channel status of approach mode"]
    pub sar_touch_status16: SAR_TOUCH_STATUS16,
    #[doc = "0xe4 - get cocpu status"]
    pub sar_cocpu_state: SAR_COCPU_STATE,
    #[doc = "0xe8 - the interrupt raw of ulp"]
    pub sar_cocpu_int_raw: SAR_COCPU_INT_RAW,
    #[doc = "0xec - the interrupt enable of ulp"]
    pub sar_cocpu_int_ena: SAR_COCPU_INT_ENA,
    #[doc = "0xf0 - the interrupt state of ulp"]
    pub sar_cocpu_int_st: SAR_COCPU_INT_ST,
    #[doc = "0xf4 - the interrupt clear of ulp"]
    pub sar_cocpu_int_clr: SAR_COCPU_INT_CLR,
    #[doc = "0xf8 - Ulp-riscv debug signal"]
    pub sar_cocpu_debug: SAR_COCPU_DEBUG,
    #[doc = "0xfc - no public"]
    pub sar_hall_ctrl: SAR_HALL_CTRL,
    #[doc = "0x100 - no public"]
    pub sar_nouse: SAR_NOUSE,
    #[doc = "0x104 - the peri clock gate of rtc peri"]
    pub sar_peri_clk_gate_conf: SAR_PERI_CLK_GATE_CONF,
    #[doc = "0x108 - the peri reset of rtc peri"]
    pub sar_peri_reset_conf: SAR_PERI_RESET_CONF,
    #[doc = "0x10c - the interrupt enable of ulp"]
    pub sar_cocpu_int_ena_w1ts: SAR_COCPU_INT_ENA_W1TS,
    #[doc = "0x110 - the interrupt enable clear of ulp"]
    pub sar_cocpu_int_ena_w1tc: SAR_COCPU_INT_ENA_W1TC,
    #[doc = "0x114 - rtc peri debug configure"]
    pub sar_debug_conf: SAR_DEBUG_CONF,
    _reserved70: [u8; 0xe4],
    #[doc = "0x1fc - version"]
    pub sar_sardate: SAR_SARDATE,
}
#[doc = "SAR_READER1_CTRL (rw) register accessor: an alias for `Reg<SAR_READER1_CTRL_SPEC>`"]
pub type SAR_READER1_CTRL = crate::Reg<sar_reader1_ctrl::SAR_READER1_CTRL_SPEC>;
#[doc = "configure saradc1 reader"]
pub mod sar_reader1_ctrl;
#[doc = "SAR_READER1_STATUS (r) register accessor: an alias for `Reg<SAR_READER1_STATUS_SPEC>`"]
pub type SAR_READER1_STATUS = crate::Reg<sar_reader1_status::SAR_READER1_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader1_status;
#[doc = "SAR_MEAS1_CTRL1 (rw) register accessor: an alias for `Reg<SAR_MEAS1_CTRL1_SPEC>`"]
pub type SAR_MEAS1_CTRL1 = crate::Reg<sar_meas1_ctrl1::SAR_MEAS1_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_meas1_ctrl1;
#[doc = "SAR_MEAS1_CTRL2 (rw) register accessor: an alias for `Reg<SAR_MEAS1_CTRL2_SPEC>`"]
pub type SAR_MEAS1_CTRL2 = crate::Reg<sar_meas1_ctrl2::SAR_MEAS1_CTRL2_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_ctrl2;
#[doc = "SAR_MEAS1_MUX (rw) register accessor: an alias for `Reg<SAR_MEAS1_MUX_SPEC>`"]
pub type SAR_MEAS1_MUX = crate::Reg<sar_meas1_mux::SAR_MEAS1_MUX_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_meas1_mux;
#[doc = "SAR_ATTEN1 (rw) register accessor: an alias for `Reg<SAR_ATTEN1_SPEC>`"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = "configure saradc1 controller"]
pub mod sar_atten1;
#[doc = "SAR_AMP_CTRL1 (rw) register accessor: an alias for `Reg<SAR_AMP_CTRL1_SPEC>`"]
pub type SAR_AMP_CTRL1 = crate::Reg<sar_amp_ctrl1::SAR_AMP_CTRL1_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl1;
#[doc = "SAR_AMP_CTRL2 (rw) register accessor: an alias for `Reg<SAR_AMP_CTRL2_SPEC>`"]
pub type SAR_AMP_CTRL2 = crate::Reg<sar_amp_ctrl2::SAR_AMP_CTRL2_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl2;
#[doc = "SAR_AMP_CTRL3 (rw) register accessor: an alias for `Reg<SAR_AMP_CTRL3_SPEC>`"]
pub type SAR_AMP_CTRL3 = crate::Reg<sar_amp_ctrl3::SAR_AMP_CTRL3_SPEC>;
#[doc = "no public"]
pub mod sar_amp_ctrl3;
#[doc = "SAR_READER2_CTRL (rw) register accessor: an alias for `Reg<SAR_READER2_CTRL_SPEC>`"]
pub type SAR_READER2_CTRL = crate::Reg<sar_reader2_ctrl::SAR_READER2_CTRL_SPEC>;
#[doc = "configure saradc2 reader"]
pub mod sar_reader2_ctrl;
#[doc = "SAR_READER2_STATUS (r) register accessor: an alias for `Reg<SAR_READER2_STATUS_SPEC>`"]
pub type SAR_READER2_STATUS = crate::Reg<sar_reader2_status::SAR_READER2_STATUS_SPEC>;
#[doc = "get saradc1 reader controller status"]
pub mod sar_reader2_status;
#[doc = "SAR_MEAS2_CTRL1 (rw) register accessor: an alias for `Reg<SAR_MEAS2_CTRL1_SPEC>`"]
pub type SAR_MEAS2_CTRL1 = crate::Reg<sar_meas2_ctrl1::SAR_MEAS2_CTRL1_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl1;
#[doc = "SAR_MEAS2_CTRL2 (rw) register accessor: an alias for `Reg<SAR_MEAS2_CTRL2_SPEC>`"]
pub type SAR_MEAS2_CTRL2 = crate::Reg<sar_meas2_ctrl2::SAR_MEAS2_CTRL2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_ctrl2;
#[doc = "SAR_MEAS2_MUX (rw) register accessor: an alias for `Reg<SAR_MEAS2_MUX_SPEC>`"]
pub type SAR_MEAS2_MUX = crate::Reg<sar_meas2_mux::SAR_MEAS2_MUX_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_meas2_mux;
#[doc = "SAR_ATTEN2 (rw) register accessor: an alias for `Reg<SAR_ATTEN2_SPEC>`"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = "configure saradc2 controller"]
pub mod sar_atten2;
#[doc = "SAR_POWER_XPD_SAR (rw) register accessor: an alias for `Reg<SAR_POWER_XPD_SAR_SPEC>`"]
pub type SAR_POWER_XPD_SAR = crate::Reg<sar_power_xpd_sar::SAR_POWER_XPD_SAR_SPEC>;
#[doc = "configure power of saradc"]
pub mod sar_power_xpd_sar;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR1_SPEC>`"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR2_SPEC>`"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR3_SPEC>`"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR4_SPEC>`"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = "configure i2c slave address"]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: an alias for `Reg<SAR_TSENS_CTRL_SPEC>`"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl;
#[doc = "SAR_TSENS_CTRL2 (rw) register accessor: an alias for `Reg<SAR_TSENS_CTRL2_SPEC>`"]
pub type SAR_TSENS_CTRL2 = crate::Reg<sar_tsens_ctrl2::SAR_TSENS_CTRL2_SPEC>;
#[doc = "configure tsens controller"]
pub mod sar_tsens_ctrl2;
#[doc = "SAR_I2C_CTRL (rw) register accessor: an alias for `Reg<SAR_I2C_CTRL_SPEC>`"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = "configure rtc i2c controller by sw"]
pub mod sar_i2c_ctrl;
#[doc = "SAR_TOUCH_CONF (rw) register accessor: an alias for `Reg<SAR_TOUCH_CONF_SPEC>`"]
pub type SAR_TOUCH_CONF = crate::Reg<sar_touch_conf::SAR_TOUCH_CONF_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_conf;
#[doc = "SAR_TOUCH_DENOISE (r) register accessor: an alias for `Reg<SAR_TOUCH_DENOISE_SPEC>`"]
pub type SAR_TOUCH_DENOISE = crate::Reg<sar_touch_denoise::SAR_TOUCH_DENOISE_SPEC>;
#[doc = "configure touch controller"]
pub mod sar_touch_denoise;
#[doc = "SAR_TOUCH_THRES1 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES1_SPEC>`"]
pub type SAR_TOUCH_THRES1 = crate::Reg<sar_touch_thres1::SAR_TOUCH_THRES1_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres1;
#[doc = "SAR_TOUCH_THRES2 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES2_SPEC>`"]
pub type SAR_TOUCH_THRES2 = crate::Reg<sar_touch_thres2::SAR_TOUCH_THRES2_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres2;
#[doc = "SAR_TOUCH_THRES3 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES3_SPEC>`"]
pub type SAR_TOUCH_THRES3 = crate::Reg<sar_touch_thres3::SAR_TOUCH_THRES3_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres3;
#[doc = "SAR_TOUCH_THRES4 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES4_SPEC>`"]
pub type SAR_TOUCH_THRES4 = crate::Reg<sar_touch_thres4::SAR_TOUCH_THRES4_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres4;
#[doc = "SAR_TOUCH_THRES5 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES5_SPEC>`"]
pub type SAR_TOUCH_THRES5 = crate::Reg<sar_touch_thres5::SAR_TOUCH_THRES5_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres5;
#[doc = "SAR_TOUCH_THRES6 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES6_SPEC>`"]
pub type SAR_TOUCH_THRES6 = crate::Reg<sar_touch_thres6::SAR_TOUCH_THRES6_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres6;
#[doc = "SAR_TOUCH_THRES7 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES7_SPEC>`"]
pub type SAR_TOUCH_THRES7 = crate::Reg<sar_touch_thres7::SAR_TOUCH_THRES7_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres7;
#[doc = "SAR_TOUCH_THRES8 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES8_SPEC>`"]
pub type SAR_TOUCH_THRES8 = crate::Reg<sar_touch_thres8::SAR_TOUCH_THRES8_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres8;
#[doc = "SAR_TOUCH_THRES9 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES9_SPEC>`"]
pub type SAR_TOUCH_THRES9 = crate::Reg<sar_touch_thres9::SAR_TOUCH_THRES9_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres9;
#[doc = "SAR_TOUCH_THRES10 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES10_SPEC>`"]
pub type SAR_TOUCH_THRES10 = crate::Reg<sar_touch_thres10::SAR_TOUCH_THRES10_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres10;
#[doc = "SAR_TOUCH_THRES11 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES11_SPEC>`"]
pub type SAR_TOUCH_THRES11 = crate::Reg<sar_touch_thres11::SAR_TOUCH_THRES11_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres11;
#[doc = "SAR_TOUCH_THRES12 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES12_SPEC>`"]
pub type SAR_TOUCH_THRES12 = crate::Reg<sar_touch_thres12::SAR_TOUCH_THRES12_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres12;
#[doc = "SAR_TOUCH_THRES13 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES13_SPEC>`"]
pub type SAR_TOUCH_THRES13 = crate::Reg<sar_touch_thres13::SAR_TOUCH_THRES13_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres13;
#[doc = "SAR_TOUCH_THRES14 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES14_SPEC>`"]
pub type SAR_TOUCH_THRES14 = crate::Reg<sar_touch_thres14::SAR_TOUCH_THRES14_SPEC>;
#[doc = "configure touch thres of touch pad"]
pub mod sar_touch_thres14;
#[doc = "SAR_TOUCH_CHN_ST (rw) register accessor: an alias for `Reg<SAR_TOUCH_CHN_ST_SPEC>`"]
pub type SAR_TOUCH_CHN_ST = crate::Reg<sar_touch_chn_st::SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Get touch channel status"]
pub mod sar_touch_chn_st;
#[doc = "SAR_TOUCH_STATUS0 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS0_SPEC>`"]
pub type SAR_TOUCH_STATUS0 = crate::Reg<sar_touch_status0::SAR_TOUCH_STATUS0_SPEC>;
#[doc = "get touch scan status"]
pub mod sar_touch_status0;
#[doc = "SAR_TOUCH_STATUS1 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS1_SPEC>`"]
pub type SAR_TOUCH_STATUS1 = crate::Reg<sar_touch_status1::SAR_TOUCH_STATUS1_SPEC>;
#[doc = "touch channel status of touch pad 1"]
pub mod sar_touch_status1;
#[doc = "SAR_TOUCH_STATUS2 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS2_SPEC>`"]
pub type SAR_TOUCH_STATUS2 = crate::Reg<sar_touch_status2::SAR_TOUCH_STATUS2_SPEC>;
#[doc = "touch channel status of touch pad 2"]
pub mod sar_touch_status2;
#[doc = "SAR_TOUCH_STATUS3 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS3_SPEC>`"]
pub type SAR_TOUCH_STATUS3 = crate::Reg<sar_touch_status3::SAR_TOUCH_STATUS3_SPEC>;
#[doc = "touch channel status of touch pad 3"]
pub mod sar_touch_status3;
#[doc = "SAR_TOUCH_STATUS4 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS4_SPEC>`"]
pub type SAR_TOUCH_STATUS4 = crate::Reg<sar_touch_status4::SAR_TOUCH_STATUS4_SPEC>;
#[doc = "touch channel status of touch pad 4"]
pub mod sar_touch_status4;
#[doc = "SAR_TOUCH_STATUS5 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS5_SPEC>`"]
pub type SAR_TOUCH_STATUS5 = crate::Reg<sar_touch_status5::SAR_TOUCH_STATUS5_SPEC>;
#[doc = "touch channel status of touch pad 5"]
pub mod sar_touch_status5;
#[doc = "SAR_TOUCH_STATUS6 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS6_SPEC>`"]
pub type SAR_TOUCH_STATUS6 = crate::Reg<sar_touch_status6::SAR_TOUCH_STATUS6_SPEC>;
#[doc = "touch channel status of touch pad 6"]
pub mod sar_touch_status6;
#[doc = "SAR_TOUCH_STATUS7 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS7_SPEC>`"]
pub type SAR_TOUCH_STATUS7 = crate::Reg<sar_touch_status7::SAR_TOUCH_STATUS7_SPEC>;
#[doc = "touch channel status of touch pad 7"]
pub mod sar_touch_status7;
#[doc = "SAR_TOUCH_STATUS8 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS8_SPEC>`"]
pub type SAR_TOUCH_STATUS8 = crate::Reg<sar_touch_status8::SAR_TOUCH_STATUS8_SPEC>;
#[doc = "touch channel status of touch pad 8"]
pub mod sar_touch_status8;
#[doc = "SAR_TOUCH_STATUS9 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS9_SPEC>`"]
pub type SAR_TOUCH_STATUS9 = crate::Reg<sar_touch_status9::SAR_TOUCH_STATUS9_SPEC>;
#[doc = "touch channel status of touch pad 9"]
pub mod sar_touch_status9;
#[doc = "SAR_TOUCH_STATUS10 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS10_SPEC>`"]
pub type SAR_TOUCH_STATUS10 = crate::Reg<sar_touch_status10::SAR_TOUCH_STATUS10_SPEC>;
#[doc = "touch channel status of touch pad 10"]
pub mod sar_touch_status10;
#[doc = "SAR_TOUCH_STATUS11 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS11_SPEC>`"]
pub type SAR_TOUCH_STATUS11 = crate::Reg<sar_touch_status11::SAR_TOUCH_STATUS11_SPEC>;
#[doc = "touch channel status of touch pad 11"]
pub mod sar_touch_status11;
#[doc = "SAR_TOUCH_STATUS12 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS12_SPEC>`"]
pub type SAR_TOUCH_STATUS12 = crate::Reg<sar_touch_status12::SAR_TOUCH_STATUS12_SPEC>;
#[doc = "touch channel status of touch pad 12"]
pub mod sar_touch_status12;
#[doc = "SAR_TOUCH_STATUS13 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS13_SPEC>`"]
pub type SAR_TOUCH_STATUS13 = crate::Reg<sar_touch_status13::SAR_TOUCH_STATUS13_SPEC>;
#[doc = "touch channel status of touch pad 13"]
pub mod sar_touch_status13;
#[doc = "SAR_TOUCH_STATUS14 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS14_SPEC>`"]
pub type SAR_TOUCH_STATUS14 = crate::Reg<sar_touch_status14::SAR_TOUCH_STATUS14_SPEC>;
#[doc = "touch channel status of touch pad 14"]
pub mod sar_touch_status14;
#[doc = "SAR_TOUCH_STATUS15 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS15_SPEC>`"]
pub type SAR_TOUCH_STATUS15 = crate::Reg<sar_touch_status15::SAR_TOUCH_STATUS15_SPEC>;
#[doc = "touch channel status of sleep pad"]
pub mod sar_touch_status15;
#[doc = "SAR_TOUCH_STATUS16 (r) register accessor: an alias for `Reg<SAR_TOUCH_STATUS16_SPEC>`"]
pub type SAR_TOUCH_STATUS16 = crate::Reg<sar_touch_status16::SAR_TOUCH_STATUS16_SPEC>;
#[doc = "touch channel status of approach mode"]
pub mod sar_touch_status16;
#[doc = "SAR_COCPU_STATE (rw) register accessor: an alias for `Reg<SAR_COCPU_STATE_SPEC>`"]
pub type SAR_COCPU_STATE = crate::Reg<sar_cocpu_state::SAR_COCPU_STATE_SPEC>;
#[doc = "get cocpu status"]
pub mod sar_cocpu_state;
#[doc = "SAR_COCPU_INT_RAW (r) register accessor: an alias for `Reg<SAR_COCPU_INT_RAW_SPEC>`"]
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
#[doc = "the interrupt raw of ulp"]
pub mod sar_cocpu_int_raw;
#[doc = "SAR_COCPU_INT_ENA (rw) register accessor: an alias for `Reg<SAR_COCPU_INT_ENA_SPEC>`"]
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena;
#[doc = "SAR_COCPU_INT_ST (r) register accessor: an alias for `Reg<SAR_COCPU_INT_ST_SPEC>`"]
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
#[doc = "the interrupt state of ulp"]
pub mod sar_cocpu_int_st;
#[doc = "SAR_COCPU_INT_CLR (w) register accessor: an alias for `Reg<SAR_COCPU_INT_CLR_SPEC>`"]
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
#[doc = "the interrupt clear of ulp"]
pub mod sar_cocpu_int_clr;
#[doc = "SAR_COCPU_DEBUG (r) register accessor: an alias for `Reg<SAR_COCPU_DEBUG_SPEC>`"]
pub type SAR_COCPU_DEBUG = crate::Reg<sar_cocpu_debug::SAR_COCPU_DEBUG_SPEC>;
#[doc = "Ulp-riscv debug signal"]
pub mod sar_cocpu_debug;
#[doc = "SAR_HALL_CTRL (rw) register accessor: an alias for `Reg<SAR_HALL_CTRL_SPEC>`"]
pub type SAR_HALL_CTRL = crate::Reg<sar_hall_ctrl::SAR_HALL_CTRL_SPEC>;
#[doc = "no public"]
pub mod sar_hall_ctrl;
#[doc = "SAR_NOUSE (rw) register accessor: an alias for `Reg<SAR_NOUSE_SPEC>`"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = "no public"]
pub mod sar_nouse;
#[doc = "SAR_PERI_CLK_GATE_CONF (rw) register accessor: an alias for `Reg<SAR_PERI_CLK_GATE_CONF_SPEC>`"]
pub type SAR_PERI_CLK_GATE_CONF = crate::Reg<sar_peri_clk_gate_conf::SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "the peri clock gate of rtc peri"]
pub mod sar_peri_clk_gate_conf;
#[doc = "SAR_PERI_RESET_CONF (rw) register accessor: an alias for `Reg<SAR_PERI_RESET_CONF_SPEC>`"]
pub type SAR_PERI_RESET_CONF = crate::Reg<sar_peri_reset_conf::SAR_PERI_RESET_CONF_SPEC>;
#[doc = "the peri reset of rtc peri"]
pub mod sar_peri_reset_conf;
#[doc = "SAR_COCPU_INT_ENA_W1TS (w) register accessor: an alias for `Reg<SAR_COCPU_INT_ENA_W1TS_SPEC>`"]
pub type SAR_COCPU_INT_ENA_W1TS = crate::Reg<sar_cocpu_int_ena_w1ts::SAR_COCPU_INT_ENA_W1TS_SPEC>;
#[doc = "the interrupt enable of ulp"]
pub mod sar_cocpu_int_ena_w1ts;
#[doc = "SAR_COCPU_INT_ENA_W1TC (w) register accessor: an alias for `Reg<SAR_COCPU_INT_ENA_W1TC_SPEC>`"]
pub type SAR_COCPU_INT_ENA_W1TC = crate::Reg<sar_cocpu_int_ena_w1tc::SAR_COCPU_INT_ENA_W1TC_SPEC>;
#[doc = "the interrupt enable clear of ulp"]
pub mod sar_cocpu_int_ena_w1tc;
#[doc = "SAR_DEBUG_CONF (rw) register accessor: an alias for `Reg<SAR_DEBUG_CONF_SPEC>`"]
pub type SAR_DEBUG_CONF = crate::Reg<sar_debug_conf::SAR_DEBUG_CONF_SPEC>;
#[doc = "rtc peri debug configure"]
pub mod sar_debug_conf;
#[doc = "SAR_SARDATE (rw) register accessor: an alias for `Reg<SAR_SARDATE_SPEC>`"]
pub type SAR_SARDATE = crate::Reg<sar_sardate::SAR_SARDATE_SPEC>;
#[doc = "version"]
pub mod sar_sardate;
