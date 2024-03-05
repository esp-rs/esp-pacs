#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac_intr_map: MAC_INTR_MAP,
    mac_nmi_map: MAC_NMI_MAP,
    pwr_intr_map: PWR_INTR_MAP,
    bb_int_map: BB_INT_MAP,
    bt_mac_int_map: BT_MAC_INT_MAP,
    bt_bb_int_map: BT_BB_INT_MAP,
    bt_bb_nmi_map: BT_BB_NMI_MAP,
    rwbt_irq_map: RWBT_IRQ_MAP,
    rwble_irq_map: RWBLE_IRQ_MAP,
    rwbt_nmi_map: RWBT_NMI_MAP,
    rwble_nmi_map: RWBLE_NMI_MAP,
    i2c_mst_int_map: I2C_MST_INT_MAP,
    slc0_intr_map: SLC0_INTR_MAP,
    slc1_intr_map: SLC1_INTR_MAP,
    apb_ctrl_intr_map: APB_CTRL_INTR_MAP,
    uhci0_intr_map: UHCI0_INTR_MAP,
    gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    spi_intr_1_map: SPI_INTR_1_MAP,
    spi_intr_2_map: SPI_INTR_2_MAP,
    i2s1_int_map: I2S1_INT_MAP,
    uart_intr_map: UART_INTR_MAP,
    uart1_intr_map: UART1_INTR_MAP,
    ledc_int_map: LEDC_INT_MAP,
    efuse_int_map: EFUSE_INT_MAP,
    can_int_map: CAN_INT_MAP,
    usb_intr_map: USB_INTR_MAP,
    rtc_core_intr_map: RTC_CORE_INTR_MAP,
    rmt_intr_map: RMT_INTR_MAP,
    i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    timer_int1_map: TIMER_INT1_MAP,
    timer_int2_map: TIMER_INT2_MAP,
    tg_t0_int_map: TG_T0_INT_MAP,
    tg_wdt_int_map: TG_WDT_INT_MAP,
    tg1_t0_int_map: TG1_T0_INT_MAP,
    tg1_wdt_int_map: TG1_WDT_INT_MAP,
    cache_ia_int_map: CACHE_IA_INT_MAP,
    systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    apb_adc_int_map: APB_ADC_INT_MAP,
    dma_ch0_int_map: DMA_CH0_INT_MAP,
    dma_ch1_int_map: DMA_CH1_INT_MAP,
    dma_ch2_int_map: DMA_CH2_INT_MAP,
    rsa_int_map: RSA_INT_MAP,
    aes_int_map: AES_INT_MAP,
    sha_int_map: SHA_INT_MAP,
    cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    dma_apbperi_pms_monitor_violate_intr_map: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_iram0_pms_monitor_violate_intr_map: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_dram0_pms_monitor_violate_intr_map: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_pif_pms_monitor_violate_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    backup_pms_violate_intr_map: BACKUP_PMS_VIOLATE_INTR_MAP,
    cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    intr_status_reg_0: INTR_STATUS_REG_0,
    intr_status_reg_1: INTR_STATUS_REG_1,
    clock_gate: CLOCK_GATE,
    cpu_int_enable: CPU_INT_ENABLE,
    cpu_int_type: CPU_INT_TYPE,
    cpu_int_clear: CPU_INT_CLEAR,
    cpu_int_eip_status: CPU_INT_EIP_STATUS,
    cpu_int_pri_0: CPU_INT_PRI_0,
    cpu_int_pri_1: CPU_INT_PRI_1,
    cpu_int_pri_2: CPU_INT_PRI_2,
    cpu_int_pri_3: CPU_INT_PRI_3,
    cpu_int_pri_4: CPU_INT_PRI_4,
    cpu_int_pri_5: CPU_INT_PRI_5,
    cpu_int_pri_6: CPU_INT_PRI_6,
    cpu_int_pri_7: CPU_INT_PRI_7,
    cpu_int_pri_8: CPU_INT_PRI_8,
    cpu_int_pri_9: CPU_INT_PRI_9,
    cpu_int_pri_10: CPU_INT_PRI_10,
    cpu_int_pri_11: CPU_INT_PRI_11,
    cpu_int_pri_12: CPU_INT_PRI_12,
    cpu_int_pri_13: CPU_INT_PRI_13,
    cpu_int_pri_14: CPU_INT_PRI_14,
    cpu_int_pri_15: CPU_INT_PRI_15,
    cpu_int_pri_16: CPU_INT_PRI_16,
    cpu_int_pri_17: CPU_INT_PRI_17,
    cpu_int_pri_18: CPU_INT_PRI_18,
    cpu_int_pri_19: CPU_INT_PRI_19,
    cpu_int_pri_20: CPU_INT_PRI_20,
    cpu_int_pri_21: CPU_INT_PRI_21,
    cpu_int_pri_22: CPU_INT_PRI_22,
    cpu_int_pri_23: CPU_INT_PRI_23,
    cpu_int_pri_24: CPU_INT_PRI_24,
    cpu_int_pri_25: CPU_INT_PRI_25,
    cpu_int_pri_26: CPU_INT_PRI_26,
    cpu_int_pri_27: CPU_INT_PRI_27,
    cpu_int_pri_28: CPU_INT_PRI_28,
    cpu_int_pri_29: CPU_INT_PRI_29,
    cpu_int_pri_30: CPU_INT_PRI_30,
    cpu_int_pri_31: CPU_INT_PRI_31,
    cpu_int_thresh: CPU_INT_THRESH,
    _reserved102: [u8; 0x0664],
    interrupt_reg_date: INTERRUPT_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - mac intr map register"]
    #[inline(always)]
    pub const fn mac_intr_map(&self) -> &MAC_INTR_MAP {
        &self.mac_intr_map
    }
    #[doc = "0x04 - mac nmi_intr map register"]
    #[inline(always)]
    pub const fn mac_nmi_map(&self) -> &MAC_NMI_MAP {
        &self.mac_nmi_map
    }
    #[doc = "0x08 - pwr intr map register"]
    #[inline(always)]
    pub const fn pwr_intr_map(&self) -> &PWR_INTR_MAP {
        &self.pwr_intr_map
    }
    #[doc = "0x0c - bb intr map register"]
    #[inline(always)]
    pub const fn bb_int_map(&self) -> &BB_INT_MAP {
        &self.bb_int_map
    }
    #[doc = "0x10 - bt intr map register"]
    #[inline(always)]
    pub const fn bt_mac_int_map(&self) -> &BT_MAC_INT_MAP {
        &self.bt_mac_int_map
    }
    #[doc = "0x14 - bb_bt intr map register"]
    #[inline(always)]
    pub const fn bt_bb_int_map(&self) -> &BT_BB_INT_MAP {
        &self.bt_bb_int_map
    }
    #[doc = "0x18 - bb_bt_nmi intr map register"]
    #[inline(always)]
    pub const fn bt_bb_nmi_map(&self) -> &BT_BB_NMI_MAP {
        &self.bt_bb_nmi_map
    }
    #[doc = "0x1c - rwbt intr map register"]
    #[inline(always)]
    pub const fn rwbt_irq_map(&self) -> &RWBT_IRQ_MAP {
        &self.rwbt_irq_map
    }
    #[doc = "0x20 - rwble intr map register"]
    #[inline(always)]
    pub const fn rwble_irq_map(&self) -> &RWBLE_IRQ_MAP {
        &self.rwble_irq_map
    }
    #[doc = "0x24 - rwbt_nmi intr map register"]
    #[inline(always)]
    pub const fn rwbt_nmi_map(&self) -> &RWBT_NMI_MAP {
        &self.rwbt_nmi_map
    }
    #[doc = "0x28 - rwble_nmi intr map register"]
    #[inline(always)]
    pub const fn rwble_nmi_map(&self) -> &RWBLE_NMI_MAP {
        &self.rwble_nmi_map
    }
    #[doc = "0x2c - i2c intr map register"]
    #[inline(always)]
    pub const fn i2c_mst_int_map(&self) -> &I2C_MST_INT_MAP {
        &self.i2c_mst_int_map
    }
    #[doc = "0x30 - slc0 intr map register"]
    #[inline(always)]
    pub const fn slc0_intr_map(&self) -> &SLC0_INTR_MAP {
        &self.slc0_intr_map
    }
    #[doc = "0x34 - slc1 intr map register"]
    #[inline(always)]
    pub const fn slc1_intr_map(&self) -> &SLC1_INTR_MAP {
        &self.slc1_intr_map
    }
    #[doc = "0x38 - apb_ctrl intr map register"]
    #[inline(always)]
    pub const fn apb_ctrl_intr_map(&self) -> &APB_CTRL_INTR_MAP {
        &self.apb_ctrl_intr_map
    }
    #[doc = "0x3c - uchi0 intr map register"]
    #[inline(always)]
    pub const fn uhci0_intr_map(&self) -> &UHCI0_INTR_MAP {
        &self.uhci0_intr_map
    }
    #[doc = "0x40 - gpio intr map register"]
    #[inline(always)]
    pub const fn gpio_interrupt_pro_map(&self) -> &GPIO_INTERRUPT_PRO_MAP {
        &self.gpio_interrupt_pro_map
    }
    #[doc = "0x44 - gpio_pro intr map register"]
    #[inline(always)]
    pub const fn gpio_interrupt_pro_nmi_map(&self) -> &GPIO_INTERRUPT_PRO_NMI_MAP {
        &self.gpio_interrupt_pro_nmi_map
    }
    #[doc = "0x48 - gpio_pro_nmi intr map register"]
    #[inline(always)]
    pub const fn spi_intr_1_map(&self) -> &SPI_INTR_1_MAP {
        &self.spi_intr_1_map
    }
    #[doc = "0x4c - spi1 intr map register"]
    #[inline(always)]
    pub const fn spi_intr_2_map(&self) -> &SPI_INTR_2_MAP {
        &self.spi_intr_2_map
    }
    #[doc = "0x50 - spi2 intr map register"]
    #[inline(always)]
    pub const fn i2s1_int_map(&self) -> &I2S1_INT_MAP {
        &self.i2s1_int_map
    }
    #[doc = "0x54 - i2s1 intr map register"]
    #[inline(always)]
    pub const fn uart_intr_map(&self) -> &UART_INTR_MAP {
        &self.uart_intr_map
    }
    #[doc = "0x58 - uart1 intr map register"]
    #[inline(always)]
    pub const fn uart1_intr_map(&self) -> &UART1_INTR_MAP {
        &self.uart1_intr_map
    }
    #[doc = "0x5c - ledc intr map register"]
    #[inline(always)]
    pub const fn ledc_int_map(&self) -> &LEDC_INT_MAP {
        &self.ledc_int_map
    }
    #[doc = "0x60 - efuse intr map register"]
    #[inline(always)]
    pub const fn efuse_int_map(&self) -> &EFUSE_INT_MAP {
        &self.efuse_int_map
    }
    #[doc = "0x64 - can intr map register"]
    #[inline(always)]
    pub const fn can_int_map(&self) -> &CAN_INT_MAP {
        &self.can_int_map
    }
    #[doc = "0x68 - usb intr map register"]
    #[inline(always)]
    pub const fn usb_intr_map(&self) -> &USB_INTR_MAP {
        &self.usb_intr_map
    }
    #[doc = "0x6c - rtc intr map register"]
    #[inline(always)]
    pub const fn rtc_core_intr_map(&self) -> &RTC_CORE_INTR_MAP {
        &self.rtc_core_intr_map
    }
    #[doc = "0x70 - rmt intr map register"]
    #[inline(always)]
    pub const fn rmt_intr_map(&self) -> &RMT_INTR_MAP {
        &self.rmt_intr_map
    }
    #[doc = "0x74 - i2c intr map register"]
    #[inline(always)]
    pub const fn i2c_ext0_intr_map(&self) -> &I2C_EXT0_INTR_MAP {
        &self.i2c_ext0_intr_map
    }
    #[doc = "0x78 - timer1 intr map register"]
    #[inline(always)]
    pub const fn timer_int1_map(&self) -> &TIMER_INT1_MAP {
        &self.timer_int1_map
    }
    #[doc = "0x7c - timer2 intr map register"]
    #[inline(always)]
    pub const fn timer_int2_map(&self) -> &TIMER_INT2_MAP {
        &self.timer_int2_map
    }
    #[doc = "0x80 - tg to intr map register"]
    #[inline(always)]
    pub const fn tg_t0_int_map(&self) -> &TG_T0_INT_MAP {
        &self.tg_t0_int_map
    }
    #[doc = "0x84 - tg wdt intr map register"]
    #[inline(always)]
    pub const fn tg_wdt_int_map(&self) -> &TG_WDT_INT_MAP {
        &self.tg_wdt_int_map
    }
    #[doc = "0x88 - tg1 to intr map register"]
    #[inline(always)]
    pub const fn tg1_t0_int_map(&self) -> &TG1_T0_INT_MAP {
        &self.tg1_t0_int_map
    }
    #[doc = "0x8c - tg1 wdt intr map register"]
    #[inline(always)]
    pub const fn tg1_wdt_int_map(&self) -> &TG1_WDT_INT_MAP {
        &self.tg1_wdt_int_map
    }
    #[doc = "0x90 - cache ia intr map register"]
    #[inline(always)]
    pub const fn cache_ia_int_map(&self) -> &CACHE_IA_INT_MAP {
        &self.cache_ia_int_map
    }
    #[doc = "0x94 - systimer intr map register"]
    #[inline(always)]
    pub const fn systimer_target0_int_map(&self) -> &SYSTIMER_TARGET0_INT_MAP {
        &self.systimer_target0_int_map
    }
    #[doc = "0x98 - systimer target1 intr map register"]
    #[inline(always)]
    pub const fn systimer_target1_int_map(&self) -> &SYSTIMER_TARGET1_INT_MAP {
        &self.systimer_target1_int_map
    }
    #[doc = "0x9c - systimer target2 intr map register"]
    #[inline(always)]
    pub const fn systimer_target2_int_map(&self) -> &SYSTIMER_TARGET2_INT_MAP {
        &self.systimer_target2_int_map
    }
    #[doc = "0xa0 - spi mem reject intr map register"]
    #[inline(always)]
    pub const fn spi_mem_reject_intr_map(&self) -> &SPI_MEM_REJECT_INTR_MAP {
        &self.spi_mem_reject_intr_map
    }
    #[doc = "0xa4 - icache perload intr map register"]
    #[inline(always)]
    pub const fn icache_preload_int_map(&self) -> &ICACHE_PRELOAD_INT_MAP {
        &self.icache_preload_int_map
    }
    #[doc = "0xa8 - icache sync intr map register"]
    #[inline(always)]
    pub const fn icache_sync_int_map(&self) -> &ICACHE_SYNC_INT_MAP {
        &self.icache_sync_int_map
    }
    #[doc = "0xac - adc intr map register"]
    #[inline(always)]
    pub const fn apb_adc_int_map(&self) -> &APB_ADC_INT_MAP {
        &self.apb_adc_int_map
    }
    #[doc = "0xb0 - dma ch0 intr map register"]
    #[inline(always)]
    pub const fn dma_ch0_int_map(&self) -> &DMA_CH0_INT_MAP {
        &self.dma_ch0_int_map
    }
    #[doc = "0xb4 - dma ch1 intr map register"]
    #[inline(always)]
    pub const fn dma_ch1_int_map(&self) -> &DMA_CH1_INT_MAP {
        &self.dma_ch1_int_map
    }
    #[doc = "0xb8 - dma ch2 intr map register"]
    #[inline(always)]
    pub const fn dma_ch2_int_map(&self) -> &DMA_CH2_INT_MAP {
        &self.dma_ch2_int_map
    }
    #[doc = "0xbc - rsa intr map register"]
    #[inline(always)]
    pub const fn rsa_int_map(&self) -> &RSA_INT_MAP {
        &self.rsa_int_map
    }
    #[doc = "0xc0 - aes intr map register"]
    #[inline(always)]
    pub const fn aes_int_map(&self) -> &AES_INT_MAP {
        &self.aes_int_map
    }
    #[doc = "0xc4 - sha intr map register"]
    #[inline(always)]
    pub const fn sha_int_map(&self) -> &SHA_INT_MAP {
        &self.sha_int_map
    }
    #[doc = "0xc8 - cpu from cpu 0 intr map register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0_map(&self) -> &CPU_INTR_FROM_CPU_0_MAP {
        &self.cpu_intr_from_cpu_0_map
    }
    #[doc = "0xcc - cpu from cpu 0 intr map register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1_map(&self) -> &CPU_INTR_FROM_CPU_1_MAP {
        &self.cpu_intr_from_cpu_1_map
    }
    #[doc = "0xd0 - cpu from cpu 1 intr map register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2_map(&self) -> &CPU_INTR_FROM_CPU_2_MAP {
        &self.cpu_intr_from_cpu_2_map
    }
    #[doc = "0xd4 - cpu from cpu 3 intr map register"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3_map(&self) -> &CPU_INTR_FROM_CPU_3_MAP {
        &self.cpu_intr_from_cpu_3_map
    }
    #[doc = "0xd8 - assist debug intr map register"]
    #[inline(always)]
    pub const fn assist_debug_intr_map(&self) -> &ASSIST_DEBUG_INTR_MAP {
        &self.assist_debug_intr_map
    }
    #[doc = "0xdc - dma pms violatile intr map register"]
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_violate_intr_map(
        &self,
    ) -> &DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.dma_apbperi_pms_monitor_violate_intr_map
    }
    #[doc = "0xe0 - iram0 pms violatile intr map register"]
    #[inline(always)]
    pub const fn core_0_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_iram0_pms_monitor_violate_intr_map
    }
    #[doc = "0xe4 - mac intr map register"]
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_dram0_pms_monitor_violate_intr_map
    }
    #[doc = "0xe8 - mac intr map register"]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_pif_pms_monitor_violate_intr_map
    }
    #[doc = "0xec - mac intr map register"]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_violate_size_intr_map(
        &self,
    ) -> &CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {
        &self.core_0_pif_pms_monitor_violate_size_intr_map
    }
    #[doc = "0xf0 - mac intr map register"]
    #[inline(always)]
    pub const fn backup_pms_violate_intr_map(&self) -> &BACKUP_PMS_VIOLATE_INTR_MAP {
        &self.backup_pms_violate_intr_map
    }
    #[doc = "0xf4 - mac intr map register"]
    #[inline(always)]
    pub const fn cache_core0_acs_int_map(&self) -> &CACHE_CORE0_ACS_INT_MAP {
        &self.cache_core0_acs_int_map
    }
    #[doc = "0xf8 - mac intr map register"]
    #[inline(always)]
    pub const fn intr_status_reg_0(&self) -> &INTR_STATUS_REG_0 {
        &self.intr_status_reg_0
    }
    #[doc = "0xfc - mac intr map register"]
    #[inline(always)]
    pub const fn intr_status_reg_1(&self) -> &INTR_STATUS_REG_1 {
        &self.intr_status_reg_1
    }
    #[doc = "0x100 - mac intr map register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x104 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_enable(&self) -> &CPU_INT_ENABLE {
        &self.cpu_int_enable
    }
    #[doc = "0x108 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_type(&self) -> &CPU_INT_TYPE {
        &self.cpu_int_type
    }
    #[doc = "0x10c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_clear(&self) -> &CPU_INT_CLEAR {
        &self.cpu_int_clear
    }
    #[doc = "0x110 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_eip_status(&self) -> &CPU_INT_EIP_STATUS {
        &self.cpu_int_eip_status
    }
    #[doc = "0x114 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_0(&self) -> &CPU_INT_PRI_0 {
        &self.cpu_int_pri_0
    }
    #[doc = "0x118 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_1(&self) -> &CPU_INT_PRI_1 {
        &self.cpu_int_pri_1
    }
    #[doc = "0x11c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_2(&self) -> &CPU_INT_PRI_2 {
        &self.cpu_int_pri_2
    }
    #[doc = "0x120 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_3(&self) -> &CPU_INT_PRI_3 {
        &self.cpu_int_pri_3
    }
    #[doc = "0x124 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_4(&self) -> &CPU_INT_PRI_4 {
        &self.cpu_int_pri_4
    }
    #[doc = "0x128 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_5(&self) -> &CPU_INT_PRI_5 {
        &self.cpu_int_pri_5
    }
    #[doc = "0x12c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_6(&self) -> &CPU_INT_PRI_6 {
        &self.cpu_int_pri_6
    }
    #[doc = "0x130 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_7(&self) -> &CPU_INT_PRI_7 {
        &self.cpu_int_pri_7
    }
    #[doc = "0x134 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_8(&self) -> &CPU_INT_PRI_8 {
        &self.cpu_int_pri_8
    }
    #[doc = "0x138 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_9(&self) -> &CPU_INT_PRI_9 {
        &self.cpu_int_pri_9
    }
    #[doc = "0x13c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_10(&self) -> &CPU_INT_PRI_10 {
        &self.cpu_int_pri_10
    }
    #[doc = "0x140 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_11(&self) -> &CPU_INT_PRI_11 {
        &self.cpu_int_pri_11
    }
    #[doc = "0x144 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_12(&self) -> &CPU_INT_PRI_12 {
        &self.cpu_int_pri_12
    }
    #[doc = "0x148 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_13(&self) -> &CPU_INT_PRI_13 {
        &self.cpu_int_pri_13
    }
    #[doc = "0x14c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_14(&self) -> &CPU_INT_PRI_14 {
        &self.cpu_int_pri_14
    }
    #[doc = "0x150 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_15(&self) -> &CPU_INT_PRI_15 {
        &self.cpu_int_pri_15
    }
    #[doc = "0x154 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_16(&self) -> &CPU_INT_PRI_16 {
        &self.cpu_int_pri_16
    }
    #[doc = "0x158 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_17(&self) -> &CPU_INT_PRI_17 {
        &self.cpu_int_pri_17
    }
    #[doc = "0x15c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_18(&self) -> &CPU_INT_PRI_18 {
        &self.cpu_int_pri_18
    }
    #[doc = "0x160 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_19(&self) -> &CPU_INT_PRI_19 {
        &self.cpu_int_pri_19
    }
    #[doc = "0x164 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_20(&self) -> &CPU_INT_PRI_20 {
        &self.cpu_int_pri_20
    }
    #[doc = "0x168 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_21(&self) -> &CPU_INT_PRI_21 {
        &self.cpu_int_pri_21
    }
    #[doc = "0x16c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_22(&self) -> &CPU_INT_PRI_22 {
        &self.cpu_int_pri_22
    }
    #[doc = "0x170 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_23(&self) -> &CPU_INT_PRI_23 {
        &self.cpu_int_pri_23
    }
    #[doc = "0x174 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_24(&self) -> &CPU_INT_PRI_24 {
        &self.cpu_int_pri_24
    }
    #[doc = "0x178 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_25(&self) -> &CPU_INT_PRI_25 {
        &self.cpu_int_pri_25
    }
    #[doc = "0x17c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_26(&self) -> &CPU_INT_PRI_26 {
        &self.cpu_int_pri_26
    }
    #[doc = "0x180 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_27(&self) -> &CPU_INT_PRI_27 {
        &self.cpu_int_pri_27
    }
    #[doc = "0x184 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_28(&self) -> &CPU_INT_PRI_28 {
        &self.cpu_int_pri_28
    }
    #[doc = "0x188 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_29(&self) -> &CPU_INT_PRI_29 {
        &self.cpu_int_pri_29
    }
    #[doc = "0x18c - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_30(&self) -> &CPU_INT_PRI_30 {
        &self.cpu_int_pri_30
    }
    #[doc = "0x190 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_pri_31(&self) -> &CPU_INT_PRI_31 {
        &self.cpu_int_pri_31
    }
    #[doc = "0x194 - mac intr map register"]
    #[inline(always)]
    pub const fn cpu_int_thresh(&self) -> &CPU_INT_THRESH {
        &self.cpu_int_thresh
    }
    #[doc = "0x7fc - mac intr map register"]
    #[inline(always)]
    pub const fn interrupt_reg_date(&self) -> &INTERRUPT_REG_DATE {
        &self.interrupt_reg_date
    }
}
#[doc = "MAC_INTR_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_intr_map`] module"]
pub type MAC_INTR_MAP = crate::Reg<mac_intr_map::MAC_INTR_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod mac_intr_map;
#[doc = "MAC_NMI_MAP (rw) register accessor: mac nmi_intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_nmi_map`] module"]
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
#[doc = "mac nmi_intr map register"]
pub mod mac_nmi_map;
#[doc = "PWR_INTR_MAP (rw) register accessor: pwr intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_intr_map`] module"]
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
#[doc = "pwr intr map register"]
pub mod pwr_intr_map;
#[doc = "BB_INT_MAP (rw) register accessor: bb intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bb_int_map`] module"]
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
#[doc = "bb intr map register"]
pub mod bb_int_map;
#[doc = "BT_MAC_INT_MAP (rw) register accessor: bt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_mac_int_map`] module"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "bt intr map register"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP (rw) register accessor: bb_bt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_int_map`] module"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "bb_bt intr map register"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: bb_bt_nmi intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_bb_nmi_map`] module"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "bb_bt_nmi intr map register"]
pub mod bt_bb_nmi_map;
#[doc = "RWBT_IRQ_MAP (rw) register accessor: rwbt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwbt_irq_map`] module"]
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
#[doc = "rwbt intr map register"]
pub mod rwbt_irq_map;
#[doc = "RWBLE_IRQ_MAP (rw) register accessor: rwble intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwble_irq_map`] module"]
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
#[doc = "rwble intr map register"]
pub mod rwble_irq_map;
#[doc = "RWBT_NMI_MAP (rw) register accessor: rwbt_nmi intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwbt_nmi_map`] module"]
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
#[doc = "rwbt_nmi intr map register"]
pub mod rwbt_nmi_map;
#[doc = "RWBLE_NMI_MAP (rw) register accessor: rwble_nmi intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwble_nmi_map`] module"]
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
#[doc = "rwble_nmi intr map register"]
pub mod rwble_nmi_map;
#[doc = "I2C_MST_INT_MAP (rw) register accessor: i2c intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_mst_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_mst_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mst_int_map`] module"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "i2c intr map register"]
pub mod i2c_mst_int_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: slc0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_intr_map`] module"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "slc0 intr map register"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: slc1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_intr_map`] module"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "slc1 intr map register"]
pub mod slc1_intr_map;
#[doc = "APB_CTRL_INTR_MAP (rw) register accessor: apb_ctrl intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_ctrl_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_ctrl_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_ctrl_intr_map`] module"]
pub type APB_CTRL_INTR_MAP = crate::Reg<apb_ctrl_intr_map::APB_CTRL_INTR_MAP_SPEC>;
#[doc = "apb_ctrl intr map register"]
pub mod apb_ctrl_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: uchi0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci0_intr_map`] module"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "uchi0 intr map register"]
pub mod uhci0_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: gpio intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_pro_map`] module"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "gpio intr map register"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: gpio_pro intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_interrupt_pro_nmi_map`] module"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "gpio_pro intr map register"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "SPI_INTR_1_MAP (rw) register accessor: gpio_pro_nmi intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_intr_1_map`] module"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "gpio_pro_nmi intr map register"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP (rw) register accessor: spi1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_intr_2_map`] module"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "spi1 intr map register"]
pub mod spi_intr_2_map;
#[doc = "I2S1_INT_MAP (rw) register accessor: spi2 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_int_map`] module"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "spi2 intr map register"]
pub mod i2s1_int_map;
#[doc = "UART_INTR_MAP (rw) register accessor: i2s1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_intr_map`] module"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "i2s1 intr map register"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: uart1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "uart1 intr map register"]
pub mod uart1_intr_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: ledc intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_int_map`] module"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "ledc intr map register"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP (rw) register accessor: efuse intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_int_map`] module"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "efuse intr map register"]
pub mod efuse_int_map;
#[doc = "CAN_INT_MAP (rw) register accessor: can intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_int_map`] module"]
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
#[doc = "can intr map register"]
pub mod can_int_map;
#[doc = "USB_INTR_MAP (rw) register accessor: usb intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_intr_map`] module"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "usb intr map register"]
pub mod usb_intr_map;
#[doc = "RTC_CORE_INTR_MAP (rw) register accessor: rtc intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_core_intr_map`] module"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "rtc intr map register"]
pub mod rtc_core_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: rmt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_intr_map`] module"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "rmt intr map register"]
pub mod rmt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: i2c intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ext0_intr_map`] module"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "i2c intr map register"]
pub mod i2c_ext0_intr_map;
#[doc = "TIMER_INT1_MAP (rw) register accessor: timer1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int1_map`] module"]
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
#[doc = "timer1 intr map register"]
pub mod timer_int1_map;
#[doc = "TIMER_INT2_MAP (rw) register accessor: timer2 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int2_map`] module"]
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
#[doc = "timer2 intr map register"]
pub mod timer_int2_map;
#[doc = "TG_T0_INT_MAP (rw) register accessor: tg to intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg_t0_int_map`] module"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "tg to intr map register"]
pub mod tg_t0_int_map;
#[doc = "TG_WDT_INT_MAP (rw) register accessor: tg wdt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg_wdt_int_map`] module"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "tg wdt intr map register"]
pub mod tg_wdt_int_map;
#[doc = "TG1_T0_INT_MAP (rw) register accessor: tg1 to intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_t0_int_map`] module"]
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
#[doc = "tg1 to intr map register"]
pub mod tg1_t0_int_map;
#[doc = "TG1_WDT_INT_MAP (rw) register accessor: tg1 wdt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1_wdt_int_map`] module"]
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
#[doc = "tg1 wdt intr map register"]
pub mod tg1_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP (rw) register accessor: cache ia intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ia_int_map`] module"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "cache ia intr map register"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: systimer intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target0_int_map`] module"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "systimer intr map register"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: systimer target1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target1_int_map`] module"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "systimer target1 intr map register"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: systimer target2 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target2_int_map`] module"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "systimer target2 intr map register"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP (rw) register accessor: spi mem reject intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_reject_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_reject_intr_map`] module"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "spi mem reject intr map register"]
pub mod spi_mem_reject_intr_map;
#[doc = "ICACHE_PRELOAD_INT_MAP (rw) register accessor: icache perload intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_preload_int_map`] module"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "icache perload intr map register"]
pub mod icache_preload_int_map;
#[doc = "ICACHE_SYNC_INT_MAP (rw) register accessor: icache sync intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_int_map`] module"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "icache sync intr map register"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP (rw) register accessor: adc intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_adc_int_map`] module"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "adc intr map register"]
pub mod apb_adc_int_map;
#[doc = "DMA_CH0_INT_MAP (rw) register accessor: dma ch0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ch0_int_map`] module"]
pub type DMA_CH0_INT_MAP = crate::Reg<dma_ch0_int_map::DMA_CH0_INT_MAP_SPEC>;
#[doc = "dma ch0 intr map register"]
pub mod dma_ch0_int_map;
#[doc = "DMA_CH1_INT_MAP (rw) register accessor: dma ch1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ch1_int_map`] module"]
pub type DMA_CH1_INT_MAP = crate::Reg<dma_ch1_int_map::DMA_CH1_INT_MAP_SPEC>;
#[doc = "dma ch1 intr map register"]
pub mod dma_ch1_int_map;
#[doc = "DMA_CH2_INT_MAP (rw) register accessor: dma ch2 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ch2_int_map`] module"]
pub type DMA_CH2_INT_MAP = crate::Reg<dma_ch2_int_map::DMA_CH2_INT_MAP_SPEC>;
#[doc = "dma ch2 intr map register"]
pub mod dma_ch2_int_map;
#[doc = "RSA_INT_MAP (rw) register accessor: rsa intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_int_map`] module"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "rsa intr map register"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP (rw) register accessor: aes intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_int_map`] module"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "aes intr map register"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: sha intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_int_map`] module"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "sha intr map register"]
pub mod sha_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: cpu from cpu 0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "cpu from cpu 0 intr map register"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: cpu from cpu 0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "cpu from cpu 0 intr map register"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: cpu from cpu 1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "cpu from cpu 1 intr map register"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: cpu from cpu 3 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "cpu from cpu 3 intr map register"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: assist debug intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_debug_intr_map`] module"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "assist debug intr map register"]
pub mod assist_debug_intr_map;
#[doc = "DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: dma pms violatile intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_pms_monitor_violate_intr_map`] module"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "dma pms violatile intr map register"]
pub mod dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: iram0 pms violatile intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "iram0 pms violatile intr map register"]
pub mod core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_dram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_size_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_size_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_violate_size_intr_map`] module"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "BACKUP_PMS_VIOLATE_INTR_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_pms_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_pms_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_pms_violate_intr_map`] module"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod backup_pms_violate_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_core0_acs_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_core0_acs_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_core0_acs_int_map`] module"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod cache_core0_acs_int_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_0`] module"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "mac intr map register"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_1`] module"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "mac intr map register"]
pub mod intr_status_reg_1;
#[doc = "CLOCK_GATE (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "mac intr map register"]
pub mod clock_gate;
#[doc = "CPU_INT_ENABLE (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_enable`] module"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_type`] module"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_type;
#[doc = "CPU_INT_CLEAR (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_clear`] module"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_clear;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_eip_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_eip_status`] module"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_0`] module"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_1`] module"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_2`] module"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_3`] module"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_4`] module"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_5`] module"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_6`] module"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_7`] module"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_8`] module"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_9`] module"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_10`] module"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_11`] module"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_12`] module"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_13`] module"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_14`] module"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_15`] module"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_16`] module"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_17`] module"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_18`] module"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_19`] module"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_20`] module"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_21`] module"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_22`] module"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_23`] module"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_24`] module"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_25`] module"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_26`] module"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_27`] module"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_28`] module"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_29`] module"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_30`] module"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_pri_31`] module"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_thresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_thresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_thresh`] module"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_thresh;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_reg_date`] module"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "mac intr map register"]
pub mod interrupt_reg_date;
