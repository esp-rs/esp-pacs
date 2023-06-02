#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub pro_boot_remap_ctrl: PRO_BOOT_REMAP_CTRL,
    #[doc = "0x04 - "]
    pub app_boot_remap_ctrl: APP_BOOT_REMAP_CTRL,
    #[doc = "0x08 - "]
    pub access_check: ACCESS_CHECK,
    #[doc = "0x0c - "]
    pub pro_dport_apb_mask0: PRO_DPORT_APB_MASK0,
    #[doc = "0x10 - "]
    pub pro_dport_apb_mask1: PRO_DPORT_APB_MASK1,
    #[doc = "0x14 - "]
    pub app_dport_apb_mask0: APP_DPORT_APB_MASK0,
    #[doc = "0x18 - "]
    pub app_dport_apb_mask1: APP_DPORT_APB_MASK1,
    #[doc = "0x1c - "]
    pub peri_clk_en: PERI_CLK_EN,
    #[doc = "0x20 - "]
    pub peri_rst_en: PERI_RST_EN,
    #[doc = "0x24 - "]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x28 - "]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x2c - "]
    pub appcpu_ctrl_a: APPCPU_CTRL_A,
    #[doc = "0x30 - "]
    pub appcpu_ctrl_b: APPCPU_CTRL_B,
    #[doc = "0x34 - "]
    pub appcpu_ctrl_c: APPCPU_CTRL_C,
    #[doc = "0x38 - "]
    pub appcpu_ctrl_d: APPCPU_CTRL_D,
    #[doc = "0x3c - "]
    pub cpu_per_conf: CPU_PER_CONF,
    #[doc = "0x40 - "]
    pub pro_cache_ctrl: PRO_CACHE_CTRL,
    #[doc = "0x44 - "]
    pub pro_cache_ctrl1: PRO_CACHE_CTRL1,
    #[doc = "0x48 - "]
    pub pro_cache_lock_0_addr: PRO_CACHE_LOCK_0_ADDR,
    #[doc = "0x4c - "]
    pub pro_cache_lock_1_addr: PRO_CACHE_LOCK_1_ADDR,
    #[doc = "0x50 - "]
    pub pro_cache_lock_2_addr: PRO_CACHE_LOCK_2_ADDR,
    #[doc = "0x54 - "]
    pub pro_cache_lock_3_addr: PRO_CACHE_LOCK_3_ADDR,
    #[doc = "0x58 - "]
    pub app_cache_ctrl: APP_CACHE_CTRL,
    #[doc = "0x5c - "]
    pub app_cache_ctrl1: APP_CACHE_CTRL1,
    #[doc = "0x60 - "]
    pub app_cache_lock_0_addr: APP_CACHE_LOCK_0_ADDR,
    #[doc = "0x64 - "]
    pub app_cache_lock_1_addr: APP_CACHE_LOCK_1_ADDR,
    #[doc = "0x68 - "]
    pub app_cache_lock_2_addr: APP_CACHE_LOCK_2_ADDR,
    #[doc = "0x6c - "]
    pub app_cache_lock_3_addr: APP_CACHE_LOCK_3_ADDR,
    #[doc = "0x70 - "]
    pub tracemem_mux_mode: TRACEMEM_MUX_MODE,
    #[doc = "0x74 - "]
    pub pro_tracemem_ena: PRO_TRACEMEM_ENA,
    #[doc = "0x78 - "]
    pub app_tracemem_ena: APP_TRACEMEM_ENA,
    #[doc = "0x7c - "]
    pub cache_mux_mode: CACHE_MUX_MODE,
    #[doc = "0x80 - "]
    pub immu_page_mode: IMMU_PAGE_MODE,
    #[doc = "0x84 - "]
    pub dmmu_page_mode: DMMU_PAGE_MODE,
    #[doc = "0x88 - "]
    pub rom_mpu_ena: ROM_MPU_ENA,
    #[doc = "0x8c - "]
    pub mem_pd_mask: MEM_PD_MASK,
    #[doc = "0x90 - "]
    pub rom_pd_ctrl: ROM_PD_CTRL,
    #[doc = "0x94 - "]
    pub rom_fo_ctrl: ROM_FO_CTRL,
    #[doc = "0x98 - "]
    pub sram_pd_ctrl_0: SRAM_PD_CTRL_0,
    #[doc = "0x9c - "]
    pub sram_pd_ctrl_1: SRAM_PD_CTRL_1,
    #[doc = "0xa0 - "]
    pub sram_fo_ctrl_0: SRAM_FO_CTRL_0,
    #[doc = "0xa4 - "]
    pub sram_fo_ctrl_1: SRAM_FO_CTRL_1,
    #[doc = "0xa8 - "]
    pub iram_dram_ahb_sel: IRAM_DRAM_AHB_SEL,
    #[doc = "0xac - "]
    pub tag_fo_ctrl: TAG_FO_CTRL,
    #[doc = "0xb0 - "]
    pub ahb_lite_mask: AHB_LITE_MASK,
    #[doc = "0xb4 - "]
    pub ahb_mpu_table_0: AHB_MPU_TABLE_0,
    #[doc = "0xb8 - "]
    pub ahb_mpu_table_1: AHB_MPU_TABLE_1,
    #[doc = "0xbc - "]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0xc0 - "]
    pub perip_clk_en: PERIP_CLK_EN,
    #[doc = "0xc4 - "]
    pub perip_rst_en: PERIP_RST_EN,
    #[doc = "0xc8 - "]
    pub slave_spi_config: SLAVE_SPI_CONFIG,
    #[doc = "0xcc - "]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0xd0 - "]
    pub core_rst_en: CORE_RST_EN,
    #[doc = "0xd4 - "]
    pub bt_lpck_div_int: BT_LPCK_DIV_INT,
    #[doc = "0xd8 - "]
    pub bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    #[doc = "0xdc - "]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0xe0 - "]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0xe4 - "]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0xe8 - "]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0xec - "]
    pub pro_intr_status_0: PRO_INTR_STATUS_0,
    #[doc = "0xf0 - "]
    pub pro_intr_status_1: PRO_INTR_STATUS_1,
    #[doc = "0xf4 - "]
    pub pro_intr_status_2: PRO_INTR_STATUS_2,
    #[doc = "0xf8 - "]
    pub app_intr_status_0: APP_INTR_STATUS_0,
    #[doc = "0xfc - "]
    pub app_intr_status_1: APP_INTR_STATUS_1,
    #[doc = "0x100 - "]
    pub app_intr_status_2: APP_INTR_STATUS_2,
    #[doc = "0x104 - "]
    pub pro_mac_intr_map: PRO_MAC_INTR_MAP,
    #[doc = "0x108 - "]
    pub pro_mac_nmi_map: PRO_MAC_NMI_MAP,
    #[doc = "0x10c - "]
    pub pro_bb_int_map: PRO_BB_INT_MAP,
    #[doc = "0x110 - "]
    pub pro_bt_mac_int_map: PRO_BT_MAC_INT_MAP,
    #[doc = "0x114 - "]
    pub pro_bt_bb_int_map: PRO_BT_BB_INT_MAP,
    #[doc = "0x118 - "]
    pub pro_bt_bb_nmi_map: PRO_BT_BB_NMI_MAP,
    #[doc = "0x11c - "]
    pub pro_rwbt_irq_map: PRO_RWBT_IRQ_MAP,
    #[doc = "0x120 - "]
    pub pro_rwble_irq_map: PRO_RWBLE_IRQ_MAP,
    #[doc = "0x124 - "]
    pub pro_rwbt_nmi_map: PRO_RWBT_NMI_MAP,
    #[doc = "0x128 - "]
    pub pro_rwble_nmi_map: PRO_RWBLE_NMI_MAP,
    #[doc = "0x12c - "]
    pub pro_slc0_intr_map: PRO_SLC0_INTR_MAP,
    #[doc = "0x130 - "]
    pub pro_slc1_intr_map: PRO_SLC1_INTR_MAP,
    #[doc = "0x134 - "]
    pub pro_uhci0_intr_map: PRO_UHCI0_INTR_MAP,
    #[doc = "0x138 - "]
    pub pro_uhci1_intr_map: PRO_UHCI1_INTR_MAP,
    #[doc = "0x13c - "]
    pub pro_tg_t0_level_int_map: PRO_TG_T0_LEVEL_INT_MAP,
    #[doc = "0x140 - "]
    pub pro_tg_t1_level_int_map: PRO_TG_T1_LEVEL_INT_MAP,
    #[doc = "0x144 - "]
    pub pro_tg_wdt_level_int_map: PRO_TG_WDT_LEVEL_INT_MAP,
    #[doc = "0x148 - "]
    pub pro_tg_lact_level_int_map: PRO_TG_LACT_LEVEL_INT_MAP,
    #[doc = "0x14c - "]
    pub pro_tg1_t0_level_int_map: PRO_TG1_T0_LEVEL_INT_MAP,
    #[doc = "0x150 - "]
    pub pro_tg1_t1_level_int_map: PRO_TG1_T1_LEVEL_INT_MAP,
    #[doc = "0x154 - "]
    pub pro_tg1_wdt_level_int_map: PRO_TG1_WDT_LEVEL_INT_MAP,
    #[doc = "0x158 - "]
    pub pro_tg1_lact_level_int_map: PRO_TG1_LACT_LEVEL_INT_MAP,
    #[doc = "0x15c - "]
    pub pro_gpio_interrupt_map: PRO_GPIO_INTERRUPT_MAP,
    #[doc = "0x160 - "]
    pub pro_gpio_interrupt_nmi_map: PRO_GPIO_INTERRUPT_NMI_MAP,
    #[doc = "0x164 - "]
    pub pro_cpu_intr_from_cpu_0_map: PRO_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x168 - "]
    pub pro_cpu_intr_from_cpu_1_map: PRO_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x16c - "]
    pub pro_cpu_intr_from_cpu_2_map: PRO_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x170 - "]
    pub pro_cpu_intr_from_cpu_3_map: PRO_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x174 - "]
    pub pro_spi_intr_0_map: PRO_SPI_INTR_0_MAP,
    #[doc = "0x178 - "]
    pub pro_spi_intr_1_map: PRO_SPI_INTR_1_MAP,
    #[doc = "0x17c - "]
    pub pro_spi_intr_2_map: PRO_SPI_INTR_2_MAP,
    #[doc = "0x180 - "]
    pub pro_spi_intr_3_map: PRO_SPI_INTR_3_MAP,
    #[doc = "0x184 - "]
    pub pro_i2s0_int_map: PRO_I2S0_INT_MAP,
    #[doc = "0x188 - "]
    pub pro_i2s1_int_map: PRO_I2S1_INT_MAP,
    #[doc = "0x18c - "]
    pub pro_uart_intr_map: PRO_UART_INTR_MAP,
    #[doc = "0x190 - "]
    pub pro_uart1_intr_map: PRO_UART1_INTR_MAP,
    #[doc = "0x194 - "]
    pub pro_uart2_intr_map: PRO_UART2_INTR_MAP,
    #[doc = "0x198 - "]
    pub pro_sdio_host_interrupt_map: PRO_SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x19c - "]
    pub pro_emac_int_map: PRO_EMAC_INT_MAP,
    #[doc = "0x1a0 - "]
    pub pro_pwm0_intr_map: PRO_PWM0_INTR_MAP,
    #[doc = "0x1a4 - "]
    pub pro_pwm1_intr_map: PRO_PWM1_INTR_MAP,
    #[doc = "0x1a8 - "]
    pub pro_pwm2_intr_map: PRO_PWM2_INTR_MAP,
    #[doc = "0x1ac - "]
    pub pro_pwm3_intr_map: PRO_PWM3_INTR_MAP,
    #[doc = "0x1b0 - "]
    pub pro_ledc_int_map: PRO_LEDC_INT_MAP,
    #[doc = "0x1b4 - "]
    pub pro_efuse_int_map: PRO_EFUSE_INT_MAP,
    #[doc = "0x1b8 - "]
    pub pro_can_int_map: PRO_CAN_INT_MAP,
    #[doc = "0x1bc - "]
    pub pro_rtc_core_intr_map: PRO_RTC_CORE_INTR_MAP,
    #[doc = "0x1c0 - "]
    pub pro_rmt_intr_map: PRO_RMT_INTR_MAP,
    #[doc = "0x1c4 - "]
    pub pro_pcnt_intr_map: PRO_PCNT_INTR_MAP,
    #[doc = "0x1c8 - "]
    pub pro_i2c_ext0_intr_map: PRO_I2C_EXT0_INTR_MAP,
    #[doc = "0x1cc - "]
    pub pro_i2c_ext1_intr_map: PRO_I2C_EXT1_INTR_MAP,
    #[doc = "0x1d0 - "]
    pub pro_rsa_intr_map: PRO_RSA_INTR_MAP,
    #[doc = "0x1d4 - "]
    pub pro_spi1_dma_int_map: PRO_SPI1_DMA_INT_MAP,
    #[doc = "0x1d8 - "]
    pub pro_spi2_dma_int_map: PRO_SPI2_DMA_INT_MAP,
    #[doc = "0x1dc - "]
    pub pro_spi3_dma_int_map: PRO_SPI3_DMA_INT_MAP,
    #[doc = "0x1e0 - "]
    pub pro_wdg_int_map: PRO_WDG_INT_MAP,
    #[doc = "0x1e4 - "]
    pub pro_timer_int1_map: PRO_TIMER_INT1_MAP,
    #[doc = "0x1e8 - "]
    pub pro_timer_int2_map: PRO_TIMER_INT2_MAP,
    #[doc = "0x1ec - "]
    pub pro_tg_t0_edge_int_map: PRO_TG_T0_EDGE_INT_MAP,
    #[doc = "0x1f0 - "]
    pub pro_tg_t1_edge_int_map: PRO_TG_T1_EDGE_INT_MAP,
    #[doc = "0x1f4 - "]
    pub pro_tg_wdt_edge_int_map: PRO_TG_WDT_EDGE_INT_MAP,
    #[doc = "0x1f8 - "]
    pub pro_tg_lact_edge_int_map: PRO_TG_LACT_EDGE_INT_MAP,
    #[doc = "0x1fc - "]
    pub pro_tg1_t0_edge_int_map: PRO_TG1_T0_EDGE_INT_MAP,
    #[doc = "0x200 - "]
    pub pro_tg1_t1_edge_int_map: PRO_TG1_T1_EDGE_INT_MAP,
    #[doc = "0x204 - "]
    pub pro_tg1_wdt_edge_int_map: PRO_TG1_WDT_EDGE_INT_MAP,
    #[doc = "0x208 - "]
    pub pro_tg1_lact_edge_int_map: PRO_TG1_LACT_EDGE_INT_MAP,
    #[doc = "0x20c - "]
    pub pro_mmu_ia_int_map: PRO_MMU_IA_INT_MAP,
    #[doc = "0x210 - "]
    pub pro_mpu_ia_int_map: PRO_MPU_IA_INT_MAP,
    #[doc = "0x214 - "]
    pub pro_cache_ia_int_map: PRO_CACHE_IA_INT_MAP,
    #[doc = "0x218 - "]
    pub app_mac_intr_map: APP_MAC_INTR_MAP,
    #[doc = "0x21c - "]
    pub app_mac_nmi_map: APP_MAC_NMI_MAP,
    #[doc = "0x220 - "]
    pub app_bb_int_map: APP_BB_INT_MAP,
    #[doc = "0x224 - "]
    pub app_bt_mac_int_map: APP_BT_MAC_INT_MAP,
    #[doc = "0x228 - "]
    pub app_bt_bb_int_map: APP_BT_BB_INT_MAP,
    #[doc = "0x22c - "]
    pub app_bt_bb_nmi_map: APP_BT_BB_NMI_MAP,
    #[doc = "0x230 - "]
    pub app_rwbt_irq_map: APP_RWBT_IRQ_MAP,
    #[doc = "0x234 - "]
    pub app_rwble_irq_map: APP_RWBLE_IRQ_MAP,
    #[doc = "0x238 - "]
    pub app_rwbt_nmi_map: APP_RWBT_NMI_MAP,
    #[doc = "0x23c - "]
    pub app_rwble_nmi_map: APP_RWBLE_NMI_MAP,
    #[doc = "0x240 - "]
    pub app_slc0_intr_map: APP_SLC0_INTR_MAP,
    #[doc = "0x244 - "]
    pub app_slc1_intr_map: APP_SLC1_INTR_MAP,
    #[doc = "0x248 - "]
    pub app_uhci0_intr_map: APP_UHCI0_INTR_MAP,
    #[doc = "0x24c - "]
    pub app_uhci1_intr_map: APP_UHCI1_INTR_MAP,
    #[doc = "0x250 - "]
    pub app_tg_t0_level_int_map: APP_TG_T0_LEVEL_INT_MAP,
    #[doc = "0x254 - "]
    pub app_tg_t1_level_int_map: APP_TG_T1_LEVEL_INT_MAP,
    #[doc = "0x258 - "]
    pub app_tg_wdt_level_int_map: APP_TG_WDT_LEVEL_INT_MAP,
    #[doc = "0x25c - "]
    pub app_tg_lact_level_int_map: APP_TG_LACT_LEVEL_INT_MAP,
    #[doc = "0x260 - "]
    pub app_tg1_t0_level_int_map: APP_TG1_T0_LEVEL_INT_MAP,
    #[doc = "0x264 - "]
    pub app_tg1_t1_level_int_map: APP_TG1_T1_LEVEL_INT_MAP,
    #[doc = "0x268 - "]
    pub app_tg1_wdt_level_int_map: APP_TG1_WDT_LEVEL_INT_MAP,
    #[doc = "0x26c - "]
    pub app_tg1_lact_level_int_map: APP_TG1_LACT_LEVEL_INT_MAP,
    #[doc = "0x270 - "]
    pub app_gpio_interrupt_map: APP_GPIO_INTERRUPT_MAP,
    #[doc = "0x274 - "]
    pub app_gpio_interrupt_nmi_map: APP_GPIO_INTERRUPT_NMI_MAP,
    #[doc = "0x278 - "]
    pub app_cpu_intr_from_cpu_0_map: APP_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x27c - "]
    pub app_cpu_intr_from_cpu_1_map: APP_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x280 - "]
    pub app_cpu_intr_from_cpu_2_map: APP_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x284 - "]
    pub app_cpu_intr_from_cpu_3_map: APP_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x288 - "]
    pub app_spi_intr_0_map: APP_SPI_INTR_0_MAP,
    #[doc = "0x28c - "]
    pub app_spi_intr_1_map: APP_SPI_INTR_1_MAP,
    #[doc = "0x290 - "]
    pub app_spi_intr_2_map: APP_SPI_INTR_2_MAP,
    #[doc = "0x294 - "]
    pub app_spi_intr_3_map: APP_SPI_INTR_3_MAP,
    #[doc = "0x298 - "]
    pub app_i2s0_int_map: APP_I2S0_INT_MAP,
    #[doc = "0x29c - "]
    pub app_i2s1_int_map: APP_I2S1_INT_MAP,
    #[doc = "0x2a0 - "]
    pub app_uart_intr_map: APP_UART_INTR_MAP,
    #[doc = "0x2a4 - "]
    pub app_uart1_intr_map: APP_UART1_INTR_MAP,
    #[doc = "0x2a8 - "]
    pub app_uart2_intr_map: APP_UART2_INTR_MAP,
    #[doc = "0x2ac - "]
    pub app_sdio_host_interrupt_map: APP_SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x2b0 - "]
    pub app_emac_int_map: APP_EMAC_INT_MAP,
    #[doc = "0x2b4 - "]
    pub app_pwm0_intr_map: APP_PWM0_INTR_MAP,
    #[doc = "0x2b8 - "]
    pub app_pwm1_intr_map: APP_PWM1_INTR_MAP,
    #[doc = "0x2bc - "]
    pub app_pwm2_intr_map: APP_PWM2_INTR_MAP,
    #[doc = "0x2c0 - "]
    pub app_pwm3_intr_map: APP_PWM3_INTR_MAP,
    #[doc = "0x2c4 - "]
    pub app_ledc_int_map: APP_LEDC_INT_MAP,
    #[doc = "0x2c8 - "]
    pub app_efuse_int_map: APP_EFUSE_INT_MAP,
    #[doc = "0x2cc - "]
    pub app_can_int_map: APP_CAN_INT_MAP,
    #[doc = "0x2d0 - "]
    pub app_rtc_core_intr_map: APP_RTC_CORE_INTR_MAP,
    #[doc = "0x2d4 - "]
    pub app_rmt_intr_map: APP_RMT_INTR_MAP,
    #[doc = "0x2d8 - "]
    pub app_pcnt_intr_map: APP_PCNT_INTR_MAP,
    #[doc = "0x2dc - "]
    pub app_i2c_ext0_intr_map: APP_I2C_EXT0_INTR_MAP,
    #[doc = "0x2e0 - "]
    pub app_i2c_ext1_intr_map: APP_I2C_EXT1_INTR_MAP,
    #[doc = "0x2e4 - "]
    pub app_rsa_intr_map: APP_RSA_INTR_MAP,
    #[doc = "0x2e8 - "]
    pub app_spi1_dma_int_map: APP_SPI1_DMA_INT_MAP,
    #[doc = "0x2ec - "]
    pub app_spi2_dma_int_map: APP_SPI2_DMA_INT_MAP,
    #[doc = "0x2f0 - "]
    pub app_spi3_dma_int_map: APP_SPI3_DMA_INT_MAP,
    #[doc = "0x2f4 - "]
    pub app_wdg_int_map: APP_WDG_INT_MAP,
    #[doc = "0x2f8 - "]
    pub app_timer_int1_map: APP_TIMER_INT1_MAP,
    #[doc = "0x2fc - "]
    pub app_timer_int2_map: APP_TIMER_INT2_MAP,
    #[doc = "0x300 - "]
    pub app_tg_t0_edge_int_map: APP_TG_T0_EDGE_INT_MAP,
    #[doc = "0x304 - "]
    pub app_tg_t1_edge_int_map: APP_TG_T1_EDGE_INT_MAP,
    #[doc = "0x308 - "]
    pub app_tg_wdt_edge_int_map: APP_TG_WDT_EDGE_INT_MAP,
    #[doc = "0x30c - "]
    pub app_tg_lact_edge_int_map: APP_TG_LACT_EDGE_INT_MAP,
    #[doc = "0x310 - "]
    pub app_tg1_t0_edge_int_map: APP_TG1_T0_EDGE_INT_MAP,
    #[doc = "0x314 - "]
    pub app_tg1_t1_edge_int_map: APP_TG1_T1_EDGE_INT_MAP,
    #[doc = "0x318 - "]
    pub app_tg1_wdt_edge_int_map: APP_TG1_WDT_EDGE_INT_MAP,
    #[doc = "0x31c - "]
    pub app_tg1_lact_edge_int_map: APP_TG1_LACT_EDGE_INT_MAP,
    #[doc = "0x320 - "]
    pub app_mmu_ia_int_map: APP_MMU_IA_INT_MAP,
    #[doc = "0x324 - "]
    pub app_mpu_ia_int_map: APP_MPU_IA_INT_MAP,
    #[doc = "0x328 - "]
    pub app_cache_ia_int_map: APP_CACHE_IA_INT_MAP,
    #[doc = "0x32c - "]
    pub ahblite_mpu_table_uart: AHBLITE_MPU_TABLE_UART,
    #[doc = "0x330 - "]
    pub ahblite_mpu_table_spi1: AHBLITE_MPU_TABLE_SPI1,
    #[doc = "0x334 - "]
    pub ahblite_mpu_table_spi0: AHBLITE_MPU_TABLE_SPI0,
    #[doc = "0x338 - "]
    pub ahblite_mpu_table_gpio: AHBLITE_MPU_TABLE_GPIO,
    #[doc = "0x33c - "]
    pub ahblite_mpu_table_fe2: AHBLITE_MPU_TABLE_FE2,
    #[doc = "0x340 - "]
    pub ahblite_mpu_table_fe: AHBLITE_MPU_TABLE_FE,
    #[doc = "0x344 - "]
    pub ahblite_mpu_table_timer: AHBLITE_MPU_TABLE_TIMER,
    #[doc = "0x348 - "]
    pub ahblite_mpu_table_rtc: AHBLITE_MPU_TABLE_RTC,
    #[doc = "0x34c - "]
    pub ahblite_mpu_table_io_mux: AHBLITE_MPU_TABLE_IO_MUX,
    #[doc = "0x350 - "]
    pub ahblite_mpu_table_wdg: AHBLITE_MPU_TABLE_WDG,
    #[doc = "0x354 - "]
    pub ahblite_mpu_table_hinf: AHBLITE_MPU_TABLE_HINF,
    #[doc = "0x358 - "]
    pub ahblite_mpu_table_uhci1: AHBLITE_MPU_TABLE_UHCI1,
    #[doc = "0x35c - "]
    pub ahblite_mpu_table_misc: AHBLITE_MPU_TABLE_MISC,
    #[doc = "0x360 - "]
    pub ahblite_mpu_table_i2c: AHBLITE_MPU_TABLE_I2C,
    #[doc = "0x364 - "]
    pub ahblite_mpu_table_i2s0: AHBLITE_MPU_TABLE_I2S0,
    #[doc = "0x368 - "]
    pub ahblite_mpu_table_uart1: AHBLITE_MPU_TABLE_UART1,
    #[doc = "0x36c - "]
    pub ahblite_mpu_table_bt: AHBLITE_MPU_TABLE_BT,
    #[doc = "0x370 - "]
    pub ahblite_mpu_table_bt_buffer: AHBLITE_MPU_TABLE_BT_BUFFER,
    #[doc = "0x374 - "]
    pub ahblite_mpu_table_i2c_ext0: AHBLITE_MPU_TABLE_I2C_EXT0,
    #[doc = "0x378 - "]
    pub ahblite_mpu_table_uhci0: AHBLITE_MPU_TABLE_UHCI0,
    #[doc = "0x37c - "]
    pub ahblite_mpu_table_slchost: AHBLITE_MPU_TABLE_SLCHOST,
    #[doc = "0x380 - "]
    pub ahblite_mpu_table_rmt: AHBLITE_MPU_TABLE_RMT,
    #[doc = "0x384 - "]
    pub ahblite_mpu_table_pcnt: AHBLITE_MPU_TABLE_PCNT,
    #[doc = "0x388 - "]
    pub ahblite_mpu_table_slc: AHBLITE_MPU_TABLE_SLC,
    #[doc = "0x38c - "]
    pub ahblite_mpu_table_ledc: AHBLITE_MPU_TABLE_LEDC,
    #[doc = "0x390 - "]
    pub ahblite_mpu_table_efuse: AHBLITE_MPU_TABLE_EFUSE,
    #[doc = "0x394 - "]
    pub ahblite_mpu_table_spi_encrypt: AHBLITE_MPU_TABLE_SPI_ENCRYPT,
    #[doc = "0x398 - "]
    pub ahblite_mpu_table_bb: AHBLITE_MPU_TABLE_BB,
    #[doc = "0x39c - "]
    pub ahblite_mpu_table_pwm0: AHBLITE_MPU_TABLE_PWM0,
    #[doc = "0x3a0 - "]
    pub ahblite_mpu_table_timergroup: AHBLITE_MPU_TABLE_TIMERGROUP,
    #[doc = "0x3a4 - "]
    pub ahblite_mpu_table_timergroup1: AHBLITE_MPU_TABLE_TIMERGROUP1,
    #[doc = "0x3a8 - "]
    pub ahblite_mpu_table_spi2: AHBLITE_MPU_TABLE_SPI2,
    #[doc = "0x3ac - "]
    pub ahblite_mpu_table_spi3: AHBLITE_MPU_TABLE_SPI3,
    #[doc = "0x3b0 - "]
    pub ahblite_mpu_table_apb_ctrl: AHBLITE_MPU_TABLE_APB_CTRL,
    #[doc = "0x3b4 - "]
    pub ahblite_mpu_table_i2c_ext1: AHBLITE_MPU_TABLE_I2C_EXT1,
    #[doc = "0x3b8 - "]
    pub ahblite_mpu_table_sdio_host: AHBLITE_MPU_TABLE_SDIO_HOST,
    #[doc = "0x3bc - "]
    pub ahblite_mpu_table_emac: AHBLITE_MPU_TABLE_EMAC,
    #[doc = "0x3c0 - "]
    pub ahblite_mpu_table_can: AHBLITE_MPU_TABLE_CAN,
    #[doc = "0x3c4 - "]
    pub ahblite_mpu_table_pwm1: AHBLITE_MPU_TABLE_PWM1,
    #[doc = "0x3c8 - "]
    pub ahblite_mpu_table_i2s1: AHBLITE_MPU_TABLE_I2S1,
    #[doc = "0x3cc - "]
    pub ahblite_mpu_table_uart2: AHBLITE_MPU_TABLE_UART2,
    #[doc = "0x3d0 - "]
    pub ahblite_mpu_table_pwm2: AHBLITE_MPU_TABLE_PWM2,
    #[doc = "0x3d4 - "]
    pub ahblite_mpu_table_pwm3: AHBLITE_MPU_TABLE_PWM3,
    #[doc = "0x3d8 - "]
    pub ahblite_mpu_table_rwbt: AHBLITE_MPU_TABLE_RWBT,
    #[doc = "0x3dc - "]
    pub ahblite_mpu_table_btmac: AHBLITE_MPU_TABLE_BTMAC,
    #[doc = "0x3e0 - "]
    pub ahblite_mpu_table_wifimac: AHBLITE_MPU_TABLE_WIFIMAC,
    #[doc = "0x3e4 - "]
    pub ahblite_mpu_table_pwr: AHBLITE_MPU_TABLE_PWR,
    #[doc = "0x3e8 - "]
    pub mem_access_dbug0: MEM_ACCESS_DBUG0,
    #[doc = "0x3ec - "]
    pub mem_access_dbug1: MEM_ACCESS_DBUG1,
    #[doc = "0x3f0 - "]
    pub pro_dcache_dbug0: PRO_DCACHE_DBUG0,
    #[doc = "0x3f4 - "]
    pub pro_dcache_dbug1: PRO_DCACHE_DBUG1,
    #[doc = "0x3f8 - "]
    pub pro_dcache_dbug2: PRO_DCACHE_DBUG2,
    #[doc = "0x3fc - "]
    pub pro_dcache_dbug3: PRO_DCACHE_DBUG3,
    #[doc = "0x400 - "]
    pub pro_dcache_dbug4: PRO_DCACHE_DBUG4,
    #[doc = "0x404 - "]
    pub pro_dcache_dbug5: PRO_DCACHE_DBUG5,
    #[doc = "0x408 - "]
    pub pro_dcache_dbug6: PRO_DCACHE_DBUG6,
    #[doc = "0x40c - "]
    pub pro_dcache_dbug7: PRO_DCACHE_DBUG7,
    #[doc = "0x410 - "]
    pub pro_dcache_dbug8: PRO_DCACHE_DBUG8,
    #[doc = "0x414 - "]
    pub pro_dcache_dbug9: PRO_DCACHE_DBUG9,
    #[doc = "0x418 - "]
    pub app_dcache_dbug0: APP_DCACHE_DBUG0,
    #[doc = "0x41c - "]
    pub app_dcache_dbug1: APP_DCACHE_DBUG1,
    #[doc = "0x420 - "]
    pub app_dcache_dbug2: APP_DCACHE_DBUG2,
    #[doc = "0x424 - "]
    pub app_dcache_dbug3: APP_DCACHE_DBUG3,
    #[doc = "0x428 - "]
    pub app_dcache_dbug4: APP_DCACHE_DBUG4,
    #[doc = "0x42c - "]
    pub app_dcache_dbug5: APP_DCACHE_DBUG5,
    #[doc = "0x430 - "]
    pub app_dcache_dbug6: APP_DCACHE_DBUG6,
    #[doc = "0x434 - "]
    pub app_dcache_dbug7: APP_DCACHE_DBUG7,
    #[doc = "0x438 - "]
    pub app_dcache_dbug8: APP_DCACHE_DBUG8,
    #[doc = "0x43c - "]
    pub app_dcache_dbug9: APP_DCACHE_DBUG9,
    #[doc = "0x440 - "]
    pub pro_cpu_record_ctrl: PRO_CPU_RECORD_CTRL,
    #[doc = "0x444 - "]
    pub pro_cpu_record_status: PRO_CPU_RECORD_STATUS,
    #[doc = "0x448 - "]
    pub pro_cpu_record_pid: PRO_CPU_RECORD_PID,
    #[doc = "0x44c - "]
    pub pro_cpu_record_pdebuginst: PRO_CPU_RECORD_PDEBUGINST,
    #[doc = "0x450 - "]
    pub pro_cpu_record_pdebugstatus: PRO_CPU_RECORD_PDEBUGSTATUS,
    #[doc = "0x454 - "]
    pub pro_cpu_record_pdebugdata: PRO_CPU_RECORD_PDEBUGDATA,
    #[doc = "0x458 - "]
    pub pro_cpu_record_pdebugpc: PRO_CPU_RECORD_PDEBUGPC,
    #[doc = "0x45c - "]
    pub pro_cpu_record_pdebugls0stat: PRO_CPU_RECORD_PDEBUGLS0STAT,
    #[doc = "0x460 - "]
    pub pro_cpu_record_pdebugls0addr: PRO_CPU_RECORD_PDEBUGLS0ADDR,
    #[doc = "0x464 - "]
    pub pro_cpu_record_pdebugls0data: PRO_CPU_RECORD_PDEBUGLS0DATA,
    #[doc = "0x468 - "]
    pub app_cpu_record_ctrl: APP_CPU_RECORD_CTRL,
    #[doc = "0x46c - "]
    pub app_cpu_record_status: APP_CPU_RECORD_STATUS,
    #[doc = "0x470 - "]
    pub app_cpu_record_pid: APP_CPU_RECORD_PID,
    #[doc = "0x474 - "]
    pub app_cpu_record_pdebuginst: APP_CPU_RECORD_PDEBUGINST,
    #[doc = "0x478 - "]
    pub app_cpu_record_pdebugstatus: APP_CPU_RECORD_PDEBUGSTATUS,
    #[doc = "0x47c - "]
    pub app_cpu_record_pdebugdata: APP_CPU_RECORD_PDEBUGDATA,
    #[doc = "0x480 - "]
    pub app_cpu_record_pdebugpc: APP_CPU_RECORD_PDEBUGPC,
    #[doc = "0x484 - "]
    pub app_cpu_record_pdebugls0stat: APP_CPU_RECORD_PDEBUGLS0STAT,
    #[doc = "0x488 - "]
    pub app_cpu_record_pdebugls0addr: APP_CPU_RECORD_PDEBUGLS0ADDR,
    #[doc = "0x48c - "]
    pub app_cpu_record_pdebugls0data: APP_CPU_RECORD_PDEBUGLS0DATA,
    #[doc = "0x490 - "]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0x494 - "]
    pub rom_mpu_table0: ROM_MPU_TABLE0,
    #[doc = "0x498 - "]
    pub rom_mpu_table1: ROM_MPU_TABLE1,
    #[doc = "0x49c - "]
    pub rom_mpu_table2: ROM_MPU_TABLE2,
    #[doc = "0x4a0 - "]
    pub rom_mpu_table3: ROM_MPU_TABLE3,
    #[doc = "0x4a4 - "]
    pub shrom_mpu_table0: SHROM_MPU_TABLE0,
    #[doc = "0x4a8 - "]
    pub shrom_mpu_table1: SHROM_MPU_TABLE1,
    #[doc = "0x4ac - "]
    pub shrom_mpu_table2: SHROM_MPU_TABLE2,
    #[doc = "0x4b0 - "]
    pub shrom_mpu_table3: SHROM_MPU_TABLE3,
    #[doc = "0x4b4 - "]
    pub shrom_mpu_table4: SHROM_MPU_TABLE4,
    #[doc = "0x4b8 - "]
    pub shrom_mpu_table5: SHROM_MPU_TABLE5,
    #[doc = "0x4bc - "]
    pub shrom_mpu_table6: SHROM_MPU_TABLE6,
    #[doc = "0x4c0 - "]
    pub shrom_mpu_table7: SHROM_MPU_TABLE7,
    #[doc = "0x4c4 - "]
    pub shrom_mpu_table8: SHROM_MPU_TABLE8,
    #[doc = "0x4c8 - "]
    pub shrom_mpu_table9: SHROM_MPU_TABLE9,
    #[doc = "0x4cc - "]
    pub shrom_mpu_table10: SHROM_MPU_TABLE10,
    #[doc = "0x4d0 - "]
    pub shrom_mpu_table11: SHROM_MPU_TABLE11,
    #[doc = "0x4d4 - "]
    pub shrom_mpu_table12: SHROM_MPU_TABLE12,
    #[doc = "0x4d8 - "]
    pub shrom_mpu_table13: SHROM_MPU_TABLE13,
    #[doc = "0x4dc - "]
    pub shrom_mpu_table14: SHROM_MPU_TABLE14,
    #[doc = "0x4e0 - "]
    pub shrom_mpu_table15: SHROM_MPU_TABLE15,
    #[doc = "0x4e4 - "]
    pub shrom_mpu_table16: SHROM_MPU_TABLE16,
    #[doc = "0x4e8 - "]
    pub shrom_mpu_table17: SHROM_MPU_TABLE17,
    #[doc = "0x4ec - "]
    pub shrom_mpu_table18: SHROM_MPU_TABLE18,
    #[doc = "0x4f0 - "]
    pub shrom_mpu_table19: SHROM_MPU_TABLE19,
    #[doc = "0x4f4 - "]
    pub shrom_mpu_table20: SHROM_MPU_TABLE20,
    #[doc = "0x4f8 - "]
    pub shrom_mpu_table21: SHROM_MPU_TABLE21,
    #[doc = "0x4fc - "]
    pub shrom_mpu_table22: SHROM_MPU_TABLE22,
    #[doc = "0x500 - "]
    pub shrom_mpu_table23: SHROM_MPU_TABLE23,
    #[doc = "0x504 - "]
    pub immu_table0: IMMU_TABLE0,
    #[doc = "0x508 - "]
    pub immu_table1: IMMU_TABLE1,
    #[doc = "0x50c - "]
    pub immu_table2: IMMU_TABLE2,
    #[doc = "0x510 - "]
    pub immu_table3: IMMU_TABLE3,
    #[doc = "0x514 - "]
    pub immu_table4: IMMU_TABLE4,
    #[doc = "0x518 - "]
    pub immu_table5: IMMU_TABLE5,
    #[doc = "0x51c - "]
    pub immu_table6: IMMU_TABLE6,
    #[doc = "0x520 - "]
    pub immu_table7: IMMU_TABLE7,
    #[doc = "0x524 - "]
    pub immu_table8: IMMU_TABLE8,
    #[doc = "0x528 - "]
    pub immu_table9: IMMU_TABLE9,
    #[doc = "0x52c - "]
    pub immu_table10: IMMU_TABLE10,
    #[doc = "0x530 - "]
    pub immu_table11: IMMU_TABLE11,
    #[doc = "0x534 - "]
    pub immu_table12: IMMU_TABLE12,
    #[doc = "0x538 - "]
    pub immu_table13: IMMU_TABLE13,
    #[doc = "0x53c - "]
    pub immu_table14: IMMU_TABLE14,
    #[doc = "0x540 - "]
    pub immu_table15: IMMU_TABLE15,
    #[doc = "0x544 - "]
    pub dmmu_table0: DMMU_TABLE0,
    #[doc = "0x548 - "]
    pub dmmu_table1: DMMU_TABLE1,
    #[doc = "0x54c - "]
    pub dmmu_table2: DMMU_TABLE2,
    #[doc = "0x550 - "]
    pub dmmu_table3: DMMU_TABLE3,
    #[doc = "0x554 - "]
    pub dmmu_table4: DMMU_TABLE4,
    #[doc = "0x558 - "]
    pub dmmu_table5: DMMU_TABLE5,
    #[doc = "0x55c - "]
    pub dmmu_table6: DMMU_TABLE6,
    #[doc = "0x560 - "]
    pub dmmu_table7: DMMU_TABLE7,
    #[doc = "0x564 - "]
    pub dmmu_table8: DMMU_TABLE8,
    #[doc = "0x568 - "]
    pub dmmu_table9: DMMU_TABLE9,
    #[doc = "0x56c - "]
    pub dmmu_table10: DMMU_TABLE10,
    #[doc = "0x570 - "]
    pub dmmu_table11: DMMU_TABLE11,
    #[doc = "0x574 - "]
    pub dmmu_table12: DMMU_TABLE12,
    #[doc = "0x578 - "]
    pub dmmu_table13: DMMU_TABLE13,
    #[doc = "0x57c - "]
    pub dmmu_table14: DMMU_TABLE14,
    #[doc = "0x580 - "]
    pub dmmu_table15: DMMU_TABLE15,
    #[doc = "0x584 - "]
    pub pro_intrusion_ctrl: PRO_INTRUSION_CTRL,
    #[doc = "0x588 - "]
    pub pro_intrusion_status: PRO_INTRUSION_STATUS,
    #[doc = "0x58c - "]
    pub app_intrusion_ctrl: APP_INTRUSION_CTRL,
    #[doc = "0x590 - "]
    pub app_intrusion_status: APP_INTRUSION_STATUS,
    #[doc = "0x594 - "]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    #[doc = "0x598 - "]
    pub mmu_ia_int_en: MMU_IA_INT_EN,
    #[doc = "0x59c - "]
    pub mpu_ia_int_en: MPU_IA_INT_EN,
    #[doc = "0x5a0 - "]
    pub cache_ia_int_en: CACHE_IA_INT_EN,
    #[doc = "0x5a4 - "]
    pub secure_boot_ctrl: SECURE_BOOT_CTRL,
    #[doc = "0x5a8 - "]
    pub spi_dma_chan_sel: SPI_DMA_CHAN_SEL,
    #[doc = "0x5ac - "]
    pub pro_vecbase_ctrl: PRO_VECBASE_CTRL,
    #[doc = "0x5b0 - "]
    pub pro_vecbase_set: PRO_VECBASE_SET,
    #[doc = "0x5b4 - "]
    pub app_vecbase_ctrl: APP_VECBASE_CTRL,
    #[doc = "0x5b8 - "]
    pub app_vecbase_set: APP_VECBASE_SET,
    _reserved367: [u8; 0x0a40],
    #[doc = "0xffc - "]
    pub date: DATE,
}
#[doc = "PRO_BOOT_REMAP_CTRL (rw) register accessor: an alias for `Reg<PRO_BOOT_REMAP_CTRL_SPEC>`"]
pub type PRO_BOOT_REMAP_CTRL = crate::Reg<pro_boot_remap_ctrl::PRO_BOOT_REMAP_CTRL_SPEC>;
#[doc = ""]
pub mod pro_boot_remap_ctrl;
#[doc = "APP_BOOT_REMAP_CTRL (rw) register accessor: an alias for `Reg<APP_BOOT_REMAP_CTRL_SPEC>`"]
pub type APP_BOOT_REMAP_CTRL = crate::Reg<app_boot_remap_ctrl::APP_BOOT_REMAP_CTRL_SPEC>;
#[doc = ""]
pub mod app_boot_remap_ctrl;
#[doc = "ACCESS_CHECK (r) register accessor: an alias for `Reg<ACCESS_CHECK_SPEC>`"]
pub type ACCESS_CHECK = crate::Reg<access_check::ACCESS_CHECK_SPEC>;
#[doc = ""]
pub mod access_check;
#[doc = "PRO_DPORT_APB_MASK0 (rw) register accessor: an alias for `Reg<PRO_DPORT_APB_MASK0_SPEC>`"]
pub type PRO_DPORT_APB_MASK0 = crate::Reg<pro_dport_apb_mask0::PRO_DPORT_APB_MASK0_SPEC>;
#[doc = ""]
pub mod pro_dport_apb_mask0;
#[doc = "PRO_DPORT_APB_MASK1 (rw) register accessor: an alias for `Reg<PRO_DPORT_APB_MASK1_SPEC>`"]
pub type PRO_DPORT_APB_MASK1 = crate::Reg<pro_dport_apb_mask1::PRO_DPORT_APB_MASK1_SPEC>;
#[doc = ""]
pub mod pro_dport_apb_mask1;
#[doc = "APP_DPORT_APB_MASK0 (rw) register accessor: an alias for `Reg<APP_DPORT_APB_MASK0_SPEC>`"]
pub type APP_DPORT_APB_MASK0 = crate::Reg<app_dport_apb_mask0::APP_DPORT_APB_MASK0_SPEC>;
#[doc = ""]
pub mod app_dport_apb_mask0;
#[doc = "APP_DPORT_APB_MASK1 (rw) register accessor: an alias for `Reg<APP_DPORT_APB_MASK1_SPEC>`"]
pub type APP_DPORT_APB_MASK1 = crate::Reg<app_dport_apb_mask1::APP_DPORT_APB_MASK1_SPEC>;
#[doc = ""]
pub mod app_dport_apb_mask1;
#[doc = "PERI_CLK_EN (rw) register accessor: an alias for `Reg<PERI_CLK_EN_SPEC>`"]
pub type PERI_CLK_EN = crate::Reg<peri_clk_en::PERI_CLK_EN_SPEC>;
#[doc = ""]
pub mod peri_clk_en;
#[doc = "PERI_RST_EN (rw) register accessor: an alias for `Reg<PERI_RST_EN_SPEC>`"]
pub type PERI_RST_EN = crate::Reg<peri_rst_en::PERI_RST_EN_SPEC>;
#[doc = ""]
pub mod peri_rst_en;
#[doc = "WIFI_BB_CFG (rw) register accessor: an alias for `Reg<WIFI_BB_CFG_SPEC>`"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: an alias for `Reg<WIFI_BB_CFG_2_SPEC>`"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg_2;
#[doc = "APPCPU_CTRL_A (rw) register accessor: an alias for `Reg<APPCPU_CTRL_A_SPEC>`"]
pub type APPCPU_CTRL_A = crate::Reg<appcpu_ctrl_a::APPCPU_CTRL_A_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_a;
#[doc = "APPCPU_CTRL_B (rw) register accessor: an alias for `Reg<APPCPU_CTRL_B_SPEC>`"]
pub type APPCPU_CTRL_B = crate::Reg<appcpu_ctrl_b::APPCPU_CTRL_B_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_b;
#[doc = "APPCPU_CTRL_C (rw) register accessor: an alias for `Reg<APPCPU_CTRL_C_SPEC>`"]
pub type APPCPU_CTRL_C = crate::Reg<appcpu_ctrl_c::APPCPU_CTRL_C_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_c;
#[doc = "APPCPU_CTRL_D (rw) register accessor: an alias for `Reg<APPCPU_CTRL_D_SPEC>`"]
pub type APPCPU_CTRL_D = crate::Reg<appcpu_ctrl_d::APPCPU_CTRL_D_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_d;
#[doc = "CPU_PER_CONF (rw) register accessor: an alias for `Reg<CPU_PER_CONF_SPEC>`"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = ""]
pub mod cpu_per_conf;
#[doc = "PRO_CACHE_CTRL (rw) register accessor: an alias for `Reg<PRO_CACHE_CTRL_SPEC>`"]
pub type PRO_CACHE_CTRL = crate::Reg<pro_cache_ctrl::PRO_CACHE_CTRL_SPEC>;
#[doc = ""]
pub mod pro_cache_ctrl;
#[doc = "PRO_CACHE_CTRL1 (rw) register accessor: an alias for `Reg<PRO_CACHE_CTRL1_SPEC>`"]
pub type PRO_CACHE_CTRL1 = crate::Reg<pro_cache_ctrl1::PRO_CACHE_CTRL1_SPEC>;
#[doc = ""]
pub mod pro_cache_ctrl1;
#[doc = "PRO_CACHE_LOCK_0_ADDR (rw) register accessor: an alias for `Reg<PRO_CACHE_LOCK_0_ADDR_SPEC>`"]
pub type PRO_CACHE_LOCK_0_ADDR = crate::Reg<pro_cache_lock_0_addr::PRO_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_0_addr;
#[doc = "PRO_CACHE_LOCK_1_ADDR (rw) register accessor: an alias for `Reg<PRO_CACHE_LOCK_1_ADDR_SPEC>`"]
pub type PRO_CACHE_LOCK_1_ADDR = crate::Reg<pro_cache_lock_1_addr::PRO_CACHE_LOCK_1_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_1_addr;
#[doc = "PRO_CACHE_LOCK_2_ADDR (rw) register accessor: an alias for `Reg<PRO_CACHE_LOCK_2_ADDR_SPEC>`"]
pub type PRO_CACHE_LOCK_2_ADDR = crate::Reg<pro_cache_lock_2_addr::PRO_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_2_addr;
#[doc = "PRO_CACHE_LOCK_3_ADDR (rw) register accessor: an alias for `Reg<PRO_CACHE_LOCK_3_ADDR_SPEC>`"]
pub type PRO_CACHE_LOCK_3_ADDR = crate::Reg<pro_cache_lock_3_addr::PRO_CACHE_LOCK_3_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_3_addr;
#[doc = "APP_CACHE_CTRL (rw) register accessor: an alias for `Reg<APP_CACHE_CTRL_SPEC>`"]
pub type APP_CACHE_CTRL = crate::Reg<app_cache_ctrl::APP_CACHE_CTRL_SPEC>;
#[doc = ""]
pub mod app_cache_ctrl;
#[doc = "APP_CACHE_CTRL1 (rw) register accessor: an alias for `Reg<APP_CACHE_CTRL1_SPEC>`"]
pub type APP_CACHE_CTRL1 = crate::Reg<app_cache_ctrl1::APP_CACHE_CTRL1_SPEC>;
#[doc = ""]
pub mod app_cache_ctrl1;
#[doc = "APP_CACHE_LOCK_0_ADDR (rw) register accessor: an alias for `Reg<APP_CACHE_LOCK_0_ADDR_SPEC>`"]
pub type APP_CACHE_LOCK_0_ADDR = crate::Reg<app_cache_lock_0_addr::APP_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_0_addr;
#[doc = "APP_CACHE_LOCK_1_ADDR (rw) register accessor: an alias for `Reg<APP_CACHE_LOCK_1_ADDR_SPEC>`"]
pub type APP_CACHE_LOCK_1_ADDR = crate::Reg<app_cache_lock_1_addr::APP_CACHE_LOCK_1_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_1_addr;
#[doc = "APP_CACHE_LOCK_2_ADDR (rw) register accessor: an alias for `Reg<APP_CACHE_LOCK_2_ADDR_SPEC>`"]
pub type APP_CACHE_LOCK_2_ADDR = crate::Reg<app_cache_lock_2_addr::APP_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_2_addr;
#[doc = "APP_CACHE_LOCK_3_ADDR (rw) register accessor: an alias for `Reg<APP_CACHE_LOCK_3_ADDR_SPEC>`"]
pub type APP_CACHE_LOCK_3_ADDR = crate::Reg<app_cache_lock_3_addr::APP_CACHE_LOCK_3_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_3_addr;
#[doc = "TRACEMEM_MUX_MODE (rw) register accessor: an alias for `Reg<TRACEMEM_MUX_MODE_SPEC>`"]
pub type TRACEMEM_MUX_MODE = crate::Reg<tracemem_mux_mode::TRACEMEM_MUX_MODE_SPEC>;
#[doc = ""]
pub mod tracemem_mux_mode;
#[doc = "PRO_TRACEMEM_ENA (rw) register accessor: an alias for `Reg<PRO_TRACEMEM_ENA_SPEC>`"]
pub type PRO_TRACEMEM_ENA = crate::Reg<pro_tracemem_ena::PRO_TRACEMEM_ENA_SPEC>;
#[doc = ""]
pub mod pro_tracemem_ena;
#[doc = "APP_TRACEMEM_ENA (rw) register accessor: an alias for `Reg<APP_TRACEMEM_ENA_SPEC>`"]
pub type APP_TRACEMEM_ENA = crate::Reg<app_tracemem_ena::APP_TRACEMEM_ENA_SPEC>;
#[doc = ""]
pub mod app_tracemem_ena;
#[doc = "CACHE_MUX_MODE (rw) register accessor: an alias for `Reg<CACHE_MUX_MODE_SPEC>`"]
pub type CACHE_MUX_MODE = crate::Reg<cache_mux_mode::CACHE_MUX_MODE_SPEC>;
#[doc = ""]
pub mod cache_mux_mode;
#[doc = "IMMU_PAGE_MODE (rw) register accessor: an alias for `Reg<IMMU_PAGE_MODE_SPEC>`"]
pub type IMMU_PAGE_MODE = crate::Reg<immu_page_mode::IMMU_PAGE_MODE_SPEC>;
#[doc = ""]
pub mod immu_page_mode;
#[doc = "DMMU_PAGE_MODE (rw) register accessor: an alias for `Reg<DMMU_PAGE_MODE_SPEC>`"]
pub type DMMU_PAGE_MODE = crate::Reg<dmmu_page_mode::DMMU_PAGE_MODE_SPEC>;
#[doc = ""]
pub mod dmmu_page_mode;
#[doc = "ROM_MPU_ENA (rw) register accessor: an alias for `Reg<ROM_MPU_ENA_SPEC>`"]
pub type ROM_MPU_ENA = crate::Reg<rom_mpu_ena::ROM_MPU_ENA_SPEC>;
#[doc = ""]
pub mod rom_mpu_ena;
#[doc = "MEM_PD_MASK (rw) register accessor: an alias for `Reg<MEM_PD_MASK_SPEC>`"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = ""]
pub mod mem_pd_mask;
#[doc = "ROM_PD_CTRL (rw) register accessor: an alias for `Reg<ROM_PD_CTRL_SPEC>`"]
pub type ROM_PD_CTRL = crate::Reg<rom_pd_ctrl::ROM_PD_CTRL_SPEC>;
#[doc = ""]
pub mod rom_pd_ctrl;
#[doc = "ROM_FO_CTRL (rw) register accessor: an alias for `Reg<ROM_FO_CTRL_SPEC>`"]
pub type ROM_FO_CTRL = crate::Reg<rom_fo_ctrl::ROM_FO_CTRL_SPEC>;
#[doc = ""]
pub mod rom_fo_ctrl;
#[doc = "SRAM_PD_CTRL_0 (rw) register accessor: an alias for `Reg<SRAM_PD_CTRL_0_SPEC>`"]
pub type SRAM_PD_CTRL_0 = crate::Reg<sram_pd_ctrl_0::SRAM_PD_CTRL_0_SPEC>;
#[doc = ""]
pub mod sram_pd_ctrl_0;
#[doc = "SRAM_PD_CTRL_1 (rw) register accessor: an alias for `Reg<SRAM_PD_CTRL_1_SPEC>`"]
pub type SRAM_PD_CTRL_1 = crate::Reg<sram_pd_ctrl_1::SRAM_PD_CTRL_1_SPEC>;
#[doc = ""]
pub mod sram_pd_ctrl_1;
#[doc = "SRAM_FO_CTRL_0 (rw) register accessor: an alias for `Reg<SRAM_FO_CTRL_0_SPEC>`"]
pub type SRAM_FO_CTRL_0 = crate::Reg<sram_fo_ctrl_0::SRAM_FO_CTRL_0_SPEC>;
#[doc = ""]
pub mod sram_fo_ctrl_0;
#[doc = "SRAM_FO_CTRL_1 (rw) register accessor: an alias for `Reg<SRAM_FO_CTRL_1_SPEC>`"]
pub type SRAM_FO_CTRL_1 = crate::Reg<sram_fo_ctrl_1::SRAM_FO_CTRL_1_SPEC>;
#[doc = ""]
pub mod sram_fo_ctrl_1;
#[doc = "IRAM_DRAM_AHB_SEL (rw) register accessor: an alias for `Reg<IRAM_DRAM_AHB_SEL_SPEC>`"]
pub type IRAM_DRAM_AHB_SEL = crate::Reg<iram_dram_ahb_sel::IRAM_DRAM_AHB_SEL_SPEC>;
#[doc = ""]
pub mod iram_dram_ahb_sel;
#[doc = "TAG_FO_CTRL (rw) register accessor: an alias for `Reg<TAG_FO_CTRL_SPEC>`"]
pub type TAG_FO_CTRL = crate::Reg<tag_fo_ctrl::TAG_FO_CTRL_SPEC>;
#[doc = ""]
pub mod tag_fo_ctrl;
#[doc = "AHB_LITE_MASK (rw) register accessor: an alias for `Reg<AHB_LITE_MASK_SPEC>`"]
pub type AHB_LITE_MASK = crate::Reg<ahb_lite_mask::AHB_LITE_MASK_SPEC>;
#[doc = ""]
pub mod ahb_lite_mask;
#[doc = "AHB_MPU_TABLE_0 (rw) register accessor: an alias for `Reg<AHB_MPU_TABLE_0_SPEC>`"]
pub type AHB_MPU_TABLE_0 = crate::Reg<ahb_mpu_table_0::AHB_MPU_TABLE_0_SPEC>;
#[doc = ""]
pub mod ahb_mpu_table_0;
#[doc = "AHB_MPU_TABLE_1 (rw) register accessor: an alias for `Reg<AHB_MPU_TABLE_1_SPEC>`"]
pub type AHB_MPU_TABLE_1 = crate::Reg<ahb_mpu_table_1::AHB_MPU_TABLE_1_SPEC>;
#[doc = ""]
pub mod ahb_mpu_table_1;
#[doc = "HOST_INF_SEL (rw) register accessor: an alias for `Reg<HOST_INF_SEL_SPEC>`"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = ""]
pub mod host_inf_sel;
#[doc = "PERIP_CLK_EN (rw) register accessor: an alias for `Reg<PERIP_CLK_EN_SPEC>`"]
pub type PERIP_CLK_EN = crate::Reg<perip_clk_en::PERIP_CLK_EN_SPEC>;
#[doc = ""]
pub mod perip_clk_en;
#[doc = "PERIP_RST_EN (rw) register accessor: an alias for `Reg<PERIP_RST_EN_SPEC>`"]
pub type PERIP_RST_EN = crate::Reg<perip_rst_en::PERIP_RST_EN_SPEC>;
#[doc = ""]
pub mod perip_rst_en;
#[doc = "SLAVE_SPI_CONFIG (rw) register accessor: an alias for `Reg<SLAVE_SPI_CONFIG_SPEC>`"]
pub type SLAVE_SPI_CONFIG = crate::Reg<slave_spi_config::SLAVE_SPI_CONFIG_SPEC>;
#[doc = ""]
pub mod slave_spi_config;
#[doc = "WIFI_CLK_EN (rw) register accessor: an alias for `Reg<WIFI_CLK_EN_SPEC>`"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = ""]
pub mod wifi_clk_en;
#[doc = "CORE_RST_EN (rw) register accessor: an alias for `Reg<CORE_RST_EN_SPEC>`"]
pub type CORE_RST_EN = crate::Reg<core_rst_en::CORE_RST_EN_SPEC>;
#[doc = ""]
pub mod core_rst_en;
#[doc = "BT_LPCK_DIV_INT (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_INT_SPEC>`"]
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
#[doc = ""]
pub mod bt_lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_FRAC_SPEC>`"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = ""]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_3;
#[doc = "PRO_INTR_STATUS_0 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_0_SPEC>`"]
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
#[doc = ""]
pub mod pro_intr_status_0;
#[doc = "PRO_INTR_STATUS_1 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_1_SPEC>`"]
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
#[doc = ""]
pub mod pro_intr_status_1;
#[doc = "PRO_INTR_STATUS_2 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_2_SPEC>`"]
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
#[doc = ""]
pub mod pro_intr_status_2;
#[doc = "APP_INTR_STATUS_0 (r) register accessor: an alias for `Reg<APP_INTR_STATUS_0_SPEC>`"]
pub type APP_INTR_STATUS_0 = crate::Reg<app_intr_status_0::APP_INTR_STATUS_0_SPEC>;
#[doc = ""]
pub mod app_intr_status_0;
#[doc = "APP_INTR_STATUS_1 (r) register accessor: an alias for `Reg<APP_INTR_STATUS_1_SPEC>`"]
pub type APP_INTR_STATUS_1 = crate::Reg<app_intr_status_1::APP_INTR_STATUS_1_SPEC>;
#[doc = ""]
pub mod app_intr_status_1;
#[doc = "APP_INTR_STATUS_2 (r) register accessor: an alias for `Reg<APP_INTR_STATUS_2_SPEC>`"]
pub type APP_INTR_STATUS_2 = crate::Reg<app_intr_status_2::APP_INTR_STATUS_2_SPEC>;
#[doc = ""]
pub mod app_intr_status_2;
#[doc = "PRO_MAC_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_MAC_INTR_MAP_SPEC>`"]
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_mac_intr_map;
#[doc = "PRO_MAC_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_MAC_NMI_MAP_SPEC>`"]
pub type PRO_MAC_NMI_MAP = crate::Reg<pro_mac_nmi_map::PRO_MAC_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_mac_nmi_map;
#[doc = "PRO_BB_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BB_INT_MAP_SPEC>`"]
pub type PRO_BB_INT_MAP = crate::Reg<pro_bb_int_map::PRO_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bb_int_map;
#[doc = "PRO_BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BT_MAC_INT_MAP_SPEC>`"]
pub type PRO_BT_MAC_INT_MAP = crate::Reg<pro_bt_mac_int_map::PRO_BT_MAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_mac_int_map;
#[doc = "PRO_BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BT_BB_INT_MAP_SPEC>`"]
pub type PRO_BT_BB_INT_MAP = crate::Reg<pro_bt_bb_int_map::PRO_BT_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_bb_int_map;
#[doc = "PRO_BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_BT_BB_NMI_MAP_SPEC>`"]
pub type PRO_BT_BB_NMI_MAP = crate::Reg<pro_bt_bb_nmi_map::PRO_BT_BB_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_bb_nmi_map;
#[doc = "PRO_RWBT_IRQ_MAP (rw) register accessor: an alias for `Reg<PRO_RWBT_IRQ_MAP_SPEC>`"]
pub type PRO_RWBT_IRQ_MAP = crate::Reg<pro_rwbt_irq_map::PRO_RWBT_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwbt_irq_map;
#[doc = "PRO_RWBLE_IRQ_MAP (rw) register accessor: an alias for `Reg<PRO_RWBLE_IRQ_MAP_SPEC>`"]
pub type PRO_RWBLE_IRQ_MAP = crate::Reg<pro_rwble_irq_map::PRO_RWBLE_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwble_irq_map;
#[doc = "PRO_RWBT_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_RWBT_NMI_MAP_SPEC>`"]
pub type PRO_RWBT_NMI_MAP = crate::Reg<pro_rwbt_nmi_map::PRO_RWBT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwbt_nmi_map;
#[doc = "PRO_RWBLE_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_RWBLE_NMI_MAP_SPEC>`"]
pub type PRO_RWBLE_NMI_MAP = crate::Reg<pro_rwble_nmi_map::PRO_RWBLE_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwble_nmi_map;
#[doc = "PRO_SLC0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SLC0_INTR_MAP_SPEC>`"]
pub type PRO_SLC0_INTR_MAP = crate::Reg<pro_slc0_intr_map::PRO_SLC0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_slc0_intr_map;
#[doc = "PRO_SLC1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SLC1_INTR_MAP_SPEC>`"]
pub type PRO_SLC1_INTR_MAP = crate::Reg<pro_slc1_intr_map::PRO_SLC1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_slc1_intr_map;
#[doc = "PRO_UHCI0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UHCI0_INTR_MAP_SPEC>`"]
pub type PRO_UHCI0_INTR_MAP = crate::Reg<pro_uhci0_intr_map::PRO_UHCI0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uhci0_intr_map;
#[doc = "PRO_UHCI1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UHCI1_INTR_MAP_SPEC>`"]
pub type PRO_UHCI1_INTR_MAP = crate::Reg<pro_uhci1_intr_map::PRO_UHCI1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uhci1_intr_map;
#[doc = "PRO_TG_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T0_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t0_level_int_map::PRO_TG_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t0_level_int_map;
#[doc = "PRO_TG_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T1_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t1_level_int_map::PRO_TG_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t1_level_int_map;
#[doc = "PRO_TG_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_wdt_level_int_map::PRO_TG_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_wdt_level_int_map;
#[doc = "PRO_TG_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_lact_level_int_map::PRO_TG_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_lact_level_int_map;
#[doc = "PRO_TG1_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T0_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t0_level_int_map::PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t0_level_int_map;
#[doc = "PRO_TG1_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T1_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t1_level_int_map::PRO_TG1_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t1_level_int_map;
#[doc = "PRO_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_wdt_level_int_map::PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_wdt_level_int_map;
#[doc = "PRO_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_lact_level_int_map::PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_lact_level_int_map;
#[doc = "PRO_GPIO_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_MAP = crate::Reg<pro_gpio_interrupt_map::PRO_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod pro_gpio_interrupt_map;
#[doc = "PRO_GPIO_INTERRUPT_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_NMI_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_nmi_map::PRO_GPIO_INTERRUPT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_gpio_interrupt_nmi_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_0_map::PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_0_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_1_map::PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_1_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_2_map::PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_2_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_3_map::PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_3_map;
#[doc = "PRO_SPI_INTR_0_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_0_MAP_SPEC>`"]
pub type PRO_SPI_INTR_0_MAP = crate::Reg<pro_spi_intr_0_map::PRO_SPI_INTR_0_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_0_map;
#[doc = "PRO_SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_1_MAP_SPEC>`"]
pub type PRO_SPI_INTR_1_MAP = crate::Reg<pro_spi_intr_1_map::PRO_SPI_INTR_1_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_1_map;
#[doc = "PRO_SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_2_MAP_SPEC>`"]
pub type PRO_SPI_INTR_2_MAP = crate::Reg<pro_spi_intr_2_map::PRO_SPI_INTR_2_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_2_map;
#[doc = "PRO_SPI_INTR_3_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_3_MAP_SPEC>`"]
pub type PRO_SPI_INTR_3_MAP = crate::Reg<pro_spi_intr_3_map::PRO_SPI_INTR_3_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_3_map;
#[doc = "PRO_I2S0_INT_MAP (rw) register accessor: an alias for `Reg<PRO_I2S0_INT_MAP_SPEC>`"]
pub type PRO_I2S0_INT_MAP = crate::Reg<pro_i2s0_int_map::PRO_I2S0_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2s0_int_map;
#[doc = "PRO_I2S1_INT_MAP (rw) register accessor: an alias for `Reg<PRO_I2S1_INT_MAP_SPEC>`"]
pub type PRO_I2S1_INT_MAP = crate::Reg<pro_i2s1_int_map::PRO_I2S1_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2s1_int_map;
#[doc = "PRO_UART_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART_INTR_MAP_SPEC>`"]
pub type PRO_UART_INTR_MAP = crate::Reg<pro_uart_intr_map::PRO_UART_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart_intr_map;
#[doc = "PRO_UART1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART1_INTR_MAP_SPEC>`"]
pub type PRO_UART1_INTR_MAP = crate::Reg<pro_uart1_intr_map::PRO_UART1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart1_intr_map;
#[doc = "PRO_UART2_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART2_INTR_MAP_SPEC>`"]
pub type PRO_UART2_INTR_MAP = crate::Reg<pro_uart2_intr_map::PRO_UART2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart2_intr_map;
#[doc = "PRO_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>`"]
pub type PRO_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<pro_sdio_host_interrupt_map::PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod pro_sdio_host_interrupt_map;
#[doc = "PRO_EMAC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_EMAC_INT_MAP_SPEC>`"]
pub type PRO_EMAC_INT_MAP = crate::Reg<pro_emac_int_map::PRO_EMAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_emac_int_map;
#[doc = "PRO_PWM0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM0_INTR_MAP_SPEC>`"]
pub type PRO_PWM0_INTR_MAP = crate::Reg<pro_pwm0_intr_map::PRO_PWM0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm0_intr_map;
#[doc = "PRO_PWM1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM1_INTR_MAP_SPEC>`"]
pub type PRO_PWM1_INTR_MAP = crate::Reg<pro_pwm1_intr_map::PRO_PWM1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm1_intr_map;
#[doc = "PRO_PWM2_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM2_INTR_MAP_SPEC>`"]
pub type PRO_PWM2_INTR_MAP = crate::Reg<pro_pwm2_intr_map::PRO_PWM2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm2_intr_map;
#[doc = "PRO_PWM3_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM3_INTR_MAP_SPEC>`"]
pub type PRO_PWM3_INTR_MAP = crate::Reg<pro_pwm3_intr_map::PRO_PWM3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm3_intr_map;
#[doc = "PRO_LEDC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_LEDC_INT_MAP_SPEC>`"]
pub type PRO_LEDC_INT_MAP = crate::Reg<pro_ledc_int_map::PRO_LEDC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_ledc_int_map;
#[doc = "PRO_EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_EFUSE_INT_MAP_SPEC>`"]
pub type PRO_EFUSE_INT_MAP = crate::Reg<pro_efuse_int_map::PRO_EFUSE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_efuse_int_map;
#[doc = "PRO_CAN_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CAN_INT_MAP_SPEC>`"]
pub type PRO_CAN_INT_MAP = crate::Reg<pro_can_int_map::PRO_CAN_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_can_int_map;
#[doc = "PRO_RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RTC_CORE_INTR_MAP_SPEC>`"]
pub type PRO_RTC_CORE_INTR_MAP = crate::Reg<pro_rtc_core_intr_map::PRO_RTC_CORE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rtc_core_intr_map;
#[doc = "PRO_RMT_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RMT_INTR_MAP_SPEC>`"]
pub type PRO_RMT_INTR_MAP = crate::Reg<pro_rmt_intr_map::PRO_RMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rmt_intr_map;
#[doc = "PRO_PCNT_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PCNT_INTR_MAP_SPEC>`"]
pub type PRO_PCNT_INTR_MAP = crate::Reg<pro_pcnt_intr_map::PRO_PCNT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pcnt_intr_map;
#[doc = "PRO_I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_I2C_EXT0_INTR_MAP_SPEC>`"]
pub type PRO_I2C_EXT0_INTR_MAP = crate::Reg<pro_i2c_ext0_intr_map::PRO_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2c_ext0_intr_map;
#[doc = "PRO_I2C_EXT1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_I2C_EXT1_INTR_MAP_SPEC>`"]
pub type PRO_I2C_EXT1_INTR_MAP = crate::Reg<pro_i2c_ext1_intr_map::PRO_I2C_EXT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2c_ext1_intr_map;
#[doc = "PRO_RSA_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RSA_INTR_MAP_SPEC>`"]
pub type PRO_RSA_INTR_MAP = crate::Reg<pro_rsa_intr_map::PRO_RSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rsa_intr_map;
#[doc = "PRO_SPI1_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI1_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI1_DMA_INT_MAP = crate::Reg<pro_spi1_dma_int_map::PRO_SPI1_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi1_dma_int_map;
#[doc = "PRO_SPI2_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI2_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI2_DMA_INT_MAP = crate::Reg<pro_spi2_dma_int_map::PRO_SPI2_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi2_dma_int_map;
#[doc = "PRO_SPI3_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI3_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI3_DMA_INT_MAP = crate::Reg<pro_spi3_dma_int_map::PRO_SPI3_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi3_dma_int_map;
#[doc = "PRO_WDG_INT_MAP (rw) register accessor: an alias for `Reg<PRO_WDG_INT_MAP_SPEC>`"]
pub type PRO_WDG_INT_MAP = crate::Reg<pro_wdg_int_map::PRO_WDG_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_wdg_int_map;
#[doc = "PRO_TIMER_INT1_MAP (rw) register accessor: an alias for `Reg<PRO_TIMER_INT1_MAP_SPEC>`"]
pub type PRO_TIMER_INT1_MAP = crate::Reg<pro_timer_int1_map::PRO_TIMER_INT1_MAP_SPEC>;
#[doc = ""]
pub mod pro_timer_int1_map;
#[doc = "PRO_TIMER_INT2_MAP (rw) register accessor: an alias for `Reg<PRO_TIMER_INT2_MAP_SPEC>`"]
pub type PRO_TIMER_INT2_MAP = crate::Reg<pro_timer_int2_map::PRO_TIMER_INT2_MAP_SPEC>;
#[doc = ""]
pub mod pro_timer_int2_map;
#[doc = "PRO_TG_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T0_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_T0_EDGE_INT_MAP = crate::Reg<pro_tg_t0_edge_int_map::PRO_TG_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t0_edge_int_map;
#[doc = "PRO_TG_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T1_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_T1_EDGE_INT_MAP = crate::Reg<pro_tg_t1_edge_int_map::PRO_TG_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t1_edge_int_map;
#[doc = "PRO_TG_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_WDT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg_wdt_edge_int_map::PRO_TG_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_wdt_edge_int_map;
#[doc = "PRO_TG_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_LACT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg_lact_edge_int_map::PRO_TG_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_lact_edge_int_map;
#[doc = "PRO_TG1_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T0_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_T0_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t0_edge_int_map::PRO_TG1_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t0_edge_int_map;
#[doc = "PRO_TG1_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T1_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_T1_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t1_edge_int_map::PRO_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t1_edge_int_map;
#[doc = "PRO_TG1_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_WDT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_wdt_edge_int_map::PRO_TG1_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_wdt_edge_int_map;
#[doc = "PRO_TG1_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_LACT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_lact_edge_int_map::PRO_TG1_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_lact_edge_int_map;
#[doc = "PRO_MMU_IA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_MMU_IA_INT_MAP_SPEC>`"]
pub type PRO_MMU_IA_INT_MAP = crate::Reg<pro_mmu_ia_int_map::PRO_MMU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_mmu_ia_int_map;
#[doc = "PRO_MPU_IA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_MPU_IA_INT_MAP_SPEC>`"]
pub type PRO_MPU_IA_INT_MAP = crate::Reg<pro_mpu_ia_int_map::PRO_MPU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_mpu_ia_int_map;
#[doc = "PRO_CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CACHE_IA_INT_MAP_SPEC>`"]
pub type PRO_CACHE_IA_INT_MAP = crate::Reg<pro_cache_ia_int_map::PRO_CACHE_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_cache_ia_int_map;
#[doc = "APP_MAC_INTR_MAP (rw) register accessor: an alias for `Reg<APP_MAC_INTR_MAP_SPEC>`"]
pub type APP_MAC_INTR_MAP = crate::Reg<app_mac_intr_map::APP_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_mac_intr_map;
#[doc = "APP_MAC_NMI_MAP (rw) register accessor: an alias for `Reg<APP_MAC_NMI_MAP_SPEC>`"]
pub type APP_MAC_NMI_MAP = crate::Reg<app_mac_nmi_map::APP_MAC_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_mac_nmi_map;
#[doc = "APP_BB_INT_MAP (rw) register accessor: an alias for `Reg<APP_BB_INT_MAP_SPEC>`"]
pub type APP_BB_INT_MAP = crate::Reg<app_bb_int_map::APP_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bb_int_map;
#[doc = "APP_BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<APP_BT_MAC_INT_MAP_SPEC>`"]
pub type APP_BT_MAC_INT_MAP = crate::Reg<app_bt_mac_int_map::APP_BT_MAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_mac_int_map;
#[doc = "APP_BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<APP_BT_BB_INT_MAP_SPEC>`"]
pub type APP_BT_BB_INT_MAP = crate::Reg<app_bt_bb_int_map::APP_BT_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_bb_int_map;
#[doc = "APP_BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<APP_BT_BB_NMI_MAP_SPEC>`"]
pub type APP_BT_BB_NMI_MAP = crate::Reg<app_bt_bb_nmi_map::APP_BT_BB_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_bb_nmi_map;
#[doc = "APP_RWBT_IRQ_MAP (rw) register accessor: an alias for `Reg<APP_RWBT_IRQ_MAP_SPEC>`"]
pub type APP_RWBT_IRQ_MAP = crate::Reg<app_rwbt_irq_map::APP_RWBT_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod app_rwbt_irq_map;
#[doc = "APP_RWBLE_IRQ_MAP (rw) register accessor: an alias for `Reg<APP_RWBLE_IRQ_MAP_SPEC>`"]
pub type APP_RWBLE_IRQ_MAP = crate::Reg<app_rwble_irq_map::APP_RWBLE_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod app_rwble_irq_map;
#[doc = "APP_RWBT_NMI_MAP (rw) register accessor: an alias for `Reg<APP_RWBT_NMI_MAP_SPEC>`"]
pub type APP_RWBT_NMI_MAP = crate::Reg<app_rwbt_nmi_map::APP_RWBT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_rwbt_nmi_map;
#[doc = "APP_RWBLE_NMI_MAP (rw) register accessor: an alias for `Reg<APP_RWBLE_NMI_MAP_SPEC>`"]
pub type APP_RWBLE_NMI_MAP = crate::Reg<app_rwble_nmi_map::APP_RWBLE_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_rwble_nmi_map;
#[doc = "APP_SLC0_INTR_MAP (rw) register accessor: an alias for `Reg<APP_SLC0_INTR_MAP_SPEC>`"]
pub type APP_SLC0_INTR_MAP = crate::Reg<app_slc0_intr_map::APP_SLC0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_slc0_intr_map;
#[doc = "APP_SLC1_INTR_MAP (rw) register accessor: an alias for `Reg<APP_SLC1_INTR_MAP_SPEC>`"]
pub type APP_SLC1_INTR_MAP = crate::Reg<app_slc1_intr_map::APP_SLC1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_slc1_intr_map;
#[doc = "APP_UHCI0_INTR_MAP (rw) register accessor: an alias for `Reg<APP_UHCI0_INTR_MAP_SPEC>`"]
pub type APP_UHCI0_INTR_MAP = crate::Reg<app_uhci0_intr_map::APP_UHCI0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uhci0_intr_map;
#[doc = "APP_UHCI1_INTR_MAP (rw) register accessor: an alias for `Reg<APP_UHCI1_INTR_MAP_SPEC>`"]
pub type APP_UHCI1_INTR_MAP = crate::Reg<app_uhci1_intr_map::APP_UHCI1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uhci1_intr_map;
#[doc = "APP_TG_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_T0_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG_T0_LEVEL_INT_MAP =
    crate::Reg<app_tg_t0_level_int_map::APP_TG_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t0_level_int_map;
