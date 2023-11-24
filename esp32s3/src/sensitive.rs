#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    cache_dataarray_connect_0: CACHE_DATAARRAY_CONNECT_0,
    cache_dataarray_connect_1: CACHE_DATAARRAY_CONNECT_1,
    apb_peripheral_access_0: APB_PERIPHERAL_ACCESS_0,
    apb_peripheral_access_1: APB_PERIPHERAL_ACCESS_1,
    internal_sram_usage_0: INTERNAL_SRAM_USAGE_0,
    internal_sram_usage_1: INTERNAL_SRAM_USAGE_1,
    internal_sram_usage_2: INTERNAL_SRAM_USAGE_2,
    internal_sram_usage_3: INTERNAL_SRAM_USAGE_3,
    internal_sram_usage_4: INTERNAL_SRAM_USAGE_4,
    retention_disable: RETENTION_DISABLE,
    cache_tag_access_0: CACHE_TAG_ACCESS_0,
    cache_tag_access_1: CACHE_TAG_ACCESS_1,
    cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    dma_apbperi_spi2_pms_constrain_0: DMA_APBPERI_SPI2_PMS_CONSTRAIN_0,
    dma_apbperi_spi2_pms_constrain_1: DMA_APBPERI_SPI2_PMS_CONSTRAIN_1,
    dma_apbperi_spi3_pms_constrain_0: DMA_APBPERI_SPI3_PMS_CONSTRAIN_0,
    dma_apbperi_spi3_pms_constrain_1: DMA_APBPERI_SPI3_PMS_CONSTRAIN_1,
    dma_apbperi_uhci0_pms_constrain_0: DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0,
    dma_apbperi_uhci0_pms_constrain_1: DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1,
    dma_apbperi_i2s0_pms_constrain_0: DMA_APBPERI_I2S0_PMS_CONSTRAIN_0,
    dma_apbperi_i2s0_pms_constrain_1: DMA_APBPERI_I2S0_PMS_CONSTRAIN_1,
    dma_apbperi_i2s1_pms_constrain_0: DMA_APBPERI_I2S1_PMS_CONSTRAIN_0,
    dma_apbperi_i2s1_pms_constrain_1: DMA_APBPERI_I2S1_PMS_CONSTRAIN_1,
    dma_apbperi_mac_pms_constrain_0: DMA_APBPERI_MAC_PMS_CONSTRAIN_0,
    dma_apbperi_mac_pms_constrain_1: DMA_APBPERI_MAC_PMS_CONSTRAIN_1,
    dma_apbperi_backup_pms_constrain_0: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0,
    dma_apbperi_backup_pms_constrain_1: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1,
    dma_apbperi_aes_pms_constrain_0: DMA_APBPERI_AES_PMS_CONSTRAIN_0,
    dma_apbperi_aes_pms_constrain_1: DMA_APBPERI_AES_PMS_CONSTRAIN_1,
    dma_apbperi_sha_pms_constrain_0: DMA_APBPERI_SHA_PMS_CONSTRAIN_0,
    dma_apbperi_sha_pms_constrain_1: DMA_APBPERI_SHA_PMS_CONSTRAIN_1,
    dma_apbperi_adc_dac_pms_constrain_0: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0,
    dma_apbperi_adc_dac_pms_constrain_1: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1,
    dma_apbperi_rmt_pms_constrain_0: DMA_APBPERI_RMT_PMS_CONSTRAIN_0,
    dma_apbperi_rmt_pms_constrain_1: DMA_APBPERI_RMT_PMS_CONSTRAIN_1,
    dma_apbperi_lcd_cam_pms_constrain_0: DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0,
    dma_apbperi_lcd_cam_pms_constrain_1: DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1,
    dma_apbperi_usb_pms_constrain_0: DMA_APBPERI_USB_PMS_CONSTRAIN_0,
    dma_apbperi_usb_pms_constrain_1: DMA_APBPERI_USB_PMS_CONSTRAIN_1,
    dma_apbperi_lc_pms_constrain_0: DMA_APBPERI_LC_PMS_CONSTRAIN_0,
    dma_apbperi_lc_pms_constrain_1: DMA_APBPERI_LC_PMS_CONSTRAIN_1,
    dma_apbperi_sdio_pms_constrain_0: DMA_APBPERI_SDIO_PMS_CONSTRAIN_0,
    dma_apbperi_sdio_pms_constrain_1: DMA_APBPERI_SDIO_PMS_CONSTRAIN_1,
    dma_apbperi_pms_monitor_0: DMA_APBPERI_PMS_MONITOR_0,
    dma_apbperi_pms_monitor_1: DMA_APBPERI_PMS_MONITOR_1,
    dma_apbperi_pms_monitor_2: DMA_APBPERI_PMS_MONITOR_2,
    dma_apbperi_pms_monitor_3: DMA_APBPERI_PMS_MONITOR_3,
    core_x_iram0_dram0_dma_split_line_constrain_0: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0,
    core_x_iram0_dram0_dma_split_line_constrain_1: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1,
    core_x_iram0_dram0_dma_split_line_constrain_2: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2,
    core_x_iram0_dram0_dma_split_line_constrain_3: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3,
    core_x_iram0_dram0_dma_split_line_constrain_4: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4,
    core_x_iram0_dram0_dma_split_line_constrain_5: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5,
    core_x_iram0_pms_constrain_0: CORE_X_IRAM0_PMS_CONSTRAIN_0,
    core_x_iram0_pms_constrain_1: CORE_X_IRAM0_PMS_CONSTRAIN_1,
    core_x_iram0_pms_constrain_2: CORE_X_IRAM0_PMS_CONSTRAIN_2,
    core_0_iram0_pms_monitor_0: CORE_0_IRAM0_PMS_MONITOR_0,
    core_0_iram0_pms_monitor_1: CORE_0_IRAM0_PMS_MONITOR_1,
    core_0_iram0_pms_monitor_2: CORE_0_IRAM0_PMS_MONITOR_2,
    core_1_iram0_pms_monitor_0: CORE_1_IRAM0_PMS_MONITOR_0,
    core_1_iram0_pms_monitor_1: CORE_1_IRAM0_PMS_MONITOR_1,
    core_1_iram0_pms_monitor_2: CORE_1_IRAM0_PMS_MONITOR_2,
    core_x_dram0_pms_constrain_0: CORE_X_DRAM0_PMS_CONSTRAIN_0,
    core_x_dram0_pms_constrain_1: CORE_X_DRAM0_PMS_CONSTRAIN_1,
    core_0_dram0_pms_monitor_0: CORE_0_DRAM0_PMS_MONITOR_0,
    core_0_dram0_pms_monitor_1: CORE_0_DRAM0_PMS_MONITOR_1,
    core_0_dram0_pms_monitor_2: CORE_0_DRAM0_PMS_MONITOR_2,
    core_0_dram0_pms_monitor_3: CORE_0_DRAM0_PMS_MONITOR_3,
    core_1_dram0_pms_monitor_0: CORE_1_DRAM0_PMS_MONITOR_0,
    core_1_dram0_pms_monitor_1: CORE_1_DRAM0_PMS_MONITOR_1,
    core_1_dram0_pms_monitor_2: CORE_1_DRAM0_PMS_MONITOR_2,
    core_1_dram0_pms_monitor_3: CORE_1_DRAM0_PMS_MONITOR_3,
    core_0_pif_pms_constrain_0: CORE_0_PIF_PMS_CONSTRAIN_0,
    core_0_pif_pms_constrain_1: CORE_0_PIF_PMS_CONSTRAIN_1,
    core_0_pif_pms_constrain_2: CORE_0_PIF_PMS_CONSTRAIN_2,
    core_0_pif_pms_constrain_3: CORE_0_PIF_PMS_CONSTRAIN_3,
    core_0_pif_pms_constrain_4: CORE_0_PIF_PMS_CONSTRAIN_4,
    core_0_pif_pms_constrain_5: CORE_0_PIF_PMS_CONSTRAIN_5,
    core_0_pif_pms_constrain_6: CORE_0_PIF_PMS_CONSTRAIN_6,
    core_0_pif_pms_constrain_7: CORE_0_PIF_PMS_CONSTRAIN_7,
    core_0_pif_pms_constrain_8: CORE_0_PIF_PMS_CONSTRAIN_8,
    core_0_pif_pms_constrain_9: CORE_0_PIF_PMS_CONSTRAIN_9,
    core_0_pif_pms_constrain_10: CORE_0_PIF_PMS_CONSTRAIN_10,
    core_0_pif_pms_constrain_11: CORE_0_PIF_PMS_CONSTRAIN_11,
    core_0_pif_pms_constrain_12: CORE_0_PIF_PMS_CONSTRAIN_12,
    core_0_pif_pms_constrain_13: CORE_0_PIF_PMS_CONSTRAIN_13,
    core_0_pif_pms_constrain_14: CORE_0_PIF_PMS_CONSTRAIN_14,
    core_0_region_pms_constrain_0: CORE_0_REGION_PMS_CONSTRAIN_0,
    core_0_region_pms_constrain_1: CORE_0_REGION_PMS_CONSTRAIN_1,
    core_0_region_pms_constrain_2: CORE_0_REGION_PMS_CONSTRAIN_2,
    core_0_region_pms_constrain_3: CORE_0_REGION_PMS_CONSTRAIN_3,
    core_0_region_pms_constrain_4: CORE_0_REGION_PMS_CONSTRAIN_4,
    core_0_region_pms_constrain_5: CORE_0_REGION_PMS_CONSTRAIN_5,
    core_0_region_pms_constrain_6: CORE_0_REGION_PMS_CONSTRAIN_6,
    core_0_region_pms_constrain_7: CORE_0_REGION_PMS_CONSTRAIN_7,
    core_0_region_pms_constrain_8: CORE_0_REGION_PMS_CONSTRAIN_8,
    core_0_region_pms_constrain_9: CORE_0_REGION_PMS_CONSTRAIN_9,
    core_0_region_pms_constrain_10: CORE_0_REGION_PMS_CONSTRAIN_10,
    core_0_region_pms_constrain_11: CORE_0_REGION_PMS_CONSTRAIN_11,
    core_0_region_pms_constrain_12: CORE_0_REGION_PMS_CONSTRAIN_12,
    core_0_region_pms_constrain_13: CORE_0_REGION_PMS_CONSTRAIN_13,
    core_0_region_pms_constrain_14: CORE_0_REGION_PMS_CONSTRAIN_14,
    core_0_pif_pms_monitor_0: CORE_0_PIF_PMS_MONITOR_0,
    core_0_pif_pms_monitor_1: CORE_0_PIF_PMS_MONITOR_1,
    core_0_pif_pms_monitor_2: CORE_0_PIF_PMS_MONITOR_2,
    core_0_pif_pms_monitor_3: CORE_0_PIF_PMS_MONITOR_3,
    core_0_pif_pms_monitor_4: CORE_0_PIF_PMS_MONITOR_4,
    core_0_pif_pms_monitor_5: CORE_0_PIF_PMS_MONITOR_5,
    core_0_pif_pms_monitor_6: CORE_0_PIF_PMS_MONITOR_6,
    core_0_vecbase_override_lock: CORE_0_VECBASE_OVERRIDE_LOCK,
    core_0_vecbase_override_0: CORE_0_VECBASE_OVERRIDE_0,
    core_0_vecbase_override_1: CORE_0_VECBASE_OVERRIDE_1,
    core_0_vecbase_override_2: CORE_0_VECBASE_OVERRIDE_2,
    core_0_toomanyexceptions_m_override_0: CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0,
    core_0_toomanyexceptions_m_override_1: CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1,
    core_1_pif_pms_constrain_0: CORE_1_PIF_PMS_CONSTRAIN_0,
    core_1_pif_pms_constrain_1: CORE_1_PIF_PMS_CONSTRAIN_1,
    core_1_pif_pms_constrain_2: CORE_1_PIF_PMS_CONSTRAIN_2,
    core_1_pif_pms_constrain_3: CORE_1_PIF_PMS_CONSTRAIN_3,
    core_1_pif_pms_constrain_4: CORE_1_PIF_PMS_CONSTRAIN_4,
    core_1_pif_pms_constrain_5: CORE_1_PIF_PMS_CONSTRAIN_5,
    core_1_pif_pms_constrain_6: CORE_1_PIF_PMS_CONSTRAIN_6,
    core_1_pif_pms_constrain_7: CORE_1_PIF_PMS_CONSTRAIN_7,
    core_1_pif_pms_constrain_8: CORE_1_PIF_PMS_CONSTRAIN_8,
    core_1_pif_pms_constrain_9: CORE_1_PIF_PMS_CONSTRAIN_9,
    core_1_pif_pms_constrain_10: CORE_1_PIF_PMS_CONSTRAIN_10,
    core_1_pif_pms_constrain_11: CORE_1_PIF_PMS_CONSTRAIN_11,
    core_1_pif_pms_constrain_12: CORE_1_PIF_PMS_CONSTRAIN_12,
    core_1_pif_pms_constrain_13: CORE_1_PIF_PMS_CONSTRAIN_13,
    core_1_pif_pms_constrain_14: CORE_1_PIF_PMS_CONSTRAIN_14,
    core_1_region_pms_constrain_0: CORE_1_REGION_PMS_CONSTRAIN_0,
    core_1_region_pms_constrain_1: CORE_1_REGION_PMS_CONSTRAIN_1,
    core_1_region_pms_constrain_2: CORE_1_REGION_PMS_CONSTRAIN_2,
    core_1_region_pms_constrain_3: CORE_1_REGION_PMS_CONSTRAIN_3,
    core_1_region_pms_constrain_4: CORE_1_REGION_PMS_CONSTRAIN_4,
    core_1_region_pms_constrain_5: CORE_1_REGION_PMS_CONSTRAIN_5,
    core_1_region_pms_constrain_6: CORE_1_REGION_PMS_CONSTRAIN_6,
    core_1_region_pms_constrain_7: CORE_1_REGION_PMS_CONSTRAIN_7,
    core_1_region_pms_constrain_8: CORE_1_REGION_PMS_CONSTRAIN_8,
    core_1_region_pms_constrain_9: CORE_1_REGION_PMS_CONSTRAIN_9,
    core_1_region_pms_constrain_10: CORE_1_REGION_PMS_CONSTRAIN_10,
    core_1_region_pms_constrain_11: CORE_1_REGION_PMS_CONSTRAIN_11,
    core_1_region_pms_constrain_12: CORE_1_REGION_PMS_CONSTRAIN_12,
    core_1_region_pms_constrain_13: CORE_1_REGION_PMS_CONSTRAIN_13,
    core_1_region_pms_constrain_14: CORE_1_REGION_PMS_CONSTRAIN_14,
    core_1_pif_pms_monitor_0: CORE_1_PIF_PMS_MONITOR_0,
    core_1_pif_pms_monitor_1: CORE_1_PIF_PMS_MONITOR_1,
    core_1_pif_pms_monitor_2: CORE_1_PIF_PMS_MONITOR_2,
    core_1_pif_pms_monitor_3: CORE_1_PIF_PMS_MONITOR_3,
    core_1_pif_pms_monitor_4: CORE_1_PIF_PMS_MONITOR_4,
    core_1_pif_pms_monitor_5: CORE_1_PIF_PMS_MONITOR_5,
    core_1_pif_pms_monitor_6: CORE_1_PIF_PMS_MONITOR_6,
    core_1_vecbase_override_lock: CORE_1_VECBASE_OVERRIDE_LOCK,
    core_1_vecbase_override_0: CORE_1_VECBASE_OVERRIDE_0,
    core_1_vecbase_override_1: CORE_1_VECBASE_OVERRIDE_1,
    core_1_vecbase_override_2: CORE_1_VECBASE_OVERRIDE_2,
    core_1_toomanyexceptions_m_override_0: CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0,
    core_1_toomanyexceptions_m_override_1: CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1,
    backup_bus_pms_constrain_0: BACKUP_BUS_PMS_CONSTRAIN_0,
    backup_bus_pms_constrain_1: BACKUP_BUS_PMS_CONSTRAIN_1,
    backup_bus_pms_constrain_2: BACKUP_BUS_PMS_CONSTRAIN_2,
    backup_bus_pms_constrain_3: BACKUP_BUS_PMS_CONSTRAIN_3,
    backup_bus_pms_constrain_4: BACKUP_BUS_PMS_CONSTRAIN_4,
    backup_bus_pms_constrain_5: BACKUP_BUS_PMS_CONSTRAIN_5,
    backup_bus_pms_constrain_6: BACKUP_BUS_PMS_CONSTRAIN_6,
    backup_bus_pms_monitor_0: BACKUP_BUS_PMS_MONITOR_0,
    backup_bus_pms_monitor_1: BACKUP_BUS_PMS_MONITOR_1,
    backup_bus_pms_monitor_2: BACKUP_BUS_PMS_MONITOR_2,
    backup_bus_pms_monitor_3: BACKUP_BUS_PMS_MONITOR_3,
    edma_boundary_lock: EDMA_BOUNDARY_LOCK,
    edma_boundary_0: EDMA_BOUNDARY_0,
    edma_boundary_1: EDMA_BOUNDARY_1,
    edma_boundary_2: EDMA_BOUNDARY_2,
    edma_pms_spi2_lock: EDMA_PMS_SPI2_LOCK,
    edma_pms_spi2: EDMA_PMS_SPI2,
    edma_pms_spi3_lock: EDMA_PMS_SPI3_LOCK,
    edma_pms_spi3: EDMA_PMS_SPI3,
    edma_pms_uhci0_lock: EDMA_PMS_UHCI0_LOCK,
    edma_pms_uhci0: EDMA_PMS_UHCI0,
    edma_pms_i2s0_lock: EDMA_PMS_I2S0_LOCK,
    edma_pms_i2s0: EDMA_PMS_I2S0,
    edma_pms_i2s1_lock: EDMA_PMS_I2S1_LOCK,
    edma_pms_i2s1: EDMA_PMS_I2S1,
    edma_pms_lcd_cam_lock: EDMA_PMS_LCD_CAM_LOCK,
    edma_pms_lcd_cam: EDMA_PMS_LCD_CAM,
    edma_pms_aes_lock: EDMA_PMS_AES_LOCK,
    edma_pms_aes: EDMA_PMS_AES,
    edma_pms_sha_lock: EDMA_PMS_SHA_LOCK,
    edma_pms_sha: EDMA_PMS_SHA,
    edma_pms_adc_dac_lock: EDMA_PMS_ADC_DAC_LOCK,
    edma_pms_adc_dac: EDMA_PMS_ADC_DAC,
    edma_pms_rmt_lock: EDMA_PMS_RMT_LOCK,
    edma_pms_rmt: EDMA_PMS_RMT,
    clock_gate: CLOCK_GATE,
    rtc_pms: RTC_PMS,
    _reserved196: [u8; 0x0cec],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache data array configuration register 0."]
    #[inline(always)]
    pub const fn cache_dataarray_connect_0(&self) -> &CACHE_DATAARRAY_CONNECT_0 {
        &self.cache_dataarray_connect_0
    }
    #[doc = "0x04 - Cache data array configuration register 1."]
    #[inline(always)]
    pub const fn cache_dataarray_connect_1(&self) -> &CACHE_DATAARRAY_CONNECT_1 {
        &self.cache_dataarray_connect_1
    }
    #[doc = "0x08 - APB peripheral configuration register 0."]
    #[inline(always)]
    pub const fn apb_peripheral_access_0(&self) -> &APB_PERIPHERAL_ACCESS_0 {
        &self.apb_peripheral_access_0
    }
    #[doc = "0x0c - APB peripheral configuration register 1."]
    #[inline(always)]
    pub const fn apb_peripheral_access_1(&self) -> &APB_PERIPHERAL_ACCESS_1 {
        &self.apb_peripheral_access_1
    }
    #[doc = "0x10 - Internal SRAM configuration register 0."]
    #[inline(always)]
    pub const fn internal_sram_usage_0(&self) -> &INTERNAL_SRAM_USAGE_0 {
        &self.internal_sram_usage_0
    }
    #[doc = "0x14 - Internal SRAM configuration register 1."]
    #[inline(always)]
    pub const fn internal_sram_usage_1(&self) -> &INTERNAL_SRAM_USAGE_1 {
        &self.internal_sram_usage_1
    }
    #[doc = "0x18 - Internal SRAM configuration register 2."]
    #[inline(always)]
    pub const fn internal_sram_usage_2(&self) -> &INTERNAL_SRAM_USAGE_2 {
        &self.internal_sram_usage_2
    }
    #[doc = "0x1c - Internal SRAM configuration register 3."]
    #[inline(always)]
    pub const fn internal_sram_usage_3(&self) -> &INTERNAL_SRAM_USAGE_3 {
        &self.internal_sram_usage_3
    }
    #[doc = "0x20 - Internal SRAM configuration register 4."]
    #[inline(always)]
    pub const fn internal_sram_usage_4(&self) -> &INTERNAL_SRAM_USAGE_4 {
        &self.internal_sram_usage_4
    }
    #[doc = "0x24 - Retention configuration register."]
    #[inline(always)]
    pub const fn retention_disable(&self) -> &RETENTION_DISABLE {
        &self.retention_disable
    }
    #[doc = "0x28 - Cache tag configuration register 0."]
    #[inline(always)]
    pub const fn cache_tag_access_0(&self) -> &CACHE_TAG_ACCESS_0 {
        &self.cache_tag_access_0
    }
    #[doc = "0x2c - Cache tag configuration register 1."]
    #[inline(always)]
    pub const fn cache_tag_access_1(&self) -> &CACHE_TAG_ACCESS_1 {
        &self.cache_tag_access_1
    }
    #[doc = "0x30 - Cache MMU configuration register 0."]
    #[inline(always)]
    pub const fn cache_mmu_access_0(&self) -> &CACHE_MMU_ACCESS_0 {
        &self.cache_mmu_access_0
    }
    #[doc = "0x34 - Cache MMU configuration register 1."]
    #[inline(always)]
    pub const fn cache_mmu_access_1(&self) -> &CACHE_MMU_ACCESS_1 {
        &self.cache_mmu_access_1
    }
    #[doc = "0x38 - spi2 dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_spi2_pms_constrain_0(&self) -> &DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_spi2_pms_constrain_0
    }
    #[doc = "0x3c - spi2 dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_spi2_pms_constrain_1(&self) -> &DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_spi2_pms_constrain_1
    }
    #[doc = "0x40 - spi3 dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_spi3_pms_constrain_0(&self) -> &DMA_APBPERI_SPI3_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_spi3_pms_constrain_0
    }
    #[doc = "0x44 - spi3 dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_spi3_pms_constrain_1(&self) -> &DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_spi3_pms_constrain_1
    }
    #[doc = "0x48 - uhci0 dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_uhci0_pms_constrain_0(&self) -> &DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_uhci0_pms_constrain_0
    }
    #[doc = "0x4c - uhci0 dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_uhci0_pms_constrain_1(&self) -> &DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_uhci0_pms_constrain_1
    }
    #[doc = "0x50 - i2s0 dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_i2s0_pms_constrain_0(&self) -> &DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_i2s0_pms_constrain_0
    }
    #[doc = "0x54 - i2s0 dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_i2s0_pms_constrain_1(&self) -> &DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_i2s0_pms_constrain_1
    }
    #[doc = "0x58 - i2s1 dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_i2s1_pms_constrain_0(&self) -> &DMA_APBPERI_I2S1_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_i2s1_pms_constrain_0
    }
    #[doc = "0x5c - i2s1 dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_i2s1_pms_constrain_1(&self) -> &DMA_APBPERI_I2S1_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_i2s1_pms_constrain_1
    }
    #[doc = "0x60 - mac dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_mac_pms_constrain_0(&self) -> &DMA_APBPERI_MAC_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_mac_pms_constrain_0
    }
    #[doc = "0x64 - mac dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_mac_pms_constrain_1(&self) -> &DMA_APBPERI_MAC_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_mac_pms_constrain_1
    }
    #[doc = "0x68 - backup dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_backup_pms_constrain_0(&self) -> &DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_backup_pms_constrain_0
    }
    #[doc = "0x6c - backup dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_backup_pms_constrain_1(&self) -> &DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_backup_pms_constrain_1
    }
    #[doc = "0x70 - aes dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_aes_pms_constrain_0(&self) -> &DMA_APBPERI_AES_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_aes_pms_constrain_0
    }
    #[doc = "0x74 - aes dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_aes_pms_constrain_1(&self) -> &DMA_APBPERI_AES_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_aes_pms_constrain_1
    }
    #[doc = "0x78 - sha dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_sha_pms_constrain_0(&self) -> &DMA_APBPERI_SHA_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_sha_pms_constrain_0
    }
    #[doc = "0x7c - sha dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_sha_pms_constrain_1(&self) -> &DMA_APBPERI_SHA_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_sha_pms_constrain_1
    }
    #[doc = "0x80 - adc_dac dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_adc_dac_pms_constrain_0(
        &self,
    ) -> &DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_adc_dac_pms_constrain_0
    }
    #[doc = "0x84 - adc_dac dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_adc_dac_pms_constrain_1(
        &self,
    ) -> &DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_adc_dac_pms_constrain_1
    }
    #[doc = "0x88 - rmt dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_rmt_pms_constrain_0(&self) -> &DMA_APBPERI_RMT_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_rmt_pms_constrain_0
    }
    #[doc = "0x8c - rmt dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_rmt_pms_constrain_1(&self) -> &DMA_APBPERI_RMT_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_rmt_pms_constrain_1
    }
    #[doc = "0x90 - lcd_cam dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_lcd_cam_pms_constrain_0(
        &self,
    ) -> &DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_lcd_cam_pms_constrain_0
    }
    #[doc = "0x94 - lcd_cam dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_lcd_cam_pms_constrain_1(
        &self,
    ) -> &DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_lcd_cam_pms_constrain_1
    }
    #[doc = "0x98 - usb dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_usb_pms_constrain_0(&self) -> &DMA_APBPERI_USB_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_usb_pms_constrain_0
    }
    #[doc = "0x9c - usb dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_usb_pms_constrain_1(&self) -> &DMA_APBPERI_USB_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_usb_pms_constrain_1
    }
    #[doc = "0xa0 - lc dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_lc_pms_constrain_0(&self) -> &DMA_APBPERI_LC_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_lc_pms_constrain_0
    }
    #[doc = "0xa4 - lc dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_lc_pms_constrain_1(&self) -> &DMA_APBPERI_LC_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_lc_pms_constrain_1
    }
    #[doc = "0xa8 - sdio dma permission configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_sdio_pms_constrain_0(&self) -> &DMA_APBPERI_SDIO_PMS_CONSTRAIN_0 {
        &self.dma_apbperi_sdio_pms_constrain_0
    }
    #[doc = "0xac - sdio dma permission configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_sdio_pms_constrain_1(&self) -> &DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 {
        &self.dma_apbperi_sdio_pms_constrain_1
    }
    #[doc = "0xb0 - dma permission monitor configuration register 0."]
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_0(&self) -> &DMA_APBPERI_PMS_MONITOR_0 {
        &self.dma_apbperi_pms_monitor_0
    }
    #[doc = "0xb4 - dma permission monitor configuration register 1."]
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_1(&self) -> &DMA_APBPERI_PMS_MONITOR_1 {
        &self.dma_apbperi_pms_monitor_1
    }
    #[doc = "0xb8 - dma permission monitor configuration register 2."]
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_2(&self) -> &DMA_APBPERI_PMS_MONITOR_2 {
        &self.dma_apbperi_pms_monitor_2
    }
    #[doc = "0xbc - dma permission monitor configuration register 3."]
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_3(&self) -> &DMA_APBPERI_PMS_MONITOR_3 {
        &self.dma_apbperi_pms_monitor_3
    }
    #[doc = "0xc0 - sram split line configuration register 0"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_0(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_0
    }
    #[doc = "0xc4 - sram split line configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_1(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_1
    }
    #[doc = "0xc8 - sram split line configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_2(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_2
    }
    #[doc = "0xcc - sram split line configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_3(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_3
    }
    #[doc = "0xd0 - sram split line configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_4(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_4
    }
    #[doc = "0xd4 - sram split line configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_dma_split_line_constrain_5(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 {
        &self.core_x_iram0_dram0_dma_split_line_constrain_5
    }
    #[doc = "0xd8 - corex iram0 permission configuration register 0"]
    #[inline(always)]
    pub const fn core_x_iram0_pms_constrain_0(&self) -> &CORE_X_IRAM0_PMS_CONSTRAIN_0 {
        &self.core_x_iram0_pms_constrain_0
    }
    #[doc = "0xdc - corex iram0 permission configuration register 0"]
    #[inline(always)]
    pub const fn core_x_iram0_pms_constrain_1(&self) -> &CORE_X_IRAM0_PMS_CONSTRAIN_1 {
        &self.core_x_iram0_pms_constrain_1
    }
    #[doc = "0xe0 - corex iram0 permission configuration register 1"]
    #[inline(always)]
    pub const fn core_x_iram0_pms_constrain_2(&self) -> &CORE_X_IRAM0_PMS_CONSTRAIN_2 {
        &self.core_x_iram0_pms_constrain_2
    }
    #[doc = "0xe4 - core0 iram0 permission monitor configuration register 0"]
    #[inline(always)]
    pub const fn core_0_iram0_pms_monitor_0(&self) -> &CORE_0_IRAM0_PMS_MONITOR_0 {
        &self.core_0_iram0_pms_monitor_0
    }
    #[doc = "0xe8 - core0 iram0 permission monitor configuration register 1"]
    #[inline(always)]
    pub const fn core_0_iram0_pms_monitor_1(&self) -> &CORE_0_IRAM0_PMS_MONITOR_1 {
        &self.core_0_iram0_pms_monitor_1
    }
    #[doc = "0xec - core0 iram0 permission monitor configuration register 2"]
    #[inline(always)]
    pub const fn core_0_iram0_pms_monitor_2(&self) -> &CORE_0_IRAM0_PMS_MONITOR_2 {
        &self.core_0_iram0_pms_monitor_2
    }
    #[doc = "0xf0 - core1 iram0 permission monitor configuration register 0"]
    #[inline(always)]
    pub const fn core_1_iram0_pms_monitor_0(&self) -> &CORE_1_IRAM0_PMS_MONITOR_0 {
        &self.core_1_iram0_pms_monitor_0
    }
    #[doc = "0xf4 - core1 iram0 permission monitor configuration register 1"]
    #[inline(always)]
    pub const fn core_1_iram0_pms_monitor_1(&self) -> &CORE_1_IRAM0_PMS_MONITOR_1 {
        &self.core_1_iram0_pms_monitor_1
    }
    #[doc = "0xf8 - core1 iram0 permission monitor configuration register 2"]
    #[inline(always)]
    pub const fn core_1_iram0_pms_monitor_2(&self) -> &CORE_1_IRAM0_PMS_MONITOR_2 {
        &self.core_1_iram0_pms_monitor_2
    }
    #[doc = "0xfc - corex dram0 permission configuration register 0"]
    #[inline(always)]
    pub const fn core_x_dram0_pms_constrain_0(&self) -> &CORE_X_DRAM0_PMS_CONSTRAIN_0 {
        &self.core_x_dram0_pms_constrain_0
    }
    #[doc = "0x100 - corex dram0 permission configuration register 1"]
    #[inline(always)]
    pub const fn core_x_dram0_pms_constrain_1(&self) -> &CORE_X_DRAM0_PMS_CONSTRAIN_1 {
        &self.core_x_dram0_pms_constrain_1
    }
    #[doc = "0x104 - core0 dram0 permission monitor configuration register 0"]
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_0(&self) -> &CORE_0_DRAM0_PMS_MONITOR_0 {
        &self.core_0_dram0_pms_monitor_0
    }
    #[doc = "0x108 - core0 dram0 permission monitor configuration register 1"]
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_1(&self) -> &CORE_0_DRAM0_PMS_MONITOR_1 {
        &self.core_0_dram0_pms_monitor_1
    }
    #[doc = "0x10c - core0 dram0 permission monitor configuration register 2."]
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_2(&self) -> &CORE_0_DRAM0_PMS_MONITOR_2 {
        &self.core_0_dram0_pms_monitor_2
    }
    #[doc = "0x110 - core0 dram0 permission monitor configuration register 3."]
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_3(&self) -> &CORE_0_DRAM0_PMS_MONITOR_3 {
        &self.core_0_dram0_pms_monitor_3
    }
    #[doc = "0x114 - core1 dram0 permission monitor configuration register 0"]
    #[inline(always)]
    pub const fn core_1_dram0_pms_monitor_0(&self) -> &CORE_1_DRAM0_PMS_MONITOR_0 {
        &self.core_1_dram0_pms_monitor_0
    }
    #[doc = "0x118 - core1 dram0 permission monitor configuration register 1"]
    #[inline(always)]
    pub const fn core_1_dram0_pms_monitor_1(&self) -> &CORE_1_DRAM0_PMS_MONITOR_1 {
        &self.core_1_dram0_pms_monitor_1
    }
    #[doc = "0x11c - core1 dram0 permission monitor configuration register 2."]
    #[inline(always)]
    pub const fn core_1_dram0_pms_monitor_2(&self) -> &CORE_1_DRAM0_PMS_MONITOR_2 {
        &self.core_1_dram0_pms_monitor_2
    }
    #[doc = "0x120 - core1 dram0 permission monitor configuration register 3."]
    #[inline(always)]
    pub const fn core_1_dram0_pms_monitor_3(&self) -> &CORE_1_DRAM0_PMS_MONITOR_3 {
        &self.core_1_dram0_pms_monitor_3
    }
    #[doc = "0x124 - Core0 access peripherals permission configuration register 0."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_0(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_0 {
        &self.core_0_pif_pms_constrain_0
    }
    #[doc = "0x128 - Core0 access peripherals permission configuration register 1."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_1(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_1 {
        &self.core_0_pif_pms_constrain_1
    }
    #[doc = "0x12c - Core0 access peripherals permission configuration register 2."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_2(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_2 {
        &self.core_0_pif_pms_constrain_2
    }
    #[doc = "0x130 - Core0 access peripherals permission configuration register 3."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_3(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_3 {
        &self.core_0_pif_pms_constrain_3
    }
    #[doc = "0x134 - Core0 access peripherals permission configuration register 4."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_4(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_4 {
        &self.core_0_pif_pms_constrain_4
    }
    #[doc = "0x138 - Core0 access peripherals permission configuration register 5."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_5(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_5 {
        &self.core_0_pif_pms_constrain_5
    }
    #[doc = "0x13c - Core0 access peripherals permission configuration register 6."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_6(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_6 {
        &self.core_0_pif_pms_constrain_6
    }
    #[doc = "0x140 - Core0 access peripherals permission configuration register 7."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_7(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_7 {
        &self.core_0_pif_pms_constrain_7
    }
    #[doc = "0x144 - Core0 access peripherals permission configuration register 8."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_8(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_8 {
        &self.core_0_pif_pms_constrain_8
    }
    #[doc = "0x148 - Core0 access peripherals permission configuration register 9."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_9(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_9 {
        &self.core_0_pif_pms_constrain_9
    }
    #[doc = "0x14c - Core0 access peripherals permission configuration register 10."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_10(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_10 {
        &self.core_0_pif_pms_constrain_10
    }
    #[doc = "0x150 - Core0 access peripherals permission configuration register 11."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_11(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_11 {
        &self.core_0_pif_pms_constrain_11
    }
    #[doc = "0x154 - Core0 access peripherals permission configuration register 12."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_12(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_12 {
        &self.core_0_pif_pms_constrain_12
    }
    #[doc = "0x158 - Core0 access peripherals permission configuration register 13."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_13(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_13 {
        &self.core_0_pif_pms_constrain_13
    }
    #[doc = "0x15c - Core0 access peripherals permission configuration register 14."]
    #[inline(always)]
    pub const fn core_0_pif_pms_constrain_14(&self) -> &CORE_0_PIF_PMS_CONSTRAIN_14 {
        &self.core_0_pif_pms_constrain_14
    }
    #[doc = "0x160 - Core0 region permission register 0."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_0(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_0 {
        &self.core_0_region_pms_constrain_0
    }
    #[doc = "0x164 - Core0 region permission register 1."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_1(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_1 {
        &self.core_0_region_pms_constrain_1
    }
    #[doc = "0x168 - Core0 region permission register 2."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_2(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_2 {
        &self.core_0_region_pms_constrain_2
    }
    #[doc = "0x16c - Core0 region permission register 3."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_3(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_3 {
        &self.core_0_region_pms_constrain_3
    }
    #[doc = "0x170 - Core0 region permission register 4."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_4(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_4 {
        &self.core_0_region_pms_constrain_4
    }
    #[doc = "0x174 - Core0 region permission register 5."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_5(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_5 {
        &self.core_0_region_pms_constrain_5
    }
    #[doc = "0x178 - Core0 region permission register 6."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_6(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_6 {
        &self.core_0_region_pms_constrain_6
    }
    #[doc = "0x17c - Core0 region permission register 7."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_7(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_7 {
        &self.core_0_region_pms_constrain_7
    }
    #[doc = "0x180 - Core0 region permission register 8."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_8(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_8 {
        &self.core_0_region_pms_constrain_8
    }
    #[doc = "0x184 - Core0 region permission register 9."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_9(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_9 {
        &self.core_0_region_pms_constrain_9
    }
    #[doc = "0x188 - Core0 region permission register 10."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_10(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_10 {
        &self.core_0_region_pms_constrain_10
    }
    #[doc = "0x18c - Core0 region permission register 11."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_11(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_11 {
        &self.core_0_region_pms_constrain_11
    }
    #[doc = "0x190 - Core0 region permission register 12."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_12(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_12 {
        &self.core_0_region_pms_constrain_12
    }
    #[doc = "0x194 - Core0 region permission register 13."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_13(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_13 {
        &self.core_0_region_pms_constrain_13
    }
    #[doc = "0x198 - Core0 region permission register 14."]
    #[inline(always)]
    pub const fn core_0_region_pms_constrain_14(&self) -> &CORE_0_REGION_PMS_CONSTRAIN_14 {
        &self.core_0_region_pms_constrain_14
    }
    #[doc = "0x19c - Core0 permission report register 0."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_0(&self) -> &CORE_0_PIF_PMS_MONITOR_0 {
        &self.core_0_pif_pms_monitor_0
    }
    #[doc = "0x1a0 - Core0 permission report register 1."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_1(&self) -> &CORE_0_PIF_PMS_MONITOR_1 {
        &self.core_0_pif_pms_monitor_1
    }
    #[doc = "0x1a4 - Core0 permission report register 2."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_2(&self) -> &CORE_0_PIF_PMS_MONITOR_2 {
        &self.core_0_pif_pms_monitor_2
    }
    #[doc = "0x1a8 - Core0 permission report register 3."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_3(&self) -> &CORE_0_PIF_PMS_MONITOR_3 {
        &self.core_0_pif_pms_monitor_3
    }
    #[doc = "0x1ac - Core0 permission report register 4."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_4(&self) -> &CORE_0_PIF_PMS_MONITOR_4 {
        &self.core_0_pif_pms_monitor_4
    }
    #[doc = "0x1b0 - Core0 permission report register 5."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_5(&self) -> &CORE_0_PIF_PMS_MONITOR_5 {
        &self.core_0_pif_pms_monitor_5
    }
    #[doc = "0x1b4 - Core0 permission report register 6."]
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_6(&self) -> &CORE_0_PIF_PMS_MONITOR_6 {
        &self.core_0_pif_pms_monitor_6
    }
    #[doc = "0x1b8 - core0 vecbase override configuration register 0"]
    #[inline(always)]
    pub const fn core_0_vecbase_override_lock(&self) -> &CORE_0_VECBASE_OVERRIDE_LOCK {
        &self.core_0_vecbase_override_lock
    }
    #[doc = "0x1bc - core0 vecbase override configuration register 0"]
    #[inline(always)]
    pub const fn core_0_vecbase_override_0(&self) -> &CORE_0_VECBASE_OVERRIDE_0 {
        &self.core_0_vecbase_override_0
    }
    #[doc = "0x1c0 - core0 vecbase override configuration register 1"]
    #[inline(always)]
    pub const fn core_0_vecbase_override_1(&self) -> &CORE_0_VECBASE_OVERRIDE_1 {
        &self.core_0_vecbase_override_1
    }
    #[doc = "0x1c4 - core0 vecbase override configuration register 1"]
    #[inline(always)]
    pub const fn core_0_vecbase_override_2(&self) -> &CORE_0_VECBASE_OVERRIDE_2 {
        &self.core_0_vecbase_override_2
    }
    #[doc = "0x1c8 - core0 toomanyexception override configuration register 0."]
    #[inline(always)]
    pub const fn core_0_toomanyexceptions_m_override_0(
        &self,
    ) -> &CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0 {
        &self.core_0_toomanyexceptions_m_override_0
    }
    #[doc = "0x1cc - core0 toomanyexception override configuration register 1."]
    #[inline(always)]
    pub const fn core_0_toomanyexceptions_m_override_1(
        &self,
    ) -> &CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1 {
        &self.core_0_toomanyexceptions_m_override_1
    }
    #[doc = "0x1d0 - Core1 access peripherals permission configuration register 0."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_0(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_0 {
        &self.core_1_pif_pms_constrain_0
    }
    #[doc = "0x1d4 - Core1 access peripherals permission configuration register 1."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_1(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_1 {
        &self.core_1_pif_pms_constrain_1
    }
    #[doc = "0x1d8 - Core1 access peripherals permission configuration register 2."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_2(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_2 {
        &self.core_1_pif_pms_constrain_2
    }
    #[doc = "0x1dc - Core1 access peripherals permission configuration register 3."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_3(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_3 {
        &self.core_1_pif_pms_constrain_3
    }
    #[doc = "0x1e0 - Core1 access peripherals permission configuration register 4."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_4(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_4 {
        &self.core_1_pif_pms_constrain_4
    }
    #[doc = "0x1e4 - Core1 access peripherals permission configuration register 5."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_5(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_5 {
        &self.core_1_pif_pms_constrain_5
    }
    #[doc = "0x1e8 - Core1 access peripherals permission configuration register 6."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_6(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_6 {
        &self.core_1_pif_pms_constrain_6
    }
    #[doc = "0x1ec - Core1 access peripherals permission configuration register 7."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_7(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_7 {
        &self.core_1_pif_pms_constrain_7
    }
    #[doc = "0x1f0 - Core1 access peripherals permission configuration register 8."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_8(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_8 {
        &self.core_1_pif_pms_constrain_8
    }
    #[doc = "0x1f4 - Core1 access peripherals permission configuration register 9."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_9(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_9 {
        &self.core_1_pif_pms_constrain_9
    }
    #[doc = "0x1f8 - core1 access peripherals permission configuration register 10."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_10(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_10 {
        &self.core_1_pif_pms_constrain_10
    }
    #[doc = "0x1fc - core1 access peripherals permission configuration register 11."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_11(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_11 {
        &self.core_1_pif_pms_constrain_11
    }
    #[doc = "0x200 - core1 access peripherals permission configuration register 12."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_12(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_12 {
        &self.core_1_pif_pms_constrain_12
    }
    #[doc = "0x204 - core1 access peripherals permission configuration register 13."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_13(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_13 {
        &self.core_1_pif_pms_constrain_13
    }
    #[doc = "0x208 - core1 access peripherals permission configuration register 14."]
    #[inline(always)]
    pub const fn core_1_pif_pms_constrain_14(&self) -> &CORE_1_PIF_PMS_CONSTRAIN_14 {
        &self.core_1_pif_pms_constrain_14
    }
    #[doc = "0x20c - core1 region permission register 0."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_0(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_0 {
        &self.core_1_region_pms_constrain_0
    }
    #[doc = "0x210 - core1 region permission register 1."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_1(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_1 {
        &self.core_1_region_pms_constrain_1
    }
    #[doc = "0x214 - core1 region permission register 2."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_2(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_2 {
        &self.core_1_region_pms_constrain_2
    }
    #[doc = "0x218 - core1 region permission register 3."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_3(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_3 {
        &self.core_1_region_pms_constrain_3
    }
    #[doc = "0x21c - core1 region permission register 4."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_4(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_4 {
        &self.core_1_region_pms_constrain_4
    }
    #[doc = "0x220 - core1 region permission register 5."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_5(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_5 {
        &self.core_1_region_pms_constrain_5
    }
    #[doc = "0x224 - core1 region permission register 6."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_6(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_6 {
        &self.core_1_region_pms_constrain_6
    }
    #[doc = "0x228 - core1 region permission register 7."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_7(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_7 {
        &self.core_1_region_pms_constrain_7
    }
    #[doc = "0x22c - core1 region permission register 8."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_8(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_8 {
        &self.core_1_region_pms_constrain_8
    }
    #[doc = "0x230 - core1 region permission register 9."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_9(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_9 {
        &self.core_1_region_pms_constrain_9
    }
    #[doc = "0x234 - core1 region permission register 10."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_10(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_10 {
        &self.core_1_region_pms_constrain_10
    }
    #[doc = "0x238 - core1 region permission register 11."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_11(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_11 {
        &self.core_1_region_pms_constrain_11
    }
    #[doc = "0x23c - core1 region permission register 12."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_12(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_12 {
        &self.core_1_region_pms_constrain_12
    }
    #[doc = "0x240 - core1 region permission register 13."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_13(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_13 {
        &self.core_1_region_pms_constrain_13
    }
    #[doc = "0x244 - core1 region permission register 14."]
    #[inline(always)]
    pub const fn core_1_region_pms_constrain_14(&self) -> &CORE_1_REGION_PMS_CONSTRAIN_14 {
        &self.core_1_region_pms_constrain_14
    }
    #[doc = "0x248 - core1 permission report register 0."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_0(&self) -> &CORE_1_PIF_PMS_MONITOR_0 {
        &self.core_1_pif_pms_monitor_0
    }
    #[doc = "0x24c - core1 permission report register 1."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_1(&self) -> &CORE_1_PIF_PMS_MONITOR_1 {
        &self.core_1_pif_pms_monitor_1
    }
    #[doc = "0x250 - core1 permission report register 2."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_2(&self) -> &CORE_1_PIF_PMS_MONITOR_2 {
        &self.core_1_pif_pms_monitor_2
    }
    #[doc = "0x254 - core1 permission report register 3."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_3(&self) -> &CORE_1_PIF_PMS_MONITOR_3 {
        &self.core_1_pif_pms_monitor_3
    }
    #[doc = "0x258 - core1 permission report register 4."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_4(&self) -> &CORE_1_PIF_PMS_MONITOR_4 {
        &self.core_1_pif_pms_monitor_4
    }
    #[doc = "0x25c - core1 permission report register 5."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_5(&self) -> &CORE_1_PIF_PMS_MONITOR_5 {
        &self.core_1_pif_pms_monitor_5
    }
    #[doc = "0x260 - core1 permission report register 6."]
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_6(&self) -> &CORE_1_PIF_PMS_MONITOR_6 {
        &self.core_1_pif_pms_monitor_6
    }
    #[doc = "0x264 - core1 vecbase override configuration register 0"]
    #[inline(always)]
    pub const fn core_1_vecbase_override_lock(&self) -> &CORE_1_VECBASE_OVERRIDE_LOCK {
        &self.core_1_vecbase_override_lock
    }
    #[doc = "0x268 - core1 vecbase override configuration register 0"]
    #[inline(always)]
    pub const fn core_1_vecbase_override_0(&self) -> &CORE_1_VECBASE_OVERRIDE_0 {
        &self.core_1_vecbase_override_0
    }
    #[doc = "0x26c - core1 vecbase override configuration register 1"]
    #[inline(always)]
    pub const fn core_1_vecbase_override_1(&self) -> &CORE_1_VECBASE_OVERRIDE_1 {
        &self.core_1_vecbase_override_1
    }
    #[doc = "0x270 - core1 vecbase override configuration register 1"]
    #[inline(always)]
    pub const fn core_1_vecbase_override_2(&self) -> &CORE_1_VECBASE_OVERRIDE_2 {
        &self.core_1_vecbase_override_2
    }
    #[doc = "0x274 - core1 toomanyexception override configuration register 0."]
    #[inline(always)]
    pub const fn core_1_toomanyexceptions_m_override_0(
        &self,
    ) -> &CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0 {
        &self.core_1_toomanyexceptions_m_override_0
    }
    #[doc = "0x278 - core1 toomanyexception override configuration register 1."]
    #[inline(always)]
    pub const fn core_1_toomanyexceptions_m_override_1(
        &self,
    ) -> &CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 {
        &self.core_1_toomanyexceptions_m_override_1
    }
    #[doc = "0x27c - BackUp access peripherals permission configuration register 0."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_0(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_0 {
        &self.backup_bus_pms_constrain_0
    }
    #[doc = "0x280 - BackUp access peripherals permission configuration register 1."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_1(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_1 {
        &self.backup_bus_pms_constrain_1
    }
    #[doc = "0x284 - BackUp access peripherals permission configuration register 2."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_2(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_2 {
        &self.backup_bus_pms_constrain_2
    }
    #[doc = "0x288 - BackUp access peripherals permission configuration register 3."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_3(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_3 {
        &self.backup_bus_pms_constrain_3
    }
    #[doc = "0x28c - BackUp access peripherals permission configuration register 4."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_4(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_4 {
        &self.backup_bus_pms_constrain_4
    }
    #[doc = "0x290 - BackUp access peripherals permission configuration register 5."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_5(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_5 {
        &self.backup_bus_pms_constrain_5
    }
    #[doc = "0x294 - BackUp access peripherals permission configuration register 6."]
    #[inline(always)]
    pub const fn backup_bus_pms_constrain_6(&self) -> &BACKUP_BUS_PMS_CONSTRAIN_6 {
        &self.backup_bus_pms_constrain_6
    }
    #[doc = "0x298 - BackUp permission report register 0."]
    #[inline(always)]
    pub const fn backup_bus_pms_monitor_0(&self) -> &BACKUP_BUS_PMS_MONITOR_0 {
        &self.backup_bus_pms_monitor_0
    }
    #[doc = "0x29c - BackUp permission report register 1."]
    #[inline(always)]
    pub const fn backup_bus_pms_monitor_1(&self) -> &BACKUP_BUS_PMS_MONITOR_1 {
        &self.backup_bus_pms_monitor_1
    }
    #[doc = "0x2a0 - BackUp permission report register 2."]
    #[inline(always)]
    pub const fn backup_bus_pms_monitor_2(&self) -> &BACKUP_BUS_PMS_MONITOR_2 {
        &self.backup_bus_pms_monitor_2
    }
    #[doc = "0x2a4 - BackUp permission report register 3."]
    #[inline(always)]
    pub const fn backup_bus_pms_monitor_3(&self) -> &BACKUP_BUS_PMS_MONITOR_3 {
        &self.backup_bus_pms_monitor_3
    }
    #[doc = "0x2a8 - EDMA boundary lock register."]
    #[inline(always)]
    pub const fn edma_boundary_lock(&self) -> &EDMA_BOUNDARY_LOCK {
        &self.edma_boundary_lock
    }
    #[doc = "0x2ac - EDMA boundary 0 configuration"]
    #[inline(always)]
    pub const fn edma_boundary_0(&self) -> &EDMA_BOUNDARY_0 {
        &self.edma_boundary_0
    }
    #[doc = "0x2b0 - EDMA boundary 1 configuration"]
    #[inline(always)]
    pub const fn edma_boundary_1(&self) -> &EDMA_BOUNDARY_1 {
        &self.edma_boundary_1
    }
    #[doc = "0x2b4 - EDMA boundary 2 configuration"]
    #[inline(always)]
    pub const fn edma_boundary_2(&self) -> &EDMA_BOUNDARY_2 {
        &self.edma_boundary_2
    }
    #[doc = "0x2b8 - EDMA-SPI2 permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_spi2_lock(&self) -> &EDMA_PMS_SPI2_LOCK {
        &self.edma_pms_spi2_lock
    }
    #[doc = "0x2bc - EDMA-SPI2 permission control register."]
    #[inline(always)]
    pub const fn edma_pms_spi2(&self) -> &EDMA_PMS_SPI2 {
        &self.edma_pms_spi2
    }
    #[doc = "0x2c0 - EDMA-SPI3 permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_spi3_lock(&self) -> &EDMA_PMS_SPI3_LOCK {
        &self.edma_pms_spi3_lock
    }
    #[doc = "0x2c4 - EDMA-SPI3 permission control register."]
    #[inline(always)]
    pub const fn edma_pms_spi3(&self) -> &EDMA_PMS_SPI3 {
        &self.edma_pms_spi3
    }
    #[doc = "0x2c8 - EDMA-UHCI0 permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_uhci0_lock(&self) -> &EDMA_PMS_UHCI0_LOCK {
        &self.edma_pms_uhci0_lock
    }
    #[doc = "0x2cc - EDMA-UHCI0 permission control register."]
    #[inline(always)]
    pub const fn edma_pms_uhci0(&self) -> &EDMA_PMS_UHCI0 {
        &self.edma_pms_uhci0
    }
    #[doc = "0x2d0 - EDMA-I2S0 permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_i2s0_lock(&self) -> &EDMA_PMS_I2S0_LOCK {
        &self.edma_pms_i2s0_lock
    }
    #[doc = "0x2d4 - EDMA-I2S0 permission control register."]
    #[inline(always)]
    pub const fn edma_pms_i2s0(&self) -> &EDMA_PMS_I2S0 {
        &self.edma_pms_i2s0
    }
    #[doc = "0x2d8 - EDMA-I2S1 permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_i2s1_lock(&self) -> &EDMA_PMS_I2S1_LOCK {
        &self.edma_pms_i2s1_lock
    }
    #[doc = "0x2dc - EDMA-I2S1 permission control register."]
    #[inline(always)]
    pub const fn edma_pms_i2s1(&self) -> &EDMA_PMS_I2S1 {
        &self.edma_pms_i2s1
    }
    #[doc = "0x2e0 - EDMA-LCD/CAM permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_lcd_cam_lock(&self) -> &EDMA_PMS_LCD_CAM_LOCK {
        &self.edma_pms_lcd_cam_lock
    }
    #[doc = "0x2e4 - EDMA-LCD/CAM permission control register."]
    #[inline(always)]
    pub const fn edma_pms_lcd_cam(&self) -> &EDMA_PMS_LCD_CAM {
        &self.edma_pms_lcd_cam
    }
    #[doc = "0x2e8 - EDMA-AES permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_aes_lock(&self) -> &EDMA_PMS_AES_LOCK {
        &self.edma_pms_aes_lock
    }
    #[doc = "0x2ec - EDMA-AES permission control register."]
    #[inline(always)]
    pub const fn edma_pms_aes(&self) -> &EDMA_PMS_AES {
        &self.edma_pms_aes
    }
    #[doc = "0x2f0 - EDMA-SHA permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_sha_lock(&self) -> &EDMA_PMS_SHA_LOCK {
        &self.edma_pms_sha_lock
    }
    #[doc = "0x2f4 - EDMA-SHA permission control register."]
    #[inline(always)]
    pub const fn edma_pms_sha(&self) -> &EDMA_PMS_SHA {
        &self.edma_pms_sha
    }
    #[doc = "0x2f8 - EDMA-ADC/DAC permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_adc_dac_lock(&self) -> &EDMA_PMS_ADC_DAC_LOCK {
        &self.edma_pms_adc_dac_lock
    }
    #[doc = "0x2fc - EDMA-ADC/DAC permission control register."]
    #[inline(always)]
    pub const fn edma_pms_adc_dac(&self) -> &EDMA_PMS_ADC_DAC {
        &self.edma_pms_adc_dac
    }
    #[doc = "0x300 - EDMA-RMT permission lock register."]
    #[inline(always)]
    pub const fn edma_pms_rmt_lock(&self) -> &EDMA_PMS_RMT_LOCK {
        &self.edma_pms_rmt_lock
    }
    #[doc = "0x304 - EDMA-RMT permission control register."]
    #[inline(always)]
    pub const fn edma_pms_rmt(&self) -> &EDMA_PMS_RMT {
        &self.edma_pms_rmt
    }
    #[doc = "0x308 - Sensitive module clock gate configuration register."]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x30c - RTC coprocessor permission register."]
    #[inline(always)]
    pub const fn rtc_pms(&self) -> &RTC_PMS {
        &self.rtc_pms
    }
    #[doc = "0xffc - Sensitive version register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CACHE_DATAARRAY_CONNECT_0 (rw) register accessor: Cache data array configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_dataarray_connect_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_dataarray_connect_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dataarray_connect_0`] module"]
