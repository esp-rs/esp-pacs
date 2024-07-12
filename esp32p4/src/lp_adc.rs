#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    reader1_ctrl: READER1_CTRL,
    reader1_status: READER1_STATUS,
    meas1_ctrl1: MEAS1_CTRL1,
    meas1_ctrl2: MEAS1_CTRL2,
    meas1_mux: MEAS1_MUX,
    atten1: ATTEN1,
    amp_ctrl1: AMP_CTRL1,
    amp_ctrl2: AMP_CTRL2,
    amp_ctrl3: AMP_CTRL3,
    reader2_ctrl: READER2_CTRL,
    reader2_status: READER2_STATUS,
    meas2_ctrl1: MEAS2_CTRL1,
    meas2_ctrl2: MEAS2_CTRL2,
    meas2_mux: MEAS2_MUX,
    atten2: ATTEN2,
    force_wpd_sar: FORCE_WPD_SAR,
    meas_status: MEAS_STATUS,
    reg_clken: REG_CLKEN,
    cocpu_int_raw: COCPU_INT_RAW,
    int_ena: INT_ENA,
    int_st: INT_ST,
    int_clr: INT_CLR,
    int_ena_w1ts: INT_ENA_W1TS,
    int_ena_w1tc: INT_ENA_W1TC,
    wakeup1: WAKEUP1,
    wakeup2: WAKEUP2,
    wakeup_sel: WAKEUP_SEL,
    sar1_hw_wakeup: SAR1_HW_WAKEUP,
    sar2_hw_wakeup: SAR2_HW_WAKEUP,
    rnd_eco_low: RND_ECO_LOW,
    rnd_eco_high: RND_ECO_HIGH,
    rnd_eco_cs: RND_ECO_CS,
}
impl RegisterBlock {
    #[doc = "0x00 - Control the read operation of ADC1."]
    #[inline(always)]
    pub const fn reader1_ctrl(&self) -> &READER1_CTRL {
        &self.reader1_ctrl
    }
    #[doc = "0x04 - N/A"]
    #[inline(always)]
    pub const fn reader1_status(&self) -> &READER1_STATUS {
        &self.reader1_status
    }
    #[doc = "0x08 - N/A"]
    #[inline(always)]
    pub const fn meas1_ctrl1(&self) -> &MEAS1_CTRL1 {
        &self.meas1_ctrl1
    }
    #[doc = "0x0c - ADC1 configuration registers."]
    #[inline(always)]
    pub const fn meas1_ctrl2(&self) -> &MEAS1_CTRL2 {
        &self.meas1_ctrl2
    }
    #[doc = "0x10 - SAR ADC1 MUX register."]
    #[inline(always)]
    pub const fn meas1_mux(&self) -> &MEAS1_MUX {
        &self.meas1_mux
    }
    #[doc = "0x14 - ADC1 attenuation registers."]
    #[inline(always)]
    pub const fn atten1(&self) -> &ATTEN1 {
        &self.atten1
    }
    #[doc = "0x18 - N/A"]
    #[inline(always)]
    pub const fn amp_ctrl1(&self) -> &AMP_CTRL1 {
        &self.amp_ctrl1
    }
    #[doc = "0x1c - N/A"]
    #[inline(always)]
    pub const fn amp_ctrl2(&self) -> &AMP_CTRL2 {
        &self.amp_ctrl2
    }
    #[doc = "0x20 - N/A"]
    #[inline(always)]
    pub const fn amp_ctrl3(&self) -> &AMP_CTRL3 {
        &self.amp_ctrl3
    }
    #[doc = "0x24 - Control the read operation of ADC2."]
    #[inline(always)]
    pub const fn reader2_ctrl(&self) -> &READER2_CTRL {
        &self.reader2_ctrl
    }
    #[doc = "0x28 - N/A"]
    #[inline(always)]
    pub const fn reader2_status(&self) -> &READER2_STATUS {
        &self.reader2_status
    }
    #[doc = "0x2c - ADC2 configuration registers."]
    #[inline(always)]
    pub const fn meas2_ctrl1(&self) -> &MEAS2_CTRL1 {
        &self.meas2_ctrl1
    }
    #[doc = "0x30 - ADC2 configuration registers."]
    #[inline(always)]
    pub const fn meas2_ctrl2(&self) -> &MEAS2_CTRL2 {
        &self.meas2_ctrl2
    }
    #[doc = "0x34 - SAR ADC2 MUX register."]
    #[inline(always)]
    pub const fn meas2_mux(&self) -> &MEAS2_MUX {
        &self.meas2_mux
    }
    #[doc = "0x38 - ADC1 attenuation registers."]
    #[inline(always)]
    pub const fn atten2(&self) -> &ATTEN2 {
        &self.atten2
    }
    #[doc = "0x3c - In sleep, force to use rtc to control ADC"]
    #[inline(always)]
    pub const fn force_wpd_sar(&self) -> &FORCE_WPD_SAR {
        &self.force_wpd_sar
    }
    #[doc = "0x40 - N/A"]
    #[inline(always)]
    pub const fn meas_status(&self) -> &MEAS_STATUS {
        &self.meas_status
    }
    #[doc = "0x44 - N/A"]
    #[inline(always)]
    pub const fn reg_clken(&self) -> &REG_CLKEN {
        &self.reg_clken
    }
    #[doc = "0x48 - Interrupt raw registers."]
    #[inline(always)]
    pub const fn cocpu_int_raw(&self) -> &COCPU_INT_RAW {
        &self.cocpu_int_raw
    }
    #[doc = "0x4c - Interrupt enable registers."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x50 - Interrupt status registers."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x54 - Interrupt clear registers."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x58 - Interrupt enable assert registers."]
    #[inline(always)]
    pub const fn int_ena_w1ts(&self) -> &INT_ENA_W1TS {
        &self.int_ena_w1ts
    }
    #[doc = "0x5c - Interrupt enable deassert registers."]
    #[inline(always)]
    pub const fn int_ena_w1tc(&self) -> &INT_ENA_W1TC {
        &self.int_ena_w1tc
    }
    #[doc = "0x60 - ADC1 wakeup configuration registers."]
    #[inline(always)]
    pub const fn wakeup1(&self) -> &WAKEUP1 {
        &self.wakeup1
    }
    #[doc = "0x64 - ADC2 wakeup configuration registers."]
    #[inline(always)]
    pub const fn wakeup2(&self) -> &WAKEUP2 {
        &self.wakeup2
    }
    #[doc = "0x68 - Wakeup source select register."]
    #[inline(always)]
    pub const fn wakeup_sel(&self) -> &WAKEUP_SEL {
        &self.wakeup_sel
    }
    #[doc = "0x6c - Hardware automatic sampling registers for wakeup function."]
    #[inline(always)]
    pub const fn sar1_hw_wakeup(&self) -> &SAR1_HW_WAKEUP {
        &self.sar1_hw_wakeup
    }
    #[doc = "0x70 - Hardware automatic sampling registers for wakeup function."]
    #[inline(always)]
    pub const fn sar2_hw_wakeup(&self) -> &SAR2_HW_WAKEUP {
        &self.sar2_hw_wakeup
    }
    #[doc = "0x74 - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    #[doc = "0x78 - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
    #[doc = "0x7c - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RND_ECO_CS {
        &self.rnd_eco_cs
    }
}
#[doc = "READER1_CTRL (rw) register accessor: Control the read operation of ADC1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reader1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reader1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reader1_ctrl`] module"]
pub type READER1_CTRL = crate::Reg<reader1_ctrl::READER1_CTRL_SPEC>;
#[doc = "Control the read operation of ADC1."]
pub mod reader1_ctrl;
#[doc = "READER1_STATUS (r) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`reader1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reader1_status`] module"]
pub type READER1_STATUS = crate::Reg<reader1_status::READER1_STATUS_SPEC>;
#[doc = "N/A"]
pub mod reader1_status;
#[doc = "MEAS1_CTRL1 (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`meas1_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas1_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas1_ctrl1`] module"]
pub type MEAS1_CTRL1 = crate::Reg<meas1_ctrl1::MEAS1_CTRL1_SPEC>;
#[doc = "N/A"]
pub mod meas1_ctrl1;
#[doc = "MEAS1_CTRL2 (rw) register accessor: ADC1 configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas1_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas1_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas1_ctrl2`] module"]
pub type MEAS1_CTRL2 = crate::Reg<meas1_ctrl2::MEAS1_CTRL2_SPEC>;
#[doc = "ADC1 configuration registers."]
pub mod meas1_ctrl2;
#[doc = "MEAS1_MUX (rw) register accessor: SAR ADC1 MUX register.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas1_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas1_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas1_mux`] module"]
pub type MEAS1_MUX = crate::Reg<meas1_mux::MEAS1_MUX_SPEC>;
#[doc = "SAR ADC1 MUX register."]
pub mod meas1_mux;
#[doc = "ATTEN1 (rw) register accessor: ADC1 attenuation registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`atten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atten1`] module"]
pub type ATTEN1 = crate::Reg<atten1::ATTEN1_SPEC>;
#[doc = "ADC1 attenuation registers."]
pub mod atten1;
#[doc = "AMP_CTRL1 (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amp_ctrl1`] module"]
pub type AMP_CTRL1 = crate::Reg<amp_ctrl1::AMP_CTRL1_SPEC>;
#[doc = "N/A"]
pub mod amp_ctrl1;
#[doc = "AMP_CTRL2 (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amp_ctrl2`] module"]
pub type AMP_CTRL2 = crate::Reg<amp_ctrl2::AMP_CTRL2_SPEC>;
#[doc = "N/A"]
pub mod amp_ctrl2;
#[doc = "AMP_CTRL3 (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amp_ctrl3`] module"]
pub type AMP_CTRL3 = crate::Reg<amp_ctrl3::AMP_CTRL3_SPEC>;
#[doc = "N/A"]
pub mod amp_ctrl3;
#[doc = "READER2_CTRL (rw) register accessor: Control the read operation of ADC2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reader2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reader2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reader2_ctrl`] module"]
pub type READER2_CTRL = crate::Reg<reader2_ctrl::READER2_CTRL_SPEC>;
#[doc = "Control the read operation of ADC2."]
pub mod reader2_ctrl;
#[doc = "READER2_STATUS (r) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`reader2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reader2_status`] module"]
pub type READER2_STATUS = crate::Reg<reader2_status::READER2_STATUS_SPEC>;
#[doc = "N/A"]
pub mod reader2_status;
#[doc = "MEAS2_CTRL1 (rw) register accessor: ADC2 configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas2_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas2_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas2_ctrl1`] module"]
pub type MEAS2_CTRL1 = crate::Reg<meas2_ctrl1::MEAS2_CTRL1_SPEC>;
#[doc = "ADC2 configuration registers."]
pub mod meas2_ctrl1;
#[doc = "MEAS2_CTRL2 (rw) register accessor: ADC2 configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas2_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas2_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas2_ctrl2`] module"]
pub type MEAS2_CTRL2 = crate::Reg<meas2_ctrl2::MEAS2_CTRL2_SPEC>;
#[doc = "ADC2 configuration registers."]
pub mod meas2_ctrl2;
#[doc = "MEAS2_MUX (rw) register accessor: SAR ADC2 MUX register.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas2_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas2_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas2_mux`] module"]
pub type MEAS2_MUX = crate::Reg<meas2_mux::MEAS2_MUX_SPEC>;
#[doc = "SAR ADC2 MUX register."]
pub mod meas2_mux;
#[doc = "ATTEN2 (rw) register accessor: ADC1 attenuation registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`atten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atten2`] module"]
pub type ATTEN2 = crate::Reg<atten2::ATTEN2_SPEC>;
#[doc = "ADC1 attenuation registers."]
pub mod atten2;
#[doc = "FORCE_WPD_SAR (rw) register accessor: In sleep, force to use rtc to control ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`force_wpd_sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_wpd_sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_wpd_sar`] module"]
pub type FORCE_WPD_SAR = crate::Reg<force_wpd_sar::FORCE_WPD_SAR_SPEC>;
#[doc = "In sleep, force to use rtc to control ADC"]
pub mod force_wpd_sar;
#[doc = "MEAS_STATUS (r) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`meas_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meas_status`] module"]
pub type MEAS_STATUS = crate::Reg<meas_status::MEAS_STATUS_SPEC>;
#[doc = "N/A"]
pub mod meas_status;
#[doc = "REG_CLKEN (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_clken`] module"]
pub type REG_CLKEN = crate::Reg<reg_clken::REG_CLKEN_SPEC>;
#[doc = "N/A"]
pub mod reg_clken;
#[doc = "COCPU_INT_RAW (rw) register accessor: Interrupt raw registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cocpu_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocpu_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cocpu_int_raw`] module"]
pub type COCPU_INT_RAW = crate::Reg<cocpu_int_raw::COCPU_INT_RAW_SPEC>;
#[doc = "Interrupt raw registers."]
pub mod cocpu_int_raw;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable registers."]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: Interrupt status registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status registers."]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Interrupt clear registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear registers."]
pub mod int_clr;
#[doc = "INT_ENA_W1TS (w) register accessor: Interrupt enable assert registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_w1ts`] module"]
pub type INT_ENA_W1TS = crate::Reg<int_ena_w1ts::INT_ENA_W1TS_SPEC>;
#[doc = "Interrupt enable assert registers."]
pub mod int_ena_w1ts;
#[doc = "INT_ENA_W1TC (w) register accessor: Interrupt enable deassert registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_w1tc`] module"]
pub type INT_ENA_W1TC = crate::Reg<int_ena_w1tc::INT_ENA_W1TC_SPEC>;
#[doc = "Interrupt enable deassert registers."]
pub mod int_ena_w1tc;
#[doc = "WAKEUP1 (rw) register accessor: ADC1 wakeup configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup1`] module"]
pub type WAKEUP1 = crate::Reg<wakeup1::WAKEUP1_SPEC>;
#[doc = "ADC1 wakeup configuration registers."]
pub mod wakeup1;
#[doc = "WAKEUP2 (rw) register accessor: ADC2 wakeup configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup2`] module"]
pub type WAKEUP2 = crate::Reg<wakeup2::WAKEUP2_SPEC>;
#[doc = "ADC2 wakeup configuration registers."]
pub mod wakeup2;
#[doc = "WAKEUP_SEL (rw) register accessor: Wakeup source select register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_sel`] module"]
pub type WAKEUP_SEL = crate::Reg<wakeup_sel::WAKEUP_SEL_SPEC>;
#[doc = "Wakeup source select register."]
pub mod wakeup_sel;
#[doc = "SAR1_HW_WAKEUP (rw) register accessor: Hardware automatic sampling registers for wakeup function.\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_hw_wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_hw_wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_hw_wakeup`] module"]
pub type SAR1_HW_WAKEUP = crate::Reg<sar1_hw_wakeup::SAR1_HW_WAKEUP_SPEC>;
#[doc = "Hardware automatic sampling registers for wakeup function."]
pub mod sar1_hw_wakeup;
#[doc = "SAR2_HW_WAKEUP (rw) register accessor: Hardware automatic sampling registers for wakeup function.\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_hw_wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_hw_wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_hw_wakeup`] module"]
pub type SAR2_HW_WAKEUP = crate::Reg<sar2_hw_wakeup::SAR2_HW_WAKEUP_SPEC>;
#[doc = "Hardware automatic sampling registers for wakeup function."]
pub mod sar2_hw_wakeup;
#[doc = "RND_ECO_LOW (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "N/A"]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "N/A"]
pub mod rnd_eco_high;
#[doc = "RND_ECO_CS (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_cs`] module"]
pub type RND_ECO_CS = crate::Reg<rnd_eco_cs::RND_ECO_CS_SPEC>;
#[doc = "N/A"]
pub mod rnd_eco_cs;
