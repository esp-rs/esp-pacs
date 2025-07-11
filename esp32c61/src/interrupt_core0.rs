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
    lp_wdt_intr_map: LP_WDT_INTR_MAP,
    lp_peri_timeout_intr_map: LP_PERI_TIMEOUT_INTR_MAP,
    lp_apm_m0_intr_map: LP_APM_M0_INTR_MAP,
    cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    trace_intr_map: TRACE_INTR_MAP,
    cache_intr_map: CACHE_INTR_MAP,
    cpu_peri_timeout_intr_map: CPU_PERI_TIMEOUT_INTR_MAP,
    gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    gpio_interrupt_ext_map: GPIO_INTERRUPT_EXT_MAP,
    pau_intr_map: PAU_INTR_MAP,
    hp_peri_timeout_intr_map: HP_PERI_TIMEOUT_INTR_MAP,
    modem_peri_timeout_intr_map: MODEM_PERI_TIMEOUT_INTR_MAP,
    hp_apm_m0_intr_map: HP_APM_M0_INTR_MAP,
    hp_apm_m1_intr_map: HP_APM_M1_INTR_MAP,
    hp_apm_m2_intr_map: HP_APM_M2_INTR_MAP,
    hp_apm_m3_intr_map: HP_APM_M3_INTR_MAP,
    cpu_apm_m0_intr_map: CPU_APM_M0_INTR_MAP,
    cpu_apm_m1_intr_map: CPU_APM_M1_INTR_MAP,
    mspi_intr_map: MSPI_INTR_MAP,
    i2s1_intr_map: I2S1_INTR_MAP,
    uart0_intr_map: UART0_INTR_MAP,
    uart1_intr_map: UART1_INTR_MAP,
    uart2_intr_map: UART2_INTR_MAP,
    ledc_intr_map: LEDC_INTR_MAP,
    usb_intr_map: USB_INTR_MAP,
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
    slc0_intr_map: SLC0_INTR_MAP,
    slc1_intr_map: SLC1_INTR_MAP,
    dma_in_ch0_intr_map: DMA_IN_CH0_INTR_MAP,
    dma_in_ch1_intr_map: DMA_IN_CH1_INTR_MAP,
    dma_out_ch0_intr_map: DMA_OUT_CH0_INTR_MAP,
    dma_out_ch1_intr_map: DMA_OUT_CH1_INTR_MAP,
    gpspi2_intr_map: GPSPI2_INTR_MAP,
    sha_intr_map: SHA_INTR_MAP,
    ecc_intr_map: ECC_INTR_MAP,
    ecdsa_intr_map: ECDSA_INTR_MAP,
    int_status_reg_0: INT_STATUS_REG_0,
    int_status_reg_1: INT_STATUS_REG_1,
    int_status_reg_2: INT_STATUS_REG_2,
    src_pass_in_sec_status_0: SRC_PASS_IN_SEC_STATUS_0,
    src_pass_in_sec_status_1: SRC_PASS_IN_SEC_STATUS_1,
    src_pass_in_sec_status_2: SRC_PASS_IN_SEC_STATUS_2,
    sig_idx_assert_in_sec: SIG_IDX_ASSERT_IN_SEC,
    secure_status: SECURE_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved75: [u8; 0x06d0],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - WIFI_MAC_INTR mapping register"]
    #[inline(always)]
    pub const fn wifi_mac_intr_map(&self) -> &WIFI_MAC_INTR_MAP {
        &self.wifi_mac_intr_map
    }
    #[doc = "0x04 - WIFI_MAC_NMI mapping register"]
    #[inline(always)]
    pub const fn wifi_mac_nmi_map(&self) -> &WIFI_MAC_NMI_MAP {
        &self.wifi_mac_nmi_map
    }
    #[doc = "0x08 - WIFI_PWR_INTR mapping register"]
    #[inline(always)]
    pub const fn wifi_pwr_intr_map(&self) -> &WIFI_PWR_INTR_MAP {
        &self.wifi_pwr_intr_map
    }
    #[doc = "0x0c - WIFI_BB_INTR mapping register"]
    #[inline(always)]
    pub const fn wifi_bb_intr_map(&self) -> &WIFI_BB_INTR_MAP {
        &self.wifi_bb_intr_map
    }
    #[doc = "0x10 - BT_MAC_INTR mapping register"]
    #[inline(always)]
    pub const fn bt_mac_intr_map(&self) -> &BT_MAC_INTR_MAP {
        &self.bt_mac_intr_map
    }
    #[doc = "0x14 - BT_BB_INTR mapping register"]
    #[inline(always)]
    pub const fn bt_bb_intr_map(&self) -> &BT_BB_INTR_MAP {
        &self.bt_bb_intr_map
    }
    #[doc = "0x18 - BT_BB_NMI mapping register"]
    #[inline(always)]
    pub const fn bt_bb_nmi_map(&self) -> &BT_BB_NMI_MAP {
        &self.bt_bb_nmi_map
    }
    #[doc = "0x1c - LP_TIMER_INTR mapping register"]
    #[inline(always)]
    pub const fn lp_timer_intr_map(&self) -> &LP_TIMER_INTR_MAP {
        &self.lp_timer_intr_map
    }
    #[doc = "0x20 - COEX_INTR mapping register"]
    #[inline(always)]
    pub const fn coex_intr_map(&self) -> &COEX_INTR_MAP {
        &self.coex_intr_map
    }
    #[doc = "0x24 - BLE_TIMER_INTR mapping register"]
    #[inline(always)]
    pub const fn ble_timer_intr_map(&self) -> &BLE_TIMER_INTR_MAP {
        &self.ble_timer_intr_map
    }
    #[doc = "0x28 - BLE_SEC_INTR mapping register"]
    #[inline(always)]
    pub const fn ble_sec_intr_map(&self) -> &BLE_SEC_INTR_MAP {
        &self.ble_sec_intr_map
    }
    #[doc = "0x2c - I2C_MST_INTR mapping register"]
    #[inline(always)]
    pub const fn i2c_mst_intr_map(&self) -> &I2C_MST_INTR_MAP {
        &self.i2c_mst_intr_map
    }
    #[doc = "0x30 - ZB_MAC_INTR mapping register"]
    #[inline(always)]
    pub const fn zb_mac_intr_map(&self) -> &ZB_MAC_INTR_MAP {
        &self.zb_mac_intr_map
    }
    #[doc = "0x34 - PMU_INTR mapping register"]
    #[inline(always)]
    pub const fn pmu_intr_map(&self) -> &PMU_INTR_MAP {
        &self.pmu_intr_map
    }
    #[doc = "0x38 - EFUSE_INTR mapping register"]
    #[inline(always)]
    pub const fn efuse_intr_map(&self) -> &EFUSE_INTR_MAP {
        &self.efuse_intr_map
    }
    #[doc = "0x3c - LP_RTC_TIMER_INTR mapping register"]
    #[inline(always)]
    pub const fn lp_rtc_timer_intr_map(&self) -> &LP_RTC_TIMER_INTR_MAP {
        &self.lp_rtc_timer_intr_map
    }
    #[doc = "0x40 - LP_WDT_INTR mapping register"]
    #[inline(always)]
    pub const fn lp_wdt_intr_map(&self) -> &LP_WDT_INTR_MAP {
        &self.lp_wdt_intr_map
    }
    #[doc = "0x44 - LP_PERI_TIMEOUT_INTR mapping register"]
    #[inline(always)]
    pub const fn lp_peri_timeout_intr_map(&self) -> &LP_PERI_TIMEOUT_INTR_MAP {
        &self.lp_peri_timeout_intr_map
    }
    #[doc = "0x48 - LP_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub const fn lp_apm_m0_intr_map(&self) -> &LP_APM_M0_INTR_MAP {
        &self.lp_apm_m0_intr_map
    }
    #[doc = "0x4c - CPU_INTR_FROM_CPU_0 mapping register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0_map(&self) -> &CPU_INTR_FROM_CPU_0_MAP {
        &self.cpu_intr_from_cpu_0_map
    }
    #[doc = "0x50 - CPU_INTR_FROM_CPU_1 mapping register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1_map(&self) -> &CPU_INTR_FROM_CPU_1_MAP {
        &self.cpu_intr_from_cpu_1_map
    }
    #[doc = "0x54 - CPU_INTR_FROM_CPU_2 mapping register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2_map(&self) -> &CPU_INTR_FROM_CPU_2_MAP {
        &self.cpu_intr_from_cpu_2_map
    }
    #[doc = "0x58 - CPU_INTR_FROM_CPU_3 mapping register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3_map(&self) -> &CPU_INTR_FROM_CPU_3_MAP {
        &self.cpu_intr_from_cpu_3_map
    }
    #[doc = "0x5c - ASSIST_DEBUG_INTR mapping register"]
    #[inline(always)]
    pub const fn assist_debug_intr_map(&self) -> &ASSIST_DEBUG_INTR_MAP {
        &self.assist_debug_intr_map
    }
    #[doc = "0x60 - TRACE_INTR mapping register"]
    #[inline(always)]
    pub const fn trace_intr_map(&self) -> &TRACE_INTR_MAP {
        &self.trace_intr_map
    }
    #[doc = "0x64 - CACHE_INTR mapping register"]
    #[inline(always)]
    pub const fn cache_intr_map(&self) -> &CACHE_INTR_MAP {
        &self.cache_intr_map
    }
    #[doc = "0x68 - CPU_PERI_TIMEOUT_INTR mapping register"]
    #[inline(always)]
    pub const fn cpu_peri_timeout_intr_map(&self) -> &CPU_PERI_TIMEOUT_INTR_MAP {
        &self.cpu_peri_timeout_intr_map
    }
    #[doc = "0x6c - GPIO_INTERRUPT_PRO mapping register"]
    #[inline(always)]
    pub const fn gpio_interrupt_pro_map(&self) -> &GPIO_INTERRUPT_PRO_MAP {
        &self.gpio_interrupt_pro_map
    }
    #[doc = "0x70 - GPIO_INTERRUPT_EXT mapping register"]
    #[inline(always)]
    pub const fn gpio_interrupt_ext_map(&self) -> &GPIO_INTERRUPT_EXT_MAP {
        &self.gpio_interrupt_ext_map
    }
    #[doc = "0x74 - PAU_INTR mapping register"]
    #[inline(always)]
    pub const fn pau_intr_map(&self) -> &PAU_INTR_MAP {
        &self.pau_intr_map
    }
    #[doc = "0x78 - HP_PERI_TIMEOUT_INTR mapping register"]
    #[inline(always)]
    pub const fn hp_peri_timeout_intr_map(&self) -> &HP_PERI_TIMEOUT_INTR_MAP {
        &self.hp_peri_timeout_intr_map
    }
    #[doc = "0x7c - MODEM_PERI_TIMEOUT_INTR mapping register"]
    #[inline(always)]
    pub const fn modem_peri_timeout_intr_map(&self) -> &MODEM_PERI_TIMEOUT_INTR_MAP {
        &self.modem_peri_timeout_intr_map
    }
    #[doc = "0x80 - HP_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub const fn hp_apm_m0_intr_map(&self) -> &HP_APM_M0_INTR_MAP {
        &self.hp_apm_m0_intr_map
    }
    #[doc = "0x84 - HP_APM_M1_INTR mapping register"]
    #[inline(always)]
    pub const fn hp_apm_m1_intr_map(&self) -> &HP_APM_M1_INTR_MAP {
        &self.hp_apm_m1_intr_map
    }
    #[doc = "0x88 - HP_APM_M2_INTR mapping register"]
    #[inline(always)]
    pub const fn hp_apm_m2_intr_map(&self) -> &HP_APM_M2_INTR_MAP {
        &self.hp_apm_m2_intr_map
    }
    #[doc = "0x8c - HP_APM_M3_INTR mapping register"]
    #[inline(always)]
    pub const fn hp_apm_m3_intr_map(&self) -> &HP_APM_M3_INTR_MAP {
        &self.hp_apm_m3_intr_map
    }
    #[doc = "0x90 - CPU_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub const fn cpu_apm_m0_intr_map(&self) -> &CPU_APM_M0_INTR_MAP {
        &self.cpu_apm_m0_intr_map
    }
    #[doc = "0x94 - CPU_APM_M1_INTR mapping register"]
    #[inline(always)]
    pub const fn cpu_apm_m1_intr_map(&self) -> &CPU_APM_M1_INTR_MAP {
        &self.cpu_apm_m1_intr_map
    }
    #[doc = "0x98 - MSPI_INTR mapping register"]
    #[inline(always)]
    pub const fn mspi_intr_map(&self) -> &MSPI_INTR_MAP {
        &self.mspi_intr_map
    }
    #[doc = "0x9c - I2S1_INTR mapping register"]
    #[inline(always)]
    pub const fn i2s1_intr_map(&self) -> &I2S1_INTR_MAP {
        &self.i2s1_intr_map
    }
    #[doc = "0xa0 - UART0_INTR mapping register"]
    #[inline(always)]
    pub const fn uart0_intr_map(&self) -> &UART0_INTR_MAP {
        &self.uart0_intr_map
    }
    #[doc = "0xa4 - UART1_INTR mapping register"]
    #[inline(always)]
    pub const fn uart1_intr_map(&self) -> &UART1_INTR_MAP {
        &self.uart1_intr_map
    }
    #[doc = "0xa8 - UART2_INTR mapping register"]
    #[inline(always)]
    pub const fn uart2_intr_map(&self) -> &UART2_INTR_MAP {
        &self.uart2_intr_map
    }
    #[doc = "0xac - LEDC_INTR mapping register"]
    #[inline(always)]
    pub const fn ledc_intr_map(&self) -> &LEDC_INTR_MAP {
        &self.ledc_intr_map
    }
    #[doc = "0xb0 - USB_INTR mapping register"]
    #[inline(always)]
    pub const fn usb_intr_map(&self) -> &USB_INTR_MAP {
        &self.usb_intr_map
    }
    #[doc = "0xb4 - I2C_EXT0_INTR mapping register"]
    #[inline(always)]
    pub const fn i2c_ext0_intr_map(&self) -> &I2C_EXT0_INTR_MAP {
        &self.i2c_ext0_intr_map
    }
    #[doc = "0xb8 - TG0_T0_INTR mapping register"]
    #[inline(always)]
    pub const fn tg0_t0_intr_map(&self) -> &TG0_T0_INTR_MAP {
        &self.tg0_t0_intr_map
    }
    #[doc = "0xbc - TG0_T1_INTR mapping register"]
    #[inline(always)]
    pub const fn tg0_t1_intr_map(&self) -> &TG0_T1_INTR_MAP {
        &self.tg0_t1_intr_map
    }
    #[doc = "0xc0 - TG0_WDT_INTR mapping register"]
    #[inline(always)]
    pub const fn tg0_wdt_intr_map(&self) -> &TG0_WDT_INTR_MAP {
        &self.tg0_wdt_intr_map
    }
    #[doc = "0xc4 - TG1_T0_INTR mapping register"]
    #[inline(always)]
    pub const fn tg1_t0_intr_map(&self) -> &TG1_T0_INTR_MAP {
        &self.tg1_t0_intr_map
    }
    #[doc = "0xc8 - TG1_T1_INTR mapping register"]
    #[inline(always)]
    pub const fn tg1_t1_intr_map(&self) -> &TG1_T1_INTR_MAP {
        &self.tg1_t1_intr_map
    }
    #[doc = "0xcc - TG1_WDT_INTR mapping register"]
    #[inline(always)]
    pub const fn tg1_wdt_intr_map(&self) -> &TG1_WDT_INTR_MAP {
        &self.tg1_wdt_intr_map
    }
    #[doc = "0xd0 - SYSTIMER_TARGET0_INTR mapping register"]
    #[inline(always)]
    pub const fn systimer_target0_intr_map(&self) -> &SYSTIMER_TARGET0_INTR_MAP {
        &self.systimer_target0_intr_map
    }
    #[doc = "0xd4 - SYSTIMER_TARGET1_INTR mapping register"]
    #[inline(always)]
    pub const fn systimer_target1_intr_map(&self) -> &SYSTIMER_TARGET1_INTR_MAP {
        &self.systimer_target1_intr_map
    }
    #[doc = "0xd8 - SYSTIMER_TARGET2_INTR mapping register"]
    #[inline(always)]
    pub const fn systimer_target2_intr_map(&self) -> &SYSTIMER_TARGET2_INTR_MAP {
        &self.systimer_target2_intr_map
    }
    #[doc = "0xdc - APB_ADC_INTR mapping register"]
    #[inline(always)]
    pub const fn apb_adc_intr_map(&self) -> &APB_ADC_INTR_MAP {
        &self.apb_adc_intr_map
    }
    #[doc = "0xe0 - SLC0_INTR mapping register"]
    #[inline(always)]
    pub const fn slc0_intr_map(&self) -> &SLC0_INTR_MAP {
        &self.slc0_intr_map
    }
    #[doc = "0xe4 - SLC1_INTR mapping register"]
    #[inline(always)]
    pub const fn slc1_intr_map(&self) -> &SLC1_INTR_MAP {
        &self.slc1_intr_map
    }
    #[doc = "0xe8 - DMA_IN_CH0_INTR mapping register"]
    #[inline(always)]
    pub const fn dma_in_ch0_intr_map(&self) -> &DMA_IN_CH0_INTR_MAP {
        &self.dma_in_ch0_intr_map
    }
    #[doc = "0xec - DMA_IN_CH1_INTR mapping register"]
    #[inline(always)]
    pub const fn dma_in_ch1_intr_map(&self) -> &DMA_IN_CH1_INTR_MAP {
        &self.dma_in_ch1_intr_map
    }
    #[doc = "0xf0 - DMA_OUT_CH0_INTR mapping register"]
    #[inline(always)]
    pub const fn dma_out_ch0_intr_map(&self) -> &DMA_OUT_CH0_INTR_MAP {
        &self.dma_out_ch0_intr_map
    }
    #[doc = "0xf4 - DMA_OUT_CH1_INTR mapping register"]
    #[inline(always)]
    pub const fn dma_out_ch1_intr_map(&self) -> &DMA_OUT_CH1_INTR_MAP {
        &self.dma_out_ch1_intr_map
    }
    #[doc = "0xf8 - GPSPI2_INTR mapping register"]
    #[inline(always)]
    pub const fn gpspi2_intr_map(&self) -> &GPSPI2_INTR_MAP {
        &self.gpspi2_intr_map
    }
    #[doc = "0xfc - SHA_INTR mapping register"]
    #[inline(always)]
    pub const fn sha_intr_map(&self) -> &SHA_INTR_MAP {
        &self.sha_intr_map
    }
    #[doc = "0x100 - ECC_INTR mapping register"]
    #[inline(always)]
    pub const fn ecc_intr_map(&self) -> &ECC_INTR_MAP {
        &self.ecc_intr_map
    }
    #[doc = "0x104 - ECDSA_INTR mapping register"]
    #[inline(always)]
    pub const fn ecdsa_intr_map(&self) -> &ECDSA_INTR_MAP {
        &self.ecdsa_intr_map
    }
    #[doc = "0x108 - Status register for interrupt sources 0 ~ 31"]
    #[inline(always)]
    pub const fn int_status_reg_0(&self) -> &INT_STATUS_REG_0 {
        &self.int_status_reg_0
    }
    #[doc = "0x10c - Status register for interrupt sources 32 ~ 63"]
    #[inline(always)]
    pub const fn int_status_reg_1(&self) -> &INT_STATUS_REG_1 {
        &self.int_status_reg_1
    }
    #[doc = "0x110 - Status register for interrupt sources 64 ~ 65"]
    #[inline(always)]
    pub const fn int_status_reg_2(&self) -> &INT_STATUS_REG_2 {
        &self.int_status_reg_2
    }
    #[doc = "0x114 - PASS_IN_SEC status register for interrupt sources 0 ~ 31"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_0(&self) -> &SRC_PASS_IN_SEC_STATUS_0 {
        &self.src_pass_in_sec_status_0
    }
    #[doc = "0x118 - PASS_IN_SEC status register for interrupt sources 32 ~ 63"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_1(&self) -> &SRC_PASS_IN_SEC_STATUS_1 {
        &self.src_pass_in_sec_status_1
    }
    #[doc = "0x11c - PASS_IN_SEC status register for interrupt sources 64 ~ 65"]
    #[inline(always)]
    pub const fn src_pass_in_sec_status_2(&self) -> &SRC_PASS_IN_SEC_STATUS_2 {
        &self.src_pass_in_sec_status_2
    }
    #[doc = "0x120 - reserved"]
    #[inline(always)]
    pub const fn sig_idx_assert_in_sec(&self) -> &SIG_IDX_ASSERT_IN_SEC {
        &self.sig_idx_assert_in_sec
    }
    #[doc = "0x124 - reserved"]
    #[inline(always)]
    pub const fn secure_status(&self) -> &SECURE_STATUS {
        &self.secure_status
    }
    #[doc = "0x128 - Interrupt clock gating configure register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - Version control register"]
    #[inline(always)]
    pub const fn interrupt_date(&self) -> &INTERRUPT_DATE {
        &self.interrupt_date
    }
}
#[doc = "WIFI_MAC_INTR_MAP (rw) register accessor: WIFI_MAC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_mac_intr_map`] module"]
pub type WIFI_MAC_INTR_MAP = crate::Reg<wifi_mac_intr_map::WIFI_MAC_INTR_MAP_SPEC>;
#[doc = "WIFI_MAC_INTR mapping register"]
pub mod wifi_mac_intr_map;
#[doc = "WIFI_MAC_NMI_MAP (rw) register accessor: WIFI_MAC_NMI mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_mac_nmi_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_mac_nmi_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_mac_nmi_map`] module"]
pub type WIFI_MAC_NMI_MAP = crate::Reg<wifi_mac_nmi_map::WIFI_MAC_NMI_MAP_SPEC>;
#[doc = "WIFI_MAC_NMI mapping register"]
pub mod wifi_mac_nmi_map;
#[doc = "WIFI_PWR_INTR_MAP (rw) register accessor: WIFI_PWR_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pwr_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pwr_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pwr_intr_map`] module"]
pub type WIFI_PWR_INTR_MAP = crate::Reg<wifi_pwr_intr_map::WIFI_PWR_INTR_MAP_SPEC>;
#[doc = "WIFI_PWR_INTR mapping register"]
pub mod wifi_pwr_intr_map;
#[doc = "WIFI_BB_INTR_MAP (rw) register accessor: WIFI_BB_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_bb_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_bb_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_intr_map`] module"]
pub type WIFI_BB_INTR_MAP = crate::Reg<wifi_bb_intr_map::WIFI_BB_INTR_MAP_SPEC>;
#[doc = "WIFI_BB_INTR mapping register"]
pub mod wifi_bb_intr_map;
#[doc = "BT_MAC_INTR_MAP (rw) register accessor: BT_MAC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_mac_intr_map`] module"]
pub type BT_MAC_INTR_MAP = crate::Reg<bt_mac_intr_map::BT_MAC_INTR_MAP_SPEC>;
#[doc = "BT_MAC_INTR mapping register"]
pub mod bt_mac_intr_map;
#[doc = "BT_BB_INTR_MAP (rw) register accessor: BT_BB_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_bb_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_bb_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_intr_map`] module"]
pub type BT_BB_INTR_MAP = crate::Reg<bt_bb_intr_map::BT_BB_INTR_MAP_SPEC>;
#[doc = "BT_BB_INTR mapping register"]
pub mod bt_bb_intr_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: BT_BB_NMI mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_bb_nmi_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_nmi_map`] module"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "BT_BB_NMI mapping register"]
pub mod bt_bb_nmi_map;
#[doc = "LP_TIMER_INTR_MAP (rw) register accessor: LP_TIMER_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_intr_map`] module"]
pub type LP_TIMER_INTR_MAP = crate::Reg<lp_timer_intr_map::LP_TIMER_INTR_MAP_SPEC>;
#[doc = "LP_TIMER_INTR mapping register"]
pub mod lp_timer_intr_map;
#[doc = "COEX_INTR_MAP (rw) register accessor: COEX_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_intr_map`] module"]
pub type COEX_INTR_MAP = crate::Reg<coex_intr_map::COEX_INTR_MAP_SPEC>;
#[doc = "COEX_INTR mapping register"]
pub mod coex_intr_map;
#[doc = "BLE_TIMER_INTR_MAP (rw) register accessor: BLE_TIMER_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_timer_intr_map`] module"]
pub type BLE_TIMER_INTR_MAP = crate::Reg<ble_timer_intr_map::BLE_TIMER_INTR_MAP_SPEC>;
#[doc = "BLE_TIMER_INTR mapping register"]
pub mod ble_timer_intr_map;
#[doc = "BLE_SEC_INTR_MAP (rw) register accessor: BLE_SEC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_sec_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_sec_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_sec_intr_map`] module"]
pub type BLE_SEC_INTR_MAP = crate::Reg<ble_sec_intr_map::BLE_SEC_INTR_MAP_SPEC>;
#[doc = "BLE_SEC_INTR mapping register"]
pub mod ble_sec_intr_map;
#[doc = "I2C_MST_INTR_MAP (rw) register accessor: I2C_MST_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mst_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mst_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mst_intr_map`] module"]
pub type I2C_MST_INTR_MAP = crate::Reg<i2c_mst_intr_map::I2C_MST_INTR_MAP_SPEC>;
#[doc = "I2C_MST_INTR mapping register"]
pub mod i2c_mst_intr_map;
#[doc = "ZB_MAC_INTR_MAP (rw) register accessor: ZB_MAC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`zb_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zb_mac_intr_map`] module"]
pub type ZB_MAC_INTR_MAP = crate::Reg<zb_mac_intr_map::ZB_MAC_INTR_MAP_SPEC>;
#[doc = "ZB_MAC_INTR mapping register"]
pub mod zb_mac_intr_map;
#[doc = "PMU_INTR_MAP (rw) register accessor: PMU_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_intr_map`] module"]
pub type PMU_INTR_MAP = crate::Reg<pmu_intr_map::PMU_INTR_MAP_SPEC>;
#[doc = "PMU_INTR mapping register"]
pub mod pmu_intr_map;
#[doc = "EFUSE_INTR_MAP (rw) register accessor: EFUSE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_intr_map`] module"]
pub type EFUSE_INTR_MAP = crate::Reg<efuse_intr_map::EFUSE_INTR_MAP_SPEC>;
#[doc = "EFUSE_INTR mapping register"]
pub mod efuse_intr_map;
#[doc = "LP_RTC_TIMER_INTR_MAP (rw) register accessor: LP_RTC_TIMER_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rtc_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rtc_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rtc_timer_intr_map`] module"]
pub type LP_RTC_TIMER_INTR_MAP = crate::Reg<lp_rtc_timer_intr_map::LP_RTC_TIMER_INTR_MAP_SPEC>;
#[doc = "LP_RTC_TIMER_INTR mapping register"]
pub mod lp_rtc_timer_intr_map;
#[doc = "LP_WDT_INTR_MAP (rw) register accessor: LP_WDT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_intr_map`] module"]
pub type LP_WDT_INTR_MAP = crate::Reg<lp_wdt_intr_map::LP_WDT_INTR_MAP_SPEC>;
#[doc = "LP_WDT_INTR mapping register"]
pub mod lp_wdt_intr_map;
#[doc = "LP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: LP_PERI_TIMEOUT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_intr_map`] module"]
pub type LP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<lp_peri_timeout_intr_map::LP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "LP_PERI_TIMEOUT_INTR mapping register"]
pub mod lp_peri_timeout_intr_map;
#[doc = "LP_APM_M0_INTR_MAP (rw) register accessor: LP_APM_M0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m0_intr_map`] module"]
pub type LP_APM_M0_INTR_MAP = crate::Reg<lp_apm_m0_intr_map::LP_APM_M0_INTR_MAP_SPEC>;
#[doc = "LP_APM_M0_INTR mapping register"]
pub mod lp_apm_m0_intr_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: CPU_INTR_FROM_CPU_0 mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_0 mapping register"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: CPU_INTR_FROM_CPU_1 mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_1 mapping register"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: CPU_INTR_FROM_CPU_2 mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_2 mapping register"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: CPU_INTR_FROM_CPU_3 mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_3 mapping register"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: ASSIST_DEBUG_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_debug_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_debug_intr_map`] module"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "ASSIST_DEBUG_INTR mapping register"]
pub mod assist_debug_intr_map;
#[doc = "TRACE_INTR_MAP (rw) register accessor: TRACE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_intr_map`] module"]
pub type TRACE_INTR_MAP = crate::Reg<trace_intr_map::TRACE_INTR_MAP_SPEC>;
#[doc = "TRACE_INTR mapping register"]
pub mod trace_intr_map;
#[doc = "CACHE_INTR_MAP (rw) register accessor: CACHE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_intr_map`] module"]
pub type CACHE_INTR_MAP = crate::Reg<cache_intr_map::CACHE_INTR_MAP_SPEC>;
#[doc = "CACHE_INTR mapping register"]
pub mod cache_intr_map;
#[doc = "CPU_PERI_TIMEOUT_INTR_MAP (rw) register accessor: CPU_PERI_TIMEOUT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri_timeout_intr_map`] module"]
pub type CPU_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<cpu_peri_timeout_intr_map::CPU_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "CPU_PERI_TIMEOUT_INTR mapping register"]
pub mod cpu_peri_timeout_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: GPIO_INTERRUPT_PRO mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_interrupt_pro_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_pro_map`] module"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_PRO mapping register"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_EXT_MAP (rw) register accessor: GPIO_INTERRUPT_EXT mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_interrupt_ext_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_interrupt_ext_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_ext_map`] module"]
pub type GPIO_INTERRUPT_EXT_MAP = crate::Reg<gpio_interrupt_ext_map::GPIO_INTERRUPT_EXT_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_EXT mapping register"]
pub mod gpio_interrupt_ext_map;
#[doc = "PAU_INTR_MAP (rw) register accessor: PAU_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`pau_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pau_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pau_intr_map`] module"]
pub type PAU_INTR_MAP = crate::Reg<pau_intr_map::PAU_INTR_MAP_SPEC>;
#[doc = "PAU_INTR mapping register"]
pub mod pau_intr_map;
#[doc = "HP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: HP_PERI_TIMEOUT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri_timeout_intr_map`] module"]
pub type HP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<hp_peri_timeout_intr_map::HP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "HP_PERI_TIMEOUT_INTR mapping register"]
pub mod hp_peri_timeout_intr_map;
#[doc = "MODEM_PERI_TIMEOUT_INTR_MAP (rw) register accessor: MODEM_PERI_TIMEOUT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_peri_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_peri_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_peri_timeout_intr_map`] module"]
pub type MODEM_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<modem_peri_timeout_intr_map::MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "MODEM_PERI_TIMEOUT_INTR mapping register"]
pub mod modem_peri_timeout_intr_map;
#[doc = "HP_APM_M0_INTR_MAP (rw) register accessor: HP_APM_M0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m0_intr_map`] module"]
pub type HP_APM_M0_INTR_MAP = crate::Reg<hp_apm_m0_intr_map::HP_APM_M0_INTR_MAP_SPEC>;
#[doc = "HP_APM_M0_INTR mapping register"]
pub mod hp_apm_m0_intr_map;
#[doc = "HP_APM_M1_INTR_MAP (rw) register accessor: HP_APM_M1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m1_intr_map`] module"]
pub type HP_APM_M1_INTR_MAP = crate::Reg<hp_apm_m1_intr_map::HP_APM_M1_INTR_MAP_SPEC>;
#[doc = "HP_APM_M1_INTR mapping register"]
pub mod hp_apm_m1_intr_map;
#[doc = "HP_APM_M2_INTR_MAP (rw) register accessor: HP_APM_M2_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m2_intr_map`] module"]
pub type HP_APM_M2_INTR_MAP = crate::Reg<hp_apm_m2_intr_map::HP_APM_M2_INTR_MAP_SPEC>;
#[doc = "HP_APM_M2_INTR mapping register"]
pub mod hp_apm_m2_intr_map;
#[doc = "HP_APM_M3_INTR_MAP (rw) register accessor: HP_APM_M3_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m3_intr_map`] module"]
pub type HP_APM_M3_INTR_MAP = crate::Reg<hp_apm_m3_intr_map::HP_APM_M3_INTR_MAP_SPEC>;
#[doc = "HP_APM_M3_INTR mapping register"]
pub mod hp_apm_m3_intr_map;
#[doc = "CPU_APM_M0_INTR_MAP (rw) register accessor: CPU_APM_M0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m0_intr_map`] module"]
pub type CPU_APM_M0_INTR_MAP = crate::Reg<cpu_apm_m0_intr_map::CPU_APM_M0_INTR_MAP_SPEC>;
#[doc = "CPU_APM_M0_INTR mapping register"]
pub mod cpu_apm_m0_intr_map;
#[doc = "CPU_APM_M1_INTR_MAP (rw) register accessor: CPU_APM_M1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m1_intr_map`] module"]
pub type CPU_APM_M1_INTR_MAP = crate::Reg<cpu_apm_m1_intr_map::CPU_APM_M1_INTR_MAP_SPEC>;
#[doc = "CPU_APM_M1_INTR mapping register"]
pub mod cpu_apm_m1_intr_map;
#[doc = "MSPI_INTR_MAP (rw) register accessor: MSPI_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_intr_map`] module"]
pub type MSPI_INTR_MAP = crate::Reg<mspi_intr_map::MSPI_INTR_MAP_SPEC>;
#[doc = "MSPI_INTR mapping register"]
pub mod mspi_intr_map;
#[doc = "I2S1_INTR_MAP (rw) register accessor: I2S1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_intr_map`] module"]
pub type I2S1_INTR_MAP = crate::Reg<i2s1_intr_map::I2S1_INTR_MAP_SPEC>;
#[doc = "I2S1_INTR mapping register"]
pub mod i2s1_intr_map;
#[doc = "UART0_INTR_MAP (rw) register accessor: UART0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_intr_map`] module"]
pub type UART0_INTR_MAP = crate::Reg<uart0_intr_map::UART0_INTR_MAP_SPEC>;
#[doc = "UART0_INTR mapping register"]
pub mod uart0_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: UART1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "UART1_INTR mapping register"]
pub mod uart1_intr_map;
#[doc = "UART2_INTR_MAP (rw) register accessor: UART2_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_intr_map`] module"]
pub type UART2_INTR_MAP = crate::Reg<uart2_intr_map::UART2_INTR_MAP_SPEC>;
#[doc = "UART2_INTR mapping register"]
pub mod uart2_intr_map;
#[doc = "LEDC_INTR_MAP (rw) register accessor: LEDC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_intr_map`] module"]
pub type LEDC_INTR_MAP = crate::Reg<ledc_intr_map::LEDC_INTR_MAP_SPEC>;
#[doc = "LEDC_INTR mapping register"]
pub mod ledc_intr_map;
#[doc = "USB_INTR_MAP (rw) register accessor: USB_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_intr_map`] module"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "USB_INTR mapping register"]
pub mod usb_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: I2C_EXT0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ext0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ext0_intr_map`] module"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "I2C_EXT0_INTR mapping register"]
pub mod i2c_ext0_intr_map;
#[doc = "TG0_T0_INTR_MAP (rw) register accessor: TG0_T0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0_t0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0_t0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_t0_intr_map`] module"]
pub type TG0_T0_INTR_MAP = crate::Reg<tg0_t0_intr_map::TG0_T0_INTR_MAP_SPEC>;
#[doc = "TG0_T0_INTR mapping register"]
pub mod tg0_t0_intr_map;
#[doc = "TG0_T1_INTR_MAP (rw) register accessor: TG0_T1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0_t1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0_t1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_t1_intr_map`] module"]
pub type TG0_T1_INTR_MAP = crate::Reg<tg0_t1_intr_map::TG0_T1_INTR_MAP_SPEC>;
#[doc = "TG0_T1_INTR mapping register"]
pub mod tg0_t1_intr_map;
#[doc = "TG0_WDT_INTR_MAP (rw) register accessor: TG0_WDT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0_wdt_intr_map`] module"]
pub type TG0_WDT_INTR_MAP = crate::Reg<tg0_wdt_intr_map::TG0_WDT_INTR_MAP_SPEC>;
#[doc = "TG0_WDT_INTR mapping register"]
pub mod tg0_wdt_intr_map;
#[doc = "TG1_T0_INTR_MAP (rw) register accessor: TG1_T0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg1_t0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg1_t0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_t0_intr_map`] module"]
pub type TG1_T0_INTR_MAP = crate::Reg<tg1_t0_intr_map::TG1_T0_INTR_MAP_SPEC>;
#[doc = "TG1_T0_INTR mapping register"]
pub mod tg1_t0_intr_map;
#[doc = "TG1_T1_INTR_MAP (rw) register accessor: TG1_T1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg1_t1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg1_t1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_t1_intr_map`] module"]
pub type TG1_T1_INTR_MAP = crate::Reg<tg1_t1_intr_map::TG1_T1_INTR_MAP_SPEC>;
#[doc = "TG1_T1_INTR mapping register"]
pub mod tg1_t1_intr_map;
#[doc = "TG1_WDT_INTR_MAP (rw) register accessor: TG1_WDT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg1_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg1_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_wdt_intr_map`] module"]
pub type TG1_WDT_INTR_MAP = crate::Reg<tg1_wdt_intr_map::TG1_WDT_INTR_MAP_SPEC>;
#[doc = "TG1_WDT_INTR mapping register"]
pub mod tg1_wdt_intr_map;
#[doc = "SYSTIMER_TARGET0_INTR_MAP (rw) register accessor: SYSTIMER_TARGET0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target0_intr_map`] module"]
pub type SYSTIMER_TARGET0_INTR_MAP =
    crate::Reg<systimer_target0_intr_map::SYSTIMER_TARGET0_INTR_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET0_INTR mapping register"]