pub type CACHE_DATAARRAY_CONNECT_0 =
    crate::Reg<cache_dataarray_connect_0::CACHE_DATAARRAY_CONNECT_0_SPEC>;
#[doc = "Cache data array configuration register 0."]
pub mod cache_dataarray_connect_0;
#[doc = "CACHE_DATAARRAY_CONNECT_1 (rw) register accessor: Cache data array configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_dataarray_connect_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_dataarray_connect_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dataarray_connect_1`] module"]
pub type CACHE_DATAARRAY_CONNECT_1 =
    crate::Reg<cache_dataarray_connect_1::CACHE_DATAARRAY_CONNECT_1_SPEC>;
#[doc = "Cache data array configuration register 1."]
pub mod cache_dataarray_connect_1;
#[doc = "APB_PERIPHERAL_ACCESS_0 (rw) register accessor: APB peripheral configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_access_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_access_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_access_0`] module"]
pub type APB_PERIPHERAL_ACCESS_0 =
    crate::Reg<apb_peripheral_access_0::APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "APB peripheral configuration register 0."]
pub mod apb_peripheral_access_0;
#[doc = "APB_PERIPHERAL_ACCESS_1 (rw) register accessor: APB peripheral configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_access_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_access_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_access_1`] module"]
pub type APB_PERIPHERAL_ACCESS_1 =
    crate::Reg<apb_peripheral_access_1::APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "APB peripheral configuration register 1."]