#[doc = "APP_TG_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_T1_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG_T1_LEVEL_INT_MAP =
    crate::Reg<app_tg_t1_level_int_map::APP_TG_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t1_level_int_map;
#[doc = "APP_TG_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<app_tg_wdt_level_int_map::APP_TG_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_wdt_level_int_map;
#[doc = "APP_TG_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<app_tg_lact_level_int_map::APP_TG_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_lact_level_int_map;
#[doc = "APP_TG1_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_T0_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<app_tg1_t0_level_int_map::APP_TG1_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t0_level_int_map;
#[doc = "APP_TG1_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_T1_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<app_tg1_t1_level_int_map::APP_TG1_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t1_level_int_map;
#[doc = "APP_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<app_tg1_wdt_level_int_map::APP_TG1_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_wdt_level_int_map;
#[doc = "APP_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type APP_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<app_tg1_lact_level_int_map::APP_TG1_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_lact_level_int_map;
#[doc = "APP_GPIO_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<APP_GPIO_INTERRUPT_MAP_SPEC>`"]
pub type APP_GPIO_INTERRUPT_MAP = crate::Reg<app_gpio_interrupt_map::APP_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod app_gpio_interrupt_map;
#[doc = "APP_GPIO_INTERRUPT_NMI_MAP (rw) register accessor: an alias for `Reg<APP_GPIO_INTERRUPT_NMI_MAP_SPEC>`"]
pub type APP_GPIO_INTERRUPT_NMI_MAP =
    crate::Reg<app_gpio_interrupt_nmi_map::APP_GPIO_INTERRUPT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_gpio_interrupt_nmi_map;
