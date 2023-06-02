#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache data array configuration register 0."]
    pub cache_dataarray_connect_0: CACHE_DATAARRAY_CONNECT_0,
    #[doc = "0x04 - Cache data array configuration register 1."]
    pub cache_dataarray_connect_1: CACHE_DATAARRAY_CONNECT_1,
    #[doc = "0x08 - APB peripheral configuration register 0."]
    pub apb_peripheral_access_0: APB_PERIPHERAL_ACCESS_0,
    #[doc = "0x0c - APB peripheral configuration register 1."]
    pub apb_peripheral_access_1: APB_PERIPHERAL_ACCESS_1,
    #[doc = "0x10 - Internal SRAM configuration register 0."]
    pub internal_sram_usage_0: INTERNAL_SRAM_USAGE_0,
    #[doc = "0x14 - Internal SRAM configuration register 1."]
    pub internal_sram_usage_1: INTERNAL_SRAM_USAGE_1,
    #[doc = "0x18 - Internal SRAM configuration register 2."]
    pub internal_sram_usage_2: INTERNAL_SRAM_USAGE_2,
    #[doc = "0x1c - Internal SRAM configuration register 3."]
    pub internal_sram_usage_3: INTERNAL_SRAM_USAGE_3,
    #[doc = "0x20 - Internal SRAM configuration register 4."]
    pub internal_sram_usage_4: INTERNAL_SRAM_USAGE_4,
    #[doc = "0x24 - Retention configuration register."]
    pub retention_disable: RETENTION_DISABLE,
    #[doc = "0x28 - Cache tag configuration register 0."]
    pub cache_tag_access_0: CACHE_TAG_ACCESS_0,
    #[doc = "0x2c - Cache tag configuration register 1."]
    pub cache_tag_access_1: CACHE_TAG_ACCESS_1,
    #[doc = "0x30 - Cache MMU configuration register 0."]
    pub cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    #[doc = "0x34 - Cache MMU configuration register 1."]
    pub cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    #[doc = "0x38 - spi2 dma permission configuration register 0."]
    pub dma_apbperi_spi2_pms_constrain_0: DMA_APBPERI_SPI2_PMS_CONSTRAIN_0,
    #[doc = "0x3c - spi2 dma permission configuration register 1."]
    pub dma_apbperi_spi2_pms_constrain_1: DMA_APBPERI_SPI2_PMS_CONSTRAIN_1,
    #[doc = "0x40 - spi3 dma permission configuration register 0."]
    pub dma_apbperi_spi3_pms_constrain_0: DMA_APBPERI_SPI3_PMS_CONSTRAIN_0,
    #[doc = "0x44 - spi3 dma permission configuration register 1."]
    pub dma_apbperi_spi3_pms_constrain_1: DMA_APBPERI_SPI3_PMS_CONSTRAIN_1,
    #[doc = "0x48 - uhci0 dma permission configuration register 0."]
    pub dma_apbperi_uhci0_pms_constrain_0: DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0,
    #[doc = "0x4c - uhci0 dma permission configuration register 1."]
    pub dma_apbperi_uhci0_pms_constrain_1: DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1,
    #[doc = "0x50 - i2s0 dma permission configuration register 0."]
    pub dma_apbperi_i2s0_pms_constrain_0: DMA_APBPERI_I2S0_PMS_CONSTRAIN_0,
    #[doc = "0x54 - i2s0 dma permission configuration register 1."]
    pub dma_apbperi_i2s0_pms_constrain_1: DMA_APBPERI_I2S0_PMS_CONSTRAIN_1,
    #[doc = "0x58 - i2s1 dma permission configuration register 0."]
    pub dma_apbperi_i2s1_pms_constrain_0: DMA_APBPERI_I2S1_PMS_CONSTRAIN_0,
    #[doc = "0x5c - i2s1 dma permission configuration register 1."]
    pub dma_apbperi_i2s1_pms_constrain_1: DMA_APBPERI_I2S1_PMS_CONSTRAIN_1,
    #[doc = "0x60 - mac dma permission configuration register 0."]
    pub dma_apbperi_mac_pms_constrain_0: DMA_APBPERI_MAC_PMS_CONSTRAIN_0,
    #[doc = "0x64 - mac dma permission configuration register 1."]
    pub dma_apbperi_mac_pms_constrain_1: DMA_APBPERI_MAC_PMS_CONSTRAIN_1,
    #[doc = "0x68 - backup dma permission configuration register 0."]
    pub dma_apbperi_backup_pms_constrain_0: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0,
    #[doc = "0x6c - backup dma permission configuration register 1."]
    pub dma_apbperi_backup_pms_constrain_1: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1,
    #[doc = "0x70 - aes dma permission configuration register 0."]
    pub dma_apbperi_aes_pms_constrain_0: DMA_APBPERI_AES_PMS_CONSTRAIN_0,
    #[doc = "0x74 - aes dma permission configuration register 1."]
    pub dma_apbperi_aes_pms_constrain_1: DMA_APBPERI_AES_PMS_CONSTRAIN_1,
    #[doc = "0x78 - sha dma permission configuration register 0."]
    pub dma_apbperi_sha_pms_constrain_0: DMA_APBPERI_SHA_PMS_CONSTRAIN_0,
    #[doc = "0x7c - sha dma permission configuration register 1."]
    pub dma_apbperi_sha_pms_constrain_1: DMA_APBPERI_SHA_PMS_CONSTRAIN_1,
    #[doc = "0x80 - adc_dac dma permission configuration register 0."]
    pub dma_apbperi_adc_dac_pms_constrain_0: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0,
    #[doc = "0x84 - adc_dac dma permission configuration register 1."]
    pub dma_apbperi_adc_dac_pms_constrain_1: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1,
    #[doc = "0x88 - rmt dma permission configuration register 0."]
    pub dma_apbperi_rmt_pms_constrain_0: DMA_APBPERI_RMT_PMS_CONSTRAIN_0,
    #[doc = "0x8c - rmt dma permission configuration register 1."]
    pub dma_apbperi_rmt_pms_constrain_1: DMA_APBPERI_RMT_PMS_CONSTRAIN_1,
    #[doc = "0x90 - lcd_cam dma permission configuration register 0."]
    pub dma_apbperi_lcd_cam_pms_constrain_0: DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0,
    #[doc = "0x94 - lcd_cam dma permission configuration register 1."]
    pub dma_apbperi_lcd_cam_pms_constrain_1: DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1,
    #[doc = "0x98 - usb dma permission configuration register 0."]
    pub dma_apbperi_usb_pms_constrain_0: DMA_APBPERI_USB_PMS_CONSTRAIN_0,
    #[doc = "0x9c - usb dma permission configuration register 1."]
    pub dma_apbperi_usb_pms_constrain_1: DMA_APBPERI_USB_PMS_CONSTRAIN_1,
    #[doc = "0xa0 - lc dma permission configuration register 0."]
    pub dma_apbperi_lc_pms_constrain_0: DMA_APBPERI_LC_PMS_CONSTRAIN_0,
    #[doc = "0xa4 - lc dma permission configuration register 1."]
    pub dma_apbperi_lc_pms_constrain_1: DMA_APBPERI_LC_PMS_CONSTRAIN_1,
    #[doc = "0xa8 - sdio dma permission configuration register 0."]
    pub dma_apbperi_sdio_pms_constrain_0: DMA_APBPERI_SDIO_PMS_CONSTRAIN_0,
    #[doc = "0xac - sdio dma permission configuration register 1."]
    pub dma_apbperi_sdio_pms_constrain_1: DMA_APBPERI_SDIO_PMS_CONSTRAIN_1,
    #[doc = "0xb0 - dma permission monitor configuration register 0."]
    pub dma_apbperi_pms_monitor_0: DMA_APBPERI_PMS_MONITOR_0,
    #[doc = "0xb4 - dma permission monitor configuration register 1."]
    pub dma_apbperi_pms_monitor_1: DMA_APBPERI_PMS_MONITOR_1,
    #[doc = "0xb8 - dma permission monitor configuration register 2."]
    pub dma_apbperi_pms_monitor_2: DMA_APBPERI_PMS_MONITOR_2,
    #[doc = "0xbc - dma permission monitor configuration register 3."]
    pub dma_apbperi_pms_monitor_3: DMA_APBPERI_PMS_MONITOR_3,
    #[doc = "0xc0 - sram split line configuration register 0"]
    pub core_x_iram0_dram0_dma_split_line_constrain_0:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0,
    #[doc = "0xc4 - sram split line configuration register 1"]
    pub core_x_iram0_dram0_dma_split_line_constrain_1:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1,
    #[doc = "0xc8 - sram split line configuration register 1"]
    pub core_x_iram0_dram0_dma_split_line_constrain_2:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2,
    #[doc = "0xcc - sram split line configuration register 1"]
    pub core_x_iram0_dram0_dma_split_line_constrain_3:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3,
    #[doc = "0xd0 - sram split line configuration register 1"]
    pub core_x_iram0_dram0_dma_split_line_constrain_4:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4,
    #[doc = "0xd4 - sram split line configuration register 1"]
    pub core_x_iram0_dram0_dma_split_line_constrain_5:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5,
    #[doc = "0xd8 - corex iram0 permission configuration register 0"]
    pub core_x_iram0_pms_constrain_0: CORE_X_IRAM0_PMS_CONSTRAIN_0,
    #[doc = "0xdc - corex iram0 permission configuration register 0"]
    pub core_x_iram0_pms_constrain_1: CORE_X_IRAM0_PMS_CONSTRAIN_1,
    #[doc = "0xe0 - corex iram0 permission configuration register 1"]
    pub core_x_iram0_pms_constrain_2: CORE_X_IRAM0_PMS_CONSTRAIN_2,
    #[doc = "0xe4 - core0 iram0 permission monitor configuration register 0"]
    pub core_0_iram0_pms_monitor_0: CORE_0_IRAM0_PMS_MONITOR_0,
    #[doc = "0xe8 - core0 iram0 permission monitor configuration register 1"]
    pub core_0_iram0_pms_monitor_1: CORE_0_IRAM0_PMS_MONITOR_1,
    #[doc = "0xec - core0 iram0 permission monitor configuration register 2"]
    pub core_0_iram0_pms_monitor_2: CORE_0_IRAM0_PMS_MONITOR_2,
    #[doc = "0xf0 - core1 iram0 permission monitor configuration register 0"]
    pub core_1_iram0_pms_monitor_0: CORE_1_IRAM0_PMS_MONITOR_0,
    #[doc = "0xf4 - core1 iram0 permission monitor configuration register 1"]
    pub core_1_iram0_pms_monitor_1: CORE_1_IRAM0_PMS_MONITOR_1,
    #[doc = "0xf8 - core1 iram0 permission monitor configuration register 2"]
    pub core_1_iram0_pms_monitor_2: CORE_1_IRAM0_PMS_MONITOR_2,
    #[doc = "0xfc - corex dram0 permission configuration register 0"]
    pub core_x_dram0_pms_constrain_0: CORE_X_DRAM0_PMS_CONSTRAIN_0,
    #[doc = "0x100 - corex dram0 permission configuration register 1"]
    pub core_x_dram0_pms_constrain_1: CORE_X_DRAM0_PMS_CONSTRAIN_1,
    #[doc = "0x104 - core0 dram0 permission monitor configuration register 0"]
    pub core_0_dram0_pms_monitor_0: CORE_0_DRAM0_PMS_MONITOR_0,
    #[doc = "0x108 - core0 dram0 permission monitor configuration register 1"]
    pub core_0_dram0_pms_monitor_1: CORE_0_DRAM0_PMS_MONITOR_1,
    #[doc = "0x10c - core0 dram0 permission monitor configuration register 2."]
    pub core_0_dram0_pms_monitor_2: CORE_0_DRAM0_PMS_MONITOR_2,
    #[doc = "0x110 - core0 dram0 permission monitor configuration register 3."]
    pub core_0_dram0_pms_monitor_3: CORE_0_DRAM0_PMS_MONITOR_3,
    #[doc = "0x114 - core1 dram0 permission monitor configuration register 0"]
    pub core_1_dram0_pms_monitor_0: CORE_1_DRAM0_PMS_MONITOR_0,
    #[doc = "0x118 - core1 dram0 permission monitor configuration register 1"]
    pub core_1_dram0_pms_monitor_1: CORE_1_DRAM0_PMS_MONITOR_1,
    #[doc = "0x11c - core1 dram0 permission monitor configuration register 2."]
    pub core_1_dram0_pms_monitor_2: CORE_1_DRAM0_PMS_MONITOR_2,
    #[doc = "0x120 - core1 dram0 permission monitor configuration register 3."]
    pub core_1_dram0_pms_monitor_3: CORE_1_DRAM0_PMS_MONITOR_3,
    #[doc = "0x124 - Core0 access peripherals permission configuration register 0."]
    pub core_0_pif_pms_constrain_0: CORE_0_PIF_PMS_CONSTRAIN_0,
    #[doc = "0x128 - Core0 access peripherals permission configuration register 1."]
    pub core_0_pif_pms_constrain_1: CORE_0_PIF_PMS_CONSTRAIN_1,
    #[doc = "0x12c - Core0 access peripherals permission configuration register 2."]
    pub core_0_pif_pms_constrain_2: CORE_0_PIF_PMS_CONSTRAIN_2,
    #[doc = "0x130 - Core0 access peripherals permission configuration register 3."]
    pub core_0_pif_pms_constrain_3: CORE_0_PIF_PMS_CONSTRAIN_3,
    #[doc = "0x134 - Core0 access peripherals permission configuration register 4."]
    pub core_0_pif_pms_constrain_4: CORE_0_PIF_PMS_CONSTRAIN_4,
    #[doc = "0x138 - Core0 access peripherals permission configuration register 5."]
    pub core_0_pif_pms_constrain_5: CORE_0_PIF_PMS_CONSTRAIN_5,
    #[doc = "0x13c - Core0 access peripherals permission configuration register 6."]
    pub core_0_pif_pms_constrain_6: CORE_0_PIF_PMS_CONSTRAIN_6,
    #[doc = "0x140 - Core0 access peripherals permission configuration register 7."]
    pub core_0_pif_pms_constrain_7: CORE_0_PIF_PMS_CONSTRAIN_7,
    #[doc = "0x144 - Core0 access peripherals permission configuration register 8."]
    pub core_0_pif_pms_constrain_8: CORE_0_PIF_PMS_CONSTRAIN_8,
    #[doc = "0x148 - Core0 access peripherals permission configuration register 9."]
    pub core_0_pif_pms_constrain_9: CORE_0_PIF_PMS_CONSTRAIN_9,
    #[doc = "0x14c - Core0 access peripherals permission configuration register 10."]
    pub core_0_pif_pms_constrain_10: CORE_0_PIF_PMS_CONSTRAIN_10,
    #[doc = "0x150 - Core0 access peripherals permission configuration register 11."]
    pub core_0_pif_pms_constrain_11: CORE_0_PIF_PMS_CONSTRAIN_11,
    #[doc = "0x154 - Core0 access peripherals permission configuration register 12."]
    pub core_0_pif_pms_constrain_12: CORE_0_PIF_PMS_CONSTRAIN_12,
    #[doc = "0x158 - Core0 access peripherals permission configuration register 13."]
    pub core_0_pif_pms_constrain_13: CORE_0_PIF_PMS_CONSTRAIN_13,
    #[doc = "0x15c - Core0 access peripherals permission configuration register 14."]
    pub core_0_pif_pms_constrain_14: CORE_0_PIF_PMS_CONSTRAIN_14,
    #[doc = "0x160 - Core0 region permission register 0."]
    pub core_0_region_pms_constrain_0: CORE_0_REGION_PMS_CONSTRAIN_0,
    #[doc = "0x164 - Core0 region permission register 1."]
    pub core_0_region_pms_constrain_1: CORE_0_REGION_PMS_CONSTRAIN_1,
    #[doc = "0x168 - Core0 region permission register 2."]
    pub core_0_region_pms_constrain_2: CORE_0_REGION_PMS_CONSTRAIN_2,
    #[doc = "0x16c - Core0 region permission register 3."]
    pub core_0_region_pms_constrain_3: CORE_0_REGION_PMS_CONSTRAIN_3,
    #[doc = "0x170 - Core0 region permission register 4."]
    pub core_0_region_pms_constrain_4: CORE_0_REGION_PMS_CONSTRAIN_4,
    #[doc = "0x174 - Core0 region permission register 5."]
    pub core_0_region_pms_constrain_5: CORE_0_REGION_PMS_CONSTRAIN_5,
    #[doc = "0x178 - Core0 region permission register 6."]
    pub core_0_region_pms_constrain_6: CORE_0_REGION_PMS_CONSTRAIN_6,
    #[doc = "0x17c - Core0 region permission register 7."]
    pub core_0_region_pms_constrain_7: CORE_0_REGION_PMS_CONSTRAIN_7,
    #[doc = "0x180 - Core0 region permission register 8."]
    pub core_0_region_pms_constrain_8: CORE_0_REGION_PMS_CONSTRAIN_8,
    #[doc = "0x184 - Core0 region permission register 9."]
    pub core_0_region_pms_constrain_9: CORE_0_REGION_PMS_CONSTRAIN_9,
    #[doc = "0x188 - Core0 region permission register 10."]
    pub core_0_region_pms_constrain_10: CORE_0_REGION_PMS_CONSTRAIN_10,
    #[doc = "0x18c - Core0 region permission register 11."]
    pub core_0_region_pms_constrain_11: CORE_0_REGION_PMS_CONSTRAIN_11,
    #[doc = "0x190 - Core0 region permission register 12."]
    pub core_0_region_pms_constrain_12: CORE_0_REGION_PMS_CONSTRAIN_12,
    #[doc = "0x194 - Core0 region permission register 13."]
    pub core_0_region_pms_constrain_13: CORE_0_REGION_PMS_CONSTRAIN_13,
    #[doc = "0x198 - Core0 region permission register 14."]
    pub core_0_region_pms_constrain_14: CORE_0_REGION_PMS_CONSTRAIN_14,
    #[doc = "0x19c - Core0 permission report register 0."]
    pub core_0_pif_pms_monitor_0: CORE_0_PIF_PMS_MONITOR_0,
    #[doc = "0x1a0 - Core0 permission report register 1."]
    pub core_0_pif_pms_monitor_1: CORE_0_PIF_PMS_MONITOR_1,
    #[doc = "0x1a4 - Core0 permission report register 2."]
    pub core_0_pif_pms_monitor_2: CORE_0_PIF_PMS_MONITOR_2,
    #[doc = "0x1a8 - Core0 permission report register 3."]
    pub core_0_pif_pms_monitor_3: CORE_0_PIF_PMS_MONITOR_3,
    #[doc = "0x1ac - Core0 permission report register 4."]
    pub core_0_pif_pms_monitor_4: CORE_0_PIF_PMS_MONITOR_4,
    #[doc = "0x1b0 - Core0 permission report register 5."]
    pub core_0_pif_pms_monitor_5: CORE_0_PIF_PMS_MONITOR_5,
    #[doc = "0x1b4 - Core0 permission report register 6."]
    pub core_0_pif_pms_monitor_6: CORE_0_PIF_PMS_MONITOR_6,
    #[doc = "0x1b8 - core0 vecbase override configuration register 0"]
    pub core_0_vecbase_override_lock: CORE_0_VECBASE_OVERRIDE_LOCK,
    #[doc = "0x1bc - core0 vecbase override configuration register 0"]
    pub core_0_vecbase_override_0: CORE_0_VECBASE_OVERRIDE_0,
    #[doc = "0x1c0 - core0 vecbase override configuration register 1"]
    pub core_0_vecbase_override_1: CORE_0_VECBASE_OVERRIDE_1,
    #[doc = "0x1c4 - core0 vecbase override configuration register 1"]
    pub core_0_vecbase_override_2: CORE_0_VECBASE_OVERRIDE_2,
    #[doc = "0x1c8 - core0 toomanyexception override configuration register 0."]
    pub core_0_toomanyexceptions_m_override_0: CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0,
    #[doc = "0x1cc - core0 toomanyexception override configuration register 1."]
    pub core_0_toomanyexceptions_m_override_1: CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1,
    #[doc = "0x1d0 - Core1 access peripherals permission configuration register 0."]
    pub core_1_pif_pms_constrain_0: CORE_1_PIF_PMS_CONSTRAIN_0,
    #[doc = "0x1d4 - Core1 access peripherals permission configuration register 1."]
    pub core_1_pif_pms_constrain_1: CORE_1_PIF_PMS_CONSTRAIN_1,
    #[doc = "0x1d8 - Core1 access peripherals permission configuration register 2."]
    pub core_1_pif_pms_constrain_2: CORE_1_PIF_PMS_CONSTRAIN_2,
    #[doc = "0x1dc - Core1 access peripherals permission configuration register 3."]
    pub core_1_pif_pms_constrain_3: CORE_1_PIF_PMS_CONSTRAIN_3,
    #[doc = "0x1e0 - Core1 access peripherals permission configuration register 4."]
    pub core_1_pif_pms_constrain_4: CORE_1_PIF_PMS_CONSTRAIN_4,
    #[doc = "0x1e4 - Core1 access peripherals permission configuration register 5."]
    pub core_1_pif_pms_constrain_5: CORE_1_PIF_PMS_CONSTRAIN_5,
    #[doc = "0x1e8 - Core1 access peripherals permission configuration register 6."]
    pub core_1_pif_pms_constrain_6: CORE_1_PIF_PMS_CONSTRAIN_6,
    #[doc = "0x1ec - Core1 access peripherals permission configuration register 7."]
    pub core_1_pif_pms_constrain_7: CORE_1_PIF_PMS_CONSTRAIN_7,
    #[doc = "0x1f0 - Core1 access peripherals permission configuration register 8."]
    pub core_1_pif_pms_constrain_8: CORE_1_PIF_PMS_CONSTRAIN_8,
    #[doc = "0x1f4 - Core1 access peripherals permission configuration register 9."]
    pub core_1_pif_pms_constrain_9: CORE_1_PIF_PMS_CONSTRAIN_9,
    #[doc = "0x1f8 - core1 access peripherals permission configuration register 10."]
    pub core_1_pif_pms_constrain_10: CORE_1_PIF_PMS_CONSTRAIN_10,
    #[doc = "0x1fc - core1 access peripherals permission configuration register 11."]
    pub core_1_pif_pms_constrain_11: CORE_1_PIF_PMS_CONSTRAIN_11,
    #[doc = "0x200 - core1 access peripherals permission configuration register 12."]
    pub core_1_pif_pms_constrain_12: CORE_1_PIF_PMS_CONSTRAIN_12,
    #[doc = "0x204 - core1 access peripherals permission configuration register 13."]
    pub core_1_pif_pms_constrain_13: CORE_1_PIF_PMS_CONSTRAIN_13,
    #[doc = "0x208 - core1 access peripherals permission configuration register 14."]
    pub core_1_pif_pms_constrain_14: CORE_1_PIF_PMS_CONSTRAIN_14,
    #[doc = "0x20c - core1 region permission register 0."]
    pub core_1_region_pms_constrain_0: CORE_1_REGION_PMS_CONSTRAIN_0,
    #[doc = "0x210 - core1 region permission register 1."]
    pub core_1_region_pms_constrain_1: CORE_1_REGION_PMS_CONSTRAIN_1,
    #[doc = "0x214 - core1 region permission register 2."]
    pub core_1_region_pms_constrain_2: CORE_1_REGION_PMS_CONSTRAIN_2,
    #[doc = "0x218 - core1 region permission register 3."]
    pub core_1_region_pms_constrain_3: CORE_1_REGION_PMS_CONSTRAIN_3,
    #[doc = "0x21c - core1 region permission register 4."]
    pub core_1_region_pms_constrain_4: CORE_1_REGION_PMS_CONSTRAIN_4,
    #[doc = "0x220 - core1 region permission register 5."]
    pub core_1_region_pms_constrain_5: CORE_1_REGION_PMS_CONSTRAIN_5,
    #[doc = "0x224 - core1 region permission register 6."]
    pub core_1_region_pms_constrain_6: CORE_1_REGION_PMS_CONSTRAIN_6,
    #[doc = "0x228 - core1 region permission register 7."]
    pub core_1_region_pms_constrain_7: CORE_1_REGION_PMS_CONSTRAIN_7,
    #[doc = "0x22c - core1 region permission register 8."]
    pub core_1_region_pms_constrain_8: CORE_1_REGION_PMS_CONSTRAIN_8,
    #[doc = "0x230 - core1 region permission register 9."]
    pub core_1_region_pms_constrain_9: CORE_1_REGION_PMS_CONSTRAIN_9,
    #[doc = "0x234 - core1 region permission register 10."]
    pub core_1_region_pms_constrain_10: CORE_1_REGION_PMS_CONSTRAIN_10,
    #[doc = "0x238 - core1 region permission register 11."]
    pub core_1_region_pms_constrain_11: CORE_1_REGION_PMS_CONSTRAIN_11,
    #[doc = "0x23c - core1 region permission register 12."]
    pub core_1_region_pms_constrain_12: CORE_1_REGION_PMS_CONSTRAIN_12,
    #[doc = "0x240 - core1 region permission register 13."]
    pub core_1_region_pms_constrain_13: CORE_1_REGION_PMS_CONSTRAIN_13,
    #[doc = "0x244 - core1 region permission register 14."]
    pub core_1_region_pms_constrain_14: CORE_1_REGION_PMS_CONSTRAIN_14,
    #[doc = "0x248 - core1 permission report register 0."]
    pub core_1_pif_pms_monitor_0: CORE_1_PIF_PMS_MONITOR_0,
    #[doc = "0x24c - core1 permission report register 1."]
    pub core_1_pif_pms_monitor_1: CORE_1_PIF_PMS_MONITOR_1,
    #[doc = "0x250 - core1 permission report register 2."]
    pub core_1_pif_pms_monitor_2: CORE_1_PIF_PMS_MONITOR_2,
    #[doc = "0x254 - core1 permission report register 3."]
    pub core_1_pif_pms_monitor_3: CORE_1_PIF_PMS_MONITOR_3,
    #[doc = "0x258 - core1 permission report register 4."]
    pub core_1_pif_pms_monitor_4: CORE_1_PIF_PMS_MONITOR_4,
    #[doc = "0x25c - core1 permission report register 5."]
    pub core_1_pif_pms_monitor_5: CORE_1_PIF_PMS_MONITOR_5,
    #[doc = "0x260 - core1 permission report register 6."]
    pub core_1_pif_pms_monitor_6: CORE_1_PIF_PMS_MONITOR_6,
    #[doc = "0x264 - core1 vecbase override configuration register 0"]
    pub core_1_vecbase_override_lock: CORE_1_VECBASE_OVERRIDE_LOCK,
    #[doc = "0x268 - core1 vecbase override configuration register 0"]
    pub core_1_vecbase_override_0: CORE_1_VECBASE_OVERRIDE_0,
    #[doc = "0x26c - core1 vecbase override configuration register 1"]
    pub core_1_vecbase_override_1: CORE_1_VECBASE_OVERRIDE_1,
    #[doc = "0x270 - core1 vecbase override configuration register 1"]
    pub core_1_vecbase_override_2: CORE_1_VECBASE_OVERRIDE_2,
    #[doc = "0x274 - core1 toomanyexception override configuration register 0."]
    pub core_1_toomanyexceptions_m_override_0: CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0,
    #[doc = "0x278 - core1 toomanyexception override configuration register 1."]
    pub core_1_toomanyexceptions_m_override_1: CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1,
    #[doc = "0x27c - BackUp access peripherals permission configuration register 0."]
    pub backup_bus_pms_constrain_0: BACKUP_BUS_PMS_CONSTRAIN_0,
    #[doc = "0x280 - BackUp access peripherals permission configuration register 1."]
    pub backup_bus_pms_constrain_1: BACKUP_BUS_PMS_CONSTRAIN_1,
    #[doc = "0x284 - BackUp access peripherals permission configuration register 2."]
    pub backup_bus_pms_constrain_2: BACKUP_BUS_PMS_CONSTRAIN_2,
    #[doc = "0x288 - BackUp access peripherals permission configuration register 3."]
    pub backup_bus_pms_constrain_3: BACKUP_BUS_PMS_CONSTRAIN_3,
    #[doc = "0x28c - BackUp access peripherals permission configuration register 4."]
    pub backup_bus_pms_constrain_4: BACKUP_BUS_PMS_CONSTRAIN_4,
    #[doc = "0x290 - BackUp access peripherals permission configuration register 5."]
    pub backup_bus_pms_constrain_5: BACKUP_BUS_PMS_CONSTRAIN_5,
    #[doc = "0x294 - BackUp access peripherals permission configuration register 6."]
    pub backup_bus_pms_constrain_6: BACKUP_BUS_PMS_CONSTRAIN_6,
    #[doc = "0x298 - BackUp permission report register 0."]
    pub backup_bus_pms_monitor_0: BACKUP_BUS_PMS_MONITOR_0,
    #[doc = "0x29c - BackUp permission report register 1."]
    pub backup_bus_pms_monitor_1: BACKUP_BUS_PMS_MONITOR_1,
    #[doc = "0x2a0 - BackUp permission report register 2."]
    pub backup_bus_pms_monitor_2: BACKUP_BUS_PMS_MONITOR_2,
    #[doc = "0x2a4 - BackUp permission report register 3."]
    pub backup_bus_pms_monitor_3: BACKUP_BUS_PMS_MONITOR_3,
    #[doc = "0x2a8 - EDMA boundary lock register."]
    pub edma_boundary_lock: EDMA_BOUNDARY_LOCK,
    #[doc = "0x2ac - EDMA boundary 0 configuration"]
    pub edma_boundary_0: EDMA_BOUNDARY_0,
    #[doc = "0x2b0 - EDMA boundary 1 configuration"]
    pub edma_boundary_1: EDMA_BOUNDARY_1,
    #[doc = "0x2b4 - EDMA boundary 2 configuration"]
    pub edma_boundary_2: EDMA_BOUNDARY_2,
    #[doc = "0x2b8 - EDMA-SPI2 permission lock register."]
    pub edma_pms_spi2_lock: EDMA_PMS_SPI2_LOCK,
    #[doc = "0x2bc - EDMA-SPI2 permission control register."]
    pub edma_pms_spi2: EDMA_PMS_SPI2,
    #[doc = "0x2c0 - EDMA-SPI3 permission lock register."]
    pub edma_pms_spi3_lock: EDMA_PMS_SPI3_LOCK,
    #[doc = "0x2c4 - EDMA-SPI3 permission control register."]
    pub edma_pms_spi3: EDMA_PMS_SPI3,
    #[doc = "0x2c8 - EDMA-UHCI0 permission lock register."]
    pub edma_pms_uhci0_lock: EDMA_PMS_UHCI0_LOCK,
    #[doc = "0x2cc - EDMA-UHCI0 permission control register."]
    pub edma_pms_uhci0: EDMA_PMS_UHCI0,
    #[doc = "0x2d0 - EDMA-I2S0 permission lock register."]
    pub edma_pms_i2s0_lock: EDMA_PMS_I2S0_LOCK,
    #[doc = "0x2d4 - EDMA-I2S0 permission control register."]
    pub edma_pms_i2s0: EDMA_PMS_I2S0,
    #[doc = "0x2d8 - EDMA-I2S1 permission lock register."]
    pub edma_pms_i2s1_lock: EDMA_PMS_I2S1_LOCK,
    #[doc = "0x2dc - EDMA-I2S1 permission control register."]
    pub edma_pms_i2s1: EDMA_PMS_I2S1,
    #[doc = "0x2e0 - EDMA-LCD/CAM permission lock register."]
    pub edma_pms_lcd_cam_lock: EDMA_PMS_LCD_CAM_LOCK,
    #[doc = "0x2e4 - EDMA-LCD/CAM permission control register."]
    pub edma_pms_lcd_cam: EDMA_PMS_LCD_CAM,
    #[doc = "0x2e8 - EDMA-AES permission lock register."]
    pub edma_pms_aes_lock: EDMA_PMS_AES_LOCK,
    #[doc = "0x2ec - EDMA-AES permission control register."]
    pub edma_pms_aes: EDMA_PMS_AES,
    #[doc = "0x2f0 - EDMA-SHA permission lock register."]
    pub edma_pms_sha_lock: EDMA_PMS_SHA_LOCK,
    #[doc = "0x2f4 - EDMA-SHA permission control register."]
    pub edma_pms_sha: EDMA_PMS_SHA,
    #[doc = "0x2f8 - EDMA-ADC/DAC permission lock register."]
    pub edma_pms_adc_dac_lock: EDMA_PMS_ADC_DAC_LOCK,
    #[doc = "0x2fc - EDMA-ADC/DAC permission control register."]
    pub edma_pms_adc_dac: EDMA_PMS_ADC_DAC,
    #[doc = "0x300 - EDMA-RMT permission lock register."]
    pub edma_pms_rmt_lock: EDMA_PMS_RMT_LOCK,
    #[doc = "0x304 - EDMA-RMT permission control register."]
    pub edma_pms_rmt: EDMA_PMS_RMT,
    #[doc = "0x308 - Sensitive module clock gate configuration register."]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x30c - RTC coprocessor permission register."]
    pub rtc_pms: RTC_PMS,
    _reserved196: [u8; 0x0cec],
    #[doc = "0xffc - Sensitive version register."]
    pub date: DATE,
}
#[doc = "CACHE_DATAARRAY_CONNECT_0 (rw) register accessor: an alias for `Reg<CACHE_DATAARRAY_CONNECT_0_SPEC>`"]
pub type CACHE_DATAARRAY_CONNECT_0 =
    crate::Reg<cache_dataarray_connect_0::CACHE_DATAARRAY_CONNECT_0_SPEC>;