pub mod apb_peripheral_access_1;
#[doc = "INTERNAL_SRAM_USAGE_0 (rw) register accessor: Internal SRAM configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_0`] module"]
pub type INTERNAL_SRAM_USAGE_0 = crate::Reg<internal_sram_usage_0::INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "Internal SRAM configuration register 0."]
pub mod internal_sram_usage_0;
#[doc = "INTERNAL_SRAM_USAGE_1 (rw) register accessor: Internal SRAM configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_1`] module"]
pub type INTERNAL_SRAM_USAGE_1 = crate::Reg<internal_sram_usage_1::INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "Internal SRAM configuration register 1."]
pub mod internal_sram_usage_1;
#[doc = "INTERNAL_SRAM_USAGE_2 (rw) register accessor: Internal SRAM configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_2`] module"]
pub type INTERNAL_SRAM_USAGE_2 = crate::Reg<internal_sram_usage_2::INTERNAL_SRAM_USAGE_2_SPEC>;
#[doc = "Internal SRAM configuration register 2."]
pub mod internal_sram_usage_2;
#[doc = "INTERNAL_SRAM_USAGE_3 (rw) register accessor: Internal SRAM configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_3`] module"]
pub type INTERNAL_SRAM_USAGE_3 = crate::Reg<internal_sram_usage_3::INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Internal SRAM configuration register 3."]
pub mod internal_sram_usage_3;
#[doc = "INTERNAL_SRAM_USAGE_4 (rw) register accessor: Internal SRAM configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_4`] module"]
pub type INTERNAL_SRAM_USAGE_4 = crate::Reg<internal_sram_usage_4::INTERNAL_SRAM_USAGE_4_SPEC>;
#[doc = "Internal SRAM configuration register 4."]
pub mod internal_sram_usage_4;
#[doc = "RETENTION_DISABLE (rw) register accessor: Retention configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retention_disable`] module"]
pub type RETENTION_DISABLE = crate::Reg<retention_disable::RETENTION_DISABLE_SPEC>;
#[doc = "Retention configuration register."]
pub mod retention_disable;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: Cache tag configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_0`] module"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "Cache tag configuration register 0."]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: Cache tag configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_1`] module"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "Cache tag configuration register 1."]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: Cache MMU configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_access_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_access_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_0`] module"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "Cache MMU configuration register 0."]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: Cache MMU configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_access_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_access_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_1`] module"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "Cache MMU configuration register 1."]
pub mod cache_mmu_access_1;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 (rw) register accessor: spi2 dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi2_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi2_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_spi2_pms_constrain_0`] module"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_0::DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>;
#[doc = "spi2 dma permission configuration register 0."]
pub mod dma_apbperi_spi2_pms_constrain_0;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 (rw) register accessor: spi2 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi2_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi2_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_spi2_pms_constrain_1`] module"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_1::DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>;
#[doc = "spi2 dma permission configuration register 1."]
pub mod dma_apbperi_spi2_pms_constrain_1;
#[doc = "DMA_APBPERI_SPI3_PMS_CONSTRAIN_0 (rw) register accessor: spi3 dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi3_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi3_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_spi3_pms_constrain_0`] module"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_spi3_pms_constrain_0::DMA_APBPERI_SPI3_PMS_CONSTRAIN_0_SPEC>;
#[doc = "spi3 dma permission configuration register 0."]
pub mod dma_apbperi_spi3_pms_constrain_0;
#[doc = "DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 (rw) register accessor: spi3 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi3_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi3_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_spi3_pms_constrain_1`] module"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_spi3_pms_constrain_1::DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC>;
#[doc = "spi3 dma permission configuration register 1."]
pub mod dma_apbperi_spi3_pms_constrain_1;
#[doc = "DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 (rw) register accessor: uhci0 dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_uhci0_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_uhci0_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_uhci0_pms_constrain_0`] module"]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_uhci0_pms_constrain_0::DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "uhci0 dma permission configuration register 0."]
pub mod dma_apbperi_uhci0_pms_constrain_0;
#[doc = "DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1 (rw) register accessor: uhci0 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_uhci0_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_uhci0_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_uhci0_pms_constrain_1`] module"]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_uhci0_pms_constrain_1::DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "uhci0 dma permission configuration register 1."]
pub mod dma_apbperi_uhci0_pms_constrain_1;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 (rw) register accessor: i2s0 dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_i2s0_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_i2s0_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_i2s0_pms_constrain_0`] module"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_0::DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "i2s0 dma permission configuration register 0."]
pub mod dma_apbperi_i2s0_pms_constrain_0;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 (rw) register accessor: i2s0 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_i2s0_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_i2s0_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_i2s0_pms_constrain_1`] module"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_1::DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "i2s0 dma permission configuration register 1."]
pub mod dma_apbperi_i2s0_pms_constrain_1;
#[doc = "DMA_APBPERI_I2S1_PMS_CONSTRAIN_0 (rw) register accessor: i2s1 dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_i2s1_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_i2s1_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_i2s1_pms_constrain_0`] module"]
pub type DMA_APBPERI_I2S1_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_i2s1_pms_constrain_0::DMA_APBPERI_I2S1_PMS_CONSTRAIN_0_SPEC>;
#[doc = "i2s1 dma permission configuration register 0."]
pub mod dma_apbperi_i2s1_pms_constrain_0;
#[doc = "DMA_APBPERI_I2S1_PMS_CONSTRAIN_1 (rw) register accessor: i2s1 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_i2s1_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_i2s1_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_i2s1_pms_constrain_1`] module"]
pub type DMA_APBPERI_I2S1_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_i2s1_pms_constrain_1::DMA_APBPERI_I2S1_PMS_CONSTRAIN_1_SPEC>;
#[doc = "i2s1 dma permission configuration register 1."]
pub mod dma_apbperi_i2s1_pms_constrain_1;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_0 (rw) register accessor: mac dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_mac_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_mac_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_mac_pms_constrain_0`] module"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_mac_pms_constrain_0::DMA_APBPERI_MAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "mac dma permission configuration register 0."]
pub mod dma_apbperi_mac_pms_constrain_0;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_1 (rw) register accessor: mac dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_mac_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_mac_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_mac_pms_constrain_1`] module"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_mac_pms_constrain_1::DMA_APBPERI_MAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "mac dma permission configuration register 1."]
pub mod dma_apbperi_mac_pms_constrain_1;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 (rw) register accessor: backup dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_backup_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_backup_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_backup_pms_constrain_0`] module"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_backup_pms_constrain_0::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_SPEC>;
#[doc = "backup dma permission configuration register 0."]
pub mod dma_apbperi_backup_pms_constrain_0;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 (rw) register accessor: backup dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_backup_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_backup_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_backup_pms_constrain_1`] module"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_backup_pms_constrain_1::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_SPEC>;
#[doc = "backup dma permission configuration register 1."]
pub mod dma_apbperi_backup_pms_constrain_1;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_0 (rw) register accessor: aes dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_aes_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_aes_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_aes_pms_constrain_0`] module"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_aes_pms_constrain_0::DMA_APBPERI_AES_PMS_CONSTRAIN_0_SPEC>;
#[doc = "aes dma permission configuration register 0."]
pub mod dma_apbperi_aes_pms_constrain_0;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_1 (rw) register accessor: aes dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_aes_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_aes_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_aes_pms_constrain_1`] module"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_aes_pms_constrain_1::DMA_APBPERI_AES_PMS_CONSTRAIN_1_SPEC>;
#[doc = "aes dma permission configuration register 1."]
pub mod dma_apbperi_aes_pms_constrain_1;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_0 (rw) register accessor: sha dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_sha_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_sha_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_sha_pms_constrain_0`] module"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_sha_pms_constrain_0::DMA_APBPERI_SHA_PMS_CONSTRAIN_0_SPEC>;
#[doc = "sha dma permission configuration register 0."]
pub mod dma_apbperi_sha_pms_constrain_0;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_1 (rw) register accessor: sha dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_sha_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_sha_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_sha_pms_constrain_1`] module"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_sha_pms_constrain_1::DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>;
#[doc = "sha dma permission configuration register 1."]
pub mod dma_apbperi_sha_pms_constrain_1;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 (rw) register accessor: adc_dac dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_adc_dac_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_adc_dac_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_adc_dac_pms_constrain_0`] module"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_0::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "adc_dac dma permission configuration register 0."]
pub mod dma_apbperi_adc_dac_pms_constrain_0;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 (rw) register accessor: adc_dac dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_adc_dac_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_adc_dac_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_adc_dac_pms_constrain_1`] module"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_1::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "adc_dac dma permission configuration register 1."]
pub mod dma_apbperi_adc_dac_pms_constrain_1;
#[doc = "DMA_APBPERI_RMT_PMS_CONSTRAIN_0 (rw) register accessor: rmt dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_rmt_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_rmt_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_rmt_pms_constrain_0`] module"]
pub type DMA_APBPERI_RMT_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_rmt_pms_constrain_0::DMA_APBPERI_RMT_PMS_CONSTRAIN_0_SPEC>;
#[doc = "rmt dma permission configuration register 0."]
pub mod dma_apbperi_rmt_pms_constrain_0;
#[doc = "DMA_APBPERI_RMT_PMS_CONSTRAIN_1 (rw) register accessor: rmt dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_rmt_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_rmt_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_rmt_pms_constrain_1`] module"]
pub type DMA_APBPERI_RMT_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_rmt_pms_constrain_1::DMA_APBPERI_RMT_PMS_CONSTRAIN_1_SPEC>;
#[doc = "rmt dma permission configuration register 1."]
pub mod dma_apbperi_rmt_pms_constrain_1;
#[doc = "DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0 (rw) register accessor: lcd_cam dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_lcd_cam_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_lcd_cam_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_lcd_cam_pms_constrain_0`] module"]
pub type DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_lcd_cam_pms_constrain_0::DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0_SPEC>;
#[doc = "lcd_cam dma permission configuration register 0."]
pub mod dma_apbperi_lcd_cam_pms_constrain_0;
#[doc = "DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1 (rw) register accessor: lcd_cam dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_lcd_cam_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_lcd_cam_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_lcd_cam_pms_constrain_1`] module"]
pub type DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_lcd_cam_pms_constrain_1::DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1_SPEC>;
#[doc = "lcd_cam dma permission configuration register 1."]
pub mod dma_apbperi_lcd_cam_pms_constrain_1;
#[doc = "DMA_APBPERI_USB_PMS_CONSTRAIN_0 (rw) register accessor: usb dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_usb_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_usb_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_usb_pms_constrain_0`] module"]
pub type DMA_APBPERI_USB_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_usb_pms_constrain_0::DMA_APBPERI_USB_PMS_CONSTRAIN_0_SPEC>;
#[doc = "usb dma permission configuration register 0."]
pub mod dma_apbperi_usb_pms_constrain_0;
#[doc = "DMA_APBPERI_USB_PMS_CONSTRAIN_1 (rw) register accessor: usb dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_usb_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_usb_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_usb_pms_constrain_1`] module"]
pub type DMA_APBPERI_USB_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_usb_pms_constrain_1::DMA_APBPERI_USB_PMS_CONSTRAIN_1_SPEC>;
#[doc = "usb dma permission configuration register 1."]
pub mod dma_apbperi_usb_pms_constrain_1;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_0 (rw) register accessor: lc dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_lc_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_lc_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_lc_pms_constrain_0`] module"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_lc_pms_constrain_0::DMA_APBPERI_LC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "lc dma permission configuration register 0."]
pub mod dma_apbperi_lc_pms_constrain_0;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_1 (rw) register accessor: lc dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_lc_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_lc_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_lc_pms_constrain_1`] module"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_lc_pms_constrain_1::DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "lc dma permission configuration register 1."]
pub mod dma_apbperi_lc_pms_constrain_1;
#[doc = "DMA_APBPERI_SDIO_PMS_CONSTRAIN_0 (rw) register accessor: sdio dma permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_sdio_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_sdio_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_sdio_pms_constrain_0`] module"]
pub type DMA_APBPERI_SDIO_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_sdio_pms_constrain_0::DMA_APBPERI_SDIO_PMS_CONSTRAIN_0_SPEC>;
#[doc = "sdio dma permission configuration register 0."]
pub mod dma_apbperi_sdio_pms_constrain_0;
#[doc = "DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 (rw) register accessor: sdio dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_sdio_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_sdio_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_sdio_pms_constrain_1`] module"]
pub type DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_sdio_pms_constrain_1::DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>;
#[doc = "sdio dma permission configuration register 1."]
pub mod dma_apbperi_sdio_pms_constrain_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_0 (rw) register accessor: dma permission monitor configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_pms_monitor_0`] module"]
pub type DMA_APBPERI_PMS_MONITOR_0 =
    crate::Reg<dma_apbperi_pms_monitor_0::DMA_APBPERI_PMS_MONITOR_0_SPEC>;
#[doc = "dma permission monitor configuration register 0."]
pub mod dma_apbperi_pms_monitor_0;
#[doc = "DMA_APBPERI_PMS_MONITOR_1 (rw) register accessor: dma permission monitor configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_pms_monitor_1`] module"]
pub type DMA_APBPERI_PMS_MONITOR_1 =
    crate::Reg<dma_apbperi_pms_monitor_1::DMA_APBPERI_PMS_MONITOR_1_SPEC>;