#[doc = "APP_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<APP_CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type APP_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<app_cpu_intr_from_cpu_0_map::APP_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_0_map;
#[doc = "APP_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<APP_CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type APP_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<app_cpu_intr_from_cpu_1_map::APP_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_1_map;
#[doc = "APP_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<APP_CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type APP_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<app_cpu_intr_from_cpu_2_map::APP_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_2_map;
#[doc = "APP_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<APP_CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type APP_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<app_cpu_intr_from_cpu_3_map::APP_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_3_map;
#[doc = "APP_SPI_INTR_0_MAP (rw) register accessor: an alias for `Reg<APP_SPI_INTR_0_MAP_SPEC>`"]
pub type APP_SPI_INTR_0_MAP = crate::Reg<app_spi_intr_0_map::APP_SPI_INTR_0_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_0_map;
#[doc = "APP_SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<APP_SPI_INTR_1_MAP_SPEC>`"]
pub type APP_SPI_INTR_1_MAP = crate::Reg<app_spi_intr_1_map::APP_SPI_INTR_1_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_1_map;
#[doc = "APP_SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<APP_SPI_INTR_2_MAP_SPEC>`"]
pub type APP_SPI_INTR_2_MAP = crate::Reg<app_spi_intr_2_map::APP_SPI_INTR_2_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_2_map;
#[doc = "APP_SPI_INTR_3_MAP (rw) register accessor: an alias for `Reg<APP_SPI_INTR_3_MAP_SPEC>`"]
pub type APP_SPI_INTR_3_MAP = crate::Reg<app_spi_intr_3_map::APP_SPI_INTR_3_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_3_map;
#[doc = "APP_I2S0_INT_MAP (rw) register accessor: an alias for `Reg<APP_I2S0_INT_MAP_SPEC>`"]
pub type APP_I2S0_INT_MAP = crate::Reg<app_i2s0_int_map::APP_I2S0_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_i2s0_int_map;
#[doc = "APP_I2S1_INT_MAP (rw) register accessor: an alias for `Reg<APP_I2S1_INT_MAP_SPEC>`"]
pub type APP_I2S1_INT_MAP = crate::Reg<app_i2s1_int_map::APP_I2S1_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_i2s1_int_map;
#[doc = "APP_UART_INTR_MAP (rw) register accessor: an alias for `Reg<APP_UART_INTR_MAP_SPEC>`"]
pub type APP_UART_INTR_MAP = crate::Reg<app_uart_intr_map::APP_UART_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart_intr_map;
#[doc = "APP_UART1_INTR_MAP (rw) register accessor: an alias for `Reg<APP_UART1_INTR_MAP_SPEC>`"]
pub type APP_UART1_INTR_MAP = crate::Reg<app_uart1_intr_map::APP_UART1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart1_intr_map;
#[doc = "APP_UART2_INTR_MAP (rw) register accessor: an alias for `Reg<APP_UART2_INTR_MAP_SPEC>`"]
pub type APP_UART2_INTR_MAP = crate::Reg<app_uart2_intr_map::APP_UART2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart2_intr_map;
#[doc = "APP_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<APP_SDIO_HOST_INTERRUPT_MAP_SPEC>`"]
pub type APP_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<app_sdio_host_interrupt_map::APP_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod app_sdio_host_interrupt_map;
#[doc = "APP_EMAC_INT_MAP (rw) register accessor: an alias for `Reg<APP_EMAC_INT_MAP_SPEC>`"]
pub type APP_EMAC_INT_MAP = crate::Reg<app_emac_int_map::APP_EMAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_emac_int_map;
#[doc = "APP_PWM0_INTR_MAP (rw) register accessor: an alias for `Reg<APP_PWM0_INTR_MAP_SPEC>`"]
pub type APP_PWM0_INTR_MAP = crate::Reg<app_pwm0_intr_map::APP_PWM0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm0_intr_map;
#[doc = "APP_PWM1_INTR_MAP (rw) register accessor: an alias for `Reg<APP_PWM1_INTR_MAP_SPEC>`"]
pub type APP_PWM1_INTR_MAP = crate::Reg<app_pwm1_intr_map::APP_PWM1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm1_intr_map;
#[doc = "APP_PWM2_INTR_MAP (rw) register accessor: an alias for `Reg<APP_PWM2_INTR_MAP_SPEC>`"]
pub type APP_PWM2_INTR_MAP = crate::Reg<app_pwm2_intr_map::APP_PWM2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm2_intr_map;
#[doc = "APP_PWM3_INTR_MAP (rw) register accessor: an alias for `Reg<APP_PWM3_INTR_MAP_SPEC>`"]
pub type APP_PWM3_INTR_MAP = crate::Reg<app_pwm3_intr_map::APP_PWM3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm3_intr_map;
#[doc = "APP_LEDC_INT_MAP (rw) register accessor: an alias for `Reg<APP_LEDC_INT_MAP_SPEC>`"]
pub type APP_LEDC_INT_MAP = crate::Reg<app_ledc_int_map::APP_LEDC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_ledc_int_map;
#[doc = "APP_EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<APP_EFUSE_INT_MAP_SPEC>`"]
pub type APP_EFUSE_INT_MAP = crate::Reg<app_efuse_int_map::APP_EFUSE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_efuse_int_map;
#[doc = "APP_CAN_INT_MAP (rw) register accessor: an alias for `Reg<APP_CAN_INT_MAP_SPEC>`"]
pub type APP_CAN_INT_MAP = crate::Reg<app_can_int_map::APP_CAN_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_can_int_map;
#[doc = "APP_RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<APP_RTC_CORE_INTR_MAP_SPEC>`"]
pub type APP_RTC_CORE_INTR_MAP = crate::Reg<app_rtc_core_intr_map::APP_RTC_CORE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rtc_core_intr_map;
#[doc = "APP_RMT_INTR_MAP (rw) register accessor: an alias for `Reg<APP_RMT_INTR_MAP_SPEC>`"]
pub type APP_RMT_INTR_MAP = crate::Reg<app_rmt_intr_map::APP_RMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rmt_intr_map;
#[doc = "APP_PCNT_INTR_MAP (rw) register accessor: an alias for `Reg<APP_PCNT_INTR_MAP_SPEC>`"]
pub type APP_PCNT_INTR_MAP = crate::Reg<app_pcnt_intr_map::APP_PCNT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pcnt_intr_map;
#[doc = "APP_I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<APP_I2C_EXT0_INTR_MAP_SPEC>`"]
pub type APP_I2C_EXT0_INTR_MAP = crate::Reg<app_i2c_ext0_intr_map::APP_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_i2c_ext0_intr_map;
#[doc = "APP_I2C_EXT1_INTR_MAP (rw) register accessor: an alias for `Reg<APP_I2C_EXT1_INTR_MAP_SPEC>`"]
pub type APP_I2C_EXT1_INTR_MAP = crate::Reg<app_i2c_ext1_intr_map::APP_I2C_EXT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_i2c_ext1_intr_map;
#[doc = "APP_RSA_INTR_MAP (rw) register accessor: an alias for `Reg<APP_RSA_INTR_MAP_SPEC>`"]
pub type APP_RSA_INTR_MAP = crate::Reg<app_rsa_intr_map::APP_RSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rsa_intr_map;
#[doc = "APP_SPI1_DMA_INT_MAP (rw) register accessor: an alias for `Reg<APP_SPI1_DMA_INT_MAP_SPEC>`"]
pub type APP_SPI1_DMA_INT_MAP = crate::Reg<app_spi1_dma_int_map::APP_SPI1_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi1_dma_int_map;
#[doc = "APP_SPI2_DMA_INT_MAP (rw) register accessor: an alias for `Reg<APP_SPI2_DMA_INT_MAP_SPEC>`"]
pub type APP_SPI2_DMA_INT_MAP = crate::Reg<app_spi2_dma_int_map::APP_SPI2_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi2_dma_int_map;
#[doc = "APP_SPI3_DMA_INT_MAP (rw) register accessor: an alias for `Reg<APP_SPI3_DMA_INT_MAP_SPEC>`"]
pub type APP_SPI3_DMA_INT_MAP = crate::Reg<app_spi3_dma_int_map::APP_SPI3_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi3_dma_int_map;
#[doc = "APP_WDG_INT_MAP (rw) register accessor: an alias for `Reg<APP_WDG_INT_MAP_SPEC>`"]
pub type APP_WDG_INT_MAP = crate::Reg<app_wdg_int_map::APP_WDG_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_wdg_int_map;
#[doc = "APP_TIMER_INT1_MAP (rw) register accessor: an alias for `Reg<APP_TIMER_INT1_MAP_SPEC>`"]
pub type APP_TIMER_INT1_MAP = crate::Reg<app_timer_int1_map::APP_TIMER_INT1_MAP_SPEC>;
#[doc = ""]
pub mod app_timer_int1_map;
#[doc = "APP_TIMER_INT2_MAP (rw) register accessor: an alias for `Reg<APP_TIMER_INT2_MAP_SPEC>`"]
pub type APP_TIMER_INT2_MAP = crate::Reg<app_timer_int2_map::APP_TIMER_INT2_MAP_SPEC>;
#[doc = ""]
pub mod app_timer_int2_map;
#[doc = "APP_TG_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_T0_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG_T0_EDGE_INT_MAP = crate::Reg<app_tg_t0_edge_int_map::APP_TG_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t0_edge_int_map;
#[doc = "APP_TG_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_T1_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG_T1_EDGE_INT_MAP = crate::Reg<app_tg_t1_edge_int_map::APP_TG_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t1_edge_int_map;
#[doc = "APP_TG_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_WDT_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG_WDT_EDGE_INT_MAP =
    crate::Reg<app_tg_wdt_edge_int_map::APP_TG_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_wdt_edge_int_map;
