#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar_read_ctrl: SAR_READ_CTRL,
    sar_read_status1: SAR_READ_STATUS1,
    sar_meas_wait1: SAR_MEAS_WAIT1,
    sar_meas_wait2: SAR_MEAS_WAIT2,
    sar_meas_ctrl: SAR_MEAS_CTRL,
    sar_read_status2: SAR_READ_STATUS2,
    ulp_cp_sleep_cyc0: ULP_CP_SLEEP_CYC0,
    ulp_cp_sleep_cyc1: ULP_CP_SLEEP_CYC1,
    ulp_cp_sleep_cyc2: ULP_CP_SLEEP_CYC2,
    ulp_cp_sleep_cyc3: ULP_CP_SLEEP_CYC3,
    ulp_cp_sleep_cyc4: ULP_CP_SLEEP_CYC4,
    sar_start_force: SAR_START_FORCE,
    sar_mem_wr_ctrl: SAR_MEM_WR_CTRL,
    sar_atten1: SAR_ATTEN1,
    sar_atten2: SAR_ATTEN2,
    sar_slave_addr1: SAR_SLAVE_ADDR1,
    sar_slave_addr2: SAR_SLAVE_ADDR2,
    sar_slave_addr3: SAR_SLAVE_ADDR3,
    sar_slave_addr4: SAR_SLAVE_ADDR4,
    sar_tsens_ctrl: SAR_TSENS_CTRL,
    sar_i2c_ctrl: SAR_I2C_CTRL,
    sar_meas_start1: SAR_MEAS_START1,
    sar_touch_ctrl1: SAR_TOUCH_CTRL1,
    sar_touch_thres: [SAR_TOUCH_THRES; 5],
    sar_touch_out: [SAR_TOUCH_OUT; 5],
    sar_touch_ctrl2: SAR_TOUCH_CTRL2,
    _reserved26: [u8; 0x04],
    sar_touch_enable: SAR_TOUCH_ENABLE,
    sar_read_ctrl2: SAR_READ_CTRL2,
    sar_meas_start2: SAR_MEAS_START2,
    sar_dac_ctrl1: SAR_DAC_CTRL1,
    sar_dac_ctrl2: SAR_DAC_CTRL2,
    sar_meas_ctrl2: SAR_MEAS_CTRL2,
    _reserved32: [u8; 0x54],
    sar_nouse: SAR_NOUSE,
    sardate: SARDATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sar_read_ctrl(&self) -> &SAR_READ_CTRL {
        &self.sar_read_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn sar_read_status1(&self) -> &SAR_READ_STATUS1 {
        &self.sar_read_status1
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sar_meas_wait1(&self) -> &SAR_MEAS_WAIT1 {
        &self.sar_meas_wait1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sar_meas_wait2(&self) -> &SAR_MEAS_WAIT2 {
        &self.sar_meas_wait2
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sar_meas_ctrl(&self) -> &SAR_MEAS_CTRL {
        &self.sar_meas_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sar_read_status2(&self) -> &SAR_READ_STATUS2 {
        &self.sar_read_status2
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ulp_cp_sleep_cyc0(&self) -> &ULP_CP_SLEEP_CYC0 {
        &self.ulp_cp_sleep_cyc0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ulp_cp_sleep_cyc1(&self) -> &ULP_CP_SLEEP_CYC1 {
        &self.ulp_cp_sleep_cyc1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn ulp_cp_sleep_cyc2(&self) -> &ULP_CP_SLEEP_CYC2 {
        &self.ulp_cp_sleep_cyc2
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn ulp_cp_sleep_cyc3(&self) -> &ULP_CP_SLEEP_CYC3 {
        &self.ulp_cp_sleep_cyc3
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn ulp_cp_sleep_cyc4(&self) -> &ULP_CP_SLEEP_CYC4 {
        &self.ulp_cp_sleep_cyc4
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn sar_start_force(&self) -> &SAR_START_FORCE {
        &self.sar_start_force
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn sar_mem_wr_ctrl(&self) -> &SAR_MEM_WR_CTRL {
        &self.sar_mem_wr_ctrl
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sar_atten1(&self) -> &SAR_ATTEN1 {
        &self.sar_atten1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn sar_atten2(&self) -> &SAR_ATTEN2 {
        &self.sar_atten2
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn sar_slave_addr1(&self) -> &SAR_SLAVE_ADDR1 {
        &self.sar_slave_addr1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn sar_slave_addr2(&self) -> &SAR_SLAVE_ADDR2 {
        &self.sar_slave_addr2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn sar_slave_addr3(&self) -> &SAR_SLAVE_ADDR3 {
        &self.sar_slave_addr3
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn sar_slave_addr4(&self) -> &SAR_SLAVE_ADDR4 {
        &self.sar_slave_addr4
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sar_tsens_ctrl(&self) -> &SAR_TSENS_CTRL {
        &self.sar_tsens_ctrl
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn sar_i2c_ctrl(&self) -> &SAR_I2C_CTRL {
        &self.sar_i2c_ctrl
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn sar_meas_start1(&self) -> &SAR_MEAS_START1 {
        &self.sar_meas_start1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn sar_touch_ctrl1(&self) -> &SAR_TOUCH_CTRL1 {
        &self.sar_touch_ctrl1
    }
    #[doc = "0x5c..0x70 - "]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `SAR_TOUCH_THRES1` register.</div>"]
    #[inline(always)]
    pub const fn sar_touch_thres(&self, n: usize) -> &SAR_TOUCH_THRES {
        &self.sar_touch_thres[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x70 - "]
    #[inline(always)]
    pub fn sar_touch_thres_iter(&self) -> impl Iterator<Item = &SAR_TOUCH_THRES> {
        self.sar_touch_thres.iter()
    }
    #[doc = "0x5c - SAR_TOUCH_THRES1"]
    #[inline(always)]
    pub const fn sar_touch_thres1(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(0)
    }
    #[doc = "0x60 - SAR_TOUCH_THRES2"]
    #[inline(always)]
    pub const fn sar_touch_thres2(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(1)
    }
    #[doc = "0x64 - SAR_TOUCH_THRES3"]
    #[inline(always)]
    pub const fn sar_touch_thres3(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(2)
    }
    #[doc = "0x68 - SAR_TOUCH_THRES4"]
    #[inline(always)]
    pub const fn sar_touch_thres4(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(3)
    }
    #[doc = "0x6c - SAR_TOUCH_THRES5"]
    #[inline(always)]
    pub const fn sar_touch_thres5(&self) -> &SAR_TOUCH_THRES {
        self.sar_touch_thres(4)
    }
    #[doc = "0x70..0x84 - "]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `SAR_TOUCH_OUT1` register.</div>"]
    #[inline(always)]
    pub const fn sar_touch_out(&self, n: usize) -> &SAR_TOUCH_OUT {
        &self.sar_touch_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x84 - "]
    #[inline(always)]
    pub fn sar_touch_out_iter(&self) -> impl Iterator<Item = &SAR_TOUCH_OUT> {
        self.sar_touch_out.iter()
    }
    #[doc = "0x70 - SAR_TOUCH_OUT1"]
    #[inline(always)]
    pub const fn sar_touch_out1(&self) -> &SAR_TOUCH_OUT {
        self.sar_touch_out(0)
    }
    #[doc = "0x74 - SAR_TOUCH_OUT2"]
    #[inline(always)]
    pub const fn sar_touch_out2(&self) -> &SAR_TOUCH_OUT {
        self.sar_touch_out(1)
    }
    #[doc = "0x78 - SAR_TOUCH_OUT3"]
    #[inline(always)]
    pub const fn sar_touch_out3(&self) -> &SAR_TOUCH_OUT {
        self.sar_touch_out(2)
    }
    #[doc = "0x7c - SAR_TOUCH_OUT4"]
    #[inline(always)]
    pub const fn sar_touch_out4(&self) -> &SAR_TOUCH_OUT {
        self.sar_touch_out(3)
    }
    #[doc = "0x80 - SAR_TOUCH_OUT5"]
    #[inline(always)]
    pub const fn sar_touch_out5(&self) -> &SAR_TOUCH_OUT {
        self.sar_touch_out(4)
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn sar_touch_ctrl2(&self) -> &SAR_TOUCH_CTRL2 {
        &self.sar_touch_ctrl2
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn sar_touch_enable(&self) -> &SAR_TOUCH_ENABLE {
        &self.sar_touch_enable
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn sar_read_ctrl2(&self) -> &SAR_READ_CTRL2 {
        &self.sar_read_ctrl2
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn sar_meas_start2(&self) -> &SAR_MEAS_START2 {
        &self.sar_meas_start2
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn sar_dac_ctrl1(&self) -> &SAR_DAC_CTRL1 {
        &self.sar_dac_ctrl1
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn sar_dac_ctrl2(&self) -> &SAR_DAC_CTRL2 {
        &self.sar_dac_ctrl2
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn sar_meas_ctrl2(&self) -> &SAR_MEAS_CTRL2 {
        &self.sar_meas_ctrl2
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn sar_nouse(&self) -> &SAR_NOUSE {
        &self.sar_nouse
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn sardate(&self) -> &SARDATE {
        &self.sardate
    }
}
#[doc = "SAR_READ_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_read_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_read_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_read_ctrl`] module"]
pub type SAR_READ_CTRL = crate::Reg<sar_read_ctrl::SAR_READ_CTRL_SPEC>;
#[doc = ""]
pub mod sar_read_ctrl;
#[doc = "SAR_READ_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_read_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_read_status1`] module"]
pub type SAR_READ_STATUS1 = crate::Reg<sar_read_status1::SAR_READ_STATUS1_SPEC>;
#[doc = ""]
pub mod sar_read_status1;
#[doc = "SAR_MEAS_WAIT1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_wait1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_wait1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_wait1`] module"]
pub type SAR_MEAS_WAIT1 = crate::Reg<sar_meas_wait1::SAR_MEAS_WAIT1_SPEC>;
#[doc = ""]
pub mod sar_meas_wait1;
#[doc = "SAR_MEAS_WAIT2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_wait2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_wait2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_wait2`] module"]
pub type SAR_MEAS_WAIT2 = crate::Reg<sar_meas_wait2::SAR_MEAS_WAIT2_SPEC>;
#[doc = ""]
pub mod sar_meas_wait2;
#[doc = "SAR_MEAS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_ctrl`] module"]
pub type SAR_MEAS_CTRL = crate::Reg<sar_meas_ctrl::SAR_MEAS_CTRL_SPEC>;
#[doc = ""]
pub mod sar_meas_ctrl;
#[doc = "SAR_READ_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_read_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_read_status2`] module"]
pub type SAR_READ_STATUS2 = crate::Reg<sar_read_status2::SAR_READ_STATUS2_SPEC>;
#[doc = ""]
pub mod sar_read_status2;
#[doc = "ULP_CP_SLEEP_CYC0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_sleep_cyc0`] module"]
pub type ULP_CP_SLEEP_CYC0 = crate::Reg<ulp_cp_sleep_cyc0::ULP_CP_SLEEP_CYC0_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc0;
#[doc = "ULP_CP_SLEEP_CYC1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_sleep_cyc1`] module"]
pub type ULP_CP_SLEEP_CYC1 = crate::Reg<ulp_cp_sleep_cyc1::ULP_CP_SLEEP_CYC1_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc1;
#[doc = "ULP_CP_SLEEP_CYC2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_sleep_cyc2`] module"]
pub type ULP_CP_SLEEP_CYC2 = crate::Reg<ulp_cp_sleep_cyc2::ULP_CP_SLEEP_CYC2_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc2;
#[doc = "ULP_CP_SLEEP_CYC3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_sleep_cyc3`] module"]
pub type ULP_CP_SLEEP_CYC3 = crate::Reg<ulp_cp_sleep_cyc3::ULP_CP_SLEEP_CYC3_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc3;
#[doc = "ULP_CP_SLEEP_CYC4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_sleep_cyc4`] module"]
pub type ULP_CP_SLEEP_CYC4 = crate::Reg<ulp_cp_sleep_cyc4::ULP_CP_SLEEP_CYC4_SPEC>;
#[doc = ""]
pub mod ulp_cp_sleep_cyc4;
#[doc = "SAR_START_FORCE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_start_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_start_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_start_force`] module"]
pub type SAR_START_FORCE = crate::Reg<sar_start_force::SAR_START_FORCE_SPEC>;
#[doc = ""]
pub mod sar_start_force;
#[doc = "SAR_MEM_WR_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_mem_wr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_mem_wr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_mem_wr_ctrl`] module"]
pub type SAR_MEM_WR_CTRL = crate::Reg<sar_mem_wr_ctrl::SAR_MEM_WR_CTRL_SPEC>;
#[doc = ""]
pub mod sar_mem_wr_ctrl;
#[doc = "SAR_ATTEN1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten1`] module"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = ""]
pub mod sar_atten1;
#[doc = "SAR_ATTEN2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten2`] module"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = ""]
pub mod sar_atten2;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr1`] module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = ""]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr2`] module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = ""]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr3`] module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = ""]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr4`] module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = ""]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl`] module"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = ""]
pub mod sar_tsens_ctrl;
#[doc = "SAR_I2C_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_ctrl`] module"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = ""]
pub mod sar_i2c_ctrl;
#[doc = "SAR_MEAS_START1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_start1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_start1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_start1`] module"]
pub type SAR_MEAS_START1 = crate::Reg<sar_meas_start1::SAR_MEAS_START1_SPEC>;
#[doc = ""]
pub mod sar_meas_start1;
#[doc = "SAR_TOUCH_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_ctrl1`] module"]
pub type SAR_TOUCH_CTRL1 = crate::Reg<sar_touch_ctrl1::SAR_TOUCH_CTRL1_SPEC>;
#[doc = ""]
pub mod sar_touch_ctrl1;
#[doc = "SAR_TOUCH_THRES (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres`] module"]
pub type SAR_TOUCH_THRES = crate::Reg<sar_touch_thres::SAR_TOUCH_THRES_SPEC>;
#[doc = ""]
pub mod sar_touch_thres;
#[doc = "SAR_TOUCH_OUT (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_out`] module"]
pub type SAR_TOUCH_OUT = crate::Reg<sar_touch_out::SAR_TOUCH_OUT_SPEC>;
#[doc = ""]
pub mod sar_touch_out;
#[doc = "SAR_TOUCH_CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_ctrl2`] module"]
pub type SAR_TOUCH_CTRL2 = crate::Reg<sar_touch_ctrl2::SAR_TOUCH_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_touch_ctrl2;
#[doc = "SAR_TOUCH_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_enable`] module"]
pub type SAR_TOUCH_ENABLE = crate::Reg<sar_touch_enable::SAR_TOUCH_ENABLE_SPEC>;
#[doc = ""]
pub mod sar_touch_enable;
#[doc = "SAR_READ_CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_read_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_read_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_read_ctrl2`] module"]
pub type SAR_READ_CTRL2 = crate::Reg<sar_read_ctrl2::SAR_READ_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_read_ctrl2;
#[doc = "SAR_MEAS_START2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_start2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_start2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_start2`] module"]
pub type SAR_MEAS_START2 = crate::Reg<sar_meas_start2::SAR_MEAS_START2_SPEC>;
#[doc = ""]
pub mod sar_meas_start2;
#[doc = "SAR_DAC_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_dac_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_dac_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_dac_ctrl1`] module"]
pub type SAR_DAC_CTRL1 = crate::Reg<sar_dac_ctrl1::SAR_DAC_CTRL1_SPEC>;
#[doc = ""]
pub mod sar_dac_ctrl1;
#[doc = "SAR_DAC_CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_dac_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_dac_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_dac_ctrl2`] module"]
pub type SAR_DAC_CTRL2 = crate::Reg<sar_dac_ctrl2::SAR_DAC_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_dac_ctrl2;
#[doc = "SAR_MEAS_CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas_ctrl2`] module"]
pub type SAR_MEAS_CTRL2 = crate::Reg<sar_meas_ctrl2::SAR_MEAS_CTRL2_SPEC>;
#[doc = ""]
pub mod sar_meas_ctrl2;
#[doc = "SAR_NOUSE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_nouse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_nouse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_nouse`] module"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = ""]
pub mod sar_nouse;
#[doc = "SARDATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sardate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sardate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sardate`] module"]
pub type SARDATE = crate::Reg<sardate::SARDATE_SPEC>;
#[doc = ""]
pub mod sardate;