#[doc = "dma permission monitor configuration register 1."]
pub mod dma_apbperi_pms_monitor_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_2 (r) register accessor: dma permission monitor configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_pms_monitor_2`] module"]
pub type DMA_APBPERI_PMS_MONITOR_2 =
    crate::Reg<dma_apbperi_pms_monitor_2::DMA_APBPERI_PMS_MONITOR_2_SPEC>;
#[doc = "dma permission monitor configuration register 2."]
pub mod dma_apbperi_pms_monitor_2;
#[doc = "DMA_APBPERI_PMS_MONITOR_3 (r) register accessor: dma permission monitor configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apbperi_pms_monitor_3`] module"]
pub type DMA_APBPERI_PMS_MONITOR_3 =
    crate::Reg<dma_apbperi_pms_monitor_3::DMA_APBPERI_PMS_MONITOR_3_SPEC>;
#[doc = "dma permission monitor configuration register 3."]
pub mod dma_apbperi_pms_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 (rw) register accessor: sram split line configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_0`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_0 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_SPEC > ;
#[doc = "sram split line configuration register 0"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_0;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 (rw) register accessor: sram split line configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_1`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_1 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_1;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 (rw) register accessor: sram split line configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_2`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_2 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_2;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 (rw) register accessor: sram split line configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_3`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_3 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 (rw) register accessor: sram split line configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_4`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_4 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_4;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 (rw) register accessor: sram split line configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_dma_split_line_constrain_5`] module"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_5 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_5;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_0 (rw) register accessor: corex iram0 permission configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_pms_constrain_0`] module"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_iram0_pms_constrain_0::CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "corex iram0 permission configuration register 0"]
pub mod core_x_iram0_pms_constrain_0;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_1 (rw) register accessor: corex iram0 permission configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_pms_constrain_1`] module"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_iram0_pms_constrain_1::CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "corex iram0 permission configuration register 0"]
pub mod core_x_iram0_pms_constrain_1;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_2 (rw) register accessor: corex iram0 permission configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_pms_constrain_2`] module"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_2 =
    crate::Reg<core_x_iram0_pms_constrain_2::CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>;