#[doc = "Cache data array configuration register 0."]
pub mod cache_dataarray_connect_0;
#[doc = "CACHE_DATAARRAY_CONNECT_1 (rw) register accessor: an alias for `Reg<CACHE_DATAARRAY_CONNECT_1_SPEC>`"]
pub type CACHE_DATAARRAY_CONNECT_1 =
    crate::Reg<cache_dataarray_connect_1::CACHE_DATAARRAY_CONNECT_1_SPEC>;
#[doc = "Cache data array configuration register 1."]
pub mod cache_dataarray_connect_1;
#[doc = "APB_PERIPHERAL_ACCESS_0 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_0_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_0 =
    crate::Reg<apb_peripheral_access_0::APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "APB peripheral configuration register 0."]
pub mod apb_peripheral_access_0;
#[doc = "APB_PERIPHERAL_ACCESS_1 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_1_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_1 =
    crate::Reg<apb_peripheral_access_1::APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "APB peripheral configuration register 1."]
pub mod apb_peripheral_access_1;
#[doc = "INTERNAL_SRAM_USAGE_0 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_0_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_0 = crate::Reg<internal_sram_usage_0::INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "Internal SRAM configuration register 0."]
pub mod internal_sram_usage_0;
#[doc = "INTERNAL_SRAM_USAGE_1 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_1_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_1 = crate::Reg<internal_sram_usage_1::INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "Internal SRAM configuration register 1."]
pub mod internal_sram_usage_1;
#[doc = "INTERNAL_SRAM_USAGE_2 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_2_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_2 = crate::Reg<internal_sram_usage_2::INTERNAL_SRAM_USAGE_2_SPEC>;
#[doc = "Internal SRAM configuration register 2."]
pub mod internal_sram_usage_2;
#[doc = "INTERNAL_SRAM_USAGE_3 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_3_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_3 = crate::Reg<internal_sram_usage_3::INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Internal SRAM configuration register 3."]
pub mod internal_sram_usage_3;
#[doc = "INTERNAL_SRAM_USAGE_4 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_4_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_4 = crate::Reg<internal_sram_usage_4::INTERNAL_SRAM_USAGE_4_SPEC>;
#[doc = "Internal SRAM configuration register 4."]
pub mod internal_sram_usage_4;
#[doc = "RETENTION_DISABLE (rw) register accessor: an alias for `Reg<RETENTION_DISABLE_SPEC>`"]
pub type RETENTION_DISABLE = crate::Reg<retention_disable::RETENTION_DISABLE_SPEC>;
#[doc = "Retention configuration register."]
pub mod retention_disable;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_0_SPEC>`"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "Cache tag configuration register 0."]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_1_SPEC>`"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "Cache tag configuration register 1."]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_0_SPEC>`"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "Cache MMU configuration register 0."]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_1_SPEC>`"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "Cache MMU configuration register 1."]
pub mod cache_mmu_access_1;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_0::DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>;
#[doc = "spi2 dma permission configuration register 0."]
pub mod dma_apbperi_spi2_pms_constrain_0;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_1::DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>;
#[doc = "spi2 dma permission configuration register 1."]
pub mod dma_apbperi_spi2_pms_constrain_1;
#[doc = "DMA_APBPERI_SPI3_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI3_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_spi3_pms_constrain_0::DMA_APBPERI_SPI3_PMS_CONSTRAIN_0_SPEC>;
#[doc = "spi3 dma permission configuration register 0."]
pub mod dma_apbperi_spi3_pms_constrain_0;
#[doc = "DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_spi3_pms_constrain_1::DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC>;
#[doc = "spi3 dma permission configuration register 1."]
pub mod dma_apbperi_spi3_pms_constrain_1;
#[doc = "DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_uhci0_pms_constrain_0::DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "uhci0 dma permission configuration register 0."]
pub mod dma_apbperi_uhci0_pms_constrain_0;
#[doc = "DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_uhci0_pms_constrain_1::DMA_APBPERI_UHCI0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "uhci0 dma permission configuration register 1."]
pub mod dma_apbperi_uhci0_pms_constrain_1;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_0::DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "i2s0 dma permission configuration register 0."]
pub mod dma_apbperi_i2s0_pms_constrain_0;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_1::DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "i2s0 dma permission configuration register 1."]
pub mod dma_apbperi_i2s0_pms_constrain_1;
#[doc = "DMA_APBPERI_I2S1_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S1_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_I2S1_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_i2s1_pms_constrain_0::DMA_APBPERI_I2S1_PMS_CONSTRAIN_0_SPEC>;
#[doc = "i2s1 dma permission configuration register 0."]
pub mod dma_apbperi_i2s1_pms_constrain_0;
#[doc = "DMA_APBPERI_I2S1_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S1_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_I2S1_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_i2s1_pms_constrain_1::DMA_APBPERI_I2S1_PMS_CONSTRAIN_1_SPEC>;
#[doc = "i2s1 dma permission configuration register 1."]
pub mod dma_apbperi_i2s1_pms_constrain_1;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_MAC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_mac_pms_constrain_0::DMA_APBPERI_MAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "mac dma permission configuration register 0."]
pub mod dma_apbperi_mac_pms_constrain_0;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_MAC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_mac_pms_constrain_1::DMA_APBPERI_MAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "mac dma permission configuration register 1."]
pub mod dma_apbperi_mac_pms_constrain_1;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_backup_pms_constrain_0::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_SPEC>;
#[doc = "backup dma permission configuration register 0."]
pub mod dma_apbperi_backup_pms_constrain_0;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_backup_pms_constrain_1::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_SPEC>;
#[doc = "backup dma permission configuration register 1."]
pub mod dma_apbperi_backup_pms_constrain_1;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_AES_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_aes_pms_constrain_0::DMA_APBPERI_AES_PMS_CONSTRAIN_0_SPEC>;
#[doc = "aes dma permission configuration register 0."]
pub mod dma_apbperi_aes_pms_constrain_0;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_AES_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_aes_pms_constrain_1::DMA_APBPERI_AES_PMS_CONSTRAIN_1_SPEC>;
#[doc = "aes dma permission configuration register 1."]
pub mod dma_apbperi_aes_pms_constrain_1;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SHA_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_sha_pms_constrain_0::DMA_APBPERI_SHA_PMS_CONSTRAIN_0_SPEC>;
#[doc = "sha dma permission configuration register 0."]
pub mod dma_apbperi_sha_pms_constrain_0;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_sha_pms_constrain_1::DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>;
#[doc = "sha dma permission configuration register 1."]
pub mod dma_apbperi_sha_pms_constrain_1;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_0::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "adc_dac dma permission configuration register 0."]
pub mod dma_apbperi_adc_dac_pms_constrain_0;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_1::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "adc_dac dma permission configuration register 1."]
pub mod dma_apbperi_adc_dac_pms_constrain_1;
#[doc = "DMA_APBPERI_RMT_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_RMT_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_RMT_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_rmt_pms_constrain_0::DMA_APBPERI_RMT_PMS_CONSTRAIN_0_SPEC>;
#[doc = "rmt dma permission configuration register 0."]
pub mod dma_apbperi_rmt_pms_constrain_0;
#[doc = "DMA_APBPERI_RMT_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_RMT_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_RMT_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_rmt_pms_constrain_1::DMA_APBPERI_RMT_PMS_CONSTRAIN_1_SPEC>;
#[doc = "rmt dma permission configuration register 1."]
pub mod dma_apbperi_rmt_pms_constrain_1;
#[doc = "DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_lcd_cam_pms_constrain_0::DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_0_SPEC>;
#[doc = "lcd_cam dma permission configuration register 0."]
pub mod dma_apbperi_lcd_cam_pms_constrain_0;
#[doc = "DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_lcd_cam_pms_constrain_1::DMA_APBPERI_LCD_CAM_PMS_CONSTRAIN_1_SPEC>;
#[doc = "lcd_cam dma permission configuration register 1."]
pub mod dma_apbperi_lcd_cam_pms_constrain_1;
#[doc = "DMA_APBPERI_USB_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_USB_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_USB_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_usb_pms_constrain_0::DMA_APBPERI_USB_PMS_CONSTRAIN_0_SPEC>;
#[doc = "usb dma permission configuration register 0."]
pub mod dma_apbperi_usb_pms_constrain_0;
#[doc = "DMA_APBPERI_USB_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_USB_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_USB_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_usb_pms_constrain_1::DMA_APBPERI_USB_PMS_CONSTRAIN_1_SPEC>;
#[doc = "usb dma permission configuration register 1."]
pub mod dma_apbperi_usb_pms_constrain_1;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_lc_pms_constrain_0::DMA_APBPERI_LC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "lc dma permission configuration register 0."]
pub mod dma_apbperi_lc_pms_constrain_0;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_lc_pms_constrain_1::DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "lc dma permission configuration register 1."]
pub mod dma_apbperi_lc_pms_constrain_1;
#[doc = "DMA_APBPERI_SDIO_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SDIO_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SDIO_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_sdio_pms_constrain_0::DMA_APBPERI_SDIO_PMS_CONSTRAIN_0_SPEC>;
#[doc = "sdio dma permission configuration register 0."]
pub mod dma_apbperi_sdio_pms_constrain_0;
#[doc = "DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_sdio_pms_constrain_1::DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>;
#[doc = "sdio dma permission configuration register 1."]
pub mod dma_apbperi_sdio_pms_constrain_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_0_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_0 =
    crate::Reg<dma_apbperi_pms_monitor_0::DMA_APBPERI_PMS_MONITOR_0_SPEC>;
#[doc = "dma permission monitor configuration register 0."]
pub mod dma_apbperi_pms_monitor_0;
#[doc = "DMA_APBPERI_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_1_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_1 =
    crate::Reg<dma_apbperi_pms_monitor_1::DMA_APBPERI_PMS_MONITOR_1_SPEC>;
#[doc = "dma permission monitor configuration register 1."]
pub mod dma_apbperi_pms_monitor_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_2_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_2 =
    crate::Reg<dma_apbperi_pms_monitor_2::DMA_APBPERI_PMS_MONITOR_2_SPEC>;
#[doc = "dma permission monitor configuration register 2."]
pub mod dma_apbperi_pms_monitor_2;
#[doc = "DMA_APBPERI_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_3_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_3 =
    crate::Reg<dma_apbperi_pms_monitor_3::DMA_APBPERI_PMS_MONITOR_3_SPEC>;
#[doc = "dma permission monitor configuration register 3."]
pub mod dma_apbperi_pms_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_0 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_SPEC > ;
#[doc = "sram split line configuration register 0"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_0;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_1 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_1;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_2 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_2;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_3 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_4 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_4;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_5 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC > ;
#[doc = "sram split line configuration register 1"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_5;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_iram0_pms_constrain_0::CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "corex iram0 permission configuration register 0"]
pub mod core_x_iram0_pms_constrain_0;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_iram0_pms_constrain_1::CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "corex iram0 permission configuration register 0"]
pub mod core_x_iram0_pms_constrain_1;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_2 =
    crate::Reg<core_x_iram0_pms_constrain_2::CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>;
#[doc = "corex iram0 permission configuration register 1"]
pub mod core_x_iram0_pms_constrain_2;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_iram0_pms_monitor_0::CORE_0_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 0"]
pub mod core_0_iram0_pms_monitor_0;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_iram0_pms_monitor_1::CORE_0_IRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 1"]
pub mod core_0_iram0_pms_monitor_1;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_iram0_pms_monitor_2::CORE_0_IRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core0 iram0 permission monitor configuration register 2"]
pub mod core_0_iram0_pms_monitor_2;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_1_IRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_1_IRAM0_PMS_MONITOR_0 =
    crate::Reg<core_1_iram0_pms_monitor_0::CORE_1_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 0"]
pub mod core_1_iram0_pms_monitor_0;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_1_IRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_1_IRAM0_PMS_MONITOR_1 =
    crate::Reg<core_1_iram0_pms_monitor_1::CORE_1_IRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 1"]
pub mod core_1_iram0_pms_monitor_1;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_1_IRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_1_IRAM0_PMS_MONITOR_2 =
    crate::Reg<core_1_iram0_pms_monitor_2::CORE_1_IRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core1 iram0 permission monitor configuration register 2"]
pub mod core_1_iram0_pms_monitor_2;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_dram0_pms_constrain_0::CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "corex dram0 permission configuration register 0"]
pub mod core_x_dram0_pms_constrain_0;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_dram0_pms_constrain_1::CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "corex dram0 permission configuration register 1"]
pub mod core_x_dram0_pms_constrain_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_dram0_pms_monitor_0::CORE_0_DRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 0"]
pub mod core_0_dram0_pms_monitor_0;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_dram0_pms_monitor_1::CORE_0_DRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 1"]
pub mod core_0_dram0_pms_monitor_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_dram0_pms_monitor_2::CORE_0_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 2."]
pub mod core_0_dram0_pms_monitor_2;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_3 =
    crate::Reg<core_0_dram0_pms_monitor_3::CORE_0_DRAM0_PMS_MONITOR_3_SPEC>;
#[doc = "core0 dram0 permission monitor configuration register 3."]
pub mod core_0_dram0_pms_monitor_3;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_1_DRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_1_DRAM0_PMS_MONITOR_0 =
    crate::Reg<core_1_dram0_pms_monitor_0::CORE_1_DRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 0"]
pub mod core_1_dram0_pms_monitor_0;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_1_DRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_1_DRAM0_PMS_MONITOR_1 =
    crate::Reg<core_1_dram0_pms_monitor_1::CORE_1_DRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 1"]
pub mod core_1_dram0_pms_monitor_1;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_1_DRAM0_PMS_MONITOR_2 =
    crate::Reg<core_1_dram0_pms_monitor_2::CORE_1_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 2."]
pub mod core_1_dram0_pms_monitor_2;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_1_DRAM0_PMS_MONITOR_3_SPEC>`"]
pub type CORE_1_DRAM0_PMS_MONITOR_3 =
    crate::Reg<core_1_dram0_pms_monitor_3::CORE_1_DRAM0_PMS_MONITOR_3_SPEC>;
