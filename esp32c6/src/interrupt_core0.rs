#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    wifi_mac_intr_map: WIFI_MAC_INTR_MAP,
    wifi_mac_nmi_map: WIFI_MAC_NMI_MAP,
    wifi_pwr_intr_map: WIFI_PWR_INTR_MAP,
    wifi_bb_intr_map: WIFI_BB_INTR_MAP,
    bt_mac_intr_map: BT_MAC_INTR_MAP,
    bt_bb_intr_map: BT_BB_INTR_MAP,
    bt_bb_nmi_map: BT_BB_NMI_MAP,
    lp_timer_intr_map: LP_TIMER_INTR_MAP,
    coex_intr_map: COEX_INTR_MAP,
    ble_timer_intr_map: BLE_TIMER_INTR_MAP,
    ble_sec_intr_map: BLE_SEC_INTR_MAP,
    i2c_mst_intr_map: I2C_MST_INTR_MAP,
    zb_mac_intr_map: ZB_MAC_INTR_MAP,
    pmu_intr_map: PMU_INTR_MAP,
    efuse_intr_map: EFUSE_INTR_MAP,
    lp_rtc_timer_intr_map: LP_RTC_TIMER_INTR_MAP,
    lp_uart_intr_map: LP_UART_INTR_MAP,
    lp_i2c_intr_map: LP_I2C_INTR_MAP,
    lp_wdt_intr_map: LP_WDT_INTR_MAP,
    lp_peri_timeout_intr_map: LP_PERI_TIMEOUT_INTR_MAP,
    lp_apm_m0_intr_map: LP_APM_M0_INTR_MAP,
    lp_apm_m1_intr_map: LP_APM_M1_INTR_MAP,
    cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    trace_intr_map: TRACE_INTR_MAP,
    cache_intr_map: CACHE_INTR_MAP,
    cpu_peri_timeout_intr_map: CPU_PERI_TIMEOUT_INTR_MAP,
    gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    pau_intr_map: PAU_INTR_MAP,
    hp_peri_timeout_intr_map: HP_PERI_TIMEOUT_INTR_MAP,
    modem_peri_timeout_intr_map: MODEM_PERI_TIMEOUT_INTR_MAP,
    hp_apm_m0_intr_map: HP_APM_M0_INTR_MAP,
    hp_apm_m1_intr_map: HP_APM_M1_INTR_MAP,
    hp_apm_m2_intr_map: HP_APM_M2_INTR_MAP,
    hp_apm_m3_intr_map: HP_APM_M3_INTR_MAP,
    lp_apm0_intr_map: LP_APM0_INTR_MAP,
    mspi_intr_map: MSPI_INTR_MAP,
    i2s1_intr_map: I2S1_INTR_MAP,
    uhci0_intr_map: UHCI0_INTR_MAP,
    uart0_intr_map: UART0_INTR_MAP,
    uart1_intr_map: UART1_INTR_MAP,
    ledc_intr_map: LEDC_INTR_MAP,
    can0_intr_map: CAN0_INTR_MAP,
    can1_intr_map: CAN1_INTR_MAP,
    usb_intr_map: USB_INTR_MAP,
    rmt_intr_map: RMT_INTR_MAP,
    i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    tg0_t0_intr_map: TG0_T0_INTR_MAP,
    tg0_t1_intr_map: TG0_T1_INTR_MAP,
    tg0_wdt_intr_map: TG0_WDT_INTR_MAP,
    tg1_t0_intr_map: TG1_T0_INTR_MAP,
    tg1_t1_intr_map: TG1_T1_INTR_MAP,
    tg1_wdt_intr_map: TG1_WDT_INTR_MAP,
    systimer_target0_intr_map: SYSTIMER_TARGET0_INTR_MAP,
    systimer_target1_intr_map: SYSTIMER_TARGET1_INTR_MAP,
    systimer_target2_intr_map: SYSTIMER_TARGET2_INTR_MAP,
    apb_adc_intr_map: APB_ADC_INTR_MAP,
    pwm_intr_map: PWM_INTR_MAP,
    pcnt_intr_map: PCNT_INTR_MAP,
    parl_io_intr_map: PARL_IO_INTR_MAP,
    slc0_intr_map: SLC0_INTR_MAP,
    slc1_intr_map: SLC1_INTR_MAP,
    dma_in_ch0_intr_map: DMA_IN_CH0_INTR_MAP,
    dma_in_ch1_intr_map: DMA_IN_CH1_INTR_MAP,
    dma_in_ch2_intr_map: DMA_IN_CH2_INTR_MAP,
    dma_out_ch0_intr_map: DMA_OUT_CH0_INTR_MAP,
    dma_out_ch1_intr_map: DMA_OUT_CH1_INTR_MAP,
    dma_out_ch2_intr_map: DMA_OUT_CH2_INTR_MAP,
    gpspi2_intr_map: GPSPI2_INTR_MAP,
    aes_intr_map: AES_INTR_MAP,
    sha_intr_map: SHA_INTR_MAP,
    rsa_intr_map: RSA_INTR_MAP,
    ecc_intr_map: ECC_INTR_MAP,
    intr_status_reg_0: INTR_STATUS_REG_0,
    intr_status_reg_1: INTR_STATUS_REG_1,
    int_status_reg_2: INT_STATUS_REG_2,
    clock_gate: CLOCK_GATE,
    _reserved81: [u8; 0x06b8],
    interrupt_reg_date: INTERRUPT_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - register description"]
    #[inline(always)]
    pub const fn wifi_mac_intr_map(&self) -> &WIFI_MAC_INTR_MAP {
        &self.wifi_mac_intr_map
    }
    #[doc = "0x04 - register description"]
    #[inline(always)]
    pub const fn wifi_mac_nmi_map(&self) -> &WIFI_MAC_NMI_MAP {
        &self.wifi_mac_nmi_map
    }
    #[doc = "0x08 - register description"]
    #[inline(always)]
    pub const fn wifi_pwr_intr_map(&self) -> &WIFI_PWR_INTR_MAP {
        &self.wifi_pwr_intr_map
    }
    #[doc = "0x0c - register description"]
    #[inline(always)]
    pub const fn wifi_bb_intr_map(&self) -> &WIFI_BB_INTR_MAP {
        &self.wifi_bb_intr_map
    }
    #[doc = "0x10 - register description"]
    #[inline(always)]
    pub const fn bt_mac_intr_map(&self) -> &BT_MAC_INTR_MAP {
        &self.bt_mac_intr_map
    }
    #[doc = "0x14 - register description"]
    #[inline(always)]
    pub const fn bt_bb_intr_map(&self) -> &BT_BB_INTR_MAP {
        &self.bt_bb_intr_map
    }
    #[doc = "0x18 - register description"]
    #[inline(always)]
    pub const fn bt_bb_nmi_map(&self) -> &BT_BB_NMI_MAP {
        &self.bt_bb_nmi_map
    }
    #[doc = "0x1c - register description"]
    #[inline(always)]
    pub const fn lp_timer_intr_map(&self) -> &LP_TIMER_INTR_MAP {
        &self.lp_timer_intr_map
    }
    #[doc = "0x20 - register description"]
    #[inline(always)]
    pub const fn coex_intr_map(&self) -> &COEX_INTR_MAP {
        &self.coex_intr_map
    }
    #[doc = "0x24 - register description"]
    #[inline(always)]
    pub const fn ble_timer_intr_map(&self) -> &BLE_TIMER_INTR_MAP {
        &self.ble_timer_intr_map
    }
    #[doc = "0x28 - register description"]
    #[inline(always)]
    pub const fn ble_sec_intr_map(&self) -> &BLE_SEC_INTR_MAP {
        &self.ble_sec_intr_map
    }
    #[doc = "0x2c - register description"]
    #[inline(always)]
    pub const fn i2c_mst_intr_map(&self) -> &I2C_MST_INTR_MAP {
        &self.i2c_mst_intr_map
    }
    #[doc = "0x30 - register description"]
    #[inline(always)]
    pub const fn zb_mac_intr_map(&self) -> &ZB_MAC_INTR_MAP {
        &self.zb_mac_intr_map
    }
    #[doc = "0x34 - register description"]
    #[inline(always)]
    pub const fn pmu_intr_map(&self) -> &PMU_INTR_MAP {
        &self.pmu_intr_map
    }
    #[doc = "0x38 - register description"]
    #[inline(always)]
    pub const fn efuse_intr_map(&self) -> &EFUSE_INTR_MAP {
        &self.efuse_intr_map
    }
    #[doc = "0x3c - register description"]
    #[inline(always)]
    pub const fn lp_rtc_timer_intr_map(&self) -> &LP_RTC_TIMER_INTR_MAP {
        &self.lp_rtc_timer_intr_map
    }
    #[doc = "0x40 - register description"]
    #[inline(always)]
    pub const fn lp_uart_intr_map(&self) -> &LP_UART_INTR_MAP {
        &self.lp_uart_intr_map
    }
    #[doc = "0x44 - register description"]
    #[inline(always)]
    pub const fn lp_i2c_intr_map(&self) -> &LP_I2C_INTR_MAP {
        &self.lp_i2c_intr_map
    }
    #[doc = "0x48 - register description"]
    #[inline(always)]
    pub const fn lp_wdt_intr_map(&self) -> &LP_WDT_INTR_MAP {
        &self.lp_wdt_intr_map
    }
    #[doc = "0x4c - register description"]
    #[inline(always)]
    pub const fn lp_peri_timeout_intr_map(&self) -> &LP_PERI_TIMEOUT_INTR_MAP {
        &self.lp_peri_timeout_intr_map
    }
    #[doc = "0x50 - register description"]
    #[inline(always)]
    pub const fn lp_apm_m0_intr_map(&self) -> &LP_APM_M0_INTR_MAP {
        &self.lp_apm_m0_intr_map
    }
    #[doc = "0x54 - register description"]
    #[inline(always)]
    pub const fn lp_apm_m1_intr_map(&self) -> &LP_APM_M1_INTR_MAP {
        &self.lp_apm_m1_intr_map
    }
    #[doc = "0x58 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0_map(&self) -> &CPU_INTR_FROM_CPU_0_MAP {
        &self.cpu_intr_from_cpu_0_map
    }
    #[doc = "0x5c - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1_map(&self) -> &CPU_INTR_FROM_CPU_1_MAP {
        &self.cpu_intr_from_cpu_1_map
    }
    #[doc = "0x60 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2_map(&self) -> &CPU_INTR_FROM_CPU_2_MAP {
        &self.cpu_intr_from_cpu_2_map
    }
    #[doc = "0x64 - register description"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3_map(&self) -> &CPU_INTR_FROM_CPU_3_MAP {
        &self.cpu_intr_from_cpu_3_map
    }
    #[doc = "0x68 - register description"]
    #[inline(always)]
    pub const fn assist_debug_intr_map(&self) -> &ASSIST_DEBUG_INTR_MAP {
        &self.assist_debug_intr_map
    }
    #[doc = "0x6c - register description"]
    #[inline(always)]
    pub const fn trace_intr_map(&self) -> &TRACE_INTR_MAP {
        &self.trace_intr_map
    }
    #[doc = "0x70 - register description"]
    #[inline(always)]
    pub const fn cache_intr_map(&self) -> &CACHE_INTR_MAP {
        &self.cache_intr_map
    }
    #[doc = "0x74 - register description"]
    #[inline(always)]
    pub const fn cpu_peri_timeout_intr_map(&self) -> &CPU_PERI_TIMEOUT_INTR_MAP {
        &self.cpu_peri_timeout_intr_map
    }
    #[doc = "0x78 - register description"]
    #[inline(always)]
    pub const fn gpio_interrupt_pro_map(&self) -> &GPIO_INTERRUPT_PRO_MAP {
        &self.gpio_interrupt_pro_map
    }
    #[doc = "0x7c - register description"]
    #[inline(always)]
    pub const fn gpio_interrupt_pro_nmi_map(&self) -> &GPIO_INTERRUPT_PRO_NMI_MAP {
        &self.gpio_interrupt_pro_nmi_map
    }
    #[doc = "0x80 - register description"]
    #[inline(always)]
    pub const fn pau_intr_map(&self) -> &PAU_INTR_MAP {
        &self.pau_intr_map
    }
    #[doc = "0x84 - register description"]
    #[inline(always)]
    pub const fn hp_peri_timeout_intr_map(&self) -> &HP_PERI_TIMEOUT_INTR_MAP {
        &self.hp_peri_timeout_intr_map
    }
    #[doc = "0x88 - register description"]
    #[inline(always)]
    pub const fn modem_peri_timeout_intr_map(&self) -> &MODEM_PERI_TIMEOUT_INTR_MAP {
        &self.modem_peri_timeout_intr_map
    }
    #[doc = "0x8c - register description"]
    #[inline(always)]
    pub const fn hp_apm_m0_intr_map(&self) -> &HP_APM_M0_INTR_MAP {
        &self.hp_apm_m0_intr_map
    }
    #[doc = "0x90 - register description"]
    #[inline(always)]
    pub const fn hp_apm_m1_intr_map(&self) -> &HP_APM_M1_INTR_MAP {
        &self.hp_apm_m1_intr_map
    }
    #[doc = "0x94 - register description"]
    #[inline(always)]
    pub const fn hp_apm_m2_intr_map(&self) -> &HP_APM_M2_INTR_MAP {
        &self.hp_apm_m2_intr_map
    }
    #[doc = "0x98 - register description"]
    #[inline(always)]
    pub const fn hp_apm_m3_intr_map(&self) -> &HP_APM_M3_INTR_MAP {
        &self.hp_apm_m3_intr_map
    }
    #[doc = "0x9c - register description"]
    #[inline(always)]
    pub const fn lp_apm0_intr_map(&self) -> &LP_APM0_INTR_MAP {
        &self.lp_apm0_intr_map
    }
    #[doc = "0xa0 - register description"]
    #[inline(always)]
    pub const fn mspi_intr_map(&self) -> &MSPI_INTR_MAP {
        &self.mspi_intr_map
    }
    #[doc = "0xa4 - register description"]
    #[inline(always)]
    pub const fn i2s1_intr_map(&self) -> &I2S1_INTR_MAP {
        &self.i2s1_intr_map
    }
    #[doc = "0xa8 - register description"]
    #[inline(always)]
    pub const fn uhci0_intr_map(&self) -> &UHCI0_INTR_MAP {
        &self.uhci0_intr_map
    }
    #[doc = "0xac - register description"]
    #[inline(always)]
    pub const fn uart0_intr_map(&self) -> &UART0_INTR_MAP {
        &self.uart0_intr_map
    }
    #[doc = "0xb0 - register description"]
    #[inline(always)]
    pub const fn uart1_intr_map(&self) -> &UART1_INTR_MAP {
        &self.uart1_intr_map
    }
    #[doc = "0xb4 - register description"]
    #[inline(always)]
    pub const fn ledc_intr_map(&self) -> &LEDC_INTR_MAP {
        &self.ledc_intr_map
    }
    #[doc = "0xb8 - register description"]
    #[inline(always)]
    pub const fn can0_intr_map(&self) -> &CAN0_INTR_MAP {
        &self.can0_intr_map
    }
    #[doc = "0xbc - register description"]
    #[inline(always)]
    pub const fn can1_intr_map(&self) -> &CAN1_INTR_MAP {
        &self.can1_intr_map
    }
    #[doc = "0xc0 - register description"]
    #[inline(always)]
    pub const fn usb_intr_map(&self) -> &USB_INTR_MAP {
        &self.usb_intr_map
    }
    #[doc = "0xc4 - register description"]
    #[inline(always)]
    pub const fn rmt_intr_map(&self) -> &RMT_INTR_MAP {
        &self.rmt_intr_map
    }
    #[doc = "0xc8 - register description"]
    #[inline(always)]
    pub const fn i2c_ext0_intr_map(&self) -> &I2C_EXT0_INTR_MAP {
        &self.i2c_ext0_intr_map
    }
    #[doc = "0xcc - register description"]
    #[inline(always)]
    pub const fn tg0_t0_intr_map(&self) -> &TG0_T0_INTR_MAP {
        &self.tg0_t0_intr_map
    }
    #[doc = "0xd0 - register description"]
    #[inline(always)]
    pub const fn tg0_t1_intr_map(&self) -> &TG0_T1_INTR_MAP {
        &self.tg0_t1_intr_map
    }
    #[doc = "0xd4 - register description"]
    #[inline(always)]
    pub const fn tg0_wdt_intr_map(&self) -> &TG0_WDT_INTR_MAP {
        &self.tg0_wdt_intr_map
    }
    #[doc = "0xd8 - register description"]
    #[inline(always)]
    pub const fn tg1_t0_intr_map(&self) -> &TG1_T0_INTR_MAP {
        &self.tg1_t0_intr_map
    }
    #[doc = "0xdc - register description"]
    #[inline(always)]
    pub const fn tg1_t1_intr_map(&self) -> &TG1_T1_INTR_MAP {
        &self.tg1_t1_intr_map
    }
    #[doc = "0xe0 - register description"]
    #[inline(always)]
    pub const fn tg1_wdt_intr_map(&self) -> &TG1_WDT_INTR_MAP {
        &self.tg1_wdt_intr_map
    }
    #[doc = "0xe4 - register description"]
    #[inline(always)]
    pub const fn systimer_target0_intr_map(&self) -> &SYSTIMER_TARGET0_INTR_MAP {
        &self.systimer_target0_intr_map
    }
    #[doc = "0xe8 - register description"]
    #[inline(always)]
    pub const fn systimer_target1_intr_map(&self) -> &SYSTIMER_TARGET1_INTR_MAP {
        &self.systimer_target1_intr_map
    }
    #[doc = "0xec - register description"]
    #[inline(always)]
    pub const fn systimer_target2_intr_map(&self) -> &SYSTIMER_TARGET2_INTR_MAP {
        &self.systimer_target2_intr_map
    }
    #[doc = "0xf0 - register description"]
    #[inline(always)]
    pub const fn apb_adc_intr_map(&self) -> &APB_ADC_INTR_MAP {
        &self.apb_adc_intr_map
    }
    #[doc = "0xf4 - register description"]
    #[inline(always)]
    pub const fn pwm_intr_map(&self) -> &PWM_INTR_MAP {
        &self.pwm_intr_map
    }
    #[doc = "0xf8 - register description"]
    #[inline(always)]
    pub const fn pcnt_intr_map(&self) -> &PCNT_INTR_MAP {
        &self.pcnt_intr_map
    }
    #[doc = "0xfc - register description"]
    #[inline(always)]
    pub const fn parl_io_intr_map(&self) -> &PARL_IO_INTR_MAP {
        &self.parl_io_intr_map
    }
    #[doc = "0x100 - register description"]
    #[inline(always)]
    pub const fn slc0_intr_map(&self) -> &SLC0_INTR_MAP {
        &self.slc0_intr_map
    }
    #[doc = "0x104 - register description"]
    #[inline(always)]
    pub const fn slc1_intr_map(&self) -> &SLC1_INTR_MAP {
        &self.slc1_intr_map
    }
    #[doc = "0x108 - register description"]
    #[inline(always)]
    pub const fn dma_in_ch0_intr_map(&self) -> &DMA_IN_CH0_INTR_MAP {
        &self.dma_in_ch0_intr_map
    }
    #[doc = "0x10c - register description"]
    #[inline(always)]
    pub const fn dma_in_ch1_intr_map(&self) -> &DMA_IN_CH1_INTR_MAP {
        &self.dma_in_ch1_intr_map
    }
    #[doc = "0x110 - register description"]
    #[inline(always)]
    pub const fn dma_in_ch2_intr_map(&self) -> &DMA_IN_CH2_INTR_MAP {
        &self.dma_in_ch2_intr_map
    }
    #[doc = "0x114 - register description"]
    #[inline(always)]
    pub const fn dma_out_ch0_intr_map(&self) -> &DMA_OUT_CH0_INTR_MAP {
        &self.dma_out_ch0_intr_map
    }
    #[doc = "0x118 - register description"]
    #[inline(always)]
    pub const fn dma_out_ch1_intr_map(&self) -> &DMA_OUT_CH1_INTR_MAP {
        &self.dma_out_ch1_intr_map
    }
    #[doc = "0x11c - register description"]
    #[inline(always)]
    pub const fn dma_out_ch2_intr_map(&self) -> &DMA_OUT_CH2_INTR_MAP {
        &self.dma_out_ch2_intr_map
    }
    #[doc = "0x120 - register description"]
    #[inline(always)]
    pub const fn gpspi2_intr_map(&self) -> &GPSPI2_INTR_MAP {
        &self.gpspi2_intr_map
    }
    #[doc = "0x124 - register description"]
    #[inline(always)]
    pub const fn aes_intr_map(&self) -> &AES_INTR_MAP {
        &self.aes_intr_map
    }
    #[doc = "0x128 - register description"]
    #[inline(always)]
    pub const fn sha_intr_map(&self) -> &SHA_INTR_MAP {
        &self.sha_intr_map
    }
    #[doc = "0x12c - register description"]
    #[inline(always)]
    pub const fn rsa_intr_map(&self) -> &RSA_INTR_MAP {
        &self.rsa_intr_map
    }
    #[doc = "0x130 - register description"]
    #[inline(always)]
    pub const fn ecc_intr_map(&self) -> &ECC_INTR_MAP {
        &self.ecc_intr_map
    }
    #[doc = "0x134 - register description"]
    #[inline(always)]
    pub const fn intr_status_reg_0(&self) -> &INTR_STATUS_REG_0 {
        &self.intr_status_reg_0
    }
    #[doc = "0x138 - register description"]
    #[inline(always)]
    pub const fn intr_status_reg_1(&self) -> &INTR_STATUS_REG_1 {
        &self.intr_status_reg_1
    }
    #[doc = "0x13c - register description"]
    #[inline(always)]
    pub const fn int_status_reg_2(&self) -> &INT_STATUS_REG_2 {
        &self.int_status_reg_2
    }
    #[doc = "0x140 - register description"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - register description"]
    #[inline(always)]
    pub const fn interrupt_reg_date(&self) -> &INTERRUPT_REG_DATE {
        &self.interrupt_reg_date
    }
}
#[doc = "WIFI_MAC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_mac_intr_map`] module"]
pub type WIFI_MAC_INTR_MAP = crate::Reg<wifi_mac_intr_map::WIFI_MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_mac_intr_map;
#[doc = "WIFI_MAC_NMI_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_mac_nmi_map`] module"]
pub type WIFI_MAC_NMI_MAP = crate::Reg<wifi_mac_nmi_map::WIFI_MAC_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_mac_nmi_map;
#[doc = "WIFI_PWR_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pwr_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pwr_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pwr_intr_map`] module"]
pub type WIFI_PWR_INTR_MAP = crate::Reg<wifi_pwr_intr_map::WIFI_PWR_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_pwr_intr_map;
#[doc = "WIFI_BB_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_intr_map`] module"]
pub type WIFI_BB_INTR_MAP = crate::Reg<wifi_bb_intr_map::WIFI_BB_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_bb_intr_map;
#[doc = "BT_MAC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_mac_intr_map`] module"]
pub type BT_MAC_INTR_MAP = crate::Reg<bt_mac_intr_map::BT_MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_mac_intr_map;
#[doc = "BT_BB_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_intr_map`] module"]
pub type BT_BB_INTR_MAP = crate::Reg<bt_bb_intr_map::BT_BB_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_intr_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_nmi_map`] module"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_nmi_map;
#[doc = "LP_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_intr_map`] module"]
pub type LP_TIMER_INTR_MAP = crate::Reg<lp_timer_intr_map::LP_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_timer_intr_map;
#[doc = "COEX_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_intr_map`] module"]
pub type COEX_INTR_MAP = crate::Reg<coex_intr_map::COEX_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod coex_intr_map;
#[doc = "BLE_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_timer_intr_map`] module"]
pub type BLE_TIMER_INTR_MAP = crate::Reg<ble_timer_intr_map::BLE_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_timer_intr_map;
#[doc = "BLE_SEC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_sec_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_sec_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_sec_intr_map`] module"]
pub type BLE_SEC_INTR_MAP = crate::Reg<ble_sec_intr_map::BLE_SEC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_sec_intr_map;
#[doc = "I2C_MST_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_mst_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_mst_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mst_intr_map`] module"]
pub type I2C_MST_INTR_MAP = crate::Reg<i2c_mst_intr_map::I2C_MST_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_mst_intr_map;
#[doc = "ZB_MAC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zb_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zb_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zb_mac_intr_map`] module"]
pub type ZB_MAC_INTR_MAP = crate::Reg<zb_mac_intr_map::ZB_MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod zb_mac_intr_map;
#[doc = "PMU_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_intr_map`] module"]
pub type PMU_INTR_MAP = crate::Reg<pmu_intr_map::PMU_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pmu_intr_map;
#[doc = "EFUSE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_intr_map`] module"]
pub type EFUSE_INTR_MAP = crate::Reg<efuse_intr_map::EFUSE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod efuse_intr_map;
#[doc = "LP_RTC_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rtc_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rtc_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rtc_timer_intr_map`] module"]
pub type LP_RTC_TIMER_INTR_MAP = crate::Reg<lp_rtc_timer_intr_map::LP_RTC_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_rtc_timer_intr_map;
#[doc = "LP_UART_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_uart_intr_map`] module"]
pub type LP_UART_INTR_MAP = crate::Reg<lp_uart_intr_map::LP_UART_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_uart_intr_map;
#[doc = "LP_I2C_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2c_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2c_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2c_intr_map`] module"]
pub type LP_I2C_INTR_MAP = crate::Reg<lp_i2c_intr_map::LP_I2C_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_i2c_intr_map;
#[doc = "LP_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_intr_map`] module"]
pub type LP_WDT_INTR_MAP = crate::Reg<lp_wdt_intr_map::LP_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_wdt_intr_map;
#[doc = "LP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_intr_map`] module"]
pub type LP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<lp_peri_timeout_intr_map::LP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_peri_timeout_intr_map;
#[doc = "LP_APM_M0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_apm_m0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_apm_m0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m0_intr_map`] module"]
pub type LP_APM_M0_INTR_MAP = crate::Reg<lp_apm_m0_intr_map::LP_APM_M0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_apm_m0_intr_map;
#[doc = "LP_APM_M1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_apm_m1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_apm_m1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m1_intr_map`] module"]
pub type LP_APM_M1_INTR_MAP = crate::Reg<lp_apm_m1_intr_map::LP_APM_M1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_apm_m1_intr_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_debug_intr_map`] module"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod assist_debug_intr_map;
#[doc = "TRACE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_intr_map`] module"]
pub type TRACE_INTR_MAP = crate::Reg<trace_intr_map::TRACE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod trace_intr_map;
#[doc = "CACHE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_intr_map`] module"]
pub type CACHE_INTR_MAP = crate::Reg<cache_intr_map::CACHE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod cache_intr_map;
#[doc = "CPU_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri_timeout_intr_map`] module"]
pub type CPU_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<cpu_peri_timeout_intr_map::CPU_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_peri_timeout_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_pro_map`] module"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_pro_nmi_map`] module"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "PAU_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pau_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pau_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pau_intr_map`] module"]
pub type PAU_INTR_MAP = crate::Reg<pau_intr_map::PAU_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pau_intr_map;
#[doc = "HP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri_timeout_intr_map`] module"]
pub type HP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<hp_peri_timeout_intr_map::HP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_peri_timeout_intr_map;
#[doc = "MODEM_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_peri_timeout_intr_map`] module"]
pub type MODEM_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<modem_peri_timeout_intr_map::MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod modem_peri_timeout_intr_map;
#[doc = "HP_APM_M0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m0_intr_map`] module"]
pub type HP_APM_M0_INTR_MAP = crate::Reg<hp_apm_m0_intr_map::HP_APM_M0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m0_intr_map;
#[doc = "HP_APM_M1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m1_intr_map`] module"]
pub type HP_APM_M1_INTR_MAP = crate::Reg<hp_apm_m1_intr_map::HP_APM_M1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m1_intr_map;
#[doc = "HP_APM_M2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m2_intr_map`] module"]
pub type HP_APM_M2_INTR_MAP = crate::Reg<hp_apm_m2_intr_map::HP_APM_M2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m2_intr_map;
#[doc = "HP_APM_M3_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m3_intr_map`] module"]
pub type HP_APM_M3_INTR_MAP = crate::Reg<hp_apm_m3_intr_map::HP_APM_M3_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m3_intr_map;
#[doc = "LP_APM0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_apm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_apm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm0_intr_map`] module"]
pub type LP_APM0_INTR_MAP = crate::Reg<lp_apm0_intr_map::LP_APM0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_apm0_intr_map;
#[doc = "MSPI_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspi_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_intr_map`] module"]
pub type MSPI_INTR_MAP = crate::Reg<mspi_intr_map::MSPI_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod mspi_intr_map;
#[doc = "I2S1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_intr_map`] module"]
pub type I2S1_INTR_MAP = crate::Reg<i2s1_intr_map::I2S1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2s1_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci0_intr_map`] module"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uhci0_intr_map;
#[doc = "UART0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_intr_map`] module"]
pub type UART0_INTR_MAP = crate::Reg<uart0_intr_map::UART0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart0_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart1_intr_map;
#[doc = "LEDC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_intr_map`] module"]
pub type LEDC_INTR_MAP = crate::Reg<ledc_intr_map::LEDC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ledc_intr_map;
#[doc = "CAN0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_intr_map`] module"]
pub type CAN0_INTR_MAP = crate::Reg<can0_intr_map::CAN0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod can0_intr_map;
#[doc = "CAN1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1_intr_map`] module"]
pub type CAN1_INTR_MAP = crate::Reg<can1_intr_map::CAN1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod can1_intr_map;
#[doc = "USB_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_intr_map`] module"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod usb_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_intr_map`] module"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod rmt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ext0_intr_map`] module"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_ext0_intr_map;
#[doc = "TG0_T0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_t0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_t0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_t0_intr_map`] module"]
pub type TG0_T0_INTR_MAP = crate::Reg<tg0_t0_intr_map::TG0_T0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg0_t0_intr_map;
#[doc = "TG0_T1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_t1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_t1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_t1_intr_map`] module"]
pub type TG0_T1_INTR_MAP = crate::Reg<tg0_t1_intr_map::TG0_T1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg0_t1_intr_map;
#[doc = "TG0_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_wdt_intr_map`] module"]
pub type TG0_WDT_INTR_MAP = crate::Reg<tg0_wdt_intr_map::TG0_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg0_wdt_intr_map;
#[doc = "TG1_T0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_t0_intr_map`] module"]
pub type TG1_T0_INTR_MAP = crate::Reg<tg1_t0_intr_map::TG1_T0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg1_t0_intr_map;
#[doc = "TG1_T1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_t1_intr_map`] module"]
pub type TG1_T1_INTR_MAP = crate::Reg<tg1_t1_intr_map::TG1_T1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg1_t1_intr_map;
#[doc = "TG1_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_wdt_intr_map`] module"]
pub type TG1_WDT_INTR_MAP = crate::Reg<tg1_wdt_intr_map::TG1_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg1_wdt_intr_map;
#[doc = "SYSTIMER_TARGET0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target0_intr_map`] module"]
pub type SYSTIMER_TARGET0_INTR_MAP =
    crate::Reg<systimer_target0_intr_map::SYSTIMER_TARGET0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target0_intr_map;