#[doc = "corex iram0 permission configuration register 1"]
pub mod core_x_iram0_pms_constrain_2;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_0 (rw) register accessor: core0 iram0 permission monitor configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_pms_monitor_0`] module"]
pub type CORE_0_IRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_iram0_pms_monitor_0::CORE_0_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 0"]
pub mod core_0_iram0_pms_monitor_0;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_1 (rw) register accessor: core0 iram0 permission monitor configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_pms_monitor_1`] module"]
pub type CORE_0_IRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_iram0_pms_monitor_1::CORE_0_IRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 1"]
pub mod core_0_iram0_pms_monitor_1;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_2 (r) register accessor: core0 iram0 permission monitor configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_pms_monitor_2`] module"]
pub type CORE_0_IRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_iram0_pms_monitor_2::CORE_0_IRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 2"]
pub mod core_0_iram0_pms_monitor_2;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_0 (rw) register accessor: core1 iram0 permission monitor configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_iram0_pms_monitor_0`] module"]
pub type CORE_1_IRAM0_PMS_MONITOR_0 =
    crate::Reg<core_1_iram0_pms_monitor_0::CORE_1_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 0"]
pub mod core_1_iram0_pms_monitor_0;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_1 (rw) register accessor: core1 iram0 permission monitor configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_iram0_pms_monitor_1`] module"]
pub type CORE_1_IRAM0_PMS_MONITOR_1 =
    crate::Reg<core_1_iram0_pms_monitor_1::CORE_1_IRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 1"]
pub mod core_1_iram0_pms_monitor_1;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_2 (r) register accessor: core1 iram0 permission monitor configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_iram0_pms_monitor_2`] module"]
pub type CORE_1_IRAM0_PMS_MONITOR_2 =
    crate::Reg<core_1_iram0_pms_monitor_2::CORE_1_IRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 2"]