#[doc = "core1 dram0 permission monitor configuration register 3."]
pub mod core_1_dram0_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<core_0_pif_pms_constrain_0::CORE_0_PIF_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 0."]
pub mod core_0_pif_pms_constrain_0;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<core_0_pif_pms_constrain_1::CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 1."]
pub mod core_0_pif_pms_constrain_1;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<core_0_pif_pms_constrain_2::CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 2."]
pub mod core_0_pif_pms_constrain_2;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<core_0_pif_pms_constrain_3::CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 3."]
pub mod core_0_pif_pms_constrain_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<core_0_pif_pms_constrain_4::CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 4."]
pub mod core_0_pif_pms_constrain_4;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<core_0_pif_pms_constrain_5::CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 5."]
pub mod core_0_pif_pms_constrain_5;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<core_0_pif_pms_constrain_6::CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 6."]
pub mod core_0_pif_pms_constrain_6;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<core_0_pif_pms_constrain_7::CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 7."]
pub mod core_0_pif_pms_constrain_7;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<core_0_pif_pms_constrain_8::CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 8."]
pub mod core_0_pif_pms_constrain_8;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<core_0_pif_pms_constrain_9::CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 9."]
pub mod core_0_pif_pms_constrain_9;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<core_0_pif_pms_constrain_10::CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 10."]
pub mod core_0_pif_pms_constrain_10;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_11 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_11_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_11 =
    crate::Reg<core_0_pif_pms_constrain_11::CORE_0_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 11."]