#[doc = "APP_TG_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG_LACT_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG_LACT_EDGE_INT_MAP =
    crate::Reg<app_tg_lact_edge_int_map::APP_TG_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_lact_edge_int_map;
#[doc = "APP_TG1_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_T0_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG1_T0_EDGE_INT_MAP =
    crate::Reg<app_tg1_t0_edge_int_map::APP_TG1_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t0_edge_int_map;
#[doc = "APP_TG1_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_T1_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG1_T1_EDGE_INT_MAP =
    crate::Reg<app_tg1_t1_edge_int_map::APP_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t1_edge_int_map;
#[doc = "APP_TG1_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_WDT_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<app_tg1_wdt_edge_int_map::APP_TG1_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_wdt_edge_int_map;
#[doc = "APP_TG1_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<APP_TG1_LACT_EDGE_INT_MAP_SPEC>`"]
pub type APP_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<app_tg1_lact_edge_int_map::APP_TG1_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_lact_edge_int_map;
#[doc = "APP_MMU_IA_INT_MAP (rw) register accessor: an alias for `Reg<APP_MMU_IA_INT_MAP_SPEC>`"]
pub type APP_MMU_IA_INT_MAP = crate::Reg<app_mmu_ia_int_map::APP_MMU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_mmu_ia_int_map;
#[doc = "APP_MPU_IA_INT_MAP (rw) register accessor: an alias for `Reg<APP_MPU_IA_INT_MAP_SPEC>`"]
pub type APP_MPU_IA_INT_MAP = crate::Reg<app_mpu_ia_int_map::APP_MPU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_mpu_ia_int_map;
#[doc = "APP_CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<APP_CACHE_IA_INT_MAP_SPEC>`"]
pub type APP_CACHE_IA_INT_MAP = crate::Reg<app_cache_ia_int_map::APP_CACHE_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_cache_ia_int_map;
#[doc = "AHBLITE_MPU_TABLE_UART (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_UART_SPEC>`"]
pub type AHBLITE_MPU_TABLE_UART = crate::Reg<ahblite_mpu_table_uart::AHBLITE_MPU_TABLE_UART_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart;
#[doc = "AHBLITE_MPU_TABLE_SPI1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SPI1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SPI1 = crate::Reg<ahblite_mpu_table_spi1::AHBLITE_MPU_TABLE_SPI1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi1;
#[doc = "AHBLITE_MPU_TABLE_SPI0 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SPI0_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SPI0 = crate::Reg<ahblite_mpu_table_spi0::AHBLITE_MPU_TABLE_SPI0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi0;
#[doc = "AHBLITE_MPU_TABLE_GPIO (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_GPIO_SPEC>`"]
pub type AHBLITE_MPU_TABLE_GPIO = crate::Reg<ahblite_mpu_table_gpio::AHBLITE_MPU_TABLE_GPIO_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_gpio;
#[doc = "AHBLITE_MPU_TABLE_FE2 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_FE2_SPEC>`"]
pub type AHBLITE_MPU_TABLE_FE2 = crate::Reg<ahblite_mpu_table_fe2::AHBLITE_MPU_TABLE_FE2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_fe2;
#[doc = "AHBLITE_MPU_TABLE_FE (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_FE_SPEC>`"]
pub type AHBLITE_MPU_TABLE_FE = crate::Reg<ahblite_mpu_table_fe::AHBLITE_MPU_TABLE_FE_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_fe;
#[doc = "AHBLITE_MPU_TABLE_TIMER (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_TIMER_SPEC>`"]
pub type AHBLITE_MPU_TABLE_TIMER =
    crate::Reg<ahblite_mpu_table_timer::AHBLITE_MPU_TABLE_TIMER_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timer;