pub mod systimer_target0_intr_map;
#[doc = "SYSTIMER_TARGET1_INTR_MAP (rw) register accessor: SYSTIMER_TARGET1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target1_intr_map`] module"]
pub type SYSTIMER_TARGET1_INTR_MAP =
    crate::Reg<systimer_target1_intr_map::SYSTIMER_TARGET1_INTR_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET1_INTR mapping register"]
pub mod systimer_target1_intr_map;
#[doc = "SYSTIMER_TARGET2_INTR_MAP (rw) register accessor: SYSTIMER_TARGET2_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target2_intr_map`] module"]
pub type SYSTIMER_TARGET2_INTR_MAP =
    crate::Reg<systimer_target2_intr_map::SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET2_INTR mapping register"]
pub mod systimer_target2_intr_map;
#[doc = "APB_ADC_INTR_MAP (rw) register accessor: APB_ADC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_adc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_adc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_adc_intr_map`] module"]
pub type APB_ADC_INTR_MAP = crate::Reg<apb_adc_intr_map::APB_ADC_INTR_MAP_SPEC>;
#[doc = "APB_ADC_INTR mapping register"]
pub mod apb_adc_intr_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: SLC0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`slc0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slc0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_intr_map`] module"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "SLC0_INTR mapping register"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: SLC1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`slc1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slc1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_intr_map`] module"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "SLC1_INTR mapping register"]
pub mod slc1_intr_map;
#[doc = "DMA_IN_CH0_INTR_MAP (rw) register accessor: DMA_IN_CH0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_in_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_ch0_intr_map`] module"]
pub type DMA_IN_CH0_INTR_MAP = crate::Reg<dma_in_ch0_intr_map::DMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = "DMA_IN_CH0_INTR mapping register"]
pub mod dma_in_ch0_intr_map;
#[doc = "DMA_IN_CH1_INTR_MAP (rw) register accessor: DMA_IN_CH1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_in_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_ch1_intr_map`] module"]
pub type DMA_IN_CH1_INTR_MAP = crate::Reg<dma_in_ch1_intr_map::DMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = "DMA_IN_CH1_INTR mapping register"]
pub mod dma_in_ch1_intr_map;
#[doc = "DMA_OUT_CH0_INTR_MAP (rw) register accessor: DMA_OUT_CH0_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_ch0_intr_map`] module"]
pub type DMA_OUT_CH0_INTR_MAP = crate::Reg<dma_out_ch0_intr_map::DMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = "DMA_OUT_CH0_INTR mapping register"]
pub mod dma_out_ch0_intr_map;
#[doc = "DMA_OUT_CH1_INTR_MAP (rw) register accessor: DMA_OUT_CH1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_ch1_intr_map`] module"]
pub type DMA_OUT_CH1_INTR_MAP = crate::Reg<dma_out_ch1_intr_map::DMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = "DMA_OUT_CH1_INTR mapping register"]
pub mod dma_out_ch1_intr_map;
#[doc = "GPSPI2_INTR_MAP (rw) register accessor: GPSPI2_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi2_intr_map`] module"]
pub type GPSPI2_INTR_MAP = crate::Reg<gpspi2_intr_map::GPSPI2_INTR_MAP_SPEC>;
#[doc = "GPSPI2_INTR mapping register"]
pub mod gpspi2_intr_map;
#[doc = "SHA_INTR_MAP (rw) register accessor: SHA_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_intr_map`] module"]
pub type SHA_INTR_MAP = crate::Reg<sha_intr_map::SHA_INTR_MAP_SPEC>;
#[doc = "SHA_INTR mapping register"]
pub mod sha_intr_map;
#[doc = "ECC_INTR_MAP (rw) register accessor: ECC_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_intr_map`] module"]
pub type ECC_INTR_MAP = crate::Reg<ecc_intr_map::ECC_INTR_MAP_SPEC>;
#[doc = "ECC_INTR mapping register"]
pub mod ecc_intr_map;
#[doc = "ECDSA_INTR_MAP (rw) register accessor: ECDSA_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa_intr_map`] module"]
pub type ECDSA_INTR_MAP = crate::Reg<ecdsa_intr_map::ECDSA_INTR_MAP_SPEC>;
#[doc = "ECDSA_INTR mapping register"]
pub mod ecdsa_intr_map;
#[doc = "INT_STATUS_REG_0 (r) register accessor: Status register for interrupt sources 0 ~ 31\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_reg_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_reg_0`] module"]
pub type INT_STATUS_REG_0 = crate::Reg<int_status_reg_0::INT_STATUS_REG_0_SPEC>;
#[doc = "Status register for interrupt sources 0 ~ 31"]
pub mod int_status_reg_0;
#[doc = "INT_STATUS_REG_1 (r) register accessor: Status register for interrupt sources 32 ~ 63\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_reg_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_reg_1`] module"]
pub type INT_STATUS_REG_1 = crate::Reg<int_status_reg_1::INT_STATUS_REG_1_SPEC>;
#[doc = "Status register for interrupt sources 32 ~ 63"]
pub mod int_status_reg_1;
#[doc = "INT_STATUS_REG_2 (r) register accessor: Status register for interrupt sources 64 ~ 65\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_reg_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_reg_2`] module"]
pub type INT_STATUS_REG_2 = crate::Reg<int_status_reg_2::INT_STATUS_REG_2_SPEC>;
#[doc = "Status register for interrupt sources 64 ~ 65"]
pub mod int_status_reg_2;
#[doc = "SRC_PASS_IN_SEC_STATUS_0 (r) register accessor: PASS_IN_SEC status register for interrupt sources 0 ~ 31\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_0`] module"]
pub type SRC_PASS_IN_SEC_STATUS_0 =
    crate::Reg<src_pass_in_sec_status_0::SRC_PASS_IN_SEC_STATUS_0_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 0 ~ 31"]