pub mod core_1_iram0_pms_monitor_2;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_0 (rw) register accessor: corex dram0 permission configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_dram0_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_dram0_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_dram0_pms_constrain_0`] module"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_dram0_pms_constrain_0::CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "corex dram0 permission configuration register 0"]
pub mod core_x_dram0_pms_constrain_0;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_1 (rw) register accessor: corex dram0 permission configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_dram0_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_dram0_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_dram0_pms_constrain_1`] module"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_dram0_pms_constrain_1::CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "corex dram0 permission configuration register 1"]
pub mod core_x_dram0_pms_constrain_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_0 (rw) register accessor: core0 dram0 permission monitor configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_dram0_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_pms_monitor_0`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_dram0_pms_monitor_0::CORE_0_DRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 0"]
pub mod core_0_dram0_pms_monitor_0;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_1 (rw) register accessor: core0 dram0 permission monitor configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_dram0_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_pms_monitor_1`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_dram0_pms_monitor_1::CORE_0_DRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 1"]
pub mod core_0_dram0_pms_monitor_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_2 (r) register accessor: core0 dram0 permission monitor configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_pms_monitor_2`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_dram0_pms_monitor_2::CORE_0_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 2."]
pub mod core_0_dram0_pms_monitor_2;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_3 (r) register accessor: core0 dram0 permission monitor configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_pms_monitor_3`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_3 =
    crate::Reg<core_0_dram0_pms_monitor_3::CORE_0_DRAM0_PMS_MONITOR_3_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 3."]
pub mod core_0_dram0_pms_monitor_3;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_0 (rw) register accessor: core1 dram0 permission monitor configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_dram0_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_pms_monitor_0`] module"]
pub type CORE_1_DRAM0_PMS_MONITOR_0 =
    crate::Reg<core_1_dram0_pms_monitor_0::CORE_1_DRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 0"]
pub mod core_1_dram0_pms_monitor_0;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_1 (rw) register accessor: core1 dram0 permission monitor configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_dram0_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_pms_monitor_1`] module"]
pub type CORE_1_DRAM0_PMS_MONITOR_1 =
    crate::Reg<core_1_dram0_pms_monitor_1::CORE_1_DRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 1"]
pub mod core_1_dram0_pms_monitor_1;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_2 (r) register accessor: core1 dram0 permission monitor configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_pms_monitor_2`] module"]
pub type CORE_1_DRAM0_PMS_MONITOR_2 =
    crate::Reg<core_1_dram0_pms_monitor_2::CORE_1_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 2."]
pub mod core_1_dram0_pms_monitor_2;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_3 (r) register accessor: core1 dram0 permission monitor configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_pms_monitor_3`] module"]
pub type CORE_1_DRAM0_PMS_MONITOR_3 =
    crate::Reg<core_1_dram0_pms_monitor_3::CORE_1_DRAM0_PMS_MONITOR_3_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 3."]
pub mod core_1_dram0_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_0 (rw) register accessor: Core0 access peripherals permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_0`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<core_0_pif_pms_constrain_0::CORE_0_PIF_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 0."]
pub mod core_0_pif_pms_constrain_0;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_1 (rw) register accessor: Core0 access peripherals permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_1`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<core_0_pif_pms_constrain_1::CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 1."]
pub mod core_0_pif_pms_constrain_1;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_2 (rw) register accessor: Core0 access peripherals permission configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_2`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<core_0_pif_pms_constrain_2::CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 2."]
pub mod core_0_pif_pms_constrain_2;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_3 (rw) register accessor: Core0 access peripherals permission configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_3`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<core_0_pif_pms_constrain_3::CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 3."]
pub mod core_0_pif_pms_constrain_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_4 (rw) register accessor: Core0 access peripherals permission configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_4`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<core_0_pif_pms_constrain_4::CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 4."]
pub mod core_0_pif_pms_constrain_4;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_5 (rw) register accessor: Core0 access peripherals permission configuration register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_5`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<core_0_pif_pms_constrain_5::CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 5."]
pub mod core_0_pif_pms_constrain_5;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_6 (rw) register accessor: Core0 access peripherals permission configuration register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_6`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<core_0_pif_pms_constrain_6::CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 6."]
pub mod core_0_pif_pms_constrain_6;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_7 (rw) register accessor: Core0 access peripherals permission configuration register 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_7`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<core_0_pif_pms_constrain_7::CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 7."]
pub mod core_0_pif_pms_constrain_7;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_8 (rw) register accessor: Core0 access peripherals permission configuration register 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_8`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<core_0_pif_pms_constrain_8::CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 8."]
pub mod core_0_pif_pms_constrain_8;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_9 (rw) register accessor: Core0 access peripherals permission configuration register 9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_9`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<core_0_pif_pms_constrain_9::CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 9."]
pub mod core_0_pif_pms_constrain_9;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_10 (rw) register accessor: Core0 access peripherals permission configuration register 10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_10`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<core_0_pif_pms_constrain_10::CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 10."]
pub mod core_0_pif_pms_constrain_10;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_11 (rw) register accessor: Core0 access peripherals permission configuration register 11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_11`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_11 =
    crate::Reg<core_0_pif_pms_constrain_11::CORE_0_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 11."]
pub mod core_0_pif_pms_constrain_11;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_12 (rw) register accessor: Core0 access peripherals permission configuration register 12.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_12`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_12 =
    crate::Reg<core_0_pif_pms_constrain_12::CORE_0_PIF_PMS_CONSTRAIN_12_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 12."]
pub mod core_0_pif_pms_constrain_12;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_13 (rw) register accessor: Core0 access peripherals permission configuration register 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_13`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_13 =
    crate::Reg<core_0_pif_pms_constrain_13::CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 13."]
pub mod core_0_pif_pms_constrain_13;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_14 (rw) register accessor: Core0 access peripherals permission configuration register 14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_constrain_14`] module"]
pub type CORE_0_PIF_PMS_CONSTRAIN_14 =
    crate::Reg<core_0_pif_pms_constrain_14::CORE_0_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 14."]
pub mod core_0_pif_pms_constrain_14;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_0 (rw) register accessor: Core0 region permission register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_0`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_0 =
    crate::Reg<core_0_region_pms_constrain_0::CORE_0_REGION_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core0 region permission register 0."]
pub mod core_0_region_pms_constrain_0;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_1 (rw) register accessor: Core0 region permission register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_1`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_1 =
    crate::Reg<core_0_region_pms_constrain_1::CORE_0_REGION_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core0 region permission register 1."]
pub mod core_0_region_pms_constrain_1;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_2 (rw) register accessor: Core0 region permission register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_2`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_2 =
    crate::Reg<core_0_region_pms_constrain_2::CORE_0_REGION_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core0 region permission register 2."]
pub mod core_0_region_pms_constrain_2;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_3 (rw) register accessor: Core0 region permission register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_3`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_3 =
    crate::Reg<core_0_region_pms_constrain_3::CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core0 region permission register 3."]
pub mod core_0_region_pms_constrain_3;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_4 (rw) register accessor: Core0 region permission register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_4`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_4 =
    crate::Reg<core_0_region_pms_constrain_4::CORE_0_REGION_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core0 region permission register 4."]
pub mod core_0_region_pms_constrain_4;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_5 (rw) register accessor: Core0 region permission register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_5`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_5 =
    crate::Reg<core_0_region_pms_constrain_5::CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core0 region permission register 5."]
pub mod core_0_region_pms_constrain_5;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_6 (rw) register accessor: Core0 region permission register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_6`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_6 =
    crate::Reg<core_0_region_pms_constrain_6::CORE_0_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core0 region permission register 6."]
pub mod core_0_region_pms_constrain_6;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_7 (rw) register accessor: Core0 region permission register 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_7`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_7 =
    crate::Reg<core_0_region_pms_constrain_7::CORE_0_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core0 region permission register 7."]
pub mod core_0_region_pms_constrain_7;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_8 (rw) register accessor: Core0 region permission register 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_8`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_8 =
    crate::Reg<core_0_region_pms_constrain_8::CORE_0_REGION_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core0 region permission register 8."]
pub mod core_0_region_pms_constrain_8;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_9 (rw) register accessor: Core0 region permission register 9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_9`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_9 =
    crate::Reg<core_0_region_pms_constrain_9::CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core0 region permission register 9."]
pub mod core_0_region_pms_constrain_9;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_10 (rw) register accessor: Core0 region permission register 10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_10`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_10 =
    crate::Reg<core_0_region_pms_constrain_10::CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>;
#[doc = "Core0 region permission register 10."]
pub mod core_0_region_pms_constrain_10;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_11 (rw) register accessor: Core0 region permission register 11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_11`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_11 =
    crate::Reg<core_0_region_pms_constrain_11::CORE_0_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Core0 region permission register 11."]
pub mod core_0_region_pms_constrain_11;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_12 (rw) register accessor: Core0 region permission register 12.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_12`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_12 =
    crate::Reg<core_0_region_pms_constrain_12::CORE_0_REGION_PMS_CONSTRAIN_12_SPEC>;
#[doc = "Core0 region permission register 12."]
pub mod core_0_region_pms_constrain_12;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_13 (rw) register accessor: Core0 region permission register 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_13`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_13 =
    crate::Reg<core_0_region_pms_constrain_13::CORE_0_REGION_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Core0 region permission register 13."]
pub mod core_0_region_pms_constrain_13;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_14 (rw) register accessor: Core0 region permission register 14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_region_pms_constrain_14`] module"]
pub type CORE_0_REGION_PMS_CONSTRAIN_14 =
    crate::Reg<core_0_region_pms_constrain_14::CORE_0_REGION_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Core0 region permission register 14."]
pub mod core_0_region_pms_constrain_14;
#[doc = "CORE_0_PIF_PMS_MONITOR_0 (rw) register accessor: Core0 permission report register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_0`] module"]
pub type CORE_0_PIF_PMS_MONITOR_0 =
    crate::Reg<core_0_pif_pms_monitor_0::CORE_0_PIF_PMS_MONITOR_0_SPEC>;
#[doc = "Core0 permission report register 0."]
pub mod core_0_pif_pms_monitor_0;
#[doc = "CORE_0_PIF_PMS_MONITOR_1 (rw) register accessor: Core0 permission report register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_1`] module"]
pub type CORE_0_PIF_PMS_MONITOR_1 =
    crate::Reg<core_0_pif_pms_monitor_1::CORE_0_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "Core0 permission report register 1."]
pub mod core_0_pif_pms_monitor_1;
#[doc = "CORE_0_PIF_PMS_MONITOR_2 (r) register accessor: Core0 permission report register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_2`] module"]
pub type CORE_0_PIF_PMS_MONITOR_2 =
    crate::Reg<core_0_pif_pms_monitor_2::CORE_0_PIF_PMS_MONITOR_2_SPEC>;
#[doc = "Core0 permission report register 2."]
pub mod core_0_pif_pms_monitor_2;
#[doc = "CORE_0_PIF_PMS_MONITOR_3 (r) register accessor: Core0 permission report register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_3`] module"]
pub type CORE_0_PIF_PMS_MONITOR_3 =
    crate::Reg<core_0_pif_pms_monitor_3::CORE_0_PIF_PMS_MONITOR_3_SPEC>;
#[doc = "Core0 permission report register 3."]
pub mod core_0_pif_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_MONITOR_4 (rw) register accessor: Core0 permission report register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_4`] module"]
pub type CORE_0_PIF_PMS_MONITOR_4 =
    crate::Reg<core_0_pif_pms_monitor_4::CORE_0_PIF_PMS_MONITOR_4_SPEC>;
#[doc = "Core0 permission report register 4."]
pub mod core_0_pif_pms_monitor_4;
#[doc = "CORE_0_PIF_PMS_MONITOR_5 (r) register accessor: Core0 permission report register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_5`] module"]
pub type CORE_0_PIF_PMS_MONITOR_5 =
    crate::Reg<core_0_pif_pms_monitor_5::CORE_0_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "Core0 permission report register 5."]