#[doc = "AHBLITE_MPU_TABLE_RTC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_RTC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_RTC = crate::Reg<ahblite_mpu_table_rtc::AHBLITE_MPU_TABLE_RTC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rtc;
#[doc = "AHBLITE_MPU_TABLE_IO_MUX (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_IO_MUX_SPEC>`"]
pub type AHBLITE_MPU_TABLE_IO_MUX =
    crate::Reg<ahblite_mpu_table_io_mux::AHBLITE_MPU_TABLE_IO_MUX_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_io_mux;
#[doc = "AHBLITE_MPU_TABLE_WDG (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_WDG_SPEC>`"]
pub type AHBLITE_MPU_TABLE_WDG = crate::Reg<ahblite_mpu_table_wdg::AHBLITE_MPU_TABLE_WDG_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_wdg;
#[doc = "AHBLITE_MPU_TABLE_HINF (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_HINF_SPEC>`"]
pub type AHBLITE_MPU_TABLE_HINF = crate::Reg<ahblite_mpu_table_hinf::AHBLITE_MPU_TABLE_HINF_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_hinf;
#[doc = "AHBLITE_MPU_TABLE_UHCI1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_UHCI1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_UHCI1 =
    crate::Reg<ahblite_mpu_table_uhci1::AHBLITE_MPU_TABLE_UHCI1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uhci1;