pub mod core_0_pif_pms_constrain_11;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_12 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_12_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_12 =
    crate::Reg<core_0_pif_pms_constrain_12::CORE_0_PIF_PMS_CONSTRAIN_12_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 12."]
pub mod core_0_pif_pms_constrain_12;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_13 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_13 =
    crate::Reg<core_0_pif_pms_constrain_13::CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 13."]
pub mod core_0_pif_pms_constrain_13;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_14 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_14_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_14 =
    crate::Reg<core_0_pif_pms_constrain_14::CORE_0_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Core0 access peripherals permission configuration register 14."]
pub mod core_0_pif_pms_constrain_14;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_0 =
    crate::Reg<core_0_region_pms_constrain_0::CORE_0_REGION_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core0 region permission register 0."]
pub mod core_0_region_pms_constrain_0;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_1 =
    crate::Reg<core_0_region_pms_constrain_1::CORE_0_REGION_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core0 region permission register 1."]
pub mod core_0_region_pms_constrain_1;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_2 =
    crate::Reg<core_0_region_pms_constrain_2::CORE_0_REGION_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core0 region permission register 2."]
pub mod core_0_region_pms_constrain_2;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_3 =
    crate::Reg<core_0_region_pms_constrain_3::CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core0 region permission register 3."]
pub mod core_0_region_pms_constrain_3;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_4_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_4 =
    crate::Reg<core_0_region_pms_constrain_4::CORE_0_REGION_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core0 region permission register 4."]
