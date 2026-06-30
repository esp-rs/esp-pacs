#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_sysreg_ctrl: LP_SYSREG_CTRL,
    lp_aonclkrst_ctrl: LP_AONCLKRST_CTRL,
    lp_anaperi_ctrl: LP_ANAPERI_CTRL,
    lp_huk_ctrl: LP_HUK_CTRL,
    lp_pmu_ctrl: LP_PMU_CTRL,
    lp_touch_aon_ctrl: LP_TOUCH_AON_CTRL,
    lp_peri_pms_ctrl: LP_PERI_PMS_CTRL,
    lp_tee_ctrl: LP_TEE_CTRL,
    lp_apm_ctrl: LP_APM_CTRL,
    lp_timer_ctrl: LP_TIMER_CTRL,
    lp_wdt_ctrl: LP_WDT_CTRL,
    lp_clk_cali_sosc_ctrl: LP_CLK_CALI_SOSC_CTRL,
    lp_clk_cali_fosc_ctrl: LP_CLK_CALI_FOSC_CTRL,
    lp_pwr_reg_ctrl: LP_PWR_REG_CTRL,
    lp_periclkrst_ctrl: LP_PERICLKRST_CTRL,
    lp_iomux_ctrl: LP_IOMUX_CTRL,
    lp_intr_ctrl: LP_INTR_CTRL,
    lp_efuse_ctrl: LP_EFUSE_CTRL,
    lp_uart_ctrl: LP_UART_CTRL,
    lp_i2c_ctrl: LP_I2C_CTRL,
    lp_spi_ctrl: LP_SPI_CTRL,
    lp_i2cmst_ctrl: LP_I2CMST_CTRL,
    lp_trng_ctrl: LP_TRNG_CTRL,
    lp_adc_ctrl: LP_ADC_CTRL,
    lp_touch_ctrl: LP_TOUCH_CTRL,
    lp_mailbox_ctrl: LP_MAILBOX_CTRL,
    lp_tsens_ctrl: LP_TSENS_CTRL,
    lp_ahb_pdma_ctrl: LP_AHB_PDMA_CTRL,
    lp_dac_ctrl: LP_DAC_CTRL,
    _reserved29: [u8; 0x018c],
    lp_peri0_0: LP_PERI0_0,
    lp_peri0_1: LP_PERI0_1,
    lp_peri1_0: LP_PERI1_0,
    lp_peri1_1: LP_PERI1_1,
    _reserved33: [u8; 0xf0],
    int_en: INT_EN,
    _reserved34: [u8; 0x04ec],
    bus_err_conf: BUS_ERR_CONF,
    _reserved35: [u8; 0x04],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - lp_sysreg read/write control register"]
    #[inline(always)]
    pub const fn lp_sysreg_ctrl(&self) -> &LP_SYSREG_CTRL {
        &self.lp_sysreg_ctrl
    }
    #[doc = "0x04 - lp_aonclkrst read/write control register"]
    #[inline(always)]
    pub const fn lp_aonclkrst_ctrl(&self) -> &LP_AONCLKRST_CTRL {
        &self.lp_aonclkrst_ctrl
    }
    #[doc = "0x08 - lp_anaperi read/write control register"]
    #[inline(always)]
    pub const fn lp_anaperi_ctrl(&self) -> &LP_ANAPERI_CTRL {
        &self.lp_anaperi_ctrl
    }
    #[doc = "0x0c - lp_huk read/write control register"]
    #[inline(always)]
    pub const fn lp_huk_ctrl(&self) -> &LP_HUK_CTRL {
        &self.lp_huk_ctrl
    }
    #[doc = "0x10 - lp_pmu read/write control register"]
    #[inline(always)]
    pub const fn lp_pmu_ctrl(&self) -> &LP_PMU_CTRL {
        &self.lp_pmu_ctrl
    }
    #[doc = "0x14 - lp_touch_aon read/write control register"]
    #[inline(always)]
    pub const fn lp_touch_aon_ctrl(&self) -> &LP_TOUCH_AON_CTRL {
        &self.lp_touch_aon_ctrl
    }
    #[doc = "0x18 - lp_peri_pms read/write control register"]
    #[inline(always)]
    pub const fn lp_peri_pms_ctrl(&self) -> &LP_PERI_PMS_CTRL {
        &self.lp_peri_pms_ctrl
    }
    #[doc = "0x1c - lp_tee read/write control register"]
    #[inline(always)]
    pub const fn lp_tee_ctrl(&self) -> &LP_TEE_CTRL {
        &self.lp_tee_ctrl
    }
    #[doc = "0x20 - lp_apm read/write control register"]
    #[inline(always)]
    pub const fn lp_apm_ctrl(&self) -> &LP_APM_CTRL {
        &self.lp_apm_ctrl
    }
    #[doc = "0x24 - lp_timer read/write control register"]
    #[inline(always)]
    pub const fn lp_timer_ctrl(&self) -> &LP_TIMER_CTRL {
        &self.lp_timer_ctrl
    }
    #[doc = "0x28 - lp_wdt read/write control register"]
    #[inline(always)]
    pub const fn lp_wdt_ctrl(&self) -> &LP_WDT_CTRL {
        &self.lp_wdt_ctrl
    }
    #[doc = "0x2c - lp_clk_cali_sosc read/write control register"]
    #[inline(always)]
    pub const fn lp_clk_cali_sosc_ctrl(&self) -> &LP_CLK_CALI_SOSC_CTRL {
        &self.lp_clk_cali_sosc_ctrl
    }
    #[doc = "0x30 - lp_clk_cali_fosc read/write control register"]
    #[inline(always)]
    pub const fn lp_clk_cali_fosc_ctrl(&self) -> &LP_CLK_CALI_FOSC_CTRL {
        &self.lp_clk_cali_fosc_ctrl
    }
    #[doc = "0x34 - lp_pwr_reg read/write control register"]
    #[inline(always)]
    pub const fn lp_pwr_reg_ctrl(&self) -> &LP_PWR_REG_CTRL {
        &self.lp_pwr_reg_ctrl
    }
    #[doc = "0x38 - lp_periclkrst read/write control register"]
    #[inline(always)]
    pub const fn lp_periclkrst_ctrl(&self) -> &LP_PERICLKRST_CTRL {
        &self.lp_periclkrst_ctrl
    }
    #[doc = "0x3c - lp_iomux read/write control register"]
    #[inline(always)]
    pub const fn lp_iomux_ctrl(&self) -> &LP_IOMUX_CTRL {
        &self.lp_iomux_ctrl
    }
    #[doc = "0x40 - lp_intr read/write control register"]
    #[inline(always)]
    pub const fn lp_intr_ctrl(&self) -> &LP_INTR_CTRL {
        &self.lp_intr_ctrl
    }
    #[doc = "0x44 - lp_efuse read/write control register"]
    #[inline(always)]
    pub const fn lp_efuse_ctrl(&self) -> &LP_EFUSE_CTRL {
        &self.lp_efuse_ctrl
    }
    #[doc = "0x48 - lp_uart read/write control register"]
    #[inline(always)]
    pub const fn lp_uart_ctrl(&self) -> &LP_UART_CTRL {
        &self.lp_uart_ctrl
    }
    #[doc = "0x4c - lp_i2c read/write control register"]
    #[inline(always)]
    pub const fn lp_i2c_ctrl(&self) -> &LP_I2C_CTRL {
        &self.lp_i2c_ctrl
    }
    #[doc = "0x50 - lp_spi read/write control register"]
    #[inline(always)]
    pub const fn lp_spi_ctrl(&self) -> &LP_SPI_CTRL {
        &self.lp_spi_ctrl
    }
    #[doc = "0x54 - lp_i2cmst read/write control register"]
    #[inline(always)]
    pub const fn lp_i2cmst_ctrl(&self) -> &LP_I2CMST_CTRL {
        &self.lp_i2cmst_ctrl
    }
    #[doc = "0x58 - lp_trng read/write control register"]
    #[inline(always)]
    pub const fn lp_trng_ctrl(&self) -> &LP_TRNG_CTRL {
        &self.lp_trng_ctrl
    }
    #[doc = "0x5c - lp_adc read/write control register"]
    #[inline(always)]
    pub const fn lp_adc_ctrl(&self) -> &LP_ADC_CTRL {
        &self.lp_adc_ctrl
    }
    #[doc = "0x60 - lp_touch read/write control register"]
    #[inline(always)]
    pub const fn lp_touch_ctrl(&self) -> &LP_TOUCH_CTRL {
        &self.lp_touch_ctrl
    }
    #[doc = "0x64 - lp_mailbox read/write control register"]
    #[inline(always)]
    pub const fn lp_mailbox_ctrl(&self) -> &LP_MAILBOX_CTRL {
        &self.lp_mailbox_ctrl
    }
    #[doc = "0x68 - lp_tsens read/write control register"]
    #[inline(always)]
    pub const fn lp_tsens_ctrl(&self) -> &LP_TSENS_CTRL {
        &self.lp_tsens_ctrl
    }
    #[doc = "0x6c - lp_ahb_pdma read/write control register"]
    #[inline(always)]
    pub const fn lp_ahb_pdma_ctrl(&self) -> &LP_AHB_PDMA_CTRL {
        &self.lp_ahb_pdma_ctrl
    }
    #[doc = "0x70 - lp_dac read/write control register"]
    #[inline(always)]
    pub const fn lp_dac_ctrl(&self) -> &LP_DAC_CTRL {
        &self.lp_dac_ctrl
    }
    #[doc = "0x200 - LP_PERI0 PMS configuration & info register"]
    #[inline(always)]
    pub const fn lp_peri0_0(&self) -> &LP_PERI0_0 {
        &self.lp_peri0_0
    }
    #[doc = "0x204 - LP_PERI0 PMS exception addr record register"]
    #[inline(always)]
    pub const fn lp_peri0_1(&self) -> &LP_PERI0_1 {
        &self.lp_peri0_1
    }
    #[doc = "0x208 - LP_PERI1 PMS configuration & info register"]
    #[inline(always)]
    pub const fn lp_peri1_0(&self) -> &LP_PERI1_0 {
        &self.lp_peri1_0
    }
    #[doc = "0x20c - LP_PERI1 PMS exception addr record register"]
    #[inline(always)]
    pub const fn lp_peri1_1(&self) -> &LP_PERI1_1 {
        &self.lp_peri1_1
    }
    #[doc = "0x300 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0x7f0 - Clock gating register"]
    #[inline(always)]
    pub const fn bus_err_conf(&self) -> &BUS_ERR_CONF {
        &self.bus_err_conf
    }
    #[doc = "0x7f8 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "LP_SYSREG_CTRL (rw) register accessor: lp_sysreg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sysreg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sysreg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sysreg_ctrl`] module"]
