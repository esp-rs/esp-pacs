#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hp_usbotg_phy_ctrl: HP_USBOTG_PHY_CTRL,
    hp_mcpwm0_ctrl: HP_MCPWM0_CTRL,
    hp_mcpwm1_ctrl: HP_MCPWM1_CTRL,
    hp_mcpwm2_ctrl: HP_MCPWM2_CTRL,
    hp_mcpwm3_ctrl: HP_MCPWM3_CTRL,
    hp_i2c0_ctrl: HP_I2C0_CTRL,
    hp_i2c1_ctrl: HP_I2C1_CTRL,
    hp_i2s0_ctrl: HP_I2S0_CTRL,
    hp_i2s1_ctrl: HP_I2S1_CTRL,
    hp_pcnt0_ctrl: HP_PCNT0_CTRL,
    hp_uart0_ctrl: HP_UART0_CTRL,
    hp_uart1_ctrl: HP_UART1_CTRL,
    hp_uart2_ctrl: HP_UART2_CTRL,
    hp_uart3_ctrl: HP_UART3_CTRL,
    hp_parlio_ctrl: HP_PARLIO_CTRL,
    hp_gpspi2_ctrl: HP_GPSPI2_CTRL,
    hp_gpspi3_ctrl: HP_GPSPI3_CTRL,
    hp_usbdevice_ctrl: HP_USBDEVICE_CTRL,
    hp_ledc0_ctrl: HP_LEDC0_CTRL,
    hp_etm_ctrl: HP_ETM_CTRL,
    hp_twai0_ctrl: HP_TWAI0_CTRL,
    hp_twai1_ctrl: HP_TWAI1_CTRL,
    hp_lcdcam_ctrl: HP_LCDCAM_CTRL,
    hp_uhci_ctrl: HP_UHCI_CTRL,
    hp_systimer_ctrl: HP_SYSTIMER_CTRL,
    hp_zero_det_ctrl: HP_ZERO_DET_CTRL,
    hp_cordic_ctrl: HP_CORDIC_CTRL,
    hp_ledc1_ctrl: HP_LEDC1_CTRL,
    hp_pcnt1_ctrl: HP_PCNT1_CTRL,
    hp_timer_group0_ctrl: HP_TIMER_GROUP0_CTRL,
    hp_timer_group1_ctrl: HP_TIMER_GROUP1_CTRL,
    hp_iomux_ctrl: HP_IOMUX_CTRL,
    hp_mspi_padctrl_ctrl: HP_MSPI_PADCTRL_CTRL,
    hp_intrmtx_ctrl: HP_INTRMTX_CTRL,
    hp_sys_reg_ctrl: HP_SYS_REG_CTRL,
    hp_clkrst_ctrl: HP_CLKRST_CTRL,
    cnnt_pad_ctrl_ctrl: CNNT_PAD_CTRL_CTRL,
    hp_alive_sys_reg_ctrl: HP_ALIVE_SYS_REG_CTRL,
    hp_peri1_pms_ctrl: HP_PERI1_PMS_CTRL,
    _reserved39: [u8; 0x0164],
    hp_peri1_0: HP_PERI1_0,
    hp_peri1_1: HP_PERI1_1,
    _reserved41: [u8; 0xf8],
    int_en: INT_EN,
    _reserved42: [u8; 0x0cec],
    bus_err_conf: BUS_ERR_CONF,
    _reserved43: [u8; 0x04],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - hp_usbotg_phy read/write control register"]
    #[inline(always)]
    pub const fn hp_usbotg_phy_ctrl(&self) -> &HP_USBOTG_PHY_CTRL {
        &self.hp_usbotg_phy_ctrl
    }
    #[doc = "0x04 - hp_mcpwm0 read/write control register"]
    #[inline(always)]
    pub const fn hp_mcpwm0_ctrl(&self) -> &HP_MCPWM0_CTRL {
        &self.hp_mcpwm0_ctrl
    }
    #[doc = "0x08 - hp_mcpwm1 read/write control register"]
    #[inline(always)]
    pub const fn hp_mcpwm1_ctrl(&self) -> &HP_MCPWM1_CTRL {
        &self.hp_mcpwm1_ctrl
    }
    #[doc = "0x0c - hp_mcpwm2 read/write control register"]
    #[inline(always)]
    pub const fn hp_mcpwm2_ctrl(&self) -> &HP_MCPWM2_CTRL {
        &self.hp_mcpwm2_ctrl
    }
    #[doc = "0x10 - hp_mcpwm3 read/write control register"]
    #[inline(always)]
    pub const fn hp_mcpwm3_ctrl(&self) -> &HP_MCPWM3_CTRL {
        &self.hp_mcpwm3_ctrl
    }
    #[doc = "0x14 - hp_i2c0 read/write control register"]
    #[inline(always)]
    pub const fn hp_i2c0_ctrl(&self) -> &HP_I2C0_CTRL {
        &self.hp_i2c0_ctrl
    }
    #[doc = "0x18 - hp_i2c1 read/write control register"]
    #[inline(always)]
    pub const fn hp_i2c1_ctrl(&self) -> &HP_I2C1_CTRL {
        &self.hp_i2c1_ctrl
    }
    #[doc = "0x1c - hp_i2s0 read/write control register"]
    #[inline(always)]
    pub const fn hp_i2s0_ctrl(&self) -> &HP_I2S0_CTRL {
        &self.hp_i2s0_ctrl
    }
    #[doc = "0x20 - hp_i2s1 read/write control register"]
    #[inline(always)]
    pub const fn hp_i2s1_ctrl(&self) -> &HP_I2S1_CTRL {
        &self.hp_i2s1_ctrl
    }
    #[doc = "0x24 - hp_pcnt0 read/write control register"]
    #[inline(always)]
    pub const fn hp_pcnt0_ctrl(&self) -> &HP_PCNT0_CTRL {
        &self.hp_pcnt0_ctrl
    }
    #[doc = "0x28 - hp_uart0 read/write control register"]
    #[inline(always)]
    pub const fn hp_uart0_ctrl(&self) -> &HP_UART0_CTRL {
        &self.hp_uart0_ctrl
    }
    #[doc = "0x2c - hp_uart1 read/write control register"]
    #[inline(always)]
    pub const fn hp_uart1_ctrl(&self) -> &HP_UART1_CTRL {
        &self.hp_uart1_ctrl
    }
    #[doc = "0x30 - hp_uart2 read/write control register"]
    #[inline(always)]
    pub const fn hp_uart2_ctrl(&self) -> &HP_UART2_CTRL {
        &self.hp_uart2_ctrl
    }
    #[doc = "0x34 - hp_uart3 read/write control register"]
    #[inline(always)]
    pub const fn hp_uart3_ctrl(&self) -> &HP_UART3_CTRL {
        &self.hp_uart3_ctrl
    }
    #[doc = "0x38 - hp_parlio read/write control register"]
    #[inline(always)]
    pub const fn hp_parlio_ctrl(&self) -> &HP_PARLIO_CTRL {
        &self.hp_parlio_ctrl
    }
    #[doc = "0x3c - hp_gpspi2 read/write control register"]
    #[inline(always)]
    pub const fn hp_gpspi2_ctrl(&self) -> &HP_GPSPI2_CTRL {
        &self.hp_gpspi2_ctrl
    }
    #[doc = "0x40 - hp_gpspi3 read/write control register"]
    #[inline(always)]
    pub const fn hp_gpspi3_ctrl(&self) -> &HP_GPSPI3_CTRL {
        &self.hp_gpspi3_ctrl
    }
    #[doc = "0x44 - hp_usbdevice read/write control register"]
    #[inline(always)]
    pub const fn hp_usbdevice_ctrl(&self) -> &HP_USBDEVICE_CTRL {
        &self.hp_usbdevice_ctrl
    }
    #[doc = "0x48 - hp_ledc0 read/write control register"]
    #[inline(always)]
    pub const fn hp_ledc0_ctrl(&self) -> &HP_LEDC0_CTRL {
        &self.hp_ledc0_ctrl
    }
    #[doc = "0x4c - hp_etm read/write control register"]
    #[inline(always)]
    pub const fn hp_etm_ctrl(&self) -> &HP_ETM_CTRL {
        &self.hp_etm_ctrl
    }
    #[doc = "0x50 - hp_twai0 read/write control register"]
    #[inline(always)]
    pub const fn hp_twai0_ctrl(&self) -> &HP_TWAI0_CTRL {
        &self.hp_twai0_ctrl
    }
    #[doc = "0x54 - hp_twai1 read/write control register"]
    #[inline(always)]
    pub const fn hp_twai1_ctrl(&self) -> &HP_TWAI1_CTRL {
        &self.hp_twai1_ctrl
    }
    #[doc = "0x58 - hp_lcdcam read/write control register"]
    #[inline(always)]
    pub const fn hp_lcdcam_ctrl(&self) -> &HP_LCDCAM_CTRL {
        &self.hp_lcdcam_ctrl
    }
    #[doc = "0x5c - hp_uhci read/write control register"]
    #[inline(always)]
    pub const fn hp_uhci_ctrl(&self) -> &HP_UHCI_CTRL {
        &self.hp_uhci_ctrl
    }
    #[doc = "0x60 - hp_systimer read/write control register"]
    #[inline(always)]
    pub const fn hp_systimer_ctrl(&self) -> &HP_SYSTIMER_CTRL {
        &self.hp_systimer_ctrl
    }
    #[doc = "0x64 - hp_zero_det read/write control register"]
    #[inline(always)]
    pub const fn hp_zero_det_ctrl(&self) -> &HP_ZERO_DET_CTRL {
        &self.hp_zero_det_ctrl
    }
    #[doc = "0x68 - hp_cordic read/write control register"]
    #[inline(always)]
    pub const fn hp_cordic_ctrl(&self) -> &HP_CORDIC_CTRL {
        &self.hp_cordic_ctrl
    }
    #[doc = "0x6c - hp_ledc1 read/write control register"]
    #[inline(always)]
    pub const fn hp_ledc1_ctrl(&self) -> &HP_LEDC1_CTRL {
        &self.hp_ledc1_ctrl
    }
    #[doc = "0x70 - hp_pcnt1 read/write control register"]
    #[inline(always)]
    pub const fn hp_pcnt1_ctrl(&self) -> &HP_PCNT1_CTRL {
        &self.hp_pcnt1_ctrl
    }
    #[doc = "0x74 - hp_timer_group0 read/write control register"]
    #[inline(always)]
    pub const fn hp_timer_group0_ctrl(&self) -> &HP_TIMER_GROUP0_CTRL {
        &self.hp_timer_group0_ctrl
    }
    #[doc = "0x78 - hp_timer_group1 read/write control register"]
    #[inline(always)]
    pub const fn hp_timer_group1_ctrl(&self) -> &HP_TIMER_GROUP1_CTRL {
        &self.hp_timer_group1_ctrl
    }
    #[doc = "0x7c - hp_iomux read/write control register"]
    #[inline(always)]
    pub const fn hp_iomux_ctrl(&self) -> &HP_IOMUX_CTRL {
        &self.hp_iomux_ctrl
    }
    #[doc = "0x80 - hp_mspi_padctrl read/write control register"]
    #[inline(always)]
    pub const fn hp_mspi_padctrl_ctrl(&self) -> &HP_MSPI_PADCTRL_CTRL {
        &self.hp_mspi_padctrl_ctrl
    }
    #[doc = "0x84 - hp_intrmtx read/write control register"]
    #[inline(always)]
    pub const fn hp_intrmtx_ctrl(&self) -> &HP_INTRMTX_CTRL {
        &self.hp_intrmtx_ctrl
    }
    #[doc = "0x88 - hp_sys_reg read/write control register"]
    #[inline(always)]
    pub const fn hp_sys_reg_ctrl(&self) -> &HP_SYS_REG_CTRL {
        &self.hp_sys_reg_ctrl
    }
    #[doc = "0x8c - hp_clkrst read/write control register"]
    #[inline(always)]
    pub const fn hp_clkrst_ctrl(&self) -> &HP_CLKRST_CTRL {
        &self.hp_clkrst_ctrl
    }
    #[doc = "0x90 - cnnt_pad_ctrl read/write control register"]
    #[inline(always)]
    pub const fn cnnt_pad_ctrl_ctrl(&self) -> &CNNT_PAD_CTRL_CTRL {
        &self.cnnt_pad_ctrl_ctrl
    }
    #[doc = "0x94 - hp_alive_sys_reg read/write control register"]
    #[inline(always)]
    pub const fn hp_alive_sys_reg_ctrl(&self) -> &HP_ALIVE_SYS_REG_CTRL {
        &self.hp_alive_sys_reg_ctrl
    }
    #[doc = "0x98 - hp_peri1_pms read/write control register"]
    #[inline(always)]
    pub const fn hp_peri1_pms_ctrl(&self) -> &HP_PERI1_PMS_CTRL {
        &self.hp_peri1_pms_ctrl
    }
    #[doc = "0x200 - HP_PERI1 PMS configuration & info register"]
    #[inline(always)]
    pub const fn hp_peri1_0(&self) -> &HP_PERI1_0 {
        &self.hp_peri1_0
    }
    #[doc = "0x204 - HP_PERI1 PMS exception addr record register"]
    #[inline(always)]
    pub const fn hp_peri1_1(&self) -> &HP_PERI1_1 {
        &self.hp_peri1_1
    }
    #[doc = "0x300 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0xff0 - Clock gating register"]
    #[inline(always)]
    pub const fn bus_err_conf(&self) -> &BUS_ERR_CONF {
        &self.bus_err_conf
    }
    #[doc = "0xff8 - Clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "HP_USBOTG_PHY_CTRL (rw) register accessor: hp_usbotg_phy read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usbotg_phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usbotg_phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_usbotg_phy_ctrl`] module"]