pub mod core_0_region_pms_constrain_4;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_5 =
    crate::Reg<core_0_region_pms_constrain_5::CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core0 region permission register 5."]
pub mod core_0_region_pms_constrain_5;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_6_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_6 =
    crate::Reg<core_0_region_pms_constrain_6::CORE_0_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core0 region permission register 6."]
pub mod core_0_region_pms_constrain_6;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_7_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_7 =
    crate::Reg<core_0_region_pms_constrain_7::CORE_0_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core0 region permission register 7."]
pub mod core_0_region_pms_constrain_7;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_8_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_8 =
    crate::Reg<core_0_region_pms_constrain_8::CORE_0_REGION_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core0 region permission register 8."]
pub mod core_0_region_pms_constrain_8;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_9 =
    crate::Reg<core_0_region_pms_constrain_9::CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core0 region permission register 9."]
pub mod core_0_region_pms_constrain_9;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_10 =
    crate::Reg<core_0_region_pms_constrain_10::CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>;
#[doc = "Core0 region permission register 10."]
pub mod core_0_region_pms_constrain_10;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_11 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_11_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_11 =
    crate::Reg<core_0_region_pms_constrain_11::CORE_0_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Core0 region permission register 11."]
pub mod core_0_region_pms_constrain_11;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_12 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_12_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_12 =
    crate::Reg<core_0_region_pms_constrain_12::CORE_0_REGION_PMS_CONSTRAIN_12_SPEC>;
