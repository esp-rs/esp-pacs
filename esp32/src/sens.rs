#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub sar_read_ctrl: SAR_READ_CTRL,
    #[doc = "0x04 - "]
    pub sar_read_status1: SAR_READ_STATUS1,
    #[doc = "0x08 - "]
    pub sar_meas_wait1: SAR_MEAS_WAIT1,
    #[doc = "0x0c - "]
    pub sar_meas_wait2: SAR_MEAS_WAIT2,
    #[doc = "0x10 - "]
    pub sar_meas_ctrl: SAR_MEAS_CTRL,
    #[doc = "0x14 - "]
    pub sar_read_status2: SAR_READ_STATUS2,
    #[doc = "0x18 - "]
    pub ulp_cp_sleep_cyc0: ULP_CP_SLEEP_CYC0,
    #[doc = "0x1c - "]
    pub ulp_cp_sleep_cyc1: ULP_CP_SLEEP_CYC1,
    #[doc = "0x20 - "]
    pub ulp_cp_sleep_cyc2: ULP_CP_SLEEP_CYC2,
    #[doc = "0x24 - "]
    pub ulp_cp_sleep_cyc3: ULP_CP_SLEEP_CYC3,
    #[doc = "0x28 - "]
    pub ulp_cp_sleep_cyc4: ULP_CP_SLEEP_CYC4,
    #[doc = "0x2c - "]
    pub sar_start_force: SAR_START_FORCE,
    #[doc = "0x30 - "]
    pub sar_mem_wr_ctrl: SAR_MEM_WR_CTRL,
    #[doc = "0x34 - "]
    pub sar_atten1: SAR_ATTEN1,
    #[doc = "0x38 - "]
    pub sar_atten2: SAR_ATTEN2,
    #[doc = "0x3c - "]
    pub sar_slave_addr1: SAR_SLAVE_ADDR1,
    #[doc = "0x40 - "]
    pub sar_slave_addr2: SAR_SLAVE_ADDR2,
    #[doc = "0x44 - "]
    pub sar_slave_addr3: SAR_SLAVE_ADDR3,
    #[doc = "0x48 - "]
    pub sar_slave_addr4: SAR_SLAVE_ADDR4,
    #[doc = "0x4c - "]
    pub sar_tsens_ctrl: SAR_TSENS_CTRL,
    #[doc = "0x50 - "]
    pub sar_i2c_ctrl: SAR_I2C_CTRL,
    #[doc = "0x54 - "]
    pub sar_meas_start1: SAR_MEAS_START1,
    #[doc = "0x58 - "]
    pub sar_touch_ctrl1: SAR_TOUCH_CTRL1,
    #[doc = "0x5c - "]
    pub sar_touch_thres1: SAR_TOUCH_THRES1,
    #[doc = "0x60 - "]
    pub sar_touch_thres2: SAR_TOUCH_THRES2,
    #[doc = "0x64 - "]
    pub sar_touch_thres3: SAR_TOUCH_THRES3,
    #[doc = "0x68 - "]
    pub sar_touch_thres4: SAR_TOUCH_THRES4,
    #[doc = "0x6c - "]
    pub sar_touch_thres5: SAR_TOUCH_THRES5,
    #[doc = "0x70 - "]
    pub sar_touch_out1: SAR_TOUCH_OUT1,
    #[doc = "0x74 - "]
    pub sar_touch_out2: SAR_TOUCH_OUT2,
    #[doc = "0x78 - "]
    pub sar_touch_out3: SAR_TOUCH_OUT3,
    #[doc = "0x7c - "]
    pub sar_touch_out4: SAR_TOUCH_OUT4,
    #[doc = "0x80 - "]
    pub sar_touch_out5: SAR_TOUCH_OUT5,
    #[doc = "0x84 - "]
    pub sar_touch_ctrl2: SAR_TOUCH_CTRL2,
    _reserved34: [u8; 0x04],
    #[doc = "0x8c - "]
    pub sar_touch_enable: SAR_TOUCH_ENABLE,
    #[doc = "0x90 - "]
    pub sar_read_ctrl2: SAR_READ_CTRL2,
    #[doc = "0x94 - "]
    pub sar_meas_start2: SAR_MEAS_START2,
    #[doc = "0x98 - "]
    pub sar_dac_ctrl1: SAR_DAC_CTRL1,
    #[doc = "0x9c - "]
    pub sar_dac_ctrl2: SAR_DAC_CTRL2,
    #[doc = "0xa0 - "]
    pub sar_meas_ctrl2: SAR_MEAS_CTRL2,
    _reserved40: [u8; 0x54],
    #[doc = "0xf8 - "]
    pub sar_nouse: SAR_NOUSE,
    #[doc = "0xfc - "]
    pub sardate: SARDATE,
}
#[doc = "SAR_READ_CTRL (rw) register accessor: an alias for `Reg<SAR_READ_CTRL_SPEC>`"]
pub type SAR_READ_CTRL = crate::Reg<sar_read_ctrl::SAR_READ_CTRL_SPEC>;
#[doc = ""]
pub mod sar_read_ctrl;
#[doc = "SAR_READ_STATUS1 (r) register accessor: an alias for `Reg<SAR_READ_STATUS1_SPEC>`"]
pub type SAR_READ_STATUS1 = crate::Reg<sar_read_status1::SAR_READ_STATUS1_SPEC>;
#[doc = ""]
pub mod sar_read_status1;
#[doc = "SAR_MEAS_WAIT1 (rw) register accessor: an alias for `Reg<SAR_MEAS_WAIT1_SPEC>`"]
pub type SAR_MEAS_WAIT1 = crate::Reg<sar_meas_wait1::SAR_MEAS_WAIT1_SPEC>;
#[doc = ""]
pub mod sar_meas_wait1;
#[doc = "SAR_MEAS_WAIT2 (rw) register accessor: an alias for `Reg<SAR_MEAS_WAIT2_SPEC>`"]
pub type SAR_MEAS_WAIT2 = crate::Reg<sar_meas_wait2::SAR_MEAS_WAIT2_SPEC>;
#[doc = ""]
pub mod sar_meas_wait2;
#[doc = "SAR_MEAS_CTRL (rw) register accessor: an alias for `Reg<SAR_MEAS_CTRL_SPEC>`"]
pub type SAR_MEAS_CTRL = crate::Reg<sar_meas_ctrl::SAR_MEAS_CTRL_SPEC>;
#[doc = ""]
pub mod sar_meas_ctrl;
#[doc = "SAR_READ_STATUS2 (r) register accessor: an alias for `Reg<SAR_READ_STATUS2_SPEC>`"]
pub type SAR_READ_STATUS2 = crate::Reg<sar_read_status2::SAR_READ_STATUS2_SPEC>;
#[doc = ""]
pub mod sar_read_status2;
#[doc = "ULP_CP_SLEEP_CYC0 (rw) register accessor: an alias for `Reg<ULP_CP_SLEEP_CYC0_SPEC>`"]
pub type ULP_CP_SLEEP_CYC0 = crate::Reg<ulp_cp_sleep_cyc0::ULP_CP_SLEEP_CYC0_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc0;
#[doc = "ULP_CP_SLEEP_CYC1 (rw) register accessor: an alias for `Reg<ULP_CP_SLEEP_CYC1_SPEC>`"]
pub type ULP_CP_SLEEP_CYC1 = crate::Reg<ulp_cp_sleep_cyc1::ULP_CP_SLEEP_CYC1_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc1;
#[doc = "ULP_CP_SLEEP_CYC2 (rw) register accessor: an alias for `Reg<ULP_CP_SLEEP_CYC2_SPEC>`"]
pub type ULP_CP_SLEEP_CYC2 = crate::Reg<ulp_cp_sleep_cyc2::ULP_CP_SLEEP_CYC2_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc2;
#[doc = "ULP_CP_SLEEP_CYC3 (rw) register accessor: an alias for `Reg<ULP_CP_SLEEP_CYC3_SPEC>`"]
pub type ULP_CP_SLEEP_CYC3 = crate::Reg<ulp_cp_sleep_cyc3::ULP_CP_SLEEP_CYC3_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc3;
#[doc = "ULP_CP_SLEEP_CYC4 (rw) register accessor: an alias for `Reg<ULP_CP_SLEEP_CYC4_SPEC>`"]
pub type ULP_CP_SLEEP_CYC4 = crate::Reg<ulp_cp_sleep_cyc4::ULP_CP_SLEEP_CYC4_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc4;
#[doc = "SAR_START_FORCE (rw) register accessor: an alias for `Reg<SAR_START_FORCE_SPEC>`"]
pub type SAR_START_FORCE = crate::Reg<sar_start_force::SAR_START_FORCE_SPEC>;
#[doc = ""]
pub mod sar_start_force;
#[doc = "SAR_MEM_WR_CTRL (rw) register accessor: an alias for `Reg<SAR_MEM_WR_CTRL_SPEC>`"]
pub type SAR_MEM_WR_CTRL = crate::Reg<sar_mem_wr_ctrl::SAR_MEM_WR_CTRL_SPEC>;
#[doc = ""]
pub mod sar_mem_wr_ctrl;
#[doc = "SAR_ATTEN1 (rw) register accessor: an alias for `Reg<SAR_ATTEN1_SPEC>`"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = ""]
pub mod sar_atten1;
#[doc = "SAR_ATTEN2 (rw) register accessor: an alias for `Reg<SAR_ATTEN2_SPEC>`"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = ""]
pub mod sar_atten2;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR1_SPEC>`"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = ""]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR2_SPEC>`"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = ""]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR3_SPEC>`"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = ""]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: an alias for `Reg<SAR_SLAVE_ADDR4_SPEC>`"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = ""]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: an alias for `Reg<SAR_TSENS_CTRL_SPEC>`"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = ""]
pub mod sar_tsens_ctrl;
#[doc = "SAR_I2C_CTRL (rw) register accessor: an alias for `Reg<SAR_I2C_CTRL_SPEC>`"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = ""]
pub mod sar_i2c_ctrl;
#[doc = "SAR_MEAS_START1 (rw) register accessor: an alias for `Reg<SAR_MEAS_START1_SPEC>`"]
pub type SAR_MEAS_START1 = crate::Reg<sar_meas_start1::SAR_MEAS_START1_SPEC>;
#[doc = ""]
pub mod sar_meas_start1;
#[doc = "SAR_TOUCH_CTRL1 (rw) register accessor: an alias for `Reg<SAR_TOUCH_CTRL1_SPEC>`"]
pub type SAR_TOUCH_CTRL1 = crate::Reg<sar_touch_ctrl1::SAR_TOUCH_CTRL1_SPEC>;
#[doc = ""]
pub mod sar_touch_ctrl1;
#[doc = "SAR_TOUCH_THRES1 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES1_SPEC>`"]
pub type SAR_TOUCH_THRES1 = crate::Reg<sar_touch_thres1::SAR_TOUCH_THRES1_SPEC>;
#[doc = ""]
pub mod sar_touch_thres1;
#[doc = "SAR_TOUCH_THRES2 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES2_SPEC>`"]
pub type SAR_TOUCH_THRES2 = crate::Reg<sar_touch_thres2::SAR_TOUCH_THRES2_SPEC>;
#[doc = ""]
pub mod sar_touch_thres2;
#[doc = "SAR_TOUCH_THRES3 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES3_SPEC>`"]
pub type SAR_TOUCH_THRES3 = crate::Reg<sar_touch_thres3::SAR_TOUCH_THRES3_SPEC>;
#[doc = ""]
pub mod sar_touch_thres3;
#[doc = "SAR_TOUCH_THRES4 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES4_SPEC>`"]
pub type SAR_TOUCH_THRES4 = crate::Reg<sar_touch_thres4::SAR_TOUCH_THRES4_SPEC>;
#[doc = ""]
pub mod sar_touch_thres4;
#[doc = "SAR_TOUCH_THRES5 (rw) register accessor: an alias for `Reg<SAR_TOUCH_THRES5_SPEC>`"]
pub type SAR_TOUCH_THRES5 = crate::Reg<sar_touch_thres5::SAR_TOUCH_THRES5_SPEC>;
#[doc = ""]
pub mod sar_touch_thres5;
#[doc = "SAR_TOUCH_OUT1 (r) register accessor: an alias for `Reg<SAR_TOUCH_OUT1_SPEC>`"]
pub type SAR_TOUCH_OUT1 = crate::Reg<sar_touch_out1::SAR_TOUCH_OUT1_SPEC>;
#[doc = ""]
pub mod sar_touch_out1;
#[doc = "SAR_TOUCH_OUT2 (r) register accessor: an alias for `Reg<SAR_TOUCH_OUT2_SPEC>`"]
pub type SAR_TOUCH_OUT2 = crate::Reg<sar_touch_out2::SAR_TOUCH_OUT2_SPEC>;
#[doc = ""]
pub mod sar_touch_out2;
#[doc = "SAR_TOUCH_OUT3 (r) register accessor: an alias for `Reg<SAR_TOUCH_OUT3_SPEC>`"]
pub type SAR_TOUCH_OUT3 = crate::Reg<sar_touch_out3::SAR_TOUCH_OUT3_SPEC>;
#[doc = ""]
pub mod sar_touch_out3;
#[doc = "SAR_TOUCH_OUT4 (r) register accessor: an alias for `Reg<SAR_TOUCH_OUT4_SPEC>`"]
pub type SAR_TOUCH_OUT4 = crate::Reg<sar_touch_out4::SAR_TOUCH_OUT4_SPEC>;
#[doc = ""]
pub mod sar_touch_out4;
#[doc = "SAR_TOUCH_OUT5 (r) register accessor: an alias for `Reg<SAR_TOUCH_OUT5_SPEC>`"]
pub type SAR_TOUCH_OUT5 = crate::Reg<sar_touch_out5::SAR_TOUCH_OUT5_SPEC>;
#[doc = ""]
pub mod sar_touch_out5;
#[doc = "SAR_TOUCH_CTRL2 (rw) register accessor: an alias for `Reg<SAR_TOUCH_CTRL2_SPEC>`"]
pub type SAR_TOUCH_CTRL2 = crate::Reg<sar_touch_ctrl2::SAR_TOUCH_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_touch_ctrl2;
#[doc = "SAR_TOUCH_ENABLE (rw) register accessor: an alias for `Reg<SAR_TOUCH_ENABLE_SPEC>`"]
pub type SAR_TOUCH_ENABLE = crate::Reg<sar_touch_enable::SAR_TOUCH_ENABLE_SPEC>;
#[doc = ""]
pub mod sar_touch_enable;
#[doc = "SAR_READ_CTRL2 (rw) register accessor: an alias for `Reg<SAR_READ_CTRL2_SPEC>`"]
pub type SAR_READ_CTRL2 = crate::Reg<sar_read_ctrl2::SAR_READ_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_read_ctrl2;
#[doc = "SAR_MEAS_START2 (rw) register accessor: an alias for `Reg<SAR_MEAS_START2_SPEC>`"]
pub type SAR_MEAS_START2 = crate::Reg<sar_meas_start2::SAR_MEAS_START2_SPEC>;
#[doc = ""]
pub mod sar_meas_start2;
#[doc = "SAR_DAC_CTRL1 (rw) register accessor: an alias for `Reg<SAR_DAC_CTRL1_SPEC>`"]
pub type SAR_DAC_CTRL1 = crate::Reg<sar_dac_ctrl1::SAR_DAC_CTRL1_SPEC>;
#[doc = ""]
pub mod sar_dac_ctrl1;
#[doc = "SAR_DAC_CTRL2 (rw) register accessor: an alias for `Reg<SAR_DAC_CTRL2_SPEC>`"]
pub type SAR_DAC_CTRL2 = crate::Reg<sar_dac_ctrl2::SAR_DAC_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_dac_ctrl2;
#[doc = "SAR_MEAS_CTRL2 (rw) register accessor: an alias for `Reg<SAR_MEAS_CTRL2_SPEC>`"]
pub type SAR_MEAS_CTRL2 = crate::Reg<sar_meas_ctrl2::SAR_MEAS_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_meas_ctrl2;
#[doc = "SAR_NOUSE (rw) register accessor: an alias for `Reg<SAR_NOUSE_SPEC>`"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = ""]
pub mod sar_nouse;
#[doc = "SARDATE (rw) register accessor: an alias for `Reg<SARDATE_SPEC>`"]
pub type SARDATE = crate::Reg<sardate::SARDATE_SPEC>;
#[doc = ""]
pub mod sardate;