#[doc = "SYSTIMER_TARGET1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target1_intr_map`] module"]
pub type SYSTIMER_TARGET1_INTR_MAP =
    crate::Reg<systimer_target1_intr_map::SYSTIMER_TARGET1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target1_intr_map;
#[doc = "SYSTIMER_TARGET2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target2_intr_map`] module"]
pub type SYSTIMER_TARGET2_INTR_MAP =
    crate::Reg<systimer_target2_intr_map::SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target2_intr_map;
#[doc = "APB_ADC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_adc_intr_map`] module"]
pub type APB_ADC_INTR_MAP = crate::Reg<apb_adc_intr_map::APB_ADC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod apb_adc_intr_map;
#[doc = "PWM_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_intr_map`] module"]
pub type PWM_INTR_MAP = crate::Reg<pwm_intr_map::PWM_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pwm_intr_map;
#[doc = "PCNT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt_intr_map`] module"]
pub type PCNT_INTR_MAP = crate::Reg<pcnt_intr_map::PCNT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pcnt_intr_map;
#[doc = "PARL_IO_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parl_io_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parl_io_intr_map`] module"]
pub type PARL_IO_INTR_MAP = crate::Reg<parl_io_intr_map::PARL_IO_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod parl_io_intr_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_intr_map`] module"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_intr_map`] module"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod slc1_intr_map;
#[doc = "DMA_IN_CH0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_ch0_intr_map`] module"]
pub type DMA_IN_CH0_INTR_MAP = crate::Reg<dma_in_ch0_intr_map::DMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch0_intr_map;
#[doc = "DMA_IN_CH1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_ch1_intr_map`] module"]
pub type DMA_IN_CH1_INTR_MAP = crate::Reg<dma_in_ch1_intr_map::DMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch1_intr_map;
#[doc = "DMA_IN_CH2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_ch2_intr_map`] module"]
pub type DMA_IN_CH2_INTR_MAP = crate::Reg<dma_in_ch2_intr_map::DMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch2_intr_map;
#[doc = "DMA_OUT_CH0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_ch0_intr_map`] module"]
pub type DMA_OUT_CH0_INTR_MAP = crate::Reg<dma_out_ch0_intr_map::DMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch0_intr_map;
#[doc = "DMA_OUT_CH1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_ch1_intr_map`] module"]
pub type DMA_OUT_CH1_INTR_MAP = crate::Reg<dma_out_ch1_intr_map::DMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch1_intr_map;
#[doc = "DMA_OUT_CH2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_ch2_intr_map`] module"]
pub type DMA_OUT_CH2_INTR_MAP = crate::Reg<dma_out_ch2_intr_map::DMA_OUT_CH2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch2_intr_map;
#[doc = "GPSPI2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpspi2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpspi2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi2_intr_map`] module"]
pub type GPSPI2_INTR_MAP = crate::Reg<gpspi2_intr_map::GPSPI2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod gpspi2_intr_map;
#[doc = "AES_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_intr_map`] module"]
pub type AES_INTR_MAP = crate::Reg<aes_intr_map::AES_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod aes_intr_map;
#[doc = "SHA_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_intr_map`] module"]
pub type SHA_INTR_MAP = crate::Reg<sha_intr_map::SHA_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod sha_intr_map;
#[doc = "RSA_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_intr_map`] module"]
pub type RSA_INTR_MAP = crate::Reg<rsa_intr_map::RSA_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod rsa_intr_map;
#[doc = "ECC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_intr_map`] module"]
pub type ECC_INTR_MAP = crate::Reg<ecc_intr_map::ECC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ecc_intr_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_0`] module"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_1`] module"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_1;
#[doc = "INT_STATUS_REG_2 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_reg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_reg_2`] module"]
pub type INT_STATUS_REG_2 = crate::Reg<int_status_reg_2::INT_STATUS_REG_2_SPEC>;
#[doc = "register description"]
pub mod int_status_reg_2;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_reg_date`] module"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "register description"]
pub mod interrupt_reg_date;