pub type HP_USBOTG_PHY_CTRL = crate::Reg<hp_usbotg_phy_ctrl::HP_USBOTG_PHY_CTRL_SPEC>;
#[doc = "hp_usbotg_phy read/write control register"]
pub mod hp_usbotg_phy_ctrl;
#[doc = "HP_MCPWM0_CTRL (rw) register accessor: hp_mcpwm0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mcpwm0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mcpwm0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mcpwm0_ctrl`] module"]
pub type HP_MCPWM0_CTRL = crate::Reg<hp_mcpwm0_ctrl::HP_MCPWM0_CTRL_SPEC>;
#[doc = "hp_mcpwm0 read/write control register"]
pub mod hp_mcpwm0_ctrl;
#[doc = "HP_MCPWM1_CTRL (rw) register accessor: hp_mcpwm1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mcpwm1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mcpwm1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mcpwm1_ctrl`] module"]
pub type HP_MCPWM1_CTRL = crate::Reg<hp_mcpwm1_ctrl::HP_MCPWM1_CTRL_SPEC>;
#[doc = "hp_mcpwm1 read/write control register"]
pub mod hp_mcpwm1_ctrl;
#[doc = "HP_MCPWM2_CTRL (rw) register accessor: hp_mcpwm2 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mcpwm2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mcpwm2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mcpwm2_ctrl`] module"]
pub type HP_MCPWM2_CTRL = crate::Reg<hp_mcpwm2_ctrl::HP_MCPWM2_CTRL_SPEC>;
#[doc = "hp_mcpwm2 read/write control register"]
pub mod hp_mcpwm2_ctrl;
#[doc = "HP_MCPWM3_CTRL (rw) register accessor: hp_mcpwm3 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mcpwm3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mcpwm3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mcpwm3_ctrl`] module"]
pub type HP_MCPWM3_CTRL = crate::Reg<hp_mcpwm3_ctrl::HP_MCPWM3_CTRL_SPEC>;
#[doc = "hp_mcpwm3 read/write control register"]
pub mod hp_mcpwm3_ctrl;
#[doc = "HP_I2C0_CTRL (rw) register accessor: hp_i2c0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2c0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2c0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_i2c0_ctrl`] module"]
pub type HP_I2C0_CTRL = crate::Reg<hp_i2c0_ctrl::HP_I2C0_CTRL_SPEC>;
#[doc = "hp_i2c0 read/write control register"]
pub mod hp_i2c0_ctrl;
#[doc = "HP_I2C1_CTRL (rw) register accessor: hp_i2c1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2c1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2c1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_i2c1_ctrl`] module"]
pub type HP_I2C1_CTRL = crate::Reg<hp_i2c1_ctrl::HP_I2C1_CTRL_SPEC>;
#[doc = "hp_i2c1 read/write control register"]
pub mod hp_i2c1_ctrl;
#[doc = "HP_I2S0_CTRL (rw) register accessor: hp_i2s0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2s0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2s0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_i2s0_ctrl`] module"]
pub type HP_I2S0_CTRL = crate::Reg<hp_i2s0_ctrl::HP_I2S0_CTRL_SPEC>;
#[doc = "hp_i2s0 read/write control register"]
pub mod hp_i2s0_ctrl;
#[doc = "HP_I2S1_CTRL (rw) register accessor: hp_i2s1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2s1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2s1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_i2s1_ctrl`] module"]
pub type HP_I2S1_CTRL = crate::Reg<hp_i2s1_ctrl::HP_I2S1_CTRL_SPEC>;
#[doc = "hp_i2s1 read/write control register"]
pub mod hp_i2s1_ctrl;
#[doc = "HP_PCNT0_CTRL (rw) register accessor: hp_pcnt0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pcnt0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pcnt0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pcnt0_ctrl`] module"]
pub type HP_PCNT0_CTRL = crate::Reg<hp_pcnt0_ctrl::HP_PCNT0_CTRL_SPEC>;
#[doc = "hp_pcnt0 read/write control register"]
pub mod hp_pcnt0_ctrl;
#[doc = "HP_UART0_CTRL (rw) register accessor: hp_uart0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_uart0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_uart0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uart0_ctrl`] module"]
pub type HP_UART0_CTRL = crate::Reg<hp_uart0_ctrl::HP_UART0_CTRL_SPEC>;
#[doc = "hp_uart0 read/write control register"]
pub mod hp_uart0_ctrl;
#[doc = "HP_UART1_CTRL (rw) register accessor: hp_uart1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_uart1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_uart1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uart1_ctrl`] module"]
pub type HP_UART1_CTRL = crate::Reg<hp_uart1_ctrl::HP_UART1_CTRL_SPEC>;
#[doc = "hp_uart1 read/write control register"]
pub mod hp_uart1_ctrl;
#[doc = "HP_UART2_CTRL (rw) register accessor: hp_uart2 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_uart2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_uart2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uart2_ctrl`] module"]
pub type HP_UART2_CTRL = crate::Reg<hp_uart2_ctrl::HP_UART2_CTRL_SPEC>;
#[doc = "hp_uart2 read/write control register"]
pub mod hp_uart2_ctrl;
#[doc = "HP_UART3_CTRL (rw) register accessor: hp_uart3 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_uart3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_uart3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uart3_ctrl`] module"]
pub type HP_UART3_CTRL = crate::Reg<hp_uart3_ctrl::HP_UART3_CTRL_SPEC>;
#[doc = "hp_uart3 read/write control register"]
pub mod hp_uart3_ctrl;
#[doc = "HP_PARLIO_CTRL (rw) register accessor: hp_parlio read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_parlio_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_parlio_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_parlio_ctrl`] module"]
pub type HP_PARLIO_CTRL = crate::Reg<hp_parlio_ctrl::HP_PARLIO_CTRL_SPEC>;
#[doc = "hp_parlio read/write control register"]
pub mod hp_parlio_ctrl;
#[doc = "HP_GPSPI2_CTRL (rw) register accessor: hp_gpspi2 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpspi2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpspi2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpspi2_ctrl`] module"]
pub type HP_GPSPI2_CTRL = crate::Reg<hp_gpspi2_ctrl::HP_GPSPI2_CTRL_SPEC>;
#[doc = "hp_gpspi2 read/write control register"]
pub mod hp_gpspi2_ctrl;
#[doc = "HP_GPSPI3_CTRL (rw) register accessor: hp_gpspi3 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpspi3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpspi3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpspi3_ctrl`] module"]
pub type HP_GPSPI3_CTRL = crate::Reg<hp_gpspi3_ctrl::HP_GPSPI3_CTRL_SPEC>;
#[doc = "hp_gpspi3 read/write control register"]
pub mod hp_gpspi3_ctrl;
#[doc = "HP_USBDEVICE_CTRL (rw) register accessor: hp_usbdevice read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usbdevice_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usbdevice_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_usbdevice_ctrl`] module"]
pub type HP_USBDEVICE_CTRL = crate::Reg<hp_usbdevice_ctrl::HP_USBDEVICE_CTRL_SPEC>;
#[doc = "hp_usbdevice read/write control register"]
pub mod hp_usbdevice_ctrl;
#[doc = "HP_LEDC0_CTRL (rw) register accessor: hp_ledc0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ledc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ledc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ledc0_ctrl`] module"]
pub type HP_LEDC0_CTRL = crate::Reg<hp_ledc0_ctrl::HP_LEDC0_CTRL_SPEC>;
#[doc = "hp_ledc0 read/write control register"]
pub mod hp_ledc0_ctrl;
#[doc = "HP_ETM_CTRL (rw) register accessor: hp_etm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_etm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_etm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_etm_ctrl`] module"]
pub type HP_ETM_CTRL = crate::Reg<hp_etm_ctrl::HP_ETM_CTRL_SPEC>;
#[doc = "hp_etm read/write control register"]
pub mod hp_etm_ctrl;
#[doc = "HP_TWAI0_CTRL (rw) register accessor: hp_twai0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai0_ctrl`] module"]
pub type HP_TWAI0_CTRL = crate::Reg<hp_twai0_ctrl::HP_TWAI0_CTRL_SPEC>;
#[doc = "hp_twai0 read/write control register"]
pub mod hp_twai0_ctrl;
#[doc = "HP_TWAI1_CTRL (rw) register accessor: hp_twai1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai1_ctrl`] module"]
pub type HP_TWAI1_CTRL = crate::Reg<hp_twai1_ctrl::HP_TWAI1_CTRL_SPEC>;
#[doc = "hp_twai1 read/write control register"]
pub mod hp_twai1_ctrl;
#[doc = "HP_LCDCAM_CTRL (rw) register accessor: hp_lcdcam read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_lcdcam_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_lcdcam_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_lcdcam_ctrl`] module"]
pub type HP_LCDCAM_CTRL = crate::Reg<hp_lcdcam_ctrl::HP_LCDCAM_CTRL_SPEC>;
#[doc = "hp_lcdcam read/write control register"]
pub mod hp_lcdcam_ctrl;
#[doc = "HP_UHCI_CTRL (rw) register accessor: hp_uhci read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_uhci_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_uhci_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uhci_ctrl`] module"]
pub type HP_UHCI_CTRL = crate::Reg<hp_uhci_ctrl::HP_UHCI_CTRL_SPEC>;
#[doc = "hp_uhci read/write control register"]
pub mod hp_uhci_ctrl;
#[doc = "HP_SYSTIMER_CTRL (rw) register accessor: hp_systimer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_systimer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_systimer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_systimer_ctrl`] module"]
pub type HP_SYSTIMER_CTRL = crate::Reg<hp_systimer_ctrl::HP_SYSTIMER_CTRL_SPEC>;
#[doc = "hp_systimer read/write control register"]
pub mod hp_systimer_ctrl;
#[doc = "HP_ZERO_DET_CTRL (rw) register accessor: hp_zero_det read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_zero_det_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_zero_det_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_zero_det_ctrl`] module"]
pub type HP_ZERO_DET_CTRL = crate::Reg<hp_zero_det_ctrl::HP_ZERO_DET_CTRL_SPEC>;
#[doc = "hp_zero_det read/write control register"]
pub mod hp_zero_det_ctrl;
#[doc = "HP_CORDIC_CTRL (rw) register accessor: hp_cordic read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cordic_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cordic_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cordic_ctrl`] module"]
pub type HP_CORDIC_CTRL = crate::Reg<hp_cordic_ctrl::HP_CORDIC_CTRL_SPEC>;
#[doc = "hp_cordic read/write control register"]
pub mod hp_cordic_ctrl;
#[doc = "HP_LEDC1_CTRL (rw) register accessor: hp_ledc1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ledc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ledc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ledc1_ctrl`] module"]
pub type HP_LEDC1_CTRL = crate::Reg<hp_ledc1_ctrl::HP_LEDC1_CTRL_SPEC>;
#[doc = "hp_ledc1 read/write control register"]
pub mod hp_ledc1_ctrl;
#[doc = "HP_PCNT1_CTRL (rw) register accessor: hp_pcnt1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pcnt1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pcnt1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pcnt1_ctrl`] module"]
pub type HP_PCNT1_CTRL = crate::Reg<hp_pcnt1_ctrl::HP_PCNT1_CTRL_SPEC>;
#[doc = "hp_pcnt1 read/write control register"]
pub mod hp_pcnt1_ctrl;
#[doc = "HP_TIMER_GROUP0_CTRL (rw) register accessor: hp_timer_group0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_timer_group0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_timer_group0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_timer_group0_ctrl`] module"]
pub type HP_TIMER_GROUP0_CTRL = crate::Reg<hp_timer_group0_ctrl::HP_TIMER_GROUP0_CTRL_SPEC>;
#[doc = "hp_timer_group0 read/write control register"]
pub mod hp_timer_group0_ctrl;
#[doc = "HP_TIMER_GROUP1_CTRL (rw) register accessor: hp_timer_group1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_timer_group1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_timer_group1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_timer_group1_ctrl`] module"]
pub type HP_TIMER_GROUP1_CTRL = crate::Reg<hp_timer_group1_ctrl::HP_TIMER_GROUP1_CTRL_SPEC>;
#[doc = "hp_timer_group1 read/write control register"]
pub mod hp_timer_group1_ctrl;
#[doc = "HP_IOMUX_CTRL (rw) register accessor: hp_iomux read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_iomux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_iomux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_iomux_ctrl`] module"]
pub type HP_IOMUX_CTRL = crate::Reg<hp_iomux_ctrl::HP_IOMUX_CTRL_SPEC>;
#[doc = "hp_iomux read/write control register"]
pub mod hp_iomux_ctrl;
#[doc = "HP_MSPI_PADCTRL_CTRL (rw) register accessor: hp_mspi_padctrl read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mspi_padctrl_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mspi_padctrl_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mspi_padctrl_ctrl`] module"]
pub type HP_MSPI_PADCTRL_CTRL = crate::Reg<hp_mspi_padctrl_ctrl::HP_MSPI_PADCTRL_CTRL_SPEC>;
#[doc = "hp_mspi_padctrl read/write control register"]
pub mod hp_mspi_padctrl_ctrl;
#[doc = "HP_INTRMTX_CTRL (rw) register accessor: hp_intrmtx read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_intrmtx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_intrmtx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_intrmtx_ctrl`] module"]
pub type HP_INTRMTX_CTRL = crate::Reg<hp_intrmtx_ctrl::HP_INTRMTX_CTRL_SPEC>;
#[doc = "hp_intrmtx read/write control register"]
pub mod hp_intrmtx_ctrl;
#[doc = "HP_SYS_REG_CTRL (rw) register accessor: hp_sys_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sys_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sys_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sys_reg_ctrl`] module"]
pub type HP_SYS_REG_CTRL = crate::Reg<hp_sys_reg_ctrl::HP_SYS_REG_CTRL_SPEC>;
#[doc = "hp_sys_reg read/write control register"]
pub mod hp_sys_reg_ctrl;
#[doc = "HP_CLKRST_CTRL (rw) register accessor: hp_clkrst read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clkrst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_clkrst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_clkrst_ctrl`] module"]
pub type HP_CLKRST_CTRL = crate::Reg<hp_clkrst_ctrl::HP_CLKRST_CTRL_SPEC>;
#[doc = "hp_clkrst read/write control register"]
pub mod hp_clkrst_ctrl;
#[doc = "CNNT_PAD_CTRL_CTRL (rw) register accessor: cnnt_pad_ctrl read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_pad_ctrl_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_pad_ctrl_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnnt_pad_ctrl_ctrl`] module"]
pub type CNNT_PAD_CTRL_CTRL = crate::Reg<cnnt_pad_ctrl_ctrl::CNNT_PAD_CTRL_CTRL_SPEC>;
#[doc = "cnnt_pad_ctrl read/write control register"]
pub mod cnnt_pad_ctrl_ctrl;
#[doc = "HP_ALIVE_SYS_REG_CTRL (rw) register accessor: hp_alive_sys_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_alive_sys_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_alive_sys_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_alive_sys_reg_ctrl`] module"]
pub type HP_ALIVE_SYS_REG_CTRL = crate::Reg<hp_alive_sys_reg_ctrl::HP_ALIVE_SYS_REG_CTRL_SPEC>;
#[doc = "hp_alive_sys_reg read/write control register"]
pub mod hp_alive_sys_reg_ctrl;
#[doc = "HP_PERI1_PMS_CTRL (rw) register accessor: hp_peri1_pms read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_pms_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri1_pms_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_pms_ctrl`] module"]
pub type HP_PERI1_PMS_CTRL = crate::Reg<hp_peri1_pms_ctrl::HP_PERI1_PMS_CTRL_SPEC>;
#[doc = "hp_peri1_pms read/write control register"]
pub mod hp_peri1_pms_ctrl;
#[doc = "HP_PERI1_0 (rw) register accessor: HP_PERI1 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_0`] module"]
pub type HP_PERI1_0 = crate::Reg<hp_peri1_0::HP_PERI1_0_SPEC>;
#[doc = "HP_PERI1 PMS configuration & info register"]
pub mod hp_peri1_0;
#[doc = "HP_PERI1_1 (r) register accessor: HP_PERI1 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_1`] module"]
pub type HP_PERI1_1 = crate::Reg<hp_peri1_1::HP_PERI1_1_SPEC>;
#[doc = "HP_PERI1 PMS exception addr record register"]
pub mod hp_peri1_1;
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