#[doc = "AHBLITE_MPU_TABLE_MISC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_MISC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_MISC = crate::Reg<ahblite_mpu_table_misc::AHBLITE_MPU_TABLE_MISC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_misc;
#[doc = "AHBLITE_MPU_TABLE_I2C (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_I2C_SPEC>`"]
pub type AHBLITE_MPU_TABLE_I2C = crate::Reg<ahblite_mpu_table_i2c::AHBLITE_MPU_TABLE_I2C_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c;
#[doc = "AHBLITE_MPU_TABLE_I2S0 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_I2S0_SPEC>`"]
pub type AHBLITE_MPU_TABLE_I2S0 = crate::Reg<ahblite_mpu_table_i2s0::AHBLITE_MPU_TABLE_I2S0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2s0;
#[doc = "AHBLITE_MPU_TABLE_UART1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_UART1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_UART1 =
    crate::Reg<ahblite_mpu_table_uart1::AHBLITE_MPU_TABLE_UART1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart1;
#[doc = "AHBLITE_MPU_TABLE_BT (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_BT_SPEC>`"]
pub type AHBLITE_MPU_TABLE_BT = crate::Reg<ahblite_mpu_table_bt::AHBLITE_MPU_TABLE_BT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bt;
#[doc = "AHBLITE_MPU_TABLE_BT_BUFFER (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_BT_BUFFER_SPEC>`"]
pub type AHBLITE_MPU_TABLE_BT_BUFFER =
    crate::Reg<ahblite_mpu_table_bt_buffer::AHBLITE_MPU_TABLE_BT_BUFFER_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bt_buffer;
#[doc = "AHBLITE_MPU_TABLE_I2C_EXT0 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_I2C_EXT0_SPEC>`"]
pub type AHBLITE_MPU_TABLE_I2C_EXT0 =
    crate::Reg<ahblite_mpu_table_i2c_ext0::AHBLITE_MPU_TABLE_I2C_EXT0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c_ext0;
#[doc = "AHBLITE_MPU_TABLE_UHCI0 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_UHCI0_SPEC>`"]
pub type AHBLITE_MPU_TABLE_UHCI0 =
    crate::Reg<ahblite_mpu_table_uhci0::AHBLITE_MPU_TABLE_UHCI0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uhci0;
#[doc = "AHBLITE_MPU_TABLE_SLCHOST (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SLCHOST_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SLCHOST =
    crate::Reg<ahblite_mpu_table_slchost::AHBLITE_MPU_TABLE_SLCHOST_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_slchost;
#[doc = "AHBLITE_MPU_TABLE_RMT (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_RMT_SPEC>`"]
pub type AHBLITE_MPU_TABLE_RMT = crate::Reg<ahblite_mpu_table_rmt::AHBLITE_MPU_TABLE_RMT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rmt;
#[doc = "AHBLITE_MPU_TABLE_PCNT (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PCNT_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PCNT = crate::Reg<ahblite_mpu_table_pcnt::AHBLITE_MPU_TABLE_PCNT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pcnt;
#[doc = "AHBLITE_MPU_TABLE_SLC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SLC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SLC = crate::Reg<ahblite_mpu_table_slc::AHBLITE_MPU_TABLE_SLC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_slc;
#[doc = "AHBLITE_MPU_TABLE_LEDC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_LEDC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_LEDC = crate::Reg<ahblite_mpu_table_ledc::AHBLITE_MPU_TABLE_LEDC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_ledc;
#[doc = "AHBLITE_MPU_TABLE_EFUSE (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_EFUSE_SPEC>`"]
pub type AHBLITE_MPU_TABLE_EFUSE =
    crate::Reg<ahblite_mpu_table_efuse::AHBLITE_MPU_TABLE_EFUSE_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_efuse;
#[doc = "AHBLITE_MPU_TABLE_SPI_ENCRYPT (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SPI_ENCRYPT_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SPI_ENCRYPT =
    crate::Reg<ahblite_mpu_table_spi_encrypt::AHBLITE_MPU_TABLE_SPI_ENCRYPT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi_encrypt;
#[doc = "AHBLITE_MPU_TABLE_BB (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_BB_SPEC>`"]
pub type AHBLITE_MPU_TABLE_BB = crate::Reg<ahblite_mpu_table_bb::AHBLITE_MPU_TABLE_BB_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bb;
#[doc = "AHBLITE_MPU_TABLE_PWM0 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PWM0_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PWM0 = crate::Reg<ahblite_mpu_table_pwm0::AHBLITE_MPU_TABLE_PWM0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm0;
#[doc = "AHBLITE_MPU_TABLE_TIMERGROUP (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_TIMERGROUP_SPEC>`"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP =
    crate::Reg<ahblite_mpu_table_timergroup::AHBLITE_MPU_TABLE_TIMERGROUP_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timergroup;
#[doc = "AHBLITE_MPU_TABLE_TIMERGROUP1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_TIMERGROUP1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP1 =
    crate::Reg<ahblite_mpu_table_timergroup1::AHBLITE_MPU_TABLE_TIMERGROUP1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timergroup1;
#[doc = "AHBLITE_MPU_TABLE_SPI2 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SPI2_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SPI2 = crate::Reg<ahblite_mpu_table_spi2::AHBLITE_MPU_TABLE_SPI2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi2;
#[doc = "AHBLITE_MPU_TABLE_SPI3 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SPI3_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SPI3 = crate::Reg<ahblite_mpu_table_spi3::AHBLITE_MPU_TABLE_SPI3_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi3;
#[doc = "AHBLITE_MPU_TABLE_APB_CTRL (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_APB_CTRL_SPEC>`"]
pub type AHBLITE_MPU_TABLE_APB_CTRL =
    crate::Reg<ahblite_mpu_table_apb_ctrl::AHBLITE_MPU_TABLE_APB_CTRL_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_apb_ctrl;
#[doc = "AHBLITE_MPU_TABLE_I2C_EXT1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_I2C_EXT1 =
    crate::Reg<ahblite_mpu_table_i2c_ext1::AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c_ext1;
#[doc = "AHBLITE_MPU_TABLE_SDIO_HOST (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_SDIO_HOST_SPEC>`"]
pub type AHBLITE_MPU_TABLE_SDIO_HOST =
    crate::Reg<ahblite_mpu_table_sdio_host::AHBLITE_MPU_TABLE_SDIO_HOST_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_sdio_host;