pub mod core_0_pif_pms_monitor_5;
#[doc = "CORE_0_PIF_PMS_MONITOR_6 (r) register accessor: Core0 permission report register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_pif_pms_monitor_6`] module"]
pub type CORE_0_PIF_PMS_MONITOR_6 =
    crate::Reg<core_0_pif_pms_monitor_6::CORE_0_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "Core0 permission report register 6."]
pub mod core_0_pif_pms_monitor_6;
#[doc = "CORE_0_VECBASE_OVERRIDE_LOCK (rw) register accessor: core0 vecbase override configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_vecbase_override_lock`] module"]
pub type CORE_0_VECBASE_OVERRIDE_LOCK =
    crate::Reg<core_0_vecbase_override_lock::CORE_0_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "core0 vecbase override configuration register 0"]
pub mod core_0_vecbase_override_lock;
#[doc = "CORE_0_VECBASE_OVERRIDE_0 (rw) register accessor: core0 vecbase override configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_vecbase_override_0`] module"]
pub type CORE_0_VECBASE_OVERRIDE_0 =
    crate::Reg<core_0_vecbase_override_0::CORE_0_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "core0 vecbase override configuration register 0"]
pub mod core_0_vecbase_override_0;
#[doc = "CORE_0_VECBASE_OVERRIDE_1 (rw) register accessor: core0 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_vecbase_override_1`] module"]
pub type CORE_0_VECBASE_OVERRIDE_1 =
    crate::Reg<core_0_vecbase_override_1::CORE_0_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "core0 vecbase override configuration register 1"]
pub mod core_0_vecbase_override_1;
#[doc = "CORE_0_VECBASE_OVERRIDE_2 (rw) register accessor: core0 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_vecbase_override_2`] module"]
pub type CORE_0_VECBASE_OVERRIDE_2 =
    crate::Reg<core_0_vecbase_override_2::CORE_0_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "core0 vecbase override configuration register 1"]
pub mod core_0_vecbase_override_2;
#[doc = "CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0 (rw) register accessor: core0 toomanyexception override configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_toomanyexceptions_m_override_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_toomanyexceptions_m_override_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_toomanyexceptions_m_override_0`] module"]
pub type CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0 =
    crate::Reg<core_0_toomanyexceptions_m_override_0::CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>;
#[doc = "core0 toomanyexception override configuration register 0."]
pub mod core_0_toomanyexceptions_m_override_0;
#[doc = "CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1 (rw) register accessor: core0 toomanyexception override configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_toomanyexceptions_m_override_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_toomanyexceptions_m_override_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_toomanyexceptions_m_override_1`] module"]
pub type CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1 =
    crate::Reg<core_0_toomanyexceptions_m_override_1::CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "core0 toomanyexception override configuration register 1."]
pub mod core_0_toomanyexceptions_m_override_1;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_0 (rw) register accessor: Core1 access peripherals permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_0`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<core_1_pif_pms_constrain_0::CORE_1_PIF_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 0."]
pub mod core_1_pif_pms_constrain_0;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_1 (rw) register accessor: Core1 access peripherals permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_1`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<core_1_pif_pms_constrain_1::CORE_1_PIF_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 1."]
pub mod core_1_pif_pms_constrain_1;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_2 (rw) register accessor: Core1 access peripherals permission configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_2`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<core_1_pif_pms_constrain_2::CORE_1_PIF_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 2."]
pub mod core_1_pif_pms_constrain_2;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_3 (rw) register accessor: Core1 access peripherals permission configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_3`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<core_1_pif_pms_constrain_3::CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 3."]
pub mod core_1_pif_pms_constrain_3;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_4 (rw) register accessor: Core1 access peripherals permission configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_4`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<core_1_pif_pms_constrain_4::CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 4."]
pub mod core_1_pif_pms_constrain_4;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_5 (rw) register accessor: Core1 access peripherals permission configuration register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_5`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<core_1_pif_pms_constrain_5::CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 5."]
pub mod core_1_pif_pms_constrain_5;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_6 (rw) register accessor: Core1 access peripherals permission configuration register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_6`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<core_1_pif_pms_constrain_6::CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 6."]
pub mod core_1_pif_pms_constrain_6;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_7 (rw) register accessor: Core1 access peripherals permission configuration register 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_7`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<core_1_pif_pms_constrain_7::CORE_1_PIF_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 7."]
pub mod core_1_pif_pms_constrain_7;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_8 (rw) register accessor: Core1 access peripherals permission configuration register 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_8`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<core_1_pif_pms_constrain_8::CORE_1_PIF_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 8."]
pub mod core_1_pif_pms_constrain_8;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_9 (rw) register accessor: Core1 access peripherals permission configuration register 9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_9`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<core_1_pif_pms_constrain_9::CORE_1_PIF_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 9."]
pub mod core_1_pif_pms_constrain_9;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_10 (rw) register accessor: core1 access peripherals permission configuration register 10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_10`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<core_1_pif_pms_constrain_10::CORE_1_PIF_PMS_CONSTRAIN_10_SPEC>;
#[doc = "core1 access peripherals permission configuration register 10."]
pub mod core_1_pif_pms_constrain_10;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_11 (rw) register accessor: core1 access peripherals permission configuration register 11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_11`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_11 =
    crate::Reg<core_1_pif_pms_constrain_11::CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "core1 access peripherals permission configuration register 11."]
pub mod core_1_pif_pms_constrain_11;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_12 (rw) register accessor: core1 access peripherals permission configuration register 12.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_12`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_12 =
    crate::Reg<core_1_pif_pms_constrain_12::CORE_1_PIF_PMS_CONSTRAIN_12_SPEC>;
#[doc = "core1 access peripherals permission configuration register 12."]
pub mod core_1_pif_pms_constrain_12;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_13 (rw) register accessor: core1 access peripherals permission configuration register 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_13`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_13 =
    crate::Reg<core_1_pif_pms_constrain_13::CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "core1 access peripherals permission configuration register 13."]
pub mod core_1_pif_pms_constrain_13;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_14 (rw) register accessor: core1 access peripherals permission configuration register 14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_constrain_14`] module"]
pub type CORE_1_PIF_PMS_CONSTRAIN_14 =
    crate::Reg<core_1_pif_pms_constrain_14::CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "core1 access peripherals permission configuration register 14."]
pub mod core_1_pif_pms_constrain_14;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_0 (rw) register accessor: core1 region permission register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_0`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_0 =
    crate::Reg<core_1_region_pms_constrain_0::CORE_1_REGION_PMS_CONSTRAIN_0_SPEC>;
#[doc = "core1 region permission register 0."]
pub mod core_1_region_pms_constrain_0;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_1 (rw) register accessor: core1 region permission register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_1`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_1 =
    crate::Reg<core_1_region_pms_constrain_1::CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>;
#[doc = "core1 region permission register 1."]
pub mod core_1_region_pms_constrain_1;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_2 (rw) register accessor: core1 region permission register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_2`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_2 =
    crate::Reg<core_1_region_pms_constrain_2::CORE_1_REGION_PMS_CONSTRAIN_2_SPEC>;
#[doc = "core1 region permission register 2."]
pub mod core_1_region_pms_constrain_2;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_3 (rw) register accessor: core1 region permission register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_3`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_3 =
    crate::Reg<core_1_region_pms_constrain_3::CORE_1_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "core1 region permission register 3."]
pub mod core_1_region_pms_constrain_3;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_4 (rw) register accessor: core1 region permission register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_4`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_4 =
    crate::Reg<core_1_region_pms_constrain_4::CORE_1_REGION_PMS_CONSTRAIN_4_SPEC>;
#[doc = "core1 region permission register 4."]
pub mod core_1_region_pms_constrain_4;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_5 (rw) register accessor: core1 region permission register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_5`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_5 =
    crate::Reg<core_1_region_pms_constrain_5::CORE_1_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "core1 region permission register 5."]
pub mod core_1_region_pms_constrain_5;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_6 (rw) register accessor: core1 region permission register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_6`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_6 =
    crate::Reg<core_1_region_pms_constrain_6::CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "core1 region permission register 6."]
pub mod core_1_region_pms_constrain_6;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_7 (rw) register accessor: core1 region permission register 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_7`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_7 =
    crate::Reg<core_1_region_pms_constrain_7::CORE_1_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "core1 region permission register 7."]
pub mod core_1_region_pms_constrain_7;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_8 (rw) register accessor: core1 region permission register 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_8`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_8 =
    crate::Reg<core_1_region_pms_constrain_8::CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>;
#[doc = "core1 region permission register 8."]
pub mod core_1_region_pms_constrain_8;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_9 (rw) register accessor: core1 region permission register 9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_9`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_9 =
    crate::Reg<core_1_region_pms_constrain_9::CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>;
#[doc = "core1 region permission register 9."]
pub mod core_1_region_pms_constrain_9;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_10 (rw) register accessor: core1 region permission register 10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_10`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_10 =
    crate::Reg<core_1_region_pms_constrain_10::CORE_1_REGION_PMS_CONSTRAIN_10_SPEC>;
#[doc = "core1 region permission register 10."]
pub mod core_1_region_pms_constrain_10;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_11 (rw) register accessor: core1 region permission register 11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_11`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_11 =
    crate::Reg<core_1_region_pms_constrain_11::CORE_1_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "core1 region permission register 11."]
pub mod core_1_region_pms_constrain_11;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_12 (rw) register accessor: core1 region permission register 12.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_12`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_12 =
    crate::Reg<core_1_region_pms_constrain_12::CORE_1_REGION_PMS_CONSTRAIN_12_SPEC>;
#[doc = "core1 region permission register 12."]
pub mod core_1_region_pms_constrain_12;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_13 (rw) register accessor: core1 region permission register 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_13`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_13 =
    crate::Reg<core_1_region_pms_constrain_13::CORE_1_REGION_PMS_CONSTRAIN_13_SPEC>;
#[doc = "core1 region permission register 13."]
pub mod core_1_region_pms_constrain_13;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_14 (rw) register accessor: core1 region permission register 14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_region_pms_constrain_14`] module"]
pub type CORE_1_REGION_PMS_CONSTRAIN_14 =
    crate::Reg<core_1_region_pms_constrain_14::CORE_1_REGION_PMS_CONSTRAIN_14_SPEC>;
#[doc = "core1 region permission register 14."]
pub mod core_1_region_pms_constrain_14;
#[doc = "CORE_1_PIF_PMS_MONITOR_0 (rw) register accessor: core1 permission report register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_0`] module"]
pub type CORE_1_PIF_PMS_MONITOR_0 =
    crate::Reg<core_1_pif_pms_monitor_0::CORE_1_PIF_PMS_MONITOR_0_SPEC>;
#[doc = "core1 permission report register 0."]
pub mod core_1_pif_pms_monitor_0;
#[doc = "CORE_1_PIF_PMS_MONITOR_1 (rw) register accessor: core1 permission report register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_1`] module"]
pub type CORE_1_PIF_PMS_MONITOR_1 =
    crate::Reg<core_1_pif_pms_monitor_1::CORE_1_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "core1 permission report register 1."]
pub mod core_1_pif_pms_monitor_1;
#[doc = "CORE_1_PIF_PMS_MONITOR_2 (r) register accessor: core1 permission report register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_2`] module"]
pub type CORE_1_PIF_PMS_MONITOR_2 =
    crate::Reg<core_1_pif_pms_monitor_2::CORE_1_PIF_PMS_MONITOR_2_SPEC>;
#[doc = "core1 permission report register 2."]
pub mod core_1_pif_pms_monitor_2;
#[doc = "CORE_1_PIF_PMS_MONITOR_3 (r) register accessor: core1 permission report register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_3`] module"]
pub type CORE_1_PIF_PMS_MONITOR_3 =
    crate::Reg<core_1_pif_pms_monitor_3::CORE_1_PIF_PMS_MONITOR_3_SPEC>;
#[doc = "core1 permission report register 3."]
pub mod core_1_pif_pms_monitor_3;
#[doc = "CORE_1_PIF_PMS_MONITOR_4 (rw) register accessor: core1 permission report register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_4`] module"]
pub type CORE_1_PIF_PMS_MONITOR_4 =
    crate::Reg<core_1_pif_pms_monitor_4::CORE_1_PIF_PMS_MONITOR_4_SPEC>;
#[doc = "core1 permission report register 4."]
pub mod core_1_pif_pms_monitor_4;
#[doc = "CORE_1_PIF_PMS_MONITOR_5 (r) register accessor: core1 permission report register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_5`] module"]
pub type CORE_1_PIF_PMS_MONITOR_5 =
    crate::Reg<core_1_pif_pms_monitor_5::CORE_1_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "core1 permission report register 5."]
pub mod core_1_pif_pms_monitor_5;
#[doc = "CORE_1_PIF_PMS_MONITOR_6 (r) register accessor: core1 permission report register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_pif_pms_monitor_6`] module"]
pub type CORE_1_PIF_PMS_MONITOR_6 =
    crate::Reg<core_1_pif_pms_monitor_6::CORE_1_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "core1 permission report register 6."]
pub mod core_1_pif_pms_monitor_6;
#[doc = "CORE_1_VECBASE_OVERRIDE_LOCK (rw) register accessor: core1 vecbase override configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_vecbase_override_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_vecbase_override_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_vecbase_override_lock`] module"]
pub type CORE_1_VECBASE_OVERRIDE_LOCK =
    crate::Reg<core_1_vecbase_override_lock::CORE_1_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "core1 vecbase override configuration register 0"]
pub mod core_1_vecbase_override_lock;
#[doc = "CORE_1_VECBASE_OVERRIDE_0 (rw) register accessor: core1 vecbase override configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_vecbase_override_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_vecbase_override_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_vecbase_override_0`] module"]
pub type CORE_1_VECBASE_OVERRIDE_0 =
    crate::Reg<core_1_vecbase_override_0::CORE_1_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "core1 vecbase override configuration register 0"]
pub mod core_1_vecbase_override_0;
#[doc = "CORE_1_VECBASE_OVERRIDE_1 (rw) register accessor: core1 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_vecbase_override_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_vecbase_override_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_vecbase_override_1`] module"]
pub type CORE_1_VECBASE_OVERRIDE_1 =
    crate::Reg<core_1_vecbase_override_1::CORE_1_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "core1 vecbase override configuration register 1"]
pub mod core_1_vecbase_override_1;
#[doc = "CORE_1_VECBASE_OVERRIDE_2 (rw) register accessor: core1 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_vecbase_override_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_vecbase_override_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_vecbase_override_2`] module"]
pub type CORE_1_VECBASE_OVERRIDE_2 =
    crate::Reg<core_1_vecbase_override_2::CORE_1_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "core1 vecbase override configuration register 1"]