#[doc = "Core0 region permission register 12."]
pub mod core_0_region_pms_constrain_12;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_13 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_13_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_13 =
    crate::Reg<core_0_region_pms_constrain_13::CORE_0_REGION_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Core0 region permission register 13."]
pub mod core_0_region_pms_constrain_13;
#[doc = "CORE_0_REGION_PMS_CONSTRAIN_14 (rw) register accessor: an alias for `Reg<CORE_0_REGION_PMS_CONSTRAIN_14_SPEC>`"]
pub type CORE_0_REGION_PMS_CONSTRAIN_14 =
    crate::Reg<core_0_region_pms_constrain_14::CORE_0_REGION_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Core0 region permission register 14."]
pub mod core_0_region_pms_constrain_14;
#[doc = "CORE_0_PIF_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_0 =
    crate::Reg<core_0_pif_pms_monitor_0::CORE_0_PIF_PMS_MONITOR_0_SPEC>;
#[doc = "Core0 permission report register 0."]
pub mod core_0_pif_pms_monitor_0;
#[doc = "CORE_0_PIF_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_1 =
    crate::Reg<core_0_pif_pms_monitor_1::CORE_0_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "Core0 permission report register 1."]
pub mod core_0_pif_pms_monitor_1;
#[doc = "CORE_0_PIF_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_2 =
    crate::Reg<core_0_pif_pms_monitor_2::CORE_0_PIF_PMS_MONITOR_2_SPEC>;
#[doc = "Core0 permission report register 2."]
pub mod core_0_pif_pms_monitor_2;
#[doc = "CORE_0_PIF_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_3_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_3 =
    crate::Reg<core_0_pif_pms_monitor_3::CORE_0_PIF_PMS_MONITOR_3_SPEC>;
#[doc = "Core0 permission report register 3."]
pub mod core_0_pif_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_MONITOR_4 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_4_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_4 =
    crate::Reg<core_0_pif_pms_monitor_4::CORE_0_PIF_PMS_MONITOR_4_SPEC>;
#[doc = "Core0 permission report register 4."]
pub mod core_0_pif_pms_monitor_4;
#[doc = "CORE_0_PIF_PMS_MONITOR_5 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_5_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_5 =
    crate::Reg<core_0_pif_pms_monitor_5::CORE_0_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "Core0 permission report register 5."]
pub mod core_0_pif_pms_monitor_5;
#[doc = "CORE_0_PIF_PMS_MONITOR_6 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_6_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_6 =
    crate::Reg<core_0_pif_pms_monitor_6::CORE_0_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "Core0 permission report register 6."]
pub mod core_0_pif_pms_monitor_6;
#[doc = "CORE_0_VECBASE_OVERRIDE_LOCK (rw) register accessor: an alias for `Reg<CORE_0_VECBASE_OVERRIDE_LOCK_SPEC>`"]
pub type CORE_0_VECBASE_OVERRIDE_LOCK =
    crate::Reg<core_0_vecbase_override_lock::CORE_0_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "core0 vecbase override configuration register 0"]
pub mod core_0_vecbase_override_lock;
#[doc = "CORE_0_VECBASE_OVERRIDE_0 (rw) register accessor: an alias for `Reg<CORE_0_VECBASE_OVERRIDE_0_SPEC>`"]
pub type CORE_0_VECBASE_OVERRIDE_0 =
    crate::Reg<core_0_vecbase_override_0::CORE_0_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "core0 vecbase override configuration register 0"]
pub mod core_0_vecbase_override_0;
#[doc = "CORE_0_VECBASE_OVERRIDE_1 (rw) register accessor: an alias for `Reg<CORE_0_VECBASE_OVERRIDE_1_SPEC>`"]
pub type CORE_0_VECBASE_OVERRIDE_1 =
    crate::Reg<core_0_vecbase_override_1::CORE_0_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "core0 vecbase override configuration register 1"]
pub mod core_0_vecbase_override_1;
#[doc = "CORE_0_VECBASE_OVERRIDE_2 (rw) register accessor: an alias for `Reg<CORE_0_VECBASE_OVERRIDE_2_SPEC>`"]
pub type CORE_0_VECBASE_OVERRIDE_2 =
    crate::Reg<core_0_vecbase_override_2::CORE_0_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "core0 vecbase override configuration register 1"]
pub mod core_0_vecbase_override_2;
#[doc = "CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0 (rw) register accessor: an alias for `Reg<CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>`"]
pub type CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0 =
    crate::Reg<core_0_toomanyexceptions_m_override_0::CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>;
#[doc = "core0 toomanyexception override configuration register 0."]
pub mod core_0_toomanyexceptions_m_override_0;
#[doc = "CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1 (rw) register accessor: an alias for `Reg<CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>`"]
pub type CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1 =
    crate::Reg<core_0_toomanyexceptions_m_override_1::CORE_0_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "core0 toomanyexception override configuration register 1."]
pub mod core_0_toomanyexceptions_m_override_1;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<core_1_pif_pms_constrain_0::CORE_1_PIF_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 0."]
pub mod core_1_pif_pms_constrain_0;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<core_1_pif_pms_constrain_1::CORE_1_PIF_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 1."]
pub mod core_1_pif_pms_constrain_1;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<core_1_pif_pms_constrain_2::CORE_1_PIF_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 2."]
pub mod core_1_pif_pms_constrain_2;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<core_1_pif_pms_constrain_3::CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 3."]
pub mod core_1_pif_pms_constrain_3;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<core_1_pif_pms_constrain_4::CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 4."]
pub mod core_1_pif_pms_constrain_4;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<core_1_pif_pms_constrain_5::CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 5."]
pub mod core_1_pif_pms_constrain_5;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<core_1_pif_pms_constrain_6::CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 6."]
pub mod core_1_pif_pms_constrain_6;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_7_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<core_1_pif_pms_constrain_7::CORE_1_PIF_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 7."]
pub mod core_1_pif_pms_constrain_7;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_8_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<core_1_pif_pms_constrain_8::CORE_1_PIF_PMS_CONSTRAIN_8_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 8."]
pub mod core_1_pif_pms_constrain_8;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_9_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<core_1_pif_pms_constrain_9::CORE_1_PIF_PMS_CONSTRAIN_9_SPEC>;
#[doc = "Core1 access peripherals permission configuration register 9."]
pub mod core_1_pif_pms_constrain_9;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_10_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<core_1_pif_pms_constrain_10::CORE_1_PIF_PMS_CONSTRAIN_10_SPEC>;
#[doc = "core1 access peripherals permission configuration register 10."]
pub mod core_1_pif_pms_constrain_10;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_11 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_11 =
    crate::Reg<core_1_pif_pms_constrain_11::CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "core1 access peripherals permission configuration register 11."]
pub mod core_1_pif_pms_constrain_11;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_12 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_12_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_12 =
    crate::Reg<core_1_pif_pms_constrain_12::CORE_1_PIF_PMS_CONSTRAIN_12_SPEC>;
#[doc = "core1 access peripherals permission configuration register 12."]
pub mod core_1_pif_pms_constrain_12;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_13 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_13 =
    crate::Reg<core_1_pif_pms_constrain_13::CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "core1 access peripherals permission configuration register 13."]