pub type LP_SYSREG_CTRL = crate::Reg<lp_sysreg_ctrl::LP_SYSREG_CTRL_SPEC>;
#[doc = "lp_sysreg read/write control register"]
pub mod lp_sysreg_ctrl;
#[doc = "LP_AONCLKRST_CTRL (rw) register accessor: lp_aonclkrst read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_ctrl`] module"]
pub type LP_AONCLKRST_CTRL = crate::Reg<lp_aonclkrst_ctrl::LP_AONCLKRST_CTRL_SPEC>;
#[doc = "lp_aonclkrst read/write control register"]
pub mod lp_aonclkrst_ctrl;
#[doc = "LP_ANAPERI_CTRL (rw) register accessor: lp_anaperi read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_anaperi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_anaperi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_anaperi_ctrl`] module"]
pub type LP_ANAPERI_CTRL = crate::Reg<lp_anaperi_ctrl::LP_ANAPERI_CTRL_SPEC>;
#[doc = "lp_anaperi read/write control register"]
pub mod lp_anaperi_ctrl;
#[doc = "LP_HUK_CTRL (rw) register accessor: lp_huk read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_huk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_huk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_huk_ctrl`] module"]
pub type LP_HUK_CTRL = crate::Reg<lp_huk_ctrl::LP_HUK_CTRL_SPEC>;
#[doc = "lp_huk read/write control register"]
pub mod lp_huk_ctrl;
#[doc = "LP_PMU_CTRL (rw) register accessor: lp_pmu read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_ctrl`] module"]
pub type LP_PMU_CTRL = crate::Reg<lp_pmu_ctrl::LP_PMU_CTRL_SPEC>;
#[doc = "lp_pmu read/write control register"]
pub mod lp_pmu_ctrl;
#[doc = "LP_TOUCH_AON_CTRL (rw) register accessor: lp_touch_aon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_touch_aon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_touch_aon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_touch_aon_ctrl`] module"]
pub type LP_TOUCH_AON_CTRL = crate::Reg<lp_touch_aon_ctrl::LP_TOUCH_AON_CTRL_SPEC>;
#[doc = "lp_touch_aon read/write control register"]
pub mod lp_touch_aon_ctrl;
#[doc = "LP_PERI_PMS_CTRL (rw) register accessor: lp_peri_pms read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_pms_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_pms_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_pms_ctrl`] module"]
pub type LP_PERI_PMS_CTRL = crate::Reg<lp_peri_pms_ctrl::LP_PERI_PMS_CTRL_SPEC>;
#[doc = "lp_peri_pms read/write control register"]
pub mod lp_peri_pms_ctrl;
#[doc = "LP_TEE_CTRL (rw) register accessor: lp_tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tee_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tee_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tee_ctrl`] module"]
pub type LP_TEE_CTRL = crate::Reg<lp_tee_ctrl::LP_TEE_CTRL_SPEC>;
#[doc = "lp_tee read/write control register"]
pub mod lp_tee_ctrl;
#[doc = "LP_APM_CTRL (rw) register accessor: lp_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_ctrl`] module"]
pub type LP_APM_CTRL = crate::Reg<lp_apm_ctrl::LP_APM_CTRL_SPEC>;
#[doc = "lp_apm read/write control register"]
pub mod lp_apm_ctrl;
#[doc = "LP_TIMER_CTRL (rw) register accessor: lp_timer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_ctrl`] module"]
pub type LP_TIMER_CTRL = crate::Reg<lp_timer_ctrl::LP_TIMER_CTRL_SPEC>;
#[doc = "lp_timer read/write control register"]
pub mod lp_timer_ctrl;
#[doc = "LP_WDT_CTRL (rw) register accessor: lp_wdt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_wdt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_wdt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_ctrl`] module"]
pub type LP_WDT_CTRL = crate::Reg<lp_wdt_ctrl::LP_WDT_CTRL_SPEC>;
#[doc = "lp_wdt read/write control register"]
pub mod lp_wdt_ctrl;
#[doc = "LP_CLK_CALI_SOSC_CTRL (rw) register accessor: lp_clk_cali_sosc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_cali_sosc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_cali_sosc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_cali_sosc_ctrl`] module"]
pub type LP_CLK_CALI_SOSC_CTRL = crate::Reg<lp_clk_cali_sosc_ctrl::LP_CLK_CALI_SOSC_CTRL_SPEC>;
#[doc = "lp_clk_cali_sosc read/write control register"]
pub mod lp_clk_cali_sosc_ctrl;
#[doc = "LP_CLK_CALI_FOSC_CTRL (rw) register accessor: lp_clk_cali_fosc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_cali_fosc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_cali_fosc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_cali_fosc_ctrl`] module"]
pub type LP_CLK_CALI_FOSC_CTRL = crate::Reg<lp_clk_cali_fosc_ctrl::LP_CLK_CALI_FOSC_CTRL_SPEC>;
#[doc = "lp_clk_cali_fosc read/write control register"]
pub mod lp_clk_cali_fosc_ctrl;
#[doc = "LP_PWR_REG_CTRL (rw) register accessor: lp_pwr_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pwr_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pwr_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pwr_reg_ctrl`] module"]
pub type LP_PWR_REG_CTRL = crate::Reg<lp_pwr_reg_ctrl::LP_PWR_REG_CTRL_SPEC>;
#[doc = "lp_pwr_reg read/write control register"]
pub mod lp_pwr_reg_ctrl;
#[doc = "LP_PERICLKRST_CTRL (rw) register accessor: lp_periclkrst read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_periclkrst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_periclkrst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_periclkrst_ctrl`] module"]
pub type LP_PERICLKRST_CTRL = crate::Reg<lp_periclkrst_ctrl::LP_PERICLKRST_CTRL_SPEC>;
#[doc = "lp_periclkrst read/write control register"]
pub mod lp_periclkrst_ctrl;
#[doc = "LP_IOMUX_CTRL (rw) register accessor: lp_iomux read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_iomux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_iomux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_iomux_ctrl`] module"]
pub type LP_IOMUX_CTRL = crate::Reg<lp_iomux_ctrl::LP_IOMUX_CTRL_SPEC>;
#[doc = "lp_iomux read/write control register"]
pub mod lp_iomux_ctrl;
#[doc = "LP_INTR_CTRL (rw) register accessor: lp_intr read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_intr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_intr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_intr_ctrl`] module"]
pub type LP_INTR_CTRL = crate::Reg<lp_intr_ctrl::LP_INTR_CTRL_SPEC>;
#[doc = "lp_intr read/write control register"]
pub mod lp_intr_ctrl;
#[doc = "LP_EFUSE_CTRL (rw) register accessor: lp_efuse read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_efuse_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_efuse_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_efuse_ctrl`] module"]
pub type LP_EFUSE_CTRL = crate::Reg<lp_efuse_ctrl::LP_EFUSE_CTRL_SPEC>;
#[doc = "lp_efuse read/write control register"]
pub mod lp_efuse_ctrl;
#[doc = "LP_UART_CTRL (rw) register accessor: lp_uart read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_uart_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_uart_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_uart_ctrl`] module"]
pub type LP_UART_CTRL = crate::Reg<lp_uart_ctrl::LP_UART_CTRL_SPEC>;
#[doc = "lp_uart read/write control register"]
pub mod lp_uart_ctrl;
#[doc = "LP_I2C_CTRL (rw) register accessor: lp_i2c read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2c_ctrl`] module"]
pub type LP_I2C_CTRL = crate::Reg<lp_i2c_ctrl::LP_I2C_CTRL_SPEC>;
#[doc = "lp_i2c read/write control register"]
pub mod lp_i2c_ctrl;
#[doc = "LP_SPI_CTRL (rw) register accessor: lp_spi read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_spi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_spi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_spi_ctrl`] module"]
pub type LP_SPI_CTRL = crate::Reg<lp_spi_ctrl::LP_SPI_CTRL_SPEC>;
#[doc = "lp_spi read/write control register"]
pub mod lp_spi_ctrl;
#[doc = "LP_I2CMST_CTRL (rw) register accessor: lp_i2cmst read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2cmst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2cmst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2cmst_ctrl`] module"]
pub type LP_I2CMST_CTRL = crate::Reg<lp_i2cmst_ctrl::LP_I2CMST_CTRL_SPEC>;
#[doc = "lp_i2cmst read/write control register"]
pub mod lp_i2cmst_ctrl;
#[doc = "LP_TRNG_CTRL (rw) register accessor: lp_trng read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_trng_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_trng_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_trng_ctrl`] module"]
pub type LP_TRNG_CTRL = crate::Reg<lp_trng_ctrl::LP_TRNG_CTRL_SPEC>;
#[doc = "lp_trng read/write control register"]
pub mod lp_trng_ctrl;
#[doc = "LP_ADC_CTRL (rw) register accessor: lp_adc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_adc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_adc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_adc_ctrl`] module"]
pub type LP_ADC_CTRL = crate::Reg<lp_adc_ctrl::LP_ADC_CTRL_SPEC>;
#[doc = "lp_adc read/write control register"]
pub mod lp_adc_ctrl;
#[doc = "LP_TOUCH_CTRL (rw) register accessor: lp_touch read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_touch_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_touch_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_touch_ctrl`] module"]
pub type LP_TOUCH_CTRL = crate::Reg<lp_touch_ctrl::LP_TOUCH_CTRL_SPEC>;
#[doc = "lp_touch read/write control register"]
pub mod lp_touch_ctrl;
#[doc = "LP_MAILBOX_CTRL (rw) register accessor: lp_mailbox read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mailbox_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mailbox_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mailbox_ctrl`] module"]
pub type LP_MAILBOX_CTRL = crate::Reg<lp_mailbox_ctrl::LP_MAILBOX_CTRL_SPEC>;
#[doc = "lp_mailbox read/write control register"]
pub mod lp_mailbox_ctrl;
#[doc = "LP_TSENS_CTRL (rw) register accessor: lp_tsens read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tsens_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tsens_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tsens_ctrl`] module"]
pub type LP_TSENS_CTRL = crate::Reg<lp_tsens_ctrl::LP_TSENS_CTRL_SPEC>;
#[doc = "lp_tsens read/write control register"]
pub mod lp_tsens_ctrl;
#[doc = "LP_AHB_PDMA_CTRL (rw) register accessor: lp_ahb_pdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ahb_pdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ahb_pdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ahb_pdma_ctrl`] module"]
pub type LP_AHB_PDMA_CTRL = crate::Reg<lp_ahb_pdma_ctrl::LP_AHB_PDMA_CTRL_SPEC>;
#[doc = "lp_ahb_pdma read/write control register"]
pub mod lp_ahb_pdma_ctrl;
#[doc = "LP_DAC_CTRL (rw) register accessor: lp_dac read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_dac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_dac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_dac_ctrl`] module"]
pub type LP_DAC_CTRL = crate::Reg<lp_dac_ctrl::LP_DAC_CTRL_SPEC>;
#[doc = "lp_dac read/write control register"]
pub mod lp_dac_ctrl;
#[doc = "LP_PERI0_0 (rw) register accessor: LP_PERI0 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri0_0`] module"]
pub type LP_PERI0_0 = crate::Reg<lp_peri0_0::LP_PERI0_0_SPEC>;
#[doc = "LP_PERI0 PMS configuration & info register"]
pub mod lp_peri0_0;
#[doc = "LP_PERI0_1 (r) register accessor: LP_PERI0 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri0_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri0_1`] module"]
pub type LP_PERI0_1 = crate::Reg<lp_peri0_1::LP_PERI0_1_SPEC>;
#[doc = "LP_PERI0 PMS exception addr record register"]
pub mod lp_peri0_1;
#[doc = "LP_PERI1_0 (rw) register accessor: LP_PERI1 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri1_0`] module"]
pub type LP_PERI1_0 = crate::Reg<lp_peri1_0::LP_PERI1_0_SPEC>;
#[doc = "LP_PERI1 PMS configuration & info register"]
pub mod lp_peri1_0;
#[doc = "LP_PERI1_1 (r) register accessor: LP_PERI1 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri1_1`] module"]
pub type LP_PERI1_1 = crate::Reg<lp_peri1_1::LP_PERI1_1_SPEC>;
#[doc = "LP_PERI1 PMS exception addr record register"]
pub mod lp_peri1_1;
#[doc = "INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "BUS_ERR_CONF (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_err_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_err_conf`] module"]
pub type BUS_ERR_CONF = crate::Reg<bus_err_conf::BUS_ERR_CONF_SPEC>;
#[doc = "Clock gating register"]
pub mod bus_err_conf;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