pub mod src_pass_in_sec_status_0;
#[doc = "SRC_PASS_IN_SEC_STATUS_1 (r) register accessor: PASS_IN_SEC status register for interrupt sources 32 ~ 63\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_1`] module"]
pub type SRC_PASS_IN_SEC_STATUS_1 =
    crate::Reg<src_pass_in_sec_status_1::SRC_PASS_IN_SEC_STATUS_1_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 32 ~ 63"]
pub mod src_pass_in_sec_status_1;
#[doc = "SRC_PASS_IN_SEC_STATUS_2 (r) register accessor: PASS_IN_SEC status register for interrupt sources 64 ~ 65\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_sec_status_2`] module"]
pub type SRC_PASS_IN_SEC_STATUS_2 =
    crate::Reg<src_pass_in_sec_status_2::SRC_PASS_IN_SEC_STATUS_2_SPEC>;
#[doc = "PASS_IN_SEC status register for interrupt sources 64 ~ 65"]
pub mod src_pass_in_sec_status_2;
#[doc = "SIG_IDX_ASSERT_IN_SEC (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_idx_assert_in_sec`] module"]
pub type SIG_IDX_ASSERT_IN_SEC = crate::Reg<sig_idx_assert_in_sec::SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "reserved"]
pub mod sig_idx_assert_in_sec;
#[doc = "SECURE_STATUS (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_status`] module"]
pub type SECURE_STATUS = crate::Reg<secure_status::SECURE_STATUS_SPEC>;
#[doc = "reserved"]
pub mod secure_status;
#[doc = "CLOCK_GATE (rw) register accessor: Interrupt clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Interrupt clock gating configure register"]
pub mod clock_gate;
#[doc = "INTERRUPT_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_date`] module"]
pub type INTERRUPT_DATE = crate::Reg<interrupt_date::INTERRUPT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod interrupt_date;