pub mod core_1_pif_pms_constrain_13;
#[doc = "CORE_1_PIF_PMS_CONSTRAIN_14 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>`"]
pub type CORE_1_PIF_PMS_CONSTRAIN_14 =
    crate::Reg<core_1_pif_pms_constrain_14::CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "core1 access peripherals permission configuration register 14."]
pub mod core_1_pif_pms_constrain_14;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_0 =
    crate::Reg<core_1_region_pms_constrain_0::CORE_1_REGION_PMS_CONSTRAIN_0_SPEC>;
#[doc = "core1 region permission register 0."]
pub mod core_1_region_pms_constrain_0;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_1 =
    crate::Reg<core_1_region_pms_constrain_1::CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>;
#[doc = "core1 region permission register 1."]
pub mod core_1_region_pms_constrain_1;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_2 =
    crate::Reg<core_1_region_pms_constrain_2::CORE_1_REGION_PMS_CONSTRAIN_2_SPEC>;
#[doc = "core1 region permission register 2."]
pub mod core_1_region_pms_constrain_2;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_3_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_3 =
    crate::Reg<core_1_region_pms_constrain_3::CORE_1_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "core1 region permission register 3."]
pub mod core_1_region_pms_constrain_3;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_4_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_4 =
    crate::Reg<core_1_region_pms_constrain_4::CORE_1_REGION_PMS_CONSTRAIN_4_SPEC>;
#[doc = "core1 region permission register 4."]
pub mod core_1_region_pms_constrain_4;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_5_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_5 =
    crate::Reg<core_1_region_pms_constrain_5::CORE_1_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "core1 region permission register 5."]
pub mod core_1_region_pms_constrain_5;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_6 =
    crate::Reg<core_1_region_pms_constrain_6::CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "core1 region permission register 6."]
pub mod core_1_region_pms_constrain_6;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_7_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_7 =
    crate::Reg<core_1_region_pms_constrain_7::CORE_1_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "core1 region permission register 7."]
pub mod core_1_region_pms_constrain_7;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_8 =
    crate::Reg<core_1_region_pms_constrain_8::CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>;
#[doc = "core1 region permission register 8."]
pub mod core_1_region_pms_constrain_8;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_9 =
    crate::Reg<core_1_region_pms_constrain_9::CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>;
#[doc = "core1 region permission register 9."]
pub mod core_1_region_pms_constrain_9;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_10_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_10 =
    crate::Reg<core_1_region_pms_constrain_10::CORE_1_REGION_PMS_CONSTRAIN_10_SPEC>;
#[doc = "core1 region permission register 10."]
pub mod core_1_region_pms_constrain_10;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_11 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_11_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_11 =
    crate::Reg<core_1_region_pms_constrain_11::CORE_1_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "core1 region permission register 11."]
pub mod core_1_region_pms_constrain_11;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_12 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_12_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_12 =
    crate::Reg<core_1_region_pms_constrain_12::CORE_1_REGION_PMS_CONSTRAIN_12_SPEC>;
#[doc = "core1 region permission register 12."]
pub mod core_1_region_pms_constrain_12;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_13 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_13_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_13 =
    crate::Reg<core_1_region_pms_constrain_13::CORE_1_REGION_PMS_CONSTRAIN_13_SPEC>;
#[doc = "core1 region permission register 13."]
pub mod core_1_region_pms_constrain_13;
#[doc = "CORE_1_REGION_PMS_CONSTRAIN_14 (rw) register accessor: an alias for `Reg<CORE_1_REGION_PMS_CONSTRAIN_14_SPEC>`"]
pub type CORE_1_REGION_PMS_CONSTRAIN_14 =
    crate::Reg<core_1_region_pms_constrain_14::CORE_1_REGION_PMS_CONSTRAIN_14_SPEC>;
#[doc = "core1 region permission register 14."]
pub mod core_1_region_pms_constrain_14;
#[doc = "CORE_1_PIF_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_0_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_0 =
    crate::Reg<core_1_pif_pms_monitor_0::CORE_1_PIF_PMS_MONITOR_0_SPEC>;
#[doc = "core1 permission report register 0."]
pub mod core_1_pif_pms_monitor_0;
#[doc = "CORE_1_PIF_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_1_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_1 =
    crate::Reg<core_1_pif_pms_monitor_1::CORE_1_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "core1 permission report register 1."]
pub mod core_1_pif_pms_monitor_1;
#[doc = "CORE_1_PIF_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_2_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_2 =
    crate::Reg<core_1_pif_pms_monitor_2::CORE_1_PIF_PMS_MONITOR_2_SPEC>;
#[doc = "core1 permission report register 2."]
pub mod core_1_pif_pms_monitor_2;
#[doc = "CORE_1_PIF_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_3_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_3 =
    crate::Reg<core_1_pif_pms_monitor_3::CORE_1_PIF_PMS_MONITOR_3_SPEC>;
#[doc = "core1 permission report register 3."]
pub mod core_1_pif_pms_monitor_3;
#[doc = "CORE_1_PIF_PMS_MONITOR_4 (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_4_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_4 =
    crate::Reg<core_1_pif_pms_monitor_4::CORE_1_PIF_PMS_MONITOR_4_SPEC>;
#[doc = "core1 permission report register 4."]
pub mod core_1_pif_pms_monitor_4;
#[doc = "CORE_1_PIF_PMS_MONITOR_5 (r) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_5_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_5 =
    crate::Reg<core_1_pif_pms_monitor_5::CORE_1_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "core1 permission report register 5."]
pub mod core_1_pif_pms_monitor_5;
#[doc = "CORE_1_PIF_PMS_MONITOR_6 (r) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_6_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_6 =
    crate::Reg<core_1_pif_pms_monitor_6::CORE_1_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "core1 permission report register 6."]
pub mod core_1_pif_pms_monitor_6;
#[doc = "CORE_1_VECBASE_OVERRIDE_LOCK (rw) register accessor: an alias for `Reg<CORE_1_VECBASE_OVERRIDE_LOCK_SPEC>`"]
pub type CORE_1_VECBASE_OVERRIDE_LOCK =
    crate::Reg<core_1_vecbase_override_lock::CORE_1_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "core1 vecbase override configuration register 0"]
pub mod core_1_vecbase_override_lock;
#[doc = "CORE_1_VECBASE_OVERRIDE_0 (rw) register accessor: an alias for `Reg<CORE_1_VECBASE_OVERRIDE_0_SPEC>`"]
pub type CORE_1_VECBASE_OVERRIDE_0 =
    crate::Reg<core_1_vecbase_override_0::CORE_1_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "core1 vecbase override configuration register 0"]
pub mod core_1_vecbase_override_0;
#[doc = "CORE_1_VECBASE_OVERRIDE_1 (rw) register accessor: an alias for `Reg<CORE_1_VECBASE_OVERRIDE_1_SPEC>`"]
pub type CORE_1_VECBASE_OVERRIDE_1 =
    crate::Reg<core_1_vecbase_override_1::CORE_1_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "core1 vecbase override configuration register 1"]
pub mod core_1_vecbase_override_1;
#[doc = "CORE_1_VECBASE_OVERRIDE_2 (rw) register accessor: an alias for `Reg<CORE_1_VECBASE_OVERRIDE_2_SPEC>`"]
pub type CORE_1_VECBASE_OVERRIDE_2 =
    crate::Reg<core_1_vecbase_override_2::CORE_1_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "core1 vecbase override configuration register 1"]
pub mod core_1_vecbase_override_2;
#[doc = "CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0 (rw) register accessor: an alias for `Reg<CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>`"]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0 =
    crate::Reg<core_1_toomanyexceptions_m_override_0::CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_0_SPEC>;
#[doc = "core1 toomanyexception override configuration register 0."]
pub mod core_1_toomanyexceptions_m_override_0;
#[doc = "CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 (rw) register accessor: an alias for `Reg<CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>`"]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 =
    crate::Reg<core_1_toomanyexceptions_m_override_1::CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "core1 toomanyexception override configuration register 1."]
pub mod core_1_toomanyexceptions_m_override_1;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_0 =
    crate::Reg<backup_bus_pms_constrain_0::BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 0."]
pub mod backup_bus_pms_constrain_0;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_1 =
    crate::Reg<backup_bus_pms_constrain_1::BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 1."]
pub mod backup_bus_pms_constrain_1;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_2 =
    crate::Reg<backup_bus_pms_constrain_2::BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 2."]
pub mod backup_bus_pms_constrain_2;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_3 =
    crate::Reg<backup_bus_pms_constrain_3::BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 3."]
pub mod backup_bus_pms_constrain_3;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_4 =
    crate::Reg<backup_bus_pms_constrain_4::BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 4."]
pub mod backup_bus_pms_constrain_4;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_5 =
    crate::Reg<backup_bus_pms_constrain_5::BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 5."]
pub mod backup_bus_pms_constrain_5;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_6 =
    crate::Reg<backup_bus_pms_constrain_6::BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "BackUp access peripherals permission configuration register 6."]
pub mod backup_bus_pms_constrain_6;
#[doc = "BACKUP_BUS_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_0_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_0 =
    crate::Reg<backup_bus_pms_monitor_0::BACKUP_BUS_PMS_MONITOR_0_SPEC>;
#[doc = "BackUp permission report register 0."]
pub mod backup_bus_pms_monitor_0;
#[doc = "BACKUP_BUS_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_1_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_1 =
    crate::Reg<backup_bus_pms_monitor_1::BACKUP_BUS_PMS_MONITOR_1_SPEC>;
#[doc = "BackUp permission report register 1."]
pub mod backup_bus_pms_monitor_1;
#[doc = "BACKUP_BUS_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_2_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_2 =
    crate::Reg<backup_bus_pms_monitor_2::BACKUP_BUS_PMS_MONITOR_2_SPEC>;