pub mod core_1_vecbase_override_2;
#[doc = "CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0 (rw) register accessor: core1 toomanyexception override configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_toomanyexceptions_m_override_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_toomanyexceptions_m_override_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_toomanyexceptions_m_override_0`] module"]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0 =
    crate::Reg<core_1_toomanyexceptions_m_override_0::CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>;
#[doc = "core1 toomanyexception override configuration register 0."]
pub mod core_1_toomanyexceptions_m_override_0;
#[doc = "CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 (rw) register accessor: core1 toomanyexception override configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_toomanyexceptions_m_override_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_toomanyexceptions_m_override_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_toomanyexceptions_m_override_1`] module"]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 =
    crate::Reg<core_1_toomanyexceptions_m_override_1::CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "core1 toomanyexception override configuration register 1."]
pub mod core_1_toomanyexceptions_m_override_1;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_0 (rw) register accessor: BackUp access peripherals permission configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_0`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_0 =
    crate::Reg<backup_bus_pms_constrain_0::BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 0."]
pub mod backup_bus_pms_constrain_0;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_1 (rw) register accessor: BackUp access peripherals permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_1`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_1 =
    crate::Reg<backup_bus_pms_constrain_1::BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 1."]
pub mod backup_bus_pms_constrain_1;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_2 (rw) register accessor: BackUp access peripherals permission configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_2`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_2 =
    crate::Reg<backup_bus_pms_constrain_2::BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 2."]
pub mod backup_bus_pms_constrain_2;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_3 (rw) register accessor: BackUp access peripherals permission configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_3`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_3 =
    crate::Reg<backup_bus_pms_constrain_3::BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 3."]
pub mod backup_bus_pms_constrain_3;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_4 (rw) register accessor: BackUp access peripherals permission configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_4`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_4 =
    crate::Reg<backup_bus_pms_constrain_4::BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 4."]
pub mod backup_bus_pms_constrain_4;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_5 (rw) register accessor: BackUp access peripherals permission configuration register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_5`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_5 =
    crate::Reg<backup_bus_pms_constrain_5::BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 5."]
pub mod backup_bus_pms_constrain_5;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_6 (rw) register accessor: BackUp access peripherals permission configuration register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_constrain_6`] module"]
pub type BACKUP_BUS_PMS_CONSTRAIN_6 =
    crate::Reg<backup_bus_pms_constrain_6::BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 6."]
pub mod backup_bus_pms_constrain_6;
#[doc = "BACKUP_BUS_PMS_MONITOR_0 (rw) register accessor: BackUp permission report register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_monitor_0`] module"]
pub type BACKUP_BUS_PMS_MONITOR_0 =
    crate::Reg<backup_bus_pms_monitor_0::BACKUP_BUS_PMS_MONITOR_0_SPEC>;
#[doc = "BackUp permission report register 0."]
pub mod backup_bus_pms_monitor_0;
#[doc = "BACKUP_BUS_PMS_MONITOR_1 (rw) register accessor: BackUp permission report register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_monitor_1`] module"]
pub type BACKUP_BUS_PMS_MONITOR_1 =
    crate::Reg<backup_bus_pms_monitor_1::BACKUP_BUS_PMS_MONITOR_1_SPEC>;
#[doc = "BackUp permission report register 1."]
pub mod backup_bus_pms_monitor_1;
#[doc = "BACKUP_BUS_PMS_MONITOR_2 (r) register accessor: BackUp permission report register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_monitor_2`] module"]
pub type BACKUP_BUS_PMS_MONITOR_2 =
    crate::Reg<backup_bus_pms_monitor_2::BACKUP_BUS_PMS_MONITOR_2_SPEC>;
#[doc = "BackUp permission report register 2."]
pub mod backup_bus_pms_monitor_2;
#[doc = "BACKUP_BUS_PMS_MONITOR_3 (r) register accessor: BackUp permission report register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_bus_pms_monitor_3`] module"]
pub type BACKUP_BUS_PMS_MONITOR_3 =
    crate::Reg<backup_bus_pms_monitor_3::BACKUP_BUS_PMS_MONITOR_3_SPEC>;
#[doc = "BackUp permission report register 3."]
pub mod backup_bus_pms_monitor_3;
#[doc = "EDMA_BOUNDARY_LOCK (rw) register accessor: EDMA boundary lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_boundary_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_boundary_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_boundary_lock`] module"]
pub type EDMA_BOUNDARY_LOCK = crate::Reg<edma_boundary_lock::EDMA_BOUNDARY_LOCK_SPEC>;
#[doc = "EDMA boundary lock register."]
pub mod edma_boundary_lock;
#[doc = "EDMA_BOUNDARY_0 (rw) register accessor: EDMA boundary 0 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_boundary_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_boundary_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_boundary_0`] module"]
pub type EDMA_BOUNDARY_0 = crate::Reg<edma_boundary_0::EDMA_BOUNDARY_0_SPEC>;
#[doc = "EDMA boundary 0 configuration"]
pub mod edma_boundary_0;
#[doc = "EDMA_BOUNDARY_1 (rw) register accessor: EDMA boundary 1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_boundary_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_boundary_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_boundary_1`] module"]
pub type EDMA_BOUNDARY_1 = crate::Reg<edma_boundary_1::EDMA_BOUNDARY_1_SPEC>;
#[doc = "EDMA boundary 1 configuration"]
pub mod edma_boundary_1;
#[doc = "EDMA_BOUNDARY_2 (rw) register accessor: EDMA boundary 2 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_boundary_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_boundary_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_boundary_2`] module"]
pub type EDMA_BOUNDARY_2 = crate::Reg<edma_boundary_2::EDMA_BOUNDARY_2_SPEC>;
#[doc = "EDMA boundary 2 configuration"]
pub mod edma_boundary_2;
#[doc = "EDMA_PMS_SPI2_LOCK (rw) register accessor: EDMA-SPI2 permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_spi2_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_spi2_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_spi2_lock`] module"]
pub type EDMA_PMS_SPI2_LOCK = crate::Reg<edma_pms_spi2_lock::EDMA_PMS_SPI2_LOCK_SPEC>;
#[doc = "EDMA-SPI2 permission lock register."]
pub mod edma_pms_spi2_lock;
#[doc = "EDMA_PMS_SPI2 (rw) register accessor: EDMA-SPI2 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_spi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_spi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_spi2`] module"]
pub type EDMA_PMS_SPI2 = crate::Reg<edma_pms_spi2::EDMA_PMS_SPI2_SPEC>;
#[doc = "EDMA-SPI2 permission control register."]
pub mod edma_pms_spi2;
#[doc = "EDMA_PMS_SPI3_LOCK (rw) register accessor: EDMA-SPI3 permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_spi3_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_spi3_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_spi3_lock`] module"]
pub type EDMA_PMS_SPI3_LOCK = crate::Reg<edma_pms_spi3_lock::EDMA_PMS_SPI3_LOCK_SPEC>;
#[doc = "EDMA-SPI3 permission lock register."]
pub mod edma_pms_spi3_lock;
#[doc = "EDMA_PMS_SPI3 (rw) register accessor: EDMA-SPI3 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_spi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_spi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_spi3`] module"]
pub type EDMA_PMS_SPI3 = crate::Reg<edma_pms_spi3::EDMA_PMS_SPI3_SPEC>;
#[doc = "EDMA-SPI3 permission control register."]
pub mod edma_pms_spi3;
#[doc = "EDMA_PMS_UHCI0_LOCK (rw) register accessor: EDMA-UHCI0 permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_uhci0_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_uhci0_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_uhci0_lock`] module"]
pub type EDMA_PMS_UHCI0_LOCK = crate::Reg<edma_pms_uhci0_lock::EDMA_PMS_UHCI0_LOCK_SPEC>;
#[doc = "EDMA-UHCI0 permission lock register."]
pub mod edma_pms_uhci0_lock;
#[doc = "EDMA_PMS_UHCI0 (rw) register accessor: EDMA-UHCI0 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_uhci0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_uhci0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_uhci0`] module"]
pub type EDMA_PMS_UHCI0 = crate::Reg<edma_pms_uhci0::EDMA_PMS_UHCI0_SPEC>;
#[doc = "EDMA-UHCI0 permission control register."]
pub mod edma_pms_uhci0;
#[doc = "EDMA_PMS_I2S0_LOCK (rw) register accessor: EDMA-I2S0 permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_i2s0_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_i2s0_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_i2s0_lock`] module"]
pub type EDMA_PMS_I2S0_LOCK = crate::Reg<edma_pms_i2s0_lock::EDMA_PMS_I2S0_LOCK_SPEC>;
#[doc = "EDMA-I2S0 permission lock register."]
pub mod edma_pms_i2s0_lock;
#[doc = "EDMA_PMS_I2S0 (rw) register accessor: EDMA-I2S0 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_i2s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_i2s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_i2s0`] module"]
pub type EDMA_PMS_I2S0 = crate::Reg<edma_pms_i2s0::EDMA_PMS_I2S0_SPEC>;
#[doc = "EDMA-I2S0 permission control register."]
pub mod edma_pms_i2s0;
#[doc = "EDMA_PMS_I2S1_LOCK (rw) register accessor: EDMA-I2S1 permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_i2s1_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_i2s1_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_i2s1_lock`] module"]
pub type EDMA_PMS_I2S1_LOCK = crate::Reg<edma_pms_i2s1_lock::EDMA_PMS_I2S1_LOCK_SPEC>;
#[doc = "EDMA-I2S1 permission lock register."]
pub mod edma_pms_i2s1_lock;
#[doc = "EDMA_PMS_I2S1 (rw) register accessor: EDMA-I2S1 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_i2s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_i2s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_i2s1`] module"]
pub type EDMA_PMS_I2S1 = crate::Reg<edma_pms_i2s1::EDMA_PMS_I2S1_SPEC>;
#[doc = "EDMA-I2S1 permission control register."]
pub mod edma_pms_i2s1;
#[doc = "EDMA_PMS_LCD_CAM_LOCK (rw) register accessor: EDMA-LCD/CAM permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_lcd_cam_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_lcd_cam_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_lcd_cam_lock`] module"]
pub type EDMA_PMS_LCD_CAM_LOCK = crate::Reg<edma_pms_lcd_cam_lock::EDMA_PMS_LCD_CAM_LOCK_SPEC>;
#[doc = "EDMA-LCD/CAM permission lock register."]
pub mod edma_pms_lcd_cam_lock;
#[doc = "EDMA_PMS_LCD_CAM (rw) register accessor: EDMA-LCD/CAM permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_lcd_cam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_lcd_cam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_lcd_cam`] module"]
pub type EDMA_PMS_LCD_CAM = crate::Reg<edma_pms_lcd_cam::EDMA_PMS_LCD_CAM_SPEC>;
#[doc = "EDMA-LCD/CAM permission control register."]
pub mod edma_pms_lcd_cam;
#[doc = "EDMA_PMS_AES_LOCK (rw) register accessor: EDMA-AES permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_aes_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_aes_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_aes_lock`] module"]
pub type EDMA_PMS_AES_LOCK = crate::Reg<edma_pms_aes_lock::EDMA_PMS_AES_LOCK_SPEC>;
#[doc = "EDMA-AES permission lock register."]
pub mod edma_pms_aes_lock;
#[doc = "EDMA_PMS_AES (rw) register accessor: EDMA-AES permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_aes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_aes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_aes`] module"]
pub type EDMA_PMS_AES = crate::Reg<edma_pms_aes::EDMA_PMS_AES_SPEC>;
#[doc = "EDMA-AES permission control register."]
pub mod edma_pms_aes;
#[doc = "EDMA_PMS_SHA_LOCK (rw) register accessor: EDMA-SHA permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_sha_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_sha_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_sha_lock`] module"]
pub type EDMA_PMS_SHA_LOCK = crate::Reg<edma_pms_sha_lock::EDMA_PMS_SHA_LOCK_SPEC>;
#[doc = "EDMA-SHA permission lock register."]
pub mod edma_pms_sha_lock;
#[doc = "EDMA_PMS_SHA (rw) register accessor: EDMA-SHA permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_sha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_sha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_sha`] module"]
pub type EDMA_PMS_SHA = crate::Reg<edma_pms_sha::EDMA_PMS_SHA_SPEC>;
#[doc = "EDMA-SHA permission control register."]
pub mod edma_pms_sha;
#[doc = "EDMA_PMS_ADC_DAC_LOCK (rw) register accessor: EDMA-ADC/DAC permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_adc_dac_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_adc_dac_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_adc_dac_lock`] module"]
pub type EDMA_PMS_ADC_DAC_LOCK = crate::Reg<edma_pms_adc_dac_lock::EDMA_PMS_ADC_DAC_LOCK_SPEC>;
#[doc = "EDMA-ADC/DAC permission lock register."]
pub mod edma_pms_adc_dac_lock;
#[doc = "EDMA_PMS_ADC_DAC (rw) register accessor: EDMA-ADC/DAC permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_adc_dac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_adc_dac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_adc_dac`] module"]
pub type EDMA_PMS_ADC_DAC = crate::Reg<edma_pms_adc_dac::EDMA_PMS_ADC_DAC_SPEC>;
#[doc = "EDMA-ADC/DAC permission control register."]
pub mod edma_pms_adc_dac;
#[doc = "EDMA_PMS_RMT_LOCK (rw) register accessor: EDMA-RMT permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_rmt_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_rmt_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_rmt_lock`] module"]
pub type EDMA_PMS_RMT_LOCK = crate::Reg<edma_pms_rmt_lock::EDMA_PMS_RMT_LOCK_SPEC>;
#[doc = "EDMA-RMT permission lock register."]
pub mod edma_pms_rmt_lock;
#[doc = "EDMA_PMS_RMT (rw) register accessor: EDMA-RMT permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_rmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_rmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edma_pms_rmt`] module"]
pub type EDMA_PMS_RMT = crate::Reg<edma_pms_rmt::EDMA_PMS_RMT_SPEC>;
#[doc = "EDMA-RMT permission control register."]
pub mod edma_pms_rmt;
#[doc = "CLOCK_GATE (rw) register accessor: Sensitive module clock gate configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Sensitive module clock gate configuration register."]
pub mod clock_gate;
#[doc = "RTC_PMS (rw) register accessor: RTC coprocessor permission register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_pms`] module"]
pub type RTC_PMS = crate::Reg<rtc_pms::RTC_PMS_SPEC>;
#[doc = "RTC coprocessor permission register."]
pub mod rtc_pms;
#[doc = "DATE (rw) register accessor: Sensitive version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Sensitive version register."]
pub mod date;