#[doc = "AHBLITE_MPU_TABLE_EMAC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_EMAC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_EMAC = crate::Reg<ahblite_mpu_table_emac::AHBLITE_MPU_TABLE_EMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_emac;
#[doc = "AHBLITE_MPU_TABLE_CAN (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_CAN_SPEC>`"]
pub type AHBLITE_MPU_TABLE_CAN = crate::Reg<ahblite_mpu_table_can::AHBLITE_MPU_TABLE_CAN_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_can;
#[doc = "AHBLITE_MPU_TABLE_PWM1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PWM1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PWM1 = crate::Reg<ahblite_mpu_table_pwm1::AHBLITE_MPU_TABLE_PWM1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm1;
#[doc = "AHBLITE_MPU_TABLE_I2S1 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_I2S1_SPEC>`"]
pub type AHBLITE_MPU_TABLE_I2S1 = crate::Reg<ahblite_mpu_table_i2s1::AHBLITE_MPU_TABLE_I2S1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2s1;
#[doc = "AHBLITE_MPU_TABLE_UART2 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_UART2_SPEC>`"]
pub type AHBLITE_MPU_TABLE_UART2 =
    crate::Reg<ahblite_mpu_table_uart2::AHBLITE_MPU_TABLE_UART2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart2;
#[doc = "AHBLITE_MPU_TABLE_PWM2 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PWM2_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PWM2 = crate::Reg<ahblite_mpu_table_pwm2::AHBLITE_MPU_TABLE_PWM2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm2;
#[doc = "AHBLITE_MPU_TABLE_PWM3 (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PWM3_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PWM3 = crate::Reg<ahblite_mpu_table_pwm3::AHBLITE_MPU_TABLE_PWM3_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm3;
#[doc = "AHBLITE_MPU_TABLE_RWBT (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_RWBT_SPEC>`"]
pub type AHBLITE_MPU_TABLE_RWBT = crate::Reg<ahblite_mpu_table_rwbt::AHBLITE_MPU_TABLE_RWBT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rwbt;
#[doc = "AHBLITE_MPU_TABLE_BTMAC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_BTMAC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_BTMAC =
    crate::Reg<ahblite_mpu_table_btmac::AHBLITE_MPU_TABLE_BTMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_btmac;
#[doc = "AHBLITE_MPU_TABLE_WIFIMAC (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_WIFIMAC_SPEC>`"]
pub type AHBLITE_MPU_TABLE_WIFIMAC =
    crate::Reg<ahblite_mpu_table_wifimac::AHBLITE_MPU_TABLE_WIFIMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_wifimac;
#[doc = "AHBLITE_MPU_TABLE_PWR (rw) register accessor: an alias for `Reg<AHBLITE_MPU_TABLE_PWR_SPEC>`"]
pub type AHBLITE_MPU_TABLE_PWR = crate::Reg<ahblite_mpu_table_pwr::AHBLITE_MPU_TABLE_PWR_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwr;
#[doc = "MEM_ACCESS_DBUG0 (r) register accessor: an alias for `Reg<MEM_ACCESS_DBUG0_SPEC>`"]
pub type MEM_ACCESS_DBUG0 = crate::Reg<mem_access_dbug0::MEM_ACCESS_DBUG0_SPEC>;
#[doc = ""]
pub mod mem_access_dbug0;
#[doc = "MEM_ACCESS_DBUG1 (r) register accessor: an alias for `Reg<MEM_ACCESS_DBUG1_SPEC>`"]
pub type MEM_ACCESS_DBUG1 = crate::Reg<mem_access_dbug1::MEM_ACCESS_DBUG1_SPEC>;
#[doc = ""]
pub mod mem_access_dbug1;
#[doc = "PRO_DCACHE_DBUG0 (rw) register accessor: an alias for `Reg<PRO_DCACHE_DBUG0_SPEC>`"]
pub type PRO_DCACHE_DBUG0 = crate::Reg<pro_dcache_dbug0::PRO_DCACHE_DBUG0_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug0;
#[doc = "PRO_DCACHE_DBUG1 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG1_SPEC>`"]
pub type PRO_DCACHE_DBUG1 = crate::Reg<pro_dcache_dbug1::PRO_DCACHE_DBUG1_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug1;
#[doc = "PRO_DCACHE_DBUG2 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG2_SPEC>`"]
pub type PRO_DCACHE_DBUG2 = crate::Reg<pro_dcache_dbug2::PRO_DCACHE_DBUG2_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug2;
#[doc = "PRO_DCACHE_DBUG3 (rw) register accessor: an alias for `Reg<PRO_DCACHE_DBUG3_SPEC>`"]
pub type PRO_DCACHE_DBUG3 = crate::Reg<pro_dcache_dbug3::PRO_DCACHE_DBUG3_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug3;
#[doc = "PRO_DCACHE_DBUG4 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG4_SPEC>`"]
pub type PRO_DCACHE_DBUG4 = crate::Reg<pro_dcache_dbug4::PRO_DCACHE_DBUG4_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug4;
#[doc = "PRO_DCACHE_DBUG5 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG5_SPEC>`"]
pub type PRO_DCACHE_DBUG5 = crate::Reg<pro_dcache_dbug5::PRO_DCACHE_DBUG5_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug5;
#[doc = "PRO_DCACHE_DBUG6 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG6_SPEC>`"]
pub type PRO_DCACHE_DBUG6 = crate::Reg<pro_dcache_dbug6::PRO_DCACHE_DBUG6_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug6;
#[doc = "PRO_DCACHE_DBUG7 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG7_SPEC>`"]
pub type PRO_DCACHE_DBUG7 = crate::Reg<pro_dcache_dbug7::PRO_DCACHE_DBUG7_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug7;
#[doc = "PRO_DCACHE_DBUG8 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG8_SPEC>`"]
pub type PRO_DCACHE_DBUG8 = crate::Reg<pro_dcache_dbug8::PRO_DCACHE_DBUG8_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug8;
#[doc = "PRO_DCACHE_DBUG9 (r) register accessor: an alias for `Reg<PRO_DCACHE_DBUG9_SPEC>`"]
pub type PRO_DCACHE_DBUG9 = crate::Reg<pro_dcache_dbug9::PRO_DCACHE_DBUG9_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug9;
#[doc = "APP_DCACHE_DBUG0 (rw) register accessor: an alias for `Reg<APP_DCACHE_DBUG0_SPEC>`"]
pub type APP_DCACHE_DBUG0 = crate::Reg<app_dcache_dbug0::APP_DCACHE_DBUG0_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug0;
#[doc = "APP_DCACHE_DBUG1 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG1_SPEC>`"]
pub type APP_DCACHE_DBUG1 = crate::Reg<app_dcache_dbug1::APP_DCACHE_DBUG1_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug1;
#[doc = "APP_DCACHE_DBUG2 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG2_SPEC>`"]
pub type APP_DCACHE_DBUG2 = crate::Reg<app_dcache_dbug2::APP_DCACHE_DBUG2_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug2;
#[doc = "APP_DCACHE_DBUG3 (rw) register accessor: an alias for `Reg<APP_DCACHE_DBUG3_SPEC>`"]
pub type APP_DCACHE_DBUG3 = crate::Reg<app_dcache_dbug3::APP_DCACHE_DBUG3_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug3;
#[doc = "APP_DCACHE_DBUG4 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG4_SPEC>`"]
pub type APP_DCACHE_DBUG4 = crate::Reg<app_dcache_dbug4::APP_DCACHE_DBUG4_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug4;
#[doc = "APP_DCACHE_DBUG5 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG5_SPEC>`"]
pub type APP_DCACHE_DBUG5 = crate::Reg<app_dcache_dbug5::APP_DCACHE_DBUG5_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug5;
#[doc = "APP_DCACHE_DBUG6 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG6_SPEC>`"]
pub type APP_DCACHE_DBUG6 = crate::Reg<app_dcache_dbug6::APP_DCACHE_DBUG6_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug6;
#[doc = "APP_DCACHE_DBUG7 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG7_SPEC>`"]
pub type APP_DCACHE_DBUG7 = crate::Reg<app_dcache_dbug7::APP_DCACHE_DBUG7_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug7;
#[doc = "APP_DCACHE_DBUG8 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG8_SPEC>`"]
pub type APP_DCACHE_DBUG8 = crate::Reg<app_dcache_dbug8::APP_DCACHE_DBUG8_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug8;
#[doc = "APP_DCACHE_DBUG9 (r) register accessor: an alias for `Reg<APP_DCACHE_DBUG9_SPEC>`"]
pub type APP_DCACHE_DBUG9 = crate::Reg<app_dcache_dbug9::APP_DCACHE_DBUG9_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug9;
#[doc = "PRO_CPU_RECORD_CTRL (rw) register accessor: an alias for `Reg<PRO_CPU_RECORD_CTRL_SPEC>`"]
pub type PRO_CPU_RECORD_CTRL = crate::Reg<pro_cpu_record_ctrl::PRO_CPU_RECORD_CTRL_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_ctrl;
#[doc = "PRO_CPU_RECORD_STATUS (r) register accessor: an alias for `Reg<PRO_CPU_RECORD_STATUS_SPEC>`"]
pub type PRO_CPU_RECORD_STATUS = crate::Reg<pro_cpu_record_status::PRO_CPU_RECORD_STATUS_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_status;
#[doc = "PRO_CPU_RECORD_PID (r) register accessor: an alias for `Reg<PRO_CPU_RECORD_PID_SPEC>`"]
pub type PRO_CPU_RECORD_PID = crate::Reg<pro_cpu_record_pid::PRO_CPU_RECORD_PID_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pid;
#[doc = "PRO_CPU_RECORD_PDEBUGINST (rw) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGINST_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGINST =
    crate::Reg<pro_cpu_record_pdebuginst::PRO_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebuginst;
#[doc = "PRO_CPU_RECORD_PDEBUGSTATUS (rw) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGSTATUS =
    crate::Reg<pro_cpu_record_pdebugstatus::PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugstatus;
#[doc = "PRO_CPU_RECORD_PDEBUGDATA (rw) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGDATA_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGDATA =
    crate::Reg<pro_cpu_record_pdebugdata::PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugdata;
#[doc = "PRO_CPU_RECORD_PDEBUGPC (r) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGPC_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGPC =
    crate::Reg<pro_cpu_record_pdebugpc::PRO_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugpc;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0STAT (rw) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGLS0STAT =
    crate::Reg<pro_cpu_record_pdebugls0stat::PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0stat;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0ADDR (r) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGLS0ADDR =
    crate::Reg<pro_cpu_record_pdebugls0addr::PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0addr;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0DATA (r) register accessor: an alias for `Reg<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>`"]
pub type PRO_CPU_RECORD_PDEBUGLS0DATA =
    crate::Reg<pro_cpu_record_pdebugls0data::PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0data;
#[doc = "APP_CPU_RECORD_CTRL (rw) register accessor: an alias for `Reg<APP_CPU_RECORD_CTRL_SPEC>`"]
pub type APP_CPU_RECORD_CTRL = crate::Reg<app_cpu_record_ctrl::APP_CPU_RECORD_CTRL_SPEC>;
#[doc = ""]
pub mod app_cpu_record_ctrl;
#[doc = "APP_CPU_RECORD_STATUS (r) register accessor: an alias for `Reg<APP_CPU_RECORD_STATUS_SPEC>`"]
pub type APP_CPU_RECORD_STATUS = crate::Reg<app_cpu_record_status::APP_CPU_RECORD_STATUS_SPEC>;
#[doc = ""]
pub mod app_cpu_record_status;
#[doc = "APP_CPU_RECORD_PID (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PID_SPEC>`"]
pub type APP_CPU_RECORD_PID = crate::Reg<app_cpu_record_pid::APP_CPU_RECORD_PID_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pid;
#[doc = "APP_CPU_RECORD_PDEBUGINST (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGINST_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGINST =
    crate::Reg<app_cpu_record_pdebuginst::APP_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebuginst;
#[doc = "APP_CPU_RECORD_PDEBUGSTATUS (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGSTATUS =
    crate::Reg<app_cpu_record_pdebugstatus::APP_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugstatus;
#[doc = "APP_CPU_RECORD_PDEBUGDATA (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGDATA_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGDATA =
    crate::Reg<app_cpu_record_pdebugdata::APP_CPU_RECORD_PDEBUGDATA_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugdata;
#[doc = "APP_CPU_RECORD_PDEBUGPC (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGPC_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGPC =
    crate::Reg<app_cpu_record_pdebugpc::APP_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugpc;
#[doc = "APP_CPU_RECORD_PDEBUGLS0STAT (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGLS0STAT_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGLS0STAT =
    crate::Reg<app_cpu_record_pdebugls0stat::APP_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0stat;
#[doc = "APP_CPU_RECORD_PDEBUGLS0ADDR (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGLS0ADDR_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGLS0ADDR =
    crate::Reg<app_cpu_record_pdebugls0addr::APP_CPU_RECORD_PDEBUGLS0ADDR_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0addr;
#[doc = "APP_CPU_RECORD_PDEBUGLS0DATA (r) register accessor: an alias for `Reg<APP_CPU_RECORD_PDEBUGLS0DATA_SPEC>`"]
pub type APP_CPU_RECORD_PDEBUGLS0DATA =
    crate::Reg<app_cpu_record_pdebugls0data::APP_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0data;
