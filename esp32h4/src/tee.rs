#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    m_mode_ctrl: [M_MODE_CTRL; 32],
    gpspi0_ctrl: GPSPI0_CTRL,
    gpspi1_ctrl: GPSPI1_CTRL,
    uart0_ctrl: UART0_CTRL,
    uart1_ctrl: UART1_CTRL,
    uhci_ctrl: UHCI_CTRL,
    i2c0_ctrl: I2C0_CTRL,
    i2c1_ctrl: I2C1_CTRL,
    i2s_ctrl: I2S_CTRL,
    parl_io_ctrl: PARL_IO_CTRL,
    pwm0_ctrl: PWM0_CTRL,
    pwm1_ctrl: PWM1_CTRL,
    ledc_ctrl: LEDC_CTRL,
    can_ctrl: CAN_CTRL,
    usb_serial_jtag_ctrl: USB_SERIAL_JTAG_CTRL,
    rmt_ctrl: RMT_CTRL,
    gdma_ctrl: GDMA_CTRL,
    regdma_ctrl: REGDMA_CTRL,
    etm_ctrl: ETM_CTRL,
    intmtx_core0_ctrl: INTMTX_CORE0_CTRL,
    intmtx_core1_ctrl: INTMTX_CORE1_CTRL,
    apb_adc_ctrl: APB_ADC_CTRL,
    timergroup0_ctrl: TIMERGROUP0_CTRL,
    timergroup1_ctrl: TIMERGROUP1_CTRL,
    systimer_ctrl: SYSTIMER_CTRL,
    misc_ctrl: MISC_CTRL,
    src_ctrl: SRC_CTRL,
    usb_otg_fs_core_ctrl: USB_OTG_FS_CORE_CTRL,
    usb_otg_fs_phy_ctrl: USB_OTG_FS_PHY_CTRL,
    pvt_monitor_ctrl: PVT_MONITOR_CTRL,
    pcnt_ctrl: PCNT_CTRL,
    iomux_ctrl: IOMUX_CTRL,
    psram_mem_monitor_ctrl: PSRAM_MEM_MONITOR_CTRL,
    mem_acs_monitor_ctrl: MEM_ACS_MONITOR_CTRL,
    hp_system_reg_ctrl: HP_SYSTEM_REG_CTRL,
    pcr_reg_ctrl: PCR_REG_CTRL,
    mspi_ctrl: MSPI_CTRL,
    hp_apm_ctrl: HP_APM_CTRL,
    hp_mem_apm_ctrl: HP_MEM_APM_CTRL,
    cpu_apm_ctrl: CPU_APM_CTRL,
    tee_ctrl: TEE_CTRL,
    km_ctrl: KM_CTRL,
    crypt_ctrl: CRYPT_CTRL,
    core0_trace_ctrl: CORE0_TRACE_CTRL,
    core1_trace_ctrl: CORE1_TRACE_CTRL,
    cpu_bus_monitor_ctrl: CPU_BUS_MONITOR_CTRL,
    intpri_reg_ctrl: INTPRI_REG_CTRL,
    cache_cfg_ctrl: CACHE_CFG_CTRL,
    modem_ctrl: MODEM_CTRL,
    zero_det_ctrl: ZERO_DET_CTRL,
    _reserved50: [u8; 0x0eac],
    bus_err_conf: BUS_ERR_CONF,
    _reserved51: [u8; 0x04],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - TEE mode control register"]
    #[inline(always)]
    pub const fn m_mode_ctrl(&self, n: usize) -> &M_MODE_CTRL {
        &self.m_mode_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - TEE mode control register"]
    #[inline(always)]
    pub fn m_mode_ctrl_iter(&self) -> impl Iterator<Item = &M_MODE_CTRL> {
        self.m_mode_ctrl.iter()
    }
    #[doc = "0x00 - TEE mode control register"]
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(0)
    }
    #[doc = "0x04 - TEE mode control register"]
    #[inline(always)]
    pub const fn m1_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(1)
    }
    #[doc = "0x08 - TEE mode control register"]
    #[inline(always)]
    pub const fn m2_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(2)
    }
    #[doc = "0x0c - TEE mode control register"]
    #[inline(always)]
    pub const fn m3_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(3)
    }
    #[doc = "0x10 - TEE mode control register"]
    #[inline(always)]
    pub const fn m4_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(4)
    }
    #[doc = "0x14 - TEE mode control register"]
    #[inline(always)]
    pub const fn m5_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(5)
    }
    #[doc = "0x18 - TEE mode control register"]
    #[inline(always)]
    pub const fn m6_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(6)
    }
    #[doc = "0x1c - TEE mode control register"]
    #[inline(always)]
    pub const fn m7_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(7)
    }
    #[doc = "0x20 - TEE mode control register"]
    #[inline(always)]
    pub const fn m8_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(8)
    }
    #[doc = "0x24 - TEE mode control register"]
    #[inline(always)]
    pub const fn m9_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(9)
    }
    #[doc = "0x28 - TEE mode control register"]
    #[inline(always)]
    pub const fn m10_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(10)
    }
    #[doc = "0x2c - TEE mode control register"]
    #[inline(always)]
    pub const fn m11_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(11)
    }
    #[doc = "0x30 - TEE mode control register"]
    #[inline(always)]
    pub const fn m12_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(12)
    }
    #[doc = "0x34 - TEE mode control register"]
    #[inline(always)]
    pub const fn m13_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(13)
    }
    #[doc = "0x38 - TEE mode control register"]
    #[inline(always)]
    pub const fn m14_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(14)
    }
    #[doc = "0x3c - TEE mode control register"]
    #[inline(always)]
    pub const fn m15_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(15)
    }
    #[doc = "0x40 - TEE mode control register"]
    #[inline(always)]
    pub const fn m16_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(16)
    }
    #[doc = "0x44 - TEE mode control register"]
    #[inline(always)]
    pub const fn m17_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(17)
    }
    #[doc = "0x48 - TEE mode control register"]
    #[inline(always)]
    pub const fn m18_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(18)
    }
    #[doc = "0x4c - TEE mode control register"]
    #[inline(always)]
    pub const fn m19_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(19)
    }
    #[doc = "0x50 - TEE mode control register"]
    #[inline(always)]
    pub const fn m20_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(20)
    }
    #[doc = "0x54 - TEE mode control register"]
    #[inline(always)]
    pub const fn m21_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(21)
    }
    #[doc = "0x58 - TEE mode control register"]
    #[inline(always)]
    pub const fn m22_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(22)
    }
    #[doc = "0x5c - TEE mode control register"]
    #[inline(always)]
    pub const fn m23_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(23)
    }
    #[doc = "0x60 - TEE mode control register"]
    #[inline(always)]
    pub const fn m24_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(24)
    }
    #[doc = "0x64 - TEE mode control register"]
    #[inline(always)]
    pub const fn m25_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(25)
    }
    #[doc = "0x68 - TEE mode control register"]
    #[inline(always)]
    pub const fn m26_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(26)
    }
    #[doc = "0x6c - TEE mode control register"]
    #[inline(always)]
    pub const fn m27_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(27)
    }
    #[doc = "0x70 - TEE mode control register"]
    #[inline(always)]
    pub const fn m28_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(28)
    }
    #[doc = "0x74 - TEE mode control register"]
    #[inline(always)]
    pub const fn m29_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(29)
    }
    #[doc = "0x78 - TEE mode control register"]
    #[inline(always)]
    pub const fn m30_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(30)
    }
    #[doc = "0x7c - TEE mode control register"]
    #[inline(always)]
    pub const fn m31_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(31)
    }
    #[doc = "0x80 - gpspi0 read/write control register"]
    #[inline(always)]
    pub const fn gpspi0_ctrl(&self) -> &GPSPI0_CTRL {
        &self.gpspi0_ctrl
    }
    #[doc = "0x84 - gpspi1 read/write control register"]
    #[inline(always)]
    pub const fn gpspi1_ctrl(&self) -> &GPSPI1_CTRL {
        &self.gpspi1_ctrl
    }
    #[doc = "0x88 - uart0 read/write control register"]
    #[inline(always)]
    pub const fn uart0_ctrl(&self) -> &UART0_CTRL {
        &self.uart0_ctrl
    }
    #[doc = "0x8c - uart1 read/write control register"]
    #[inline(always)]
    pub const fn uart1_ctrl(&self) -> &UART1_CTRL {
        &self.uart1_ctrl
    }
    #[doc = "0x90 - uhci read/write control register"]
    #[inline(always)]
    pub const fn uhci_ctrl(&self) -> &UHCI_CTRL {
        &self.uhci_ctrl
    }
    #[doc = "0x94 - i2c0 read/write control register"]
    #[inline(always)]
    pub const fn i2c0_ctrl(&self) -> &I2C0_CTRL {
        &self.i2c0_ctrl
    }
    #[doc = "0x98 - i2c1 read/write control register"]
    #[inline(always)]
    pub const fn i2c1_ctrl(&self) -> &I2C1_CTRL {
        &self.i2c1_ctrl
    }
    #[doc = "0x9c - i2s read/write control register"]
    #[inline(always)]
    pub const fn i2s_ctrl(&self) -> &I2S_CTRL {
        &self.i2s_ctrl
    }
    #[doc = "0xa0 - parl_io read/write control register"]
    #[inline(always)]
    pub const fn parl_io_ctrl(&self) -> &PARL_IO_CTRL {
        &self.parl_io_ctrl
    }
    #[doc = "0xa4 - pwm0 read/write control register"]
    #[inline(always)]
    pub const fn pwm0_ctrl(&self) -> &PWM0_CTRL {
        &self.pwm0_ctrl
    }
    #[doc = "0xa8 - pwm1 read/write control register"]
    #[inline(always)]
    pub const fn pwm1_ctrl(&self) -> &PWM1_CTRL {
        &self.pwm1_ctrl
    }
    #[doc = "0xac - ledc read/write control register"]
    #[inline(always)]
    pub const fn ledc_ctrl(&self) -> &LEDC_CTRL {
        &self.ledc_ctrl
    }
    #[doc = "0xb0 - can read/write control register"]
    #[inline(always)]
    pub const fn can_ctrl(&self) -> &CAN_CTRL {
        &self.can_ctrl
    }
    #[doc = "0xb4 - usb_serial_jtag read/write control register"]
    #[inline(always)]
    pub const fn usb_serial_jtag_ctrl(&self) -> &USB_SERIAL_JTAG_CTRL {
        &self.usb_serial_jtag_ctrl
    }
    #[doc = "0xb8 - rmt read/write control register"]
    #[inline(always)]
    pub const fn rmt_ctrl(&self) -> &RMT_CTRL {
        &self.rmt_ctrl
    }
    #[doc = "0xbc - gdma read/write control register"]
    #[inline(always)]
    pub const fn gdma_ctrl(&self) -> &GDMA_CTRL {
        &self.gdma_ctrl
    }
    #[doc = "0xc0 - regdma read/write control register"]
    #[inline(always)]
    pub const fn regdma_ctrl(&self) -> &REGDMA_CTRL {
        &self.regdma_ctrl
    }
    #[doc = "0xc4 - etm read/write control register"]
    #[inline(always)]
    pub const fn etm_ctrl(&self) -> &ETM_CTRL {
        &self.etm_ctrl
    }
    #[doc = "0xc8 - intmtx_core0 read/write control register"]
    #[inline(always)]
    pub const fn intmtx_core0_ctrl(&self) -> &INTMTX_CORE0_CTRL {
        &self.intmtx_core0_ctrl
    }
    #[doc = "0xcc - intmtx_core1 read/write control register"]
    #[inline(always)]
    pub const fn intmtx_core1_ctrl(&self) -> &INTMTX_CORE1_CTRL {
        &self.intmtx_core1_ctrl
    }
    #[doc = "0xd0 - apb_adc read/write control register"]
    #[inline(always)]
    pub const fn apb_adc_ctrl(&self) -> &APB_ADC_CTRL {
        &self.apb_adc_ctrl
    }
    #[doc = "0xd4 - timergroup0 read/write control register"]
    #[inline(always)]
    pub const fn timergroup0_ctrl(&self) -> &TIMERGROUP0_CTRL {
        &self.timergroup0_ctrl
    }
    #[doc = "0xd8 - timergroup1 read/write control register"]
    #[inline(always)]
    pub const fn timergroup1_ctrl(&self) -> &TIMERGROUP1_CTRL {
        &self.timergroup1_ctrl
    }
    #[doc = "0xdc - systimer read/write control register"]
    #[inline(always)]
    pub const fn systimer_ctrl(&self) -> &SYSTIMER_CTRL {
        &self.systimer_ctrl
    }
    #[doc = "0xe0 - misc read/write control register"]
    #[inline(always)]
    pub const fn misc_ctrl(&self) -> &MISC_CTRL {
        &self.misc_ctrl
    }
    #[doc = "0xe4 - src read/write control register"]
    #[inline(always)]
    pub const fn src_ctrl(&self) -> &SRC_CTRL {
        &self.src_ctrl
    }
    #[doc = "0xe8 - usb_otg_fs_core read/write control register"]
    #[inline(always)]
    pub const fn usb_otg_fs_core_ctrl(&self) -> &USB_OTG_FS_CORE_CTRL {
        &self.usb_otg_fs_core_ctrl
    }
    #[doc = "0xec - usb_otg_fs_phy read/write control register"]
    #[inline(always)]
    pub const fn usb_otg_fs_phy_ctrl(&self) -> &USB_OTG_FS_PHY_CTRL {
        &self.usb_otg_fs_phy_ctrl
    }
    #[doc = "0xf0 - pvt_monitor read/write control register"]
    #[inline(always)]
    pub const fn pvt_monitor_ctrl(&self) -> &PVT_MONITOR_CTRL {
        &self.pvt_monitor_ctrl
    }
    #[doc = "0xf4 - pcnt read/write control register"]
    #[inline(always)]
    pub const fn pcnt_ctrl(&self) -> &PCNT_CTRL {
        &self.pcnt_ctrl
    }
    #[doc = "0xf8 - iomux read/write control register"]
    #[inline(always)]
    pub const fn iomux_ctrl(&self) -> &IOMUX_CTRL {
        &self.iomux_ctrl
    }
    #[doc = "0xfc - psram_mem_monitor read/write control register"]
    #[inline(always)]
    pub const fn psram_mem_monitor_ctrl(&self) -> &PSRAM_MEM_MONITOR_CTRL {
        &self.psram_mem_monitor_ctrl
    }
    #[doc = "0x100 - mem_acs_monitor read/write control register"]
    #[inline(always)]
    pub const fn mem_acs_monitor_ctrl(&self) -> &MEM_ACS_MONITOR_CTRL {
        &self.mem_acs_monitor_ctrl
    }
    #[doc = "0x104 - hp_system_reg read/write control register"]
    #[inline(always)]
    pub const fn hp_system_reg_ctrl(&self) -> &HP_SYSTEM_REG_CTRL {
        &self.hp_system_reg_ctrl
    }
    #[doc = "0x108 - pcr_reg read/write control register"]
    #[inline(always)]
    pub const fn pcr_reg_ctrl(&self) -> &PCR_REG_CTRL {
        &self.pcr_reg_ctrl
    }
    #[doc = "0x10c - mspi read/write control register"]
    #[inline(always)]
    pub const fn mspi_ctrl(&self) -> &MSPI_CTRL {
        &self.mspi_ctrl
    }
    #[doc = "0x110 - hp_apm read/write control register"]
    #[inline(always)]
    pub const fn hp_apm_ctrl(&self) -> &HP_APM_CTRL {
        &self.hp_apm_ctrl
    }
    #[doc = "0x114 - hp_mem_apm read/write control register"]
    #[inline(always)]
    pub const fn hp_mem_apm_ctrl(&self) -> &HP_MEM_APM_CTRL {
        &self.hp_mem_apm_ctrl
    }
    #[doc = "0x118 - cpu_apm read/write control register"]
    #[inline(always)]
    pub const fn cpu_apm_ctrl(&self) -> &CPU_APM_CTRL {
        &self.cpu_apm_ctrl
    }
    #[doc = "0x11c - tee read/write control register"]
    #[inline(always)]
    pub const fn tee_ctrl(&self) -> &TEE_CTRL {
        &self.tee_ctrl
    }
    #[doc = "0x120 - crypt read/write control register"]
    #[inline(always)]
    pub const fn km_ctrl(&self) -> &KM_CTRL {
        &self.km_ctrl
    }
    #[doc = "0x124 - crypt read/write control register"]
    #[inline(always)]
    pub const fn crypt_ctrl(&self) -> &CRYPT_CTRL {
        &self.crypt_ctrl
    }
    #[doc = "0x128 - core0_trace read/write control register"]
    #[inline(always)]
    pub const fn core0_trace_ctrl(&self) -> &CORE0_TRACE_CTRL {
        &self.core0_trace_ctrl
    }
    #[doc = "0x12c - core1_trace read/write control register"]
    #[inline(always)]
    pub const fn core1_trace_ctrl(&self) -> &CORE1_TRACE_CTRL {
        &self.core1_trace_ctrl
    }
    #[doc = "0x130 - cpu_bus_monitor read/write control register"]
    #[inline(always)]
    pub const fn cpu_bus_monitor_ctrl(&self) -> &CPU_BUS_MONITOR_CTRL {
        &self.cpu_bus_monitor_ctrl
    }
    #[doc = "0x134 - intpri_reg read/write control register"]
    #[inline(always)]
    pub const fn intpri_reg_ctrl(&self) -> &INTPRI_REG_CTRL {
        &self.intpri_reg_ctrl
    }
    #[doc = "0x138 - cache_cfg read/write control register"]
    #[inline(always)]
    pub const fn cache_cfg_ctrl(&self) -> &CACHE_CFG_CTRL {
        &self.cache_cfg_ctrl
    }
    #[doc = "0x13c - modem read/write control register"]
    #[inline(always)]
    pub const fn modem_ctrl(&self) -> &MODEM_CTRL {
        &self.modem_ctrl
    }
    #[doc = "0x140 - zero_det read/write control register"]
    #[inline(always)]
    pub const fn zero_det_ctrl(&self) -> &ZERO_DET_CTRL {
        &self.zero_det_ctrl
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
#[doc = "M_MODE_CTRL (rw) register accessor: TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_mode_ctrl`] module"]
pub type M_MODE_CTRL = crate::Reg<m_mode_ctrl::M_MODE_CTRL_SPEC>;
#[doc = "TEE mode control register"]
pub mod m_mode_ctrl;
#[doc = "GPSPI0_CTRL (rw) register accessor: gpspi0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi0_ctrl`] module"]
pub type GPSPI0_CTRL = crate::Reg<gpspi0_ctrl::GPSPI0_CTRL_SPEC>;
#[doc = "gpspi0 read/write control register"]
pub mod gpspi0_ctrl;
#[doc = "GPSPI1_CTRL (rw) register accessor: gpspi1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi1_ctrl`] module"]
pub type GPSPI1_CTRL = crate::Reg<gpspi1_ctrl::GPSPI1_CTRL_SPEC>;
#[doc = "gpspi1 read/write control register"]
pub mod gpspi1_ctrl;
#[doc = "UART0_CTRL (rw) register accessor: uart0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_ctrl`] module"]
pub type UART0_CTRL = crate::Reg<uart0_ctrl::UART0_CTRL_SPEC>;
#[doc = "uart0 read/write control register"]
pub mod uart0_ctrl;
#[doc = "UART1_CTRL (rw) register accessor: uart1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_ctrl`] module"]
pub type UART1_CTRL = crate::Reg<uart1_ctrl::UART1_CTRL_SPEC>;
#[doc = "uart1 read/write control register"]
pub mod uart1_ctrl;
#[doc = "UHCI_CTRL (rw) register accessor: uhci read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci_ctrl`] module"]
pub type UHCI_CTRL = crate::Reg<uhci_ctrl::UHCI_CTRL_SPEC>;
#[doc = "uhci read/write control register"]
pub mod uhci_ctrl;
#[doc = "I2C0_CTRL (rw) register accessor: i2c0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl`] module"]
pub type I2C0_CTRL = crate::Reg<i2c0_ctrl::I2C0_CTRL_SPEC>;
#[doc = "i2c0 read/write control register"]
pub mod i2c0_ctrl;
#[doc = "I2C1_CTRL (rw) register accessor: i2c1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl`] module"]
pub type I2C1_CTRL = crate::Reg<i2c1_ctrl::I2C1_CTRL_SPEC>;
#[doc = "i2c1 read/write control register"]
pub mod i2c1_ctrl;
#[doc = "I2S_CTRL (rw) register accessor: i2s read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_ctrl`] module"]
pub type I2S_CTRL = crate::Reg<i2s_ctrl::I2S_CTRL_SPEC>;
#[doc = "i2s read/write control register"]
pub mod i2s_ctrl;
#[doc = "PARL_IO_CTRL (rw) register accessor: parl_io read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_io_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_io_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parl_io_ctrl`] module"]
pub type PARL_IO_CTRL = crate::Reg<parl_io_ctrl::PARL_IO_CTRL_SPEC>;
#[doc = "parl_io read/write control register"]
pub mod parl_io_ctrl;
#[doc = "PWM0_CTRL (rw) register accessor: pwm0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_ctrl`] module"]
pub type PWM0_CTRL = crate::Reg<pwm0_ctrl::PWM0_CTRL_SPEC>;
#[doc = "pwm0 read/write control register"]
pub mod pwm0_ctrl;
#[doc = "PWM1_CTRL (rw) register accessor: pwm1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_ctrl`] module"]
pub type PWM1_CTRL = crate::Reg<pwm1_ctrl::PWM1_CTRL_SPEC>;
#[doc = "pwm1 read/write control register"]
pub mod pwm1_ctrl;
#[doc = "LEDC_CTRL (rw) register accessor: ledc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_ctrl`] module"]
pub type LEDC_CTRL = crate::Reg<ledc_ctrl::LEDC_CTRL_SPEC>;
#[doc = "ledc read/write control register"]
pub mod ledc_ctrl;
#[doc = "CAN_CTRL (rw) register accessor: can read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ctrl`] module"]
pub type CAN_CTRL = crate::Reg<can_ctrl::CAN_CTRL_SPEC>;
#[doc = "can read/write control register"]
pub mod can_ctrl;
#[doc = "USB_SERIAL_JTAG_CTRL (rw) register accessor: usb_serial_jtag read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_ctrl`] module"]
pub type USB_SERIAL_JTAG_CTRL = crate::Reg<usb_serial_jtag_ctrl::USB_SERIAL_JTAG_CTRL_SPEC>;
#[doc = "usb_serial_jtag read/write control register"]
pub mod usb_serial_jtag_ctrl;
#[doc = "RMT_CTRL (rw) register accessor: rmt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_ctrl`] module"]
pub type RMT_CTRL = crate::Reg<rmt_ctrl::RMT_CTRL_SPEC>;
#[doc = "rmt read/write control register"]
pub mod rmt_ctrl;
#[doc = "GDMA_CTRL (rw) register accessor: gdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctrl`] module"]
pub type GDMA_CTRL = crate::Reg<gdma_ctrl::GDMA_CTRL_SPEC>;
#[doc = "gdma read/write control register"]
pub mod gdma_ctrl;
#[doc = "REGDMA_CTRL (rw) register accessor: regdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_ctrl`] module"]
pub type REGDMA_CTRL = crate::Reg<regdma_ctrl::REGDMA_CTRL_SPEC>;
#[doc = "regdma read/write control register"]
pub mod regdma_ctrl;
#[doc = "ETM_CTRL (rw) register accessor: etm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_ctrl`] module"]
pub type ETM_CTRL = crate::Reg<etm_ctrl::ETM_CTRL_SPEC>;
#[doc = "etm read/write control register"]
pub mod etm_ctrl;
#[doc = "INTMTX_CORE0_CTRL (rw) register accessor: intmtx_core0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmtx_core0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmtx_core0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmtx_core0_ctrl`] module"]
pub type INTMTX_CORE0_CTRL = crate::Reg<intmtx_core0_ctrl::INTMTX_CORE0_CTRL_SPEC>;
#[doc = "intmtx_core0 read/write control register"]
pub mod intmtx_core0_ctrl;
#[doc = "INTMTX_CORE1_CTRL (rw) register accessor: intmtx_core1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmtx_core1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmtx_core1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmtx_core1_ctrl`] module"]
pub type INTMTX_CORE1_CTRL = crate::Reg<intmtx_core1_ctrl::INTMTX_CORE1_CTRL_SPEC>;
#[doc = "intmtx_core1 read/write control register"]
pub mod intmtx_core1_ctrl;
#[doc = "APB_ADC_CTRL (rw) register accessor: apb_adc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_adc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_adc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_adc_ctrl`] module"]
pub type APB_ADC_CTRL = crate::Reg<apb_adc_ctrl::APB_ADC_CTRL_SPEC>;
#[doc = "apb_adc read/write control register"]
pub mod apb_adc_ctrl;
#[doc = "TIMERGROUP0_CTRL (rw) register accessor: timergroup0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_ctrl`] module"]
pub type TIMERGROUP0_CTRL = crate::Reg<timergroup0_ctrl::TIMERGROUP0_CTRL_SPEC>;
#[doc = "timergroup0 read/write control register"]
pub mod timergroup0_ctrl;
#[doc = "TIMERGROUP1_CTRL (rw) register accessor: timergroup1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_ctrl`] module"]
pub type TIMERGROUP1_CTRL = crate::Reg<timergroup1_ctrl::TIMERGROUP1_CTRL_SPEC>;
#[doc = "timergroup1 read/write control register"]
pub mod timergroup1_ctrl;
#[doc = "SYSTIMER_CTRL (rw) register accessor: systimer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_ctrl`] module"]
pub type SYSTIMER_CTRL = crate::Reg<systimer_ctrl::SYSTIMER_CTRL_SPEC>;
#[doc = "systimer read/write control register"]
pub mod systimer_ctrl;
#[doc = "MISC_CTRL (rw) register accessor: misc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl`] module"]
pub type MISC_CTRL = crate::Reg<misc_ctrl::MISC_CTRL_SPEC>;
#[doc = "misc read/write control register"]
pub mod misc_ctrl;
#[doc = "SRC_CTRL (rw) register accessor: src read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`src_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_ctrl`] module"]
pub type SRC_CTRL = crate::Reg<src_ctrl::SRC_CTRL_SPEC>;
#[doc = "src read/write control register"]
pub mod src_ctrl;
#[doc = "USB_OTG_FS_CORE_CTRL (rw) register accessor: usb_otg_fs_core read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otg_fs_core_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otg_fs_core_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_fs_core_ctrl`] module"]
pub type USB_OTG_FS_CORE_CTRL = crate::Reg<usb_otg_fs_core_ctrl::USB_OTG_FS_CORE_CTRL_SPEC>;
#[doc = "usb_otg_fs_core read/write control register"]
pub mod usb_otg_fs_core_ctrl;
#[doc = "USB_OTG_FS_PHY_CTRL (rw) register accessor: usb_otg_fs_phy read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otg_fs_phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otg_fs_phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_fs_phy_ctrl`] module"]
pub type USB_OTG_FS_PHY_CTRL = crate::Reg<usb_otg_fs_phy_ctrl::USB_OTG_FS_PHY_CTRL_SPEC>;
#[doc = "usb_otg_fs_phy read/write control register"]
pub mod usb_otg_fs_phy_ctrl;
#[doc = "PVT_MONITOR_CTRL (rw) register accessor: pvt_monitor read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_ctrl`] module"]
pub type PVT_MONITOR_CTRL = crate::Reg<pvt_monitor_ctrl::PVT_MONITOR_CTRL_SPEC>;
#[doc = "pvt_monitor read/write control register"]
pub mod pvt_monitor_ctrl;
#[doc = "PCNT_CTRL (rw) register accessor: pcnt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt_ctrl`] module"]
pub type PCNT_CTRL = crate::Reg<pcnt_ctrl::PCNT_CTRL_SPEC>;
#[doc = "pcnt read/write control register"]
pub mod pcnt_ctrl;
#[doc = "IOMUX_CTRL (rw) register accessor: iomux read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_ctrl`] module"]
pub type IOMUX_CTRL = crate::Reg<iomux_ctrl::IOMUX_CTRL_SPEC>;
#[doc = "iomux read/write control register"]
pub mod iomux_ctrl;
#[doc = "PSRAM_MEM_MONITOR_CTRL (rw) register accessor: psram_mem_monitor read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mem_monitor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mem_monitor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mem_monitor_ctrl`] module"]
pub type PSRAM_MEM_MONITOR_CTRL = crate::Reg<psram_mem_monitor_ctrl::PSRAM_MEM_MONITOR_CTRL_SPEC>;
#[doc = "psram_mem_monitor read/write control register"]
pub mod psram_mem_monitor_ctrl;
#[doc = "MEM_ACS_MONITOR_CTRL (rw) register accessor: mem_acs_monitor read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_acs_monitor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_acs_monitor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_acs_monitor_ctrl`] module"]
pub type MEM_ACS_MONITOR_CTRL = crate::Reg<mem_acs_monitor_ctrl::MEM_ACS_MONITOR_CTRL_SPEC>;
#[doc = "mem_acs_monitor read/write control register"]
pub mod mem_acs_monitor_ctrl;
#[doc = "HP_SYSTEM_REG_CTRL (rw) register accessor: hp_system_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_system_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_system_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_system_reg_ctrl`] module"]
pub type HP_SYSTEM_REG_CTRL = crate::Reg<hp_system_reg_ctrl::HP_SYSTEM_REG_CTRL_SPEC>;
#[doc = "hp_system_reg read/write control register"]
pub mod hp_system_reg_ctrl;
#[doc = "PCR_REG_CTRL (rw) register accessor: pcr_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr_reg_ctrl`] module"]
pub type PCR_REG_CTRL = crate::Reg<pcr_reg_ctrl::PCR_REG_CTRL_SPEC>;
#[doc = "pcr_reg read/write control register"]
pub mod pcr_reg_ctrl;
#[doc = "MSPI_CTRL (rw) register accessor: mspi read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_ctrl`] module"]
pub type MSPI_CTRL = crate::Reg<mspi_ctrl::MSPI_CTRL_SPEC>;
#[doc = "mspi read/write control register"]
pub mod mspi_ctrl;
#[doc = "HP_APM_CTRL (rw) register accessor: hp_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_ctrl`] module"]
pub type HP_APM_CTRL = crate::Reg<hp_apm_ctrl::HP_APM_CTRL_SPEC>;
#[doc = "hp_apm read/write control register"]
pub mod hp_apm_ctrl;
#[doc = "HP_MEM_APM_CTRL (rw) register accessor: hp_mem_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_ctrl`] module"]
pub type HP_MEM_APM_CTRL = crate::Reg<hp_mem_apm_ctrl::HP_MEM_APM_CTRL_SPEC>;
#[doc = "hp_mem_apm read/write control register"]
pub mod hp_mem_apm_ctrl;
#[doc = "CPU_APM_CTRL (rw) register accessor: cpu_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_ctrl`] module"]
pub type CPU_APM_CTRL = crate::Reg<cpu_apm_ctrl::CPU_APM_CTRL_SPEC>;
#[doc = "cpu_apm read/write control register"]
pub mod cpu_apm_ctrl;
#[doc = "TEE_CTRL (rw) register accessor: tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tee_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tee_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tee_ctrl`] module"]
pub type TEE_CTRL = crate::Reg<tee_ctrl::TEE_CTRL_SPEC>;
#[doc = "tee read/write control register"]
pub mod tee_ctrl;
#[doc = "KM_CTRL (rw) register accessor: crypt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_ctrl`] module"]
pub type KM_CTRL = crate::Reg<km_ctrl::KM_CTRL_SPEC>;
#[doc = "crypt read/write control register"]
pub mod km_ctrl;
#[doc = "CRYPT_CTRL (rw) register accessor: crypt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`crypt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypt_ctrl`] module"]
pub type CRYPT_CTRL = crate::Reg<crypt_ctrl::CRYPT_CTRL_SPEC>;
#[doc = "crypt read/write control register"]
pub mod crypt_ctrl;
#[doc = "CORE0_TRACE_CTRL (rw) register accessor: core0_trace read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_trace_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_trace_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_trace_ctrl`] module"]
pub type CORE0_TRACE_CTRL = crate::Reg<core0_trace_ctrl::CORE0_TRACE_CTRL_SPEC>;
#[doc = "core0_trace read/write control register"]
pub mod core0_trace_ctrl;
#[doc = "CORE1_TRACE_CTRL (rw) register accessor: core1_trace read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_trace_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_trace_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_trace_ctrl`] module"]
pub type CORE1_TRACE_CTRL = crate::Reg<core1_trace_ctrl::CORE1_TRACE_CTRL_SPEC>;
#[doc = "core1_trace read/write control register"]
pub mod core1_trace_ctrl;
#[doc = "CPU_BUS_MONITOR_CTRL (rw) register accessor: cpu_bus_monitor read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_bus_monitor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_bus_monitor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_bus_monitor_ctrl`] module"]
pub type CPU_BUS_MONITOR_CTRL = crate::Reg<cpu_bus_monitor_ctrl::CPU_BUS_MONITOR_CTRL_SPEC>;
#[doc = "cpu_bus_monitor read/write control register"]
pub mod cpu_bus_monitor_ctrl;
#[doc = "INTPRI_REG_CTRL (rw) register accessor: intpri_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpri_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpri_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpri_reg_ctrl`] module"]
pub type INTPRI_REG_CTRL = crate::Reg<intpri_reg_ctrl::INTPRI_REG_CTRL_SPEC>;
#[doc = "intpri_reg read/write control register"]
pub mod intpri_reg_ctrl;
#[doc = "CACHE_CFG_CTRL (rw) register accessor: cache_cfg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_cfg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_cfg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_cfg_ctrl`] module"]
pub type CACHE_CFG_CTRL = crate::Reg<cache_cfg_ctrl::CACHE_CFG_CTRL_SPEC>;
#[doc = "cache_cfg read/write control register"]
pub mod cache_cfg_ctrl;
#[doc = "MODEM_CTRL (rw) register accessor: modem read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ctrl`] module"]
pub type MODEM_CTRL = crate::Reg<modem_ctrl::MODEM_CTRL_SPEC>;
#[doc = "modem read/write control register"]
pub mod modem_ctrl;
#[doc = "ZERO_DET_CTRL (rw) register accessor: zero_det read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det_ctrl`] module"]
pub type ZERO_DET_CTRL = crate::Reg<zero_det_ctrl::ZERO_DET_CTRL_SPEC>;
#[doc = "zero_det read/write control register"]
pub mod zero_det_ctrl;
#[doc = "BUS_ERR_CONF (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_err_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_err_conf`] module"]
pub type BUS_ERR_CONF = crate::Reg<bus_err_conf::BUS_ERR_CONF_SPEC>;
#[doc = "Clock gating register"]
pub mod bus_err_conf;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
pub use crate::dma::{date, DATE};