#[doc = "BackUp permission report register 2."]
pub mod backup_bus_pms_monitor_2;
#[doc = "BACKUP_BUS_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_3_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_3 =
    crate::Reg<backup_bus_pms_monitor_3::BACKUP_BUS_PMS_MONITOR_3_SPEC>;
#[doc = "BackUp permission report register 3."]
pub mod backup_bus_pms_monitor_3;
#[doc = "EDMA_BOUNDARY_LOCK (rw) register accessor: an alias for `Reg<EDMA_BOUNDARY_LOCK_SPEC>`"]
pub type EDMA_BOUNDARY_LOCK = crate::Reg<edma_boundary_lock::EDMA_BOUNDARY_LOCK_SPEC>;
#[doc = "EDMA boundary lock register."]
pub mod edma_boundary_lock;
#[doc = "EDMA_BOUNDARY_0 (rw) register accessor: an alias for `Reg<EDMA_BOUNDARY_0_SPEC>`"]
pub type EDMA_BOUNDARY_0 = crate::Reg<edma_boundary_0::EDMA_BOUNDARY_0_SPEC>;
#[doc = "EDMA boundary 0 configuration"]
pub mod edma_boundary_0;
#[doc = "EDMA_BOUNDARY_1 (rw) register accessor: an alias for `Reg<EDMA_BOUNDARY_1_SPEC>`"]
pub type EDMA_BOUNDARY_1 = crate::Reg<edma_boundary_1::EDMA_BOUNDARY_1_SPEC>;
#[doc = "EDMA boundary 1 configuration"]
pub mod edma_boundary_1;
#[doc = "EDMA_BOUNDARY_2 (rw) register accessor: an alias for `Reg<EDMA_BOUNDARY_2_SPEC>`"]
pub type EDMA_BOUNDARY_2 = crate::Reg<edma_boundary_2::EDMA_BOUNDARY_2_SPEC>;
#[doc = "EDMA boundary 2 configuration"]
pub mod edma_boundary_2;
#[doc = "EDMA_PMS_SPI2_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_SPI2_LOCK_SPEC>`"]
pub type EDMA_PMS_SPI2_LOCK = crate::Reg<edma_pms_spi2_lock::EDMA_PMS_SPI2_LOCK_SPEC>;
#[doc = "EDMA-SPI2 permission lock register."]
pub mod edma_pms_spi2_lock;
#[doc = "EDMA_PMS_SPI2 (rw) register accessor: an alias for `Reg<EDMA_PMS_SPI2_SPEC>`"]
pub type EDMA_PMS_SPI2 = crate::Reg<edma_pms_spi2::EDMA_PMS_SPI2_SPEC>;
#[doc = "EDMA-SPI2 permission control register."]
pub mod edma_pms_spi2;
#[doc = "EDMA_PMS_SPI3_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_SPI3_LOCK_SPEC>`"]
pub type EDMA_PMS_SPI3_LOCK = crate::Reg<edma_pms_spi3_lock::EDMA_PMS_SPI3_LOCK_SPEC>;
#[doc = "EDMA-SPI3 permission lock register."]
pub mod edma_pms_spi3_lock;
#[doc = "EDMA_PMS_SPI3 (rw) register accessor: an alias for `Reg<EDMA_PMS_SPI3_SPEC>`"]
pub type EDMA_PMS_SPI3 = crate::Reg<edma_pms_spi3::EDMA_PMS_SPI3_SPEC>;
#[doc = "EDMA-SPI3 permission control register."]
pub mod edma_pms_spi3;
#[doc = "EDMA_PMS_UHCI0_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_UHCI0_LOCK_SPEC>`"]
pub type EDMA_PMS_UHCI0_LOCK = crate::Reg<edma_pms_uhci0_lock::EDMA_PMS_UHCI0_LOCK_SPEC>;
#[doc = "EDMA-UHCI0 permission lock register."]
pub mod edma_pms_uhci0_lock;
#[doc = "EDMA_PMS_UHCI0 (rw) register accessor: an alias for `Reg<EDMA_PMS_UHCI0_SPEC>`"]
pub type EDMA_PMS_UHCI0 = crate::Reg<edma_pms_uhci0::EDMA_PMS_UHCI0_SPEC>;
#[doc = "EDMA-UHCI0 permission control register."]
pub mod edma_pms_uhci0;
#[doc = "EDMA_PMS_I2S0_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_I2S0_LOCK_SPEC>`"]
pub type EDMA_PMS_I2S0_LOCK = crate::Reg<edma_pms_i2s0_lock::EDMA_PMS_I2S0_LOCK_SPEC>;
#[doc = "EDMA-I2S0 permission lock register."]
pub mod edma_pms_i2s0_lock;
#[doc = "EDMA_PMS_I2S0 (rw) register accessor: an alias for `Reg<EDMA_PMS_I2S0_SPEC>`"]
pub type EDMA_PMS_I2S0 = crate::Reg<edma_pms_i2s0::EDMA_PMS_I2S0_SPEC>;
#[doc = "EDMA-I2S0 permission control register."]
pub mod edma_pms_i2s0;
#[doc = "EDMA_PMS_I2S1_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_I2S1_LOCK_SPEC>`"]
pub type EDMA_PMS_I2S1_LOCK = crate::Reg<edma_pms_i2s1_lock::EDMA_PMS_I2S1_LOCK_SPEC>;
#[doc = "EDMA-I2S1 permission lock register."]
pub mod edma_pms_i2s1_lock;
#[doc = "EDMA_PMS_I2S1 (rw) register accessor: an alias for `Reg<EDMA_PMS_I2S1_SPEC>`"]
pub type EDMA_PMS_I2S1 = crate::Reg<edma_pms_i2s1::EDMA_PMS_I2S1_SPEC>;
#[doc = "EDMA-I2S1 permission control register."]
pub mod edma_pms_i2s1;
#[doc = "EDMA_PMS_LCD_CAM_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_LCD_CAM_LOCK_SPEC>`"]
pub type EDMA_PMS_LCD_CAM_LOCK = crate::Reg<edma_pms_lcd_cam_lock::EDMA_PMS_LCD_CAM_LOCK_SPEC>;
#[doc = "EDMA-LCD/CAM permission lock register."]
pub mod edma_pms_lcd_cam_lock;
#[doc = "EDMA_PMS_LCD_CAM (rw) register accessor: an alias for `Reg<EDMA_PMS_LCD_CAM_SPEC>`"]
pub type EDMA_PMS_LCD_CAM = crate::Reg<edma_pms_lcd_cam::EDMA_PMS_LCD_CAM_SPEC>;
#[doc = "EDMA-LCD/CAM permission control register."]
pub mod edma_pms_lcd_cam;
#[doc = "EDMA_PMS_AES_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_AES_LOCK_SPEC>`"]
pub type EDMA_PMS_AES_LOCK = crate::Reg<edma_pms_aes_lock::EDMA_PMS_AES_LOCK_SPEC>;
#[doc = "EDMA-AES permission lock register."]
pub mod edma_pms_aes_lock;
#[doc = "EDMA_PMS_AES (rw) register accessor: an alias for `Reg<EDMA_PMS_AES_SPEC>`"]
pub type EDMA_PMS_AES = crate::Reg<edma_pms_aes::EDMA_PMS_AES_SPEC>;
#[doc = "EDMA-AES permission control register."]
pub mod edma_pms_aes;
#[doc = "EDMA_PMS_SHA_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_SHA_LOCK_SPEC>`"]
pub type EDMA_PMS_SHA_LOCK = crate::Reg<edma_pms_sha_lock::EDMA_PMS_SHA_LOCK_SPEC>;
#[doc = "EDMA-SHA permission lock register."]
pub mod edma_pms_sha_lock;
#[doc = "EDMA_PMS_SHA (rw) register accessor: an alias for `Reg<EDMA_PMS_SHA_SPEC>`"]
pub type EDMA_PMS_SHA = crate::Reg<edma_pms_sha::EDMA_PMS_SHA_SPEC>;
#[doc = "EDMA-SHA permission control register."]
pub mod edma_pms_sha;
#[doc = "EDMA_PMS_ADC_DAC_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_ADC_DAC_LOCK_SPEC>`"]
pub type EDMA_PMS_ADC_DAC_LOCK = crate::Reg<edma_pms_adc_dac_lock::EDMA_PMS_ADC_DAC_LOCK_SPEC>;
#[doc = "EDMA-ADC/DAC permission lock register."]
pub mod edma_pms_adc_dac_lock;
#[doc = "EDMA_PMS_ADC_DAC (rw) register accessor: an alias for `Reg<EDMA_PMS_ADC_DAC_SPEC>`"]
pub type EDMA_PMS_ADC_DAC = crate::Reg<edma_pms_adc_dac::EDMA_PMS_ADC_DAC_SPEC>;
#[doc = "EDMA-ADC/DAC permission control register."]
pub mod edma_pms_adc_dac;
#[doc = "EDMA_PMS_RMT_LOCK (rw) register accessor: an alias for `Reg<EDMA_PMS_RMT_LOCK_SPEC>`"]
pub type EDMA_PMS_RMT_LOCK = crate::Reg<edma_pms_rmt_lock::EDMA_PMS_RMT_LOCK_SPEC>;
#[doc = "EDMA-RMT permission lock register."]
pub mod edma_pms_rmt_lock;
#[doc = "EDMA_PMS_RMT (rw) register accessor: an alias for `Reg<EDMA_PMS_RMT_SPEC>`"]
pub type EDMA_PMS_RMT = crate::Reg<edma_pms_rmt::EDMA_PMS_RMT_SPEC>;
#[doc = "EDMA-RMT permission control register."]
pub mod edma_pms_rmt;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Sensitive module clock gate configuration register."]
pub mod clock_gate;
#[doc = "RTC_PMS (rw) register accessor: an alias for `Reg<RTC_PMS_SPEC>`"]
pub type RTC_PMS = crate::Reg<rtc_pms::RTC_PMS_SPEC>;
#[doc = "RTC coprocessor permission register."]
pub mod rtc_pms;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Sensitive version register."]
pub mod date;