#[doc = "RSA_PD_CTRL (rw) register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = ""]
pub mod rsa_pd_ctrl;
#[doc = "ROM_MPU_TABLE0 (rw) register accessor: an alias for `Reg<ROM_MPU_TABLE0_SPEC>`"]
pub type ROM_MPU_TABLE0 = crate::Reg<rom_mpu_table0::ROM_MPU_TABLE0_SPEC>;
#[doc = ""]
pub mod rom_mpu_table0;
#[doc = "ROM_MPU_TABLE1 (rw) register accessor: an alias for `Reg<ROM_MPU_TABLE1_SPEC>`"]
pub type ROM_MPU_TABLE1 = crate::Reg<rom_mpu_table1::ROM_MPU_TABLE1_SPEC>;
#[doc = ""]
pub mod rom_mpu_table1;
#[doc = "ROM_MPU_TABLE2 (rw) register accessor: an alias for `Reg<ROM_MPU_TABLE2_SPEC>`"]
pub type ROM_MPU_TABLE2 = crate::Reg<rom_mpu_table2::ROM_MPU_TABLE2_SPEC>;
#[doc = ""]
pub mod rom_mpu_table2;
#[doc = "ROM_MPU_TABLE3 (rw) register accessor: an alias for `Reg<ROM_MPU_TABLE3_SPEC>`"]
pub type ROM_MPU_TABLE3 = crate::Reg<rom_mpu_table3::ROM_MPU_TABLE3_SPEC>;
#[doc = ""]
pub mod rom_mpu_table3;
#[doc = "SHROM_MPU_TABLE0 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE0_SPEC>`"]
pub type SHROM_MPU_TABLE0 = crate::Reg<shrom_mpu_table0::SHROM_MPU_TABLE0_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table0;
#[doc = "SHROM_MPU_TABLE1 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE1_SPEC>`"]
pub type SHROM_MPU_TABLE1 = crate::Reg<shrom_mpu_table1::SHROM_MPU_TABLE1_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table1;
#[doc = "SHROM_MPU_TABLE2 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE2_SPEC>`"]
pub type SHROM_MPU_TABLE2 = crate::Reg<shrom_mpu_table2::SHROM_MPU_TABLE2_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table2;
#[doc = "SHROM_MPU_TABLE3 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE3_SPEC>`"]
pub type SHROM_MPU_TABLE3 = crate::Reg<shrom_mpu_table3::SHROM_MPU_TABLE3_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table3;
#[doc = "SHROM_MPU_TABLE4 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE4_SPEC>`"]
pub type SHROM_MPU_TABLE4 = crate::Reg<shrom_mpu_table4::SHROM_MPU_TABLE4_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table4;
#[doc = "SHROM_MPU_TABLE5 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE5_SPEC>`"]
pub type SHROM_MPU_TABLE5 = crate::Reg<shrom_mpu_table5::SHROM_MPU_TABLE5_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table5;
#[doc = "SHROM_MPU_TABLE6 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE6_SPEC>`"]
pub type SHROM_MPU_TABLE6 = crate::Reg<shrom_mpu_table6::SHROM_MPU_TABLE6_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table6;
#[doc = "SHROM_MPU_TABLE7 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE7_SPEC>`"]
pub type SHROM_MPU_TABLE7 = crate::Reg<shrom_mpu_table7::SHROM_MPU_TABLE7_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table7;
#[doc = "SHROM_MPU_TABLE8 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE8_SPEC>`"]
pub type SHROM_MPU_TABLE8 = crate::Reg<shrom_mpu_table8::SHROM_MPU_TABLE8_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table8;
#[doc = "SHROM_MPU_TABLE9 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE9_SPEC>`"]
pub type SHROM_MPU_TABLE9 = crate::Reg<shrom_mpu_table9::SHROM_MPU_TABLE9_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table9;
#[doc = "SHROM_MPU_TABLE10 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE10_SPEC>`"]
pub type SHROM_MPU_TABLE10 = crate::Reg<shrom_mpu_table10::SHROM_MPU_TABLE10_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table10;
#[doc = "SHROM_MPU_TABLE11 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE11_SPEC>`"]
pub type SHROM_MPU_TABLE11 = crate::Reg<shrom_mpu_table11::SHROM_MPU_TABLE11_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table11;
#[doc = "SHROM_MPU_TABLE12 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE12_SPEC>`"]
pub type SHROM_MPU_TABLE12 = crate::Reg<shrom_mpu_table12::SHROM_MPU_TABLE12_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table12;
#[doc = "SHROM_MPU_TABLE13 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE13_SPEC>`"]
pub type SHROM_MPU_TABLE13 = crate::Reg<shrom_mpu_table13::SHROM_MPU_TABLE13_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table13;
#[doc = "SHROM_MPU_TABLE14 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE14_SPEC>`"]
pub type SHROM_MPU_TABLE14 = crate::Reg<shrom_mpu_table14::SHROM_MPU_TABLE14_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table14;
#[doc = "SHROM_MPU_TABLE15 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE15_SPEC>`"]
pub type SHROM_MPU_TABLE15 = crate::Reg<shrom_mpu_table15::SHROM_MPU_TABLE15_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table15;
#[doc = "SHROM_MPU_TABLE16 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE16_SPEC>`"]
pub type SHROM_MPU_TABLE16 = crate::Reg<shrom_mpu_table16::SHROM_MPU_TABLE16_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table16;
#[doc = "SHROM_MPU_TABLE17 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE17_SPEC>`"]
pub type SHROM_MPU_TABLE17 = crate::Reg<shrom_mpu_table17::SHROM_MPU_TABLE17_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table17;
#[doc = "SHROM_MPU_TABLE18 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE18_SPEC>`"]
pub type SHROM_MPU_TABLE18 = crate::Reg<shrom_mpu_table18::SHROM_MPU_TABLE18_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table18;
#[doc = "SHROM_MPU_TABLE19 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE19_SPEC>`"]
pub type SHROM_MPU_TABLE19 = crate::Reg<shrom_mpu_table19::SHROM_MPU_TABLE19_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table19;
#[doc = "SHROM_MPU_TABLE20 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE20_SPEC>`"]
pub type SHROM_MPU_TABLE20 = crate::Reg<shrom_mpu_table20::SHROM_MPU_TABLE20_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table20;
#[doc = "SHROM_MPU_TABLE21 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE21_SPEC>`"]
pub type SHROM_MPU_TABLE21 = crate::Reg<shrom_mpu_table21::SHROM_MPU_TABLE21_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table21;
#[doc = "SHROM_MPU_TABLE22 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE22_SPEC>`"]
pub type SHROM_MPU_TABLE22 = crate::Reg<shrom_mpu_table22::SHROM_MPU_TABLE22_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table22;
#[doc = "SHROM_MPU_TABLE23 (rw) register accessor: an alias for `Reg<SHROM_MPU_TABLE23_SPEC>`"]
pub type SHROM_MPU_TABLE23 = crate::Reg<shrom_mpu_table23::SHROM_MPU_TABLE23_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table23;
#[doc = "IMMU_TABLE0 (rw) register accessor: an alias for `Reg<IMMU_TABLE0_SPEC>`"]
pub type IMMU_TABLE0 = crate::Reg<immu_table0::IMMU_TABLE0_SPEC>;
#[doc = ""]
pub mod immu_table0;
#[doc = "IMMU_TABLE1 (rw) register accessor: an alias for `Reg<IMMU_TABLE1_SPEC>`"]
pub type IMMU_TABLE1 = crate::Reg<immu_table1::IMMU_TABLE1_SPEC>;
#[doc = ""]
pub mod immu_table1;
#[doc = "IMMU_TABLE2 (rw) register accessor: an alias for `Reg<IMMU_TABLE2_SPEC>`"]
pub type IMMU_TABLE2 = crate::Reg<immu_table2::IMMU_TABLE2_SPEC>;
#[doc = ""]
pub mod immu_table2;
#[doc = "IMMU_TABLE3 (rw) register accessor: an alias for `Reg<IMMU_TABLE3_SPEC>`"]
pub type IMMU_TABLE3 = crate::Reg<immu_table3::IMMU_TABLE3_SPEC>;
#[doc = ""]
pub mod immu_table3;
#[doc = "IMMU_TABLE4 (rw) register accessor: an alias for `Reg<IMMU_TABLE4_SPEC>`"]
pub type IMMU_TABLE4 = crate::Reg<immu_table4::IMMU_TABLE4_SPEC>;
#[doc = ""]
pub mod immu_table4;
#[doc = "IMMU_TABLE5 (rw) register accessor: an alias for `Reg<IMMU_TABLE5_SPEC>`"]
pub type IMMU_TABLE5 = crate::Reg<immu_table5::IMMU_TABLE5_SPEC>;
#[doc = ""]
pub mod immu_table5;
#[doc = "IMMU_TABLE6 (rw) register accessor: an alias for `Reg<IMMU_TABLE6_SPEC>`"]
pub type IMMU_TABLE6 = crate::Reg<immu_table6::IMMU_TABLE6_SPEC>;
#[doc = ""]
pub mod immu_table6;
#[doc = "IMMU_TABLE7 (rw) register accessor: an alias for `Reg<IMMU_TABLE7_SPEC>`"]
pub type IMMU_TABLE7 = crate::Reg<immu_table7::IMMU_TABLE7_SPEC>;
#[doc = ""]
pub mod immu_table7;
#[doc = "IMMU_TABLE8 (rw) register accessor: an alias for `Reg<IMMU_TABLE8_SPEC>`"]
pub type IMMU_TABLE8 = crate::Reg<immu_table8::IMMU_TABLE8_SPEC>;
#[doc = ""]
pub mod immu_table8;
#[doc = "IMMU_TABLE9 (rw) register accessor: an alias for `Reg<IMMU_TABLE9_SPEC>`"]
pub type IMMU_TABLE9 = crate::Reg<immu_table9::IMMU_TABLE9_SPEC>;
#[doc = ""]
pub mod immu_table9;
#[doc = "IMMU_TABLE10 (rw) register accessor: an alias for `Reg<IMMU_TABLE10_SPEC>`"]
pub type IMMU_TABLE10 = crate::Reg<immu_table10::IMMU_TABLE10_SPEC>;
#[doc = ""]
pub mod immu_table10;
#[doc = "IMMU_TABLE11 (rw) register accessor: an alias for `Reg<IMMU_TABLE11_SPEC>`"]
pub type IMMU_TABLE11 = crate::Reg<immu_table11::IMMU_TABLE11_SPEC>;
#[doc = ""]
pub mod immu_table11;
#[doc = "IMMU_TABLE12 (rw) register accessor: an alias for `Reg<IMMU_TABLE12_SPEC>`"]
pub type IMMU_TABLE12 = crate::Reg<immu_table12::IMMU_TABLE12_SPEC>;
#[doc = ""]
pub mod immu_table12;
#[doc = "IMMU_TABLE13 (rw) register accessor: an alias for `Reg<IMMU_TABLE13_SPEC>`"]
pub type IMMU_TABLE13 = crate::Reg<immu_table13::IMMU_TABLE13_SPEC>;
#[doc = ""]
pub mod immu_table13;
#[doc = "IMMU_TABLE14 (rw) register accessor: an alias for `Reg<IMMU_TABLE14_SPEC>`"]
pub type IMMU_TABLE14 = crate::Reg<immu_table14::IMMU_TABLE14_SPEC>;
#[doc = ""]
pub mod immu_table14;
#[doc = "IMMU_TABLE15 (rw) register accessor: an alias for `Reg<IMMU_TABLE15_SPEC>`"]
pub type IMMU_TABLE15 = crate::Reg<immu_table15::IMMU_TABLE15_SPEC>;
#[doc = ""]
pub mod immu_table15;
#[doc = "DMMU_TABLE0 (rw) register accessor: an alias for `Reg<DMMU_TABLE0_SPEC>`"]
pub type DMMU_TABLE0 = crate::Reg<dmmu_table0::DMMU_TABLE0_SPEC>;
#[doc = ""]
pub mod dmmu_table0;
#[doc = "DMMU_TABLE1 (rw) register accessor: an alias for `Reg<DMMU_TABLE1_SPEC>`"]
pub type DMMU_TABLE1 = crate::Reg<dmmu_table1::DMMU_TABLE1_SPEC>;
#[doc = ""]
pub mod dmmu_table1;
#[doc = "DMMU_TABLE2 (rw) register accessor: an alias for `Reg<DMMU_TABLE2_SPEC>`"]
pub type DMMU_TABLE2 = crate::Reg<dmmu_table2::DMMU_TABLE2_SPEC>;
#[doc = ""]
pub mod dmmu_table2;
#[doc = "DMMU_TABLE3 (rw) register accessor: an alias for `Reg<DMMU_TABLE3_SPEC>`"]
pub type DMMU_TABLE3 = crate::Reg<dmmu_table3::DMMU_TABLE3_SPEC>;
#[doc = ""]
pub mod dmmu_table3;
#[doc = "DMMU_TABLE4 (rw) register accessor: an alias for `Reg<DMMU_TABLE4_SPEC>`"]
pub type DMMU_TABLE4 = crate::Reg<dmmu_table4::DMMU_TABLE4_SPEC>;
#[doc = ""]
pub mod dmmu_table4;
#[doc = "DMMU_TABLE5 (rw) register accessor: an alias for `Reg<DMMU_TABLE5_SPEC>`"]
pub type DMMU_TABLE5 = crate::Reg<dmmu_table5::DMMU_TABLE5_SPEC>;
#[doc = ""]
pub mod dmmu_table5;
#[doc = "DMMU_TABLE6 (rw) register accessor: an alias for `Reg<DMMU_TABLE6_SPEC>`"]
pub type DMMU_TABLE6 = crate::Reg<dmmu_table6::DMMU_TABLE6_SPEC>;
#[doc = ""]
pub mod dmmu_table6;
#[doc = "DMMU_TABLE7 (rw) register accessor: an alias for `Reg<DMMU_TABLE7_SPEC>`"]
pub type DMMU_TABLE7 = crate::Reg<dmmu_table7::DMMU_TABLE7_SPEC>;
#[doc = ""]
pub mod dmmu_table7;
#[doc = "DMMU_TABLE8 (rw) register accessor: an alias for `Reg<DMMU_TABLE8_SPEC>`"]
pub type DMMU_TABLE8 = crate::Reg<dmmu_table8::DMMU_TABLE8_SPEC>;
#[doc = ""]
pub mod dmmu_table8;
#[doc = "DMMU_TABLE9 (rw) register accessor: an alias for `Reg<DMMU_TABLE9_SPEC>`"]
pub type DMMU_TABLE9 = crate::Reg<dmmu_table9::DMMU_TABLE9_SPEC>;
#[doc = ""]
pub mod dmmu_table9;
#[doc = "DMMU_TABLE10 (rw) register accessor: an alias for `Reg<DMMU_TABLE10_SPEC>`"]
pub type DMMU_TABLE10 = crate::Reg<dmmu_table10::DMMU_TABLE10_SPEC>;
#[doc = ""]
pub mod dmmu_table10;
#[doc = "DMMU_TABLE11 (rw) register accessor: an alias for `Reg<DMMU_TABLE11_SPEC>`"]
pub type DMMU_TABLE11 = crate::Reg<dmmu_table11::DMMU_TABLE11_SPEC>;
#[doc = ""]
pub mod dmmu_table11;
#[doc = "DMMU_TABLE12 (rw) register accessor: an alias for `Reg<DMMU_TABLE12_SPEC>`"]
pub type DMMU_TABLE12 = crate::Reg<dmmu_table12::DMMU_TABLE12_SPEC>;
#[doc = ""]
pub mod dmmu_table12;
#[doc = "DMMU_TABLE13 (rw) register accessor: an alias for `Reg<DMMU_TABLE13_SPEC>`"]
pub type DMMU_TABLE13 = crate::Reg<dmmu_table13::DMMU_TABLE13_SPEC>;
#[doc = ""]
pub mod dmmu_table13;
#[doc = "DMMU_TABLE14 (rw) register accessor: an alias for `Reg<DMMU_TABLE14_SPEC>`"]
pub type DMMU_TABLE14 = crate::Reg<dmmu_table14::DMMU_TABLE14_SPEC>;
#[doc = ""]
pub mod dmmu_table14;
#[doc = "DMMU_TABLE15 (rw) register accessor: an alias for `Reg<DMMU_TABLE15_SPEC>`"]
pub type DMMU_TABLE15 = crate::Reg<dmmu_table15::DMMU_TABLE15_SPEC>;
#[doc = ""]
pub mod dmmu_table15;
#[doc = "PRO_INTRUSION_CTRL (rw) register accessor: an alias for `Reg<PRO_INTRUSION_CTRL_SPEC>`"]
pub type PRO_INTRUSION_CTRL = crate::Reg<pro_intrusion_ctrl::PRO_INTRUSION_CTRL_SPEC>;
#[doc = ""]
pub mod pro_intrusion_ctrl;
#[doc = "PRO_INTRUSION_STATUS (r) register accessor: an alias for `Reg<PRO_INTRUSION_STATUS_SPEC>`"]
pub type PRO_INTRUSION_STATUS = crate::Reg<pro_intrusion_status::PRO_INTRUSION_STATUS_SPEC>;
#[doc = ""]
pub mod pro_intrusion_status;
#[doc = "APP_INTRUSION_CTRL (rw) register accessor: an alias for `Reg<APP_INTRUSION_CTRL_SPEC>`"]
pub type APP_INTRUSION_CTRL = crate::Reg<app_intrusion_ctrl::APP_INTRUSION_CTRL_SPEC>;
#[doc = ""]
pub mod app_intrusion_ctrl;
#[doc = "APP_INTRUSION_STATUS (r) register accessor: an alias for `Reg<APP_INTRUSION_STATUS_SPEC>`"]
pub type APP_INTRUSION_STATUS = crate::Reg<app_intrusion_status::APP_INTRUSION_STATUS_SPEC>;
#[doc = ""]
pub mod app_intrusion_status;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: an alias for `Reg<FRONT_END_MEM_PD_SPEC>`"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = ""]
pub mod front_end_mem_pd;
#[doc = "MMU_IA_INT_EN (rw) register accessor: an alias for `Reg<MMU_IA_INT_EN_SPEC>`"]
pub type MMU_IA_INT_EN = crate::Reg<mmu_ia_int_en::MMU_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod mmu_ia_int_en;
#[doc = "MPU_IA_INT_EN (rw) register accessor: an alias for `Reg<MPU_IA_INT_EN_SPEC>`"]
pub type MPU_IA_INT_EN = crate::Reg<mpu_ia_int_en::MPU_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod mpu_ia_int_en;
#[doc = "CACHE_IA_INT_EN (rw) register accessor: an alias for `Reg<CACHE_IA_INT_EN_SPEC>`"]
pub type CACHE_IA_INT_EN = crate::Reg<cache_ia_int_en::CACHE_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod cache_ia_int_en;
#[doc = "SECURE_BOOT_CTRL (rw) register accessor: an alias for `Reg<SECURE_BOOT_CTRL_SPEC>`"]
pub type SECURE_BOOT_CTRL = crate::Reg<secure_boot_ctrl::SECURE_BOOT_CTRL_SPEC>;
#[doc = ""]
pub mod secure_boot_ctrl;
#[doc = "SPI_DMA_CHAN_SEL (rw) register accessor: an alias for `Reg<SPI_DMA_CHAN_SEL_SPEC>`"]
pub type SPI_DMA_CHAN_SEL = crate::Reg<spi_dma_chan_sel::SPI_DMA_CHAN_SEL_SPEC>;
#[doc = ""]
pub mod spi_dma_chan_sel;
#[doc = "PRO_VECBASE_CTRL (rw) register accessor: an alias for `Reg<PRO_VECBASE_CTRL_SPEC>`"]
pub type PRO_VECBASE_CTRL = crate::Reg<pro_vecbase_ctrl::PRO_VECBASE_CTRL_SPEC>;
#[doc = ""]
pub mod pro_vecbase_ctrl;
#[doc = "PRO_VECBASE_SET (rw) register accessor: an alias for `Reg<PRO_VECBASE_SET_SPEC>`"]
pub type PRO_VECBASE_SET = crate::Reg<pro_vecbase_set::PRO_VECBASE_SET_SPEC>;
#[doc = ""]
pub mod pro_vecbase_set;
#[doc = "APP_VECBASE_CTRL (rw) register accessor: an alias for `Reg<APP_VECBASE_CTRL_SPEC>`"]
pub type APP_VECBASE_CTRL = crate::Reg<app_vecbase_ctrl::APP_VECBASE_CTRL_SPEC>;
#[doc = ""]
pub mod app_vecbase_ctrl;
#[doc = "APP_VECBASE_SET (rw) register accessor: an alias for `Reg<APP_VECBASE_SET_SPEC>`"]
pub type APP_VECBASE_SET = crate::Reg<app_vecbase_set::APP_VECBASE_SET_SPEC>;
#[doc = ""]
pub mod app_vecbase_set;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
