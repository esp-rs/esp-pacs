#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pro_boot_remap_ctrl: PRO_BOOT_REMAP_CTRL,
    app_boot_remap_ctrl: APP_BOOT_REMAP_CTRL,
    access_check: ACCESS_CHECK,
    pro_dport_apb_mask0: PRO_DPORT_APB_MASK0,
    pro_dport_apb_mask1: PRO_DPORT_APB_MASK1,
    app_dport_apb_mask0: APP_DPORT_APB_MASK0,
    app_dport_apb_mask1: APP_DPORT_APB_MASK1,
    peri_clk_en: PERI_CLK_EN,
    peri_rst_en: PERI_RST_EN,
    wifi_bb_cfg: WIFI_BB_CFG,
    wifi_bb_cfg_2: WIFI_BB_CFG_2,
    appcpu_ctrl_a: APPCPU_CTRL_A,
    appcpu_ctrl_b: APPCPU_CTRL_B,
    appcpu_ctrl_c: APPCPU_CTRL_C,
    appcpu_ctrl_d: APPCPU_CTRL_D,
    cpu_per_conf: CPU_PER_CONF,
    pro_cache_ctrl: PRO_CACHE_CTRL,
    pro_cache_ctrl1: PRO_CACHE_CTRL1,
    pro_cache_lock_0_addr: PRO_CACHE_LOCK_0_ADDR,
    pro_cache_lock_1_addr: PRO_CACHE_LOCK_1_ADDR,
    pro_cache_lock_2_addr: PRO_CACHE_LOCK_2_ADDR,
    pro_cache_lock_3_addr: PRO_CACHE_LOCK_3_ADDR,
    app_cache_ctrl: APP_CACHE_CTRL,
    app_cache_ctrl1: APP_CACHE_CTRL1,
    app_cache_lock_0_addr: APP_CACHE_LOCK_0_ADDR,
    app_cache_lock_1_addr: APP_CACHE_LOCK_1_ADDR,
    app_cache_lock_2_addr: APP_CACHE_LOCK_2_ADDR,
    app_cache_lock_3_addr: APP_CACHE_LOCK_3_ADDR,
    tracemem_mux_mode: TRACEMEM_MUX_MODE,
    pro_tracemem_ena: PRO_TRACEMEM_ENA,
    app_tracemem_ena: APP_TRACEMEM_ENA,
    cache_mux_mode: CACHE_MUX_MODE,
    immu_page_mode: IMMU_PAGE_MODE,
    dmmu_page_mode: DMMU_PAGE_MODE,
    rom_mpu_ena: ROM_MPU_ENA,
    mem_pd_mask: MEM_PD_MASK,
    rom_pd_ctrl: ROM_PD_CTRL,
    rom_fo_ctrl: ROM_FO_CTRL,
    sram_pd_ctrl_0: SRAM_PD_CTRL_0,
    sram_pd_ctrl_1: SRAM_PD_CTRL_1,
    sram_fo_ctrl_0: SRAM_FO_CTRL_0,
    sram_fo_ctrl_1: SRAM_FO_CTRL_1,
    iram_dram_ahb_sel: IRAM_DRAM_AHB_SEL,
    tag_fo_ctrl: TAG_FO_CTRL,
    ahb_lite_mask: AHB_LITE_MASK,
    ahb_mpu_table_0: AHB_MPU_TABLE_0,
    ahb_mpu_table_1: AHB_MPU_TABLE_1,
    host_inf_sel: HOST_INF_SEL,
    perip_clk_en: PERIP_CLK_EN,
    perip_rst_en: PERIP_RST_EN,
    slave_spi_config: SLAVE_SPI_CONFIG,
    wifi_clk_en: WIFI_CLK_EN,
    core_rst_en: CORE_RST_EN,
    bt_lpck_div_int: BT_LPCK_DIV_INT,
    bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    pro_intr_status_0: PRO_INTR_STATUS_0,
    pro_intr_status_1: PRO_INTR_STATUS_1,
    pro_intr_status_2: PRO_INTR_STATUS_2,
    app_intr_status_0: APP_INTR_STATUS_0,
    app_intr_status_1: APP_INTR_STATUS_1,
    app_intr_status_2: APP_INTR_STATUS_2,
    pro_mac_intr_map: PRO_MAC_INTR_MAP,
    pro_mac_nmi_map: PRO_MAC_NMI_MAP,
    pro_bb_int_map: PRO_BB_INT_MAP,
    pro_bt_mac_int_map: PRO_BT_MAC_INT_MAP,
    pro_bt_bb_int_map: PRO_BT_BB_INT_MAP,
    pro_bt_bb_nmi_map: PRO_BT_BB_NMI_MAP,
    pro_rwbt_irq_map: PRO_RWBT_IRQ_MAP,
    pro_rwble_irq_map: PRO_RWBLE_IRQ_MAP,
    pro_rwbt_nmi_map: PRO_RWBT_NMI_MAP,
    pro_rwble_nmi_map: PRO_RWBLE_NMI_MAP,
    pro_slc0_intr_map: PRO_SLC0_INTR_MAP,
    pro_slc1_intr_map: PRO_SLC1_INTR_MAP,
    pro_uhci0_intr_map: PRO_UHCI0_INTR_MAP,
    pro_uhci1_intr_map: PRO_UHCI1_INTR_MAP,
    pro_tg_t0_level_int_map: PRO_TG_T0_LEVEL_INT_MAP,
    pro_tg_t1_level_int_map: PRO_TG_T1_LEVEL_INT_MAP,
    pro_tg_wdt_level_int_map: PRO_TG_WDT_LEVEL_INT_MAP,
    pro_tg_lact_level_int_map: PRO_TG_LACT_LEVEL_INT_MAP,
    pro_tg1_t0_level_int_map: PRO_TG1_T0_LEVEL_INT_MAP,
    pro_tg1_t1_level_int_map: PRO_TG1_T1_LEVEL_INT_MAP,
    pro_tg1_wdt_level_int_map: PRO_TG1_WDT_LEVEL_INT_MAP,
    pro_tg1_lact_level_int_map: PRO_TG1_LACT_LEVEL_INT_MAP,
    pro_gpio_interrupt_map: PRO_GPIO_INTERRUPT_MAP,
    pro_gpio_interrupt_nmi_map: PRO_GPIO_INTERRUPT_NMI_MAP,
    pro_cpu_intr_from_cpu_0_map: PRO_CPU_INTR_FROM_CPU_0_MAP,
    pro_cpu_intr_from_cpu_1_map: PRO_CPU_INTR_FROM_CPU_1_MAP,
    pro_cpu_intr_from_cpu_2_map: PRO_CPU_INTR_FROM_CPU_2_MAP,
    pro_cpu_intr_from_cpu_3_map: PRO_CPU_INTR_FROM_CPU_3_MAP,
    pro_spi_intr_0_map: PRO_SPI_INTR_0_MAP,
    pro_spi_intr_1_map: PRO_SPI_INTR_1_MAP,
    pro_spi_intr_2_map: PRO_SPI_INTR_2_MAP,
    pro_spi_intr_3_map: PRO_SPI_INTR_3_MAP,
    pro_i2s0_int_map: PRO_I2S0_INT_MAP,
    pro_i2s1_int_map: PRO_I2S1_INT_MAP,
    pro_uart_intr_map: PRO_UART_INTR_MAP,
    pro_uart1_intr_map: PRO_UART1_INTR_MAP,
    pro_uart2_intr_map: PRO_UART2_INTR_MAP,
    pro_sdio_host_interrupt_map: PRO_SDIO_HOST_INTERRUPT_MAP,
    pro_emac_int_map: PRO_EMAC_INT_MAP,
    pro_pwm0_intr_map: PRO_PWM0_INTR_MAP,
    pro_pwm1_intr_map: PRO_PWM1_INTR_MAP,
    pro_pwm2_intr_map: PRO_PWM2_INTR_MAP,
    pro_pwm3_intr_map: PRO_PWM3_INTR_MAP,
    pro_ledc_int_map: PRO_LEDC_INT_MAP,
    pro_efuse_int_map: PRO_EFUSE_INT_MAP,
    pro_can_int_map: PRO_CAN_INT_MAP,
    pro_rtc_core_intr_map: PRO_RTC_CORE_INTR_MAP,
    pro_rmt_intr_map: PRO_RMT_INTR_MAP,
    pro_pcnt_intr_map: PRO_PCNT_INTR_MAP,
    pro_i2c_ext0_intr_map: PRO_I2C_EXT0_INTR_MAP,
    pro_i2c_ext1_intr_map: PRO_I2C_EXT1_INTR_MAP,
    pro_rsa_intr_map: PRO_RSA_INTR_MAP,
    pro_spi1_dma_int_map: PRO_SPI1_DMA_INT_MAP,
    pro_spi2_dma_int_map: PRO_SPI2_DMA_INT_MAP,
    pro_spi3_dma_int_map: PRO_SPI3_DMA_INT_MAP,
    pro_wdg_int_map: PRO_WDG_INT_MAP,
    pro_timer_int1_map: PRO_TIMER_INT1_MAP,
    pro_timer_int2_map: PRO_TIMER_INT2_MAP,
    pro_tg_t0_edge_int_map: PRO_TG_T0_EDGE_INT_MAP,
    pro_tg_t1_edge_int_map: PRO_TG_T1_EDGE_INT_MAP,
    pro_tg_wdt_edge_int_map: PRO_TG_WDT_EDGE_INT_MAP,
    pro_tg_lact_edge_int_map: PRO_TG_LACT_EDGE_INT_MAP,
    pro_tg1_t0_edge_int_map: PRO_TG1_T0_EDGE_INT_MAP,
    pro_tg1_t1_edge_int_map: PRO_TG1_T1_EDGE_INT_MAP,
    pro_tg1_wdt_edge_int_map: PRO_TG1_WDT_EDGE_INT_MAP,
    pro_tg1_lact_edge_int_map: PRO_TG1_LACT_EDGE_INT_MAP,
    pro_mmu_ia_int_map: PRO_MMU_IA_INT_MAP,
    pro_mpu_ia_int_map: PRO_MPU_IA_INT_MAP,
    pro_cache_ia_int_map: PRO_CACHE_IA_INT_MAP,
    app_mac_intr_map: APP_MAC_INTR_MAP,
    app_mac_nmi_map: APP_MAC_NMI_MAP,
    app_bb_int_map: APP_BB_INT_MAP,
    app_bt_mac_int_map: APP_BT_MAC_INT_MAP,
    app_bt_bb_int_map: APP_BT_BB_INT_MAP,
    app_bt_bb_nmi_map: APP_BT_BB_NMI_MAP,
    app_rwbt_irq_map: APP_RWBT_IRQ_MAP,
    app_rwble_irq_map: APP_RWBLE_IRQ_MAP,
    app_rwbt_nmi_map: APP_RWBT_NMI_MAP,
    app_rwble_nmi_map: APP_RWBLE_NMI_MAP,
    app_slc0_intr_map: APP_SLC0_INTR_MAP,
    app_slc1_intr_map: APP_SLC1_INTR_MAP,
    app_uhci0_intr_map: APP_UHCI0_INTR_MAP,
    app_uhci1_intr_map: APP_UHCI1_INTR_MAP,
    app_tg_t0_level_int_map: APP_TG_T0_LEVEL_INT_MAP,
    app_tg_t1_level_int_map: APP_TG_T1_LEVEL_INT_MAP,
    app_tg_wdt_level_int_map: APP_TG_WDT_LEVEL_INT_MAP,
    app_tg_lact_level_int_map: APP_TG_LACT_LEVEL_INT_MAP,
    app_tg1_t0_level_int_map: APP_TG1_T0_LEVEL_INT_MAP,
    app_tg1_t1_level_int_map: APP_TG1_T1_LEVEL_INT_MAP,
    app_tg1_wdt_level_int_map: APP_TG1_WDT_LEVEL_INT_MAP,
    app_tg1_lact_level_int_map: APP_TG1_LACT_LEVEL_INT_MAP,
    app_gpio_interrupt_map: APP_GPIO_INTERRUPT_MAP,
    app_gpio_interrupt_nmi_map: APP_GPIO_INTERRUPT_NMI_MAP,
    app_cpu_intr_from_cpu_0_map: APP_CPU_INTR_FROM_CPU_0_MAP,
    app_cpu_intr_from_cpu_1_map: APP_CPU_INTR_FROM_CPU_1_MAP,
    app_cpu_intr_from_cpu_2_map: APP_CPU_INTR_FROM_CPU_2_MAP,
    app_cpu_intr_from_cpu_3_map: APP_CPU_INTR_FROM_CPU_3_MAP,
    app_spi_intr_0_map: APP_SPI_INTR_0_MAP,
    app_spi_intr_1_map: APP_SPI_INTR_1_MAP,
    app_spi_intr_2_map: APP_SPI_INTR_2_MAP,
    app_spi_intr_3_map: APP_SPI_INTR_3_MAP,
    app_i2s0_int_map: APP_I2S0_INT_MAP,
    app_i2s1_int_map: APP_I2S1_INT_MAP,
    app_uart_intr_map: APP_UART_INTR_MAP,
    app_uart1_intr_map: APP_UART1_INTR_MAP,
    app_uart2_intr_map: APP_UART2_INTR_MAP,
    app_sdio_host_interrupt_map: APP_SDIO_HOST_INTERRUPT_MAP,
    app_emac_int_map: APP_EMAC_INT_MAP,
    app_pwm0_intr_map: APP_PWM0_INTR_MAP,
    app_pwm1_intr_map: APP_PWM1_INTR_MAP,
    app_pwm2_intr_map: APP_PWM2_INTR_MAP,
    app_pwm3_intr_map: APP_PWM3_INTR_MAP,
    app_ledc_int_map: APP_LEDC_INT_MAP,
    app_efuse_int_map: APP_EFUSE_INT_MAP,
    app_can_int_map: APP_CAN_INT_MAP,
    app_rtc_core_intr_map: APP_RTC_CORE_INTR_MAP,
    app_rmt_intr_map: APP_RMT_INTR_MAP,
    app_pcnt_intr_map: APP_PCNT_INTR_MAP,
    app_i2c_ext0_intr_map: APP_I2C_EXT0_INTR_MAP,
    app_i2c_ext1_intr_map: APP_I2C_EXT1_INTR_MAP,
    app_rsa_intr_map: APP_RSA_INTR_MAP,
    app_spi1_dma_int_map: APP_SPI1_DMA_INT_MAP,
    app_spi2_dma_int_map: APP_SPI2_DMA_INT_MAP,
    app_spi3_dma_int_map: APP_SPI3_DMA_INT_MAP,
    app_wdg_int_map: APP_WDG_INT_MAP,
    app_timer_int1_map: APP_TIMER_INT1_MAP,
    app_timer_int2_map: APP_TIMER_INT2_MAP,
    app_tg_t0_edge_int_map: APP_TG_T0_EDGE_INT_MAP,
    app_tg_t1_edge_int_map: APP_TG_T1_EDGE_INT_MAP,
    app_tg_wdt_edge_int_map: APP_TG_WDT_EDGE_INT_MAP,
    app_tg_lact_edge_int_map: APP_TG_LACT_EDGE_INT_MAP,
    app_tg1_t0_edge_int_map: APP_TG1_T0_EDGE_INT_MAP,
    app_tg1_t1_edge_int_map: APP_TG1_T1_EDGE_INT_MAP,
    app_tg1_wdt_edge_int_map: APP_TG1_WDT_EDGE_INT_MAP,
    app_tg1_lact_edge_int_map: APP_TG1_LACT_EDGE_INT_MAP,
    app_mmu_ia_int_map: APP_MMU_IA_INT_MAP,
    app_mpu_ia_int_map: APP_MPU_IA_INT_MAP,
    app_cache_ia_int_map: APP_CACHE_IA_INT_MAP,
    ahblite_mpu_table_uart: AHBLITE_MPU_TABLE_UART,
    ahblite_mpu_table_spi1: AHBLITE_MPU_TABLE_SPI1,
    ahblite_mpu_table_spi0: AHBLITE_MPU_TABLE_SPI0,
    ahblite_mpu_table_gpio: AHBLITE_MPU_TABLE_GPIO,
    ahblite_mpu_table_fe2: AHBLITE_MPU_TABLE_FE2,
    ahblite_mpu_table_fe: AHBLITE_MPU_TABLE_FE,
    ahblite_mpu_table_timer: AHBLITE_MPU_TABLE_TIMER,
    ahblite_mpu_table_rtc: AHBLITE_MPU_TABLE_RTC,
    ahblite_mpu_table_io_mux: AHBLITE_MPU_TABLE_IO_MUX,
    ahblite_mpu_table_wdg: AHBLITE_MPU_TABLE_WDG,
    ahblite_mpu_table_hinf: AHBLITE_MPU_TABLE_HINF,
    ahblite_mpu_table_uhci1: AHBLITE_MPU_TABLE_UHCI1,
    ahblite_mpu_table_misc: AHBLITE_MPU_TABLE_MISC,
    ahblite_mpu_table_i2c: AHBLITE_MPU_TABLE_I2C,
    ahblite_mpu_table_i2s0: AHBLITE_MPU_TABLE_I2S0,
    ahblite_mpu_table_uart1: AHBLITE_MPU_TABLE_UART1,
    ahblite_mpu_table_bt: AHBLITE_MPU_TABLE_BT,
    ahblite_mpu_table_bt_buffer: AHBLITE_MPU_TABLE_BT_BUFFER,
    ahblite_mpu_table_i2c_ext0: AHBLITE_MPU_TABLE_I2C_EXT0,
    ahblite_mpu_table_uhci0: AHBLITE_MPU_TABLE_UHCI0,
    ahblite_mpu_table_slchost: AHBLITE_MPU_TABLE_SLCHOST,
    ahblite_mpu_table_rmt: AHBLITE_MPU_TABLE_RMT,
    ahblite_mpu_table_pcnt: AHBLITE_MPU_TABLE_PCNT,
    ahblite_mpu_table_slc: AHBLITE_MPU_TABLE_SLC,
    ahblite_mpu_table_ledc: AHBLITE_MPU_TABLE_LEDC,
    ahblite_mpu_table_efuse: AHBLITE_MPU_TABLE_EFUSE,
    ahblite_mpu_table_spi_encrypt: AHBLITE_MPU_TABLE_SPI_ENCRYPT,
    ahblite_mpu_table_bb: AHBLITE_MPU_TABLE_BB,
    ahblite_mpu_table_pwm0: AHBLITE_MPU_TABLE_PWM0,
    ahblite_mpu_table_timergroup: AHBLITE_MPU_TABLE_TIMERGROUP,
    ahblite_mpu_table_timergroup1: AHBLITE_MPU_TABLE_TIMERGROUP1,
    ahblite_mpu_table_spi2: AHBLITE_MPU_TABLE_SPI2,
    ahblite_mpu_table_spi3: AHBLITE_MPU_TABLE_SPI3,
    ahblite_mpu_table_apb_ctrl: AHBLITE_MPU_TABLE_APB_CTRL,
    ahblite_mpu_table_i2c_ext1: AHBLITE_MPU_TABLE_I2C_EXT1,
    ahblite_mpu_table_sdio_host: AHBLITE_MPU_TABLE_SDIO_HOST,
    ahblite_mpu_table_emac: AHBLITE_MPU_TABLE_EMAC,
    ahblite_mpu_table_can: AHBLITE_MPU_TABLE_CAN,
    ahblite_mpu_table_pwm1: AHBLITE_MPU_TABLE_PWM1,
    ahblite_mpu_table_i2s1: AHBLITE_MPU_TABLE_I2S1,
    ahblite_mpu_table_uart2: AHBLITE_MPU_TABLE_UART2,
    ahblite_mpu_table_pwm2: AHBLITE_MPU_TABLE_PWM2,
    ahblite_mpu_table_pwm3: AHBLITE_MPU_TABLE_PWM3,
    ahblite_mpu_table_rwbt: AHBLITE_MPU_TABLE_RWBT,
    ahblite_mpu_table_btmac: AHBLITE_MPU_TABLE_BTMAC,
    ahblite_mpu_table_wifimac: AHBLITE_MPU_TABLE_WIFIMAC,
    ahblite_mpu_table_pwr: AHBLITE_MPU_TABLE_PWR,
    mem_access_dbug0: MEM_ACCESS_DBUG0,
    mem_access_dbug1: MEM_ACCESS_DBUG1,
    pro_dcache_dbug0: PRO_DCACHE_DBUG0,
    pro_dcache_dbug1: PRO_DCACHE_DBUG1,
    pro_dcache_dbug2: PRO_DCACHE_DBUG2,
    pro_dcache_dbug3: PRO_DCACHE_DBUG3,
    pro_dcache_dbug4: PRO_DCACHE_DBUG4,
    pro_dcache_dbug5: PRO_DCACHE_DBUG5,
    pro_dcache_dbug6: PRO_DCACHE_DBUG6,
    pro_dcache_dbug7: PRO_DCACHE_DBUG7,
    pro_dcache_dbug8: PRO_DCACHE_DBUG8,
    pro_dcache_dbug9: PRO_DCACHE_DBUG9,
    app_dcache_dbug0: APP_DCACHE_DBUG0,
    app_dcache_dbug1: APP_DCACHE_DBUG1,
    app_dcache_dbug2: APP_DCACHE_DBUG2,
    app_dcache_dbug3: APP_DCACHE_DBUG3,
    app_dcache_dbug4: APP_DCACHE_DBUG4,
    app_dcache_dbug5: APP_DCACHE_DBUG5,
    app_dcache_dbug6: APP_DCACHE_DBUG6,
    app_dcache_dbug7: APP_DCACHE_DBUG7,
    app_dcache_dbug8: APP_DCACHE_DBUG8,
    app_dcache_dbug9: APP_DCACHE_DBUG9,
    pro_cpu_record_ctrl: PRO_CPU_RECORD_CTRL,
    pro_cpu_record_status: PRO_CPU_RECORD_STATUS,
    pro_cpu_record_pid: PRO_CPU_RECORD_PID,
    pro_cpu_record_pdebuginst: PRO_CPU_RECORD_PDEBUGINST,
    pro_cpu_record_pdebugstatus: PRO_CPU_RECORD_PDEBUGSTATUS,
    pro_cpu_record_pdebugdata: PRO_CPU_RECORD_PDEBUGDATA,
    pro_cpu_record_pdebugpc: PRO_CPU_RECORD_PDEBUGPC,
    pro_cpu_record_pdebugls0stat: PRO_CPU_RECORD_PDEBUGLS0STAT,
    pro_cpu_record_pdebugls0addr: PRO_CPU_RECORD_PDEBUGLS0ADDR,
    pro_cpu_record_pdebugls0data: PRO_CPU_RECORD_PDEBUGLS0DATA,
    app_cpu_record_ctrl: APP_CPU_RECORD_CTRL,
    app_cpu_record_status: APP_CPU_RECORD_STATUS,
    app_cpu_record_pid: APP_CPU_RECORD_PID,
    app_cpu_record_pdebuginst: APP_CPU_RECORD_PDEBUGINST,
    app_cpu_record_pdebugstatus: APP_CPU_RECORD_PDEBUGSTATUS,
    app_cpu_record_pdebugdata: APP_CPU_RECORD_PDEBUGDATA,
    app_cpu_record_pdebugpc: APP_CPU_RECORD_PDEBUGPC,
    app_cpu_record_pdebugls0stat: APP_CPU_RECORD_PDEBUGLS0STAT,
    app_cpu_record_pdebugls0addr: APP_CPU_RECORD_PDEBUGLS0ADDR,
    app_cpu_record_pdebugls0data: APP_CPU_RECORD_PDEBUGLS0DATA,
    rsa_pd_ctrl: RSA_PD_CTRL,
    rom_mpu_table0: ROM_MPU_TABLE0,
    rom_mpu_table1: ROM_MPU_TABLE1,
    rom_mpu_table2: ROM_MPU_TABLE2,
    rom_mpu_table3: ROM_MPU_TABLE3,
    shrom_mpu_table0: SHROM_MPU_TABLE0,
    shrom_mpu_table1: SHROM_MPU_TABLE1,
    shrom_mpu_table2: SHROM_MPU_TABLE2,
    shrom_mpu_table3: SHROM_MPU_TABLE3,
    shrom_mpu_table4: SHROM_MPU_TABLE4,
    shrom_mpu_table5: SHROM_MPU_TABLE5,
    shrom_mpu_table6: SHROM_MPU_TABLE6,
    shrom_mpu_table7: SHROM_MPU_TABLE7,
    shrom_mpu_table8: SHROM_MPU_TABLE8,
    shrom_mpu_table9: SHROM_MPU_TABLE9,
    shrom_mpu_table10: SHROM_MPU_TABLE10,
    shrom_mpu_table11: SHROM_MPU_TABLE11,
    shrom_mpu_table12: SHROM_MPU_TABLE12,
    shrom_mpu_table13: SHROM_MPU_TABLE13,
    shrom_mpu_table14: SHROM_MPU_TABLE14,
    shrom_mpu_table15: SHROM_MPU_TABLE15,
    shrom_mpu_table16: SHROM_MPU_TABLE16,
    shrom_mpu_table17: SHROM_MPU_TABLE17,
    shrom_mpu_table18: SHROM_MPU_TABLE18,
    shrom_mpu_table19: SHROM_MPU_TABLE19,
    shrom_mpu_table20: SHROM_MPU_TABLE20,
    shrom_mpu_table21: SHROM_MPU_TABLE21,
    shrom_mpu_table22: SHROM_MPU_TABLE22,
    shrom_mpu_table23: SHROM_MPU_TABLE23,
    immu_table0: IMMU_TABLE0,
    immu_table1: IMMU_TABLE1,
    immu_table2: IMMU_TABLE2,
    immu_table3: IMMU_TABLE3,
    immu_table4: IMMU_TABLE4,
    immu_table5: IMMU_TABLE5,
    immu_table6: IMMU_TABLE6,
    immu_table7: IMMU_TABLE7,
    immu_table8: IMMU_TABLE8,
    immu_table9: IMMU_TABLE9,
    immu_table10: IMMU_TABLE10,
    immu_table11: IMMU_TABLE11,
    immu_table12: IMMU_TABLE12,
    immu_table13: IMMU_TABLE13,
    immu_table14: IMMU_TABLE14,
    immu_table15: IMMU_TABLE15,
    dmmu_table0: DMMU_TABLE0,
    dmmu_table1: DMMU_TABLE1,
    dmmu_table2: DMMU_TABLE2,
    dmmu_table3: DMMU_TABLE3,
    dmmu_table4: DMMU_TABLE4,
    dmmu_table5: DMMU_TABLE5,
    dmmu_table6: DMMU_TABLE6,
    dmmu_table7: DMMU_TABLE7,
    dmmu_table8: DMMU_TABLE8,
    dmmu_table9: DMMU_TABLE9,
    dmmu_table10: DMMU_TABLE10,
    dmmu_table11: DMMU_TABLE11,
    dmmu_table12: DMMU_TABLE12,
    dmmu_table13: DMMU_TABLE13,
    dmmu_table14: DMMU_TABLE14,
    dmmu_table15: DMMU_TABLE15,
    pro_intrusion_ctrl: PRO_INTRUSION_CTRL,
    pro_intrusion_status: PRO_INTRUSION_STATUS,
    app_intrusion_ctrl: APP_INTRUSION_CTRL,
    app_intrusion_status: APP_INTRUSION_STATUS,
    front_end_mem_pd: FRONT_END_MEM_PD,
    mmu_ia_int_en: MMU_IA_INT_EN,
    mpu_ia_int_en: MPU_IA_INT_EN,
    cache_ia_int_en: CACHE_IA_INT_EN,
    secure_boot_ctrl: SECURE_BOOT_CTRL,
    spi_dma_chan_sel: SPI_DMA_CHAN_SEL,
    pro_vecbase_ctrl: PRO_VECBASE_CTRL,
    pro_vecbase_set: PRO_VECBASE_SET,
    app_vecbase_ctrl: APP_VECBASE_CTRL,
    app_vecbase_set: APP_VECBASE_SET,
    _reserved367: [u8; 0x0a40],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn pro_boot_remap_ctrl(&self) -> &PRO_BOOT_REMAP_CTRL {
        &self.pro_boot_remap_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn app_boot_remap_ctrl(&self) -> &APP_BOOT_REMAP_CTRL {
        &self.app_boot_remap_ctrl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn access_check(&self) -> &ACCESS_CHECK {
        &self.access_check
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn pro_dport_apb_mask0(&self) -> &PRO_DPORT_APB_MASK0 {
        &self.pro_dport_apb_mask0
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn pro_dport_apb_mask1(&self) -> &PRO_DPORT_APB_MASK1 {
        &self.pro_dport_apb_mask1
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn app_dport_apb_mask0(&self) -> &APP_DPORT_APB_MASK0 {
        &self.app_dport_apb_mask0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn app_dport_apb_mask1(&self) -> &APP_DPORT_APB_MASK1 {
        &self.app_dport_apb_mask1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn peri_clk_en(&self) -> &PERI_CLK_EN {
        &self.peri_clk_en
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn peri_rst_en(&self) -> &PERI_RST_EN {
        &self.peri_rst_en
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn wifi_bb_cfg(&self) -> &WIFI_BB_CFG {
        &self.wifi_bb_cfg
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn wifi_bb_cfg_2(&self) -> &WIFI_BB_CFG_2 {
        &self.wifi_bb_cfg_2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn appcpu_ctrl_a(&self) -> &APPCPU_CTRL_A {
        &self.appcpu_ctrl_a
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn appcpu_ctrl_b(&self) -> &APPCPU_CTRL_B {
        &self.appcpu_ctrl_b
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn appcpu_ctrl_c(&self) -> &APPCPU_CTRL_C {
        &self.appcpu_ctrl_c
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn appcpu_ctrl_d(&self) -> &APPCPU_CTRL_D {
        &self.appcpu_ctrl_d
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn cpu_per_conf(&self) -> &CPU_PER_CONF {
        &self.cpu_per_conf
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn pro_cache_ctrl(&self) -> &PRO_CACHE_CTRL {
        &self.pro_cache_ctrl
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn pro_cache_ctrl1(&self) -> &PRO_CACHE_CTRL1 {
        &self.pro_cache_ctrl1
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn pro_cache_lock_0_addr(&self) -> &PRO_CACHE_LOCK_0_ADDR {
        &self.pro_cache_lock_0_addr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn pro_cache_lock_1_addr(&self) -> &PRO_CACHE_LOCK_1_ADDR {
        &self.pro_cache_lock_1_addr
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn pro_cache_lock_2_addr(&self) -> &PRO_CACHE_LOCK_2_ADDR {
        &self.pro_cache_lock_2_addr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn pro_cache_lock_3_addr(&self) -> &PRO_CACHE_LOCK_3_ADDR {
        &self.pro_cache_lock_3_addr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn app_cache_ctrl(&self) -> &APP_CACHE_CTRL {
        &self.app_cache_ctrl
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn app_cache_ctrl1(&self) -> &APP_CACHE_CTRL1 {
        &self.app_cache_ctrl1
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn app_cache_lock_0_addr(&self) -> &APP_CACHE_LOCK_0_ADDR {
        &self.app_cache_lock_0_addr
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn app_cache_lock_1_addr(&self) -> &APP_CACHE_LOCK_1_ADDR {
        &self.app_cache_lock_1_addr
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn app_cache_lock_2_addr(&self) -> &APP_CACHE_LOCK_2_ADDR {
        &self.app_cache_lock_2_addr
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn app_cache_lock_3_addr(&self) -> &APP_CACHE_LOCK_3_ADDR {
        &self.app_cache_lock_3_addr
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn tracemem_mux_mode(&self) -> &TRACEMEM_MUX_MODE {
        &self.tracemem_mux_mode
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn pro_tracemem_ena(&self) -> &PRO_TRACEMEM_ENA {
        &self.pro_tracemem_ena
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn app_tracemem_ena(&self) -> &APP_TRACEMEM_ENA {
        &self.app_tracemem_ena
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn cache_mux_mode(&self) -> &CACHE_MUX_MODE {
        &self.cache_mux_mode
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn immu_page_mode(&self) -> &IMMU_PAGE_MODE {
        &self.immu_page_mode
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn dmmu_page_mode(&self) -> &DMMU_PAGE_MODE {
        &self.dmmu_page_mode
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn rom_mpu_ena(&self) -> &ROM_MPU_ENA {
        &self.rom_mpu_ena
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn mem_pd_mask(&self) -> &MEM_PD_MASK {
        &self.mem_pd_mask
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn rom_pd_ctrl(&self) -> &ROM_PD_CTRL {
        &self.rom_pd_ctrl
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn rom_fo_ctrl(&self) -> &ROM_FO_CTRL {
        &self.rom_fo_ctrl
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn sram_pd_ctrl_0(&self) -> &SRAM_PD_CTRL_0 {
        &self.sram_pd_ctrl_0
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn sram_pd_ctrl_1(&self) -> &SRAM_PD_CTRL_1 {
        &self.sram_pd_ctrl_1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn sram_fo_ctrl_0(&self) -> &SRAM_FO_CTRL_0 {
        &self.sram_fo_ctrl_0
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn sram_fo_ctrl_1(&self) -> &SRAM_FO_CTRL_1 {
        &self.sram_fo_ctrl_1
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn iram_dram_ahb_sel(&self) -> &IRAM_DRAM_AHB_SEL {
        &self.iram_dram_ahb_sel
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn tag_fo_ctrl(&self) -> &TAG_FO_CTRL {
        &self.tag_fo_ctrl
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn ahb_lite_mask(&self) -> &AHB_LITE_MASK {
        &self.ahb_lite_mask
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn ahb_mpu_table_0(&self) -> &AHB_MPU_TABLE_0 {
        &self.ahb_mpu_table_0
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn ahb_mpu_table_1(&self) -> &AHB_MPU_TABLE_1 {
        &self.ahb_mpu_table_1
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn host_inf_sel(&self) -> &HOST_INF_SEL {
        &self.host_inf_sel
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn perip_clk_en(&self) -> &PERIP_CLK_EN {
        &self.perip_clk_en
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn perip_rst_en(&self) -> &PERIP_RST_EN {
        &self.perip_rst_en
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn slave_spi_config(&self) -> &SLAVE_SPI_CONFIG {
        &self.slave_spi_config
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn wifi_clk_en(&self) -> &WIFI_CLK_EN {
        &self.wifi_clk_en
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn core_rst_en(&self) -> &CORE_RST_EN {
        &self.core_rst_en
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn bt_lpck_div_int(&self) -> &BT_LPCK_DIV_INT {
        &self.bt_lpck_div_int
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn bt_lpck_div_frac(&self) -> &BT_LPCK_DIV_FRAC {
        &self.bt_lpck_div_frac
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> &CPU_INTR_FROM_CPU_0 {
        &self.cpu_intr_from_cpu_0
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> &CPU_INTR_FROM_CPU_1 {
        &self.cpu_intr_from_cpu_1
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> &CPU_INTR_FROM_CPU_2 {
        &self.cpu_intr_from_cpu_2
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> &CPU_INTR_FROM_CPU_3 {
        &self.cpu_intr_from_cpu_3
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn pro_intr_status_0(&self) -> &PRO_INTR_STATUS_0 {
        &self.pro_intr_status_0
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn pro_intr_status_1(&self) -> &PRO_INTR_STATUS_1 {
        &self.pro_intr_status_1
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn pro_intr_status_2(&self) -> &PRO_INTR_STATUS_2 {
        &self.pro_intr_status_2
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn app_intr_status_0(&self) -> &APP_INTR_STATUS_0 {
        &self.app_intr_status_0
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn app_intr_status_1(&self) -> &APP_INTR_STATUS_1 {
        &self.app_intr_status_1
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn app_intr_status_2(&self) -> &APP_INTR_STATUS_2 {
        &self.app_intr_status_2
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn pro_mac_intr_map(&self) -> &PRO_MAC_INTR_MAP {
        &self.pro_mac_intr_map
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn pro_mac_nmi_map(&self) -> &PRO_MAC_NMI_MAP {
        &self.pro_mac_nmi_map
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn pro_bb_int_map(&self) -> &PRO_BB_INT_MAP {
        &self.pro_bb_int_map
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn pro_bt_mac_int_map(&self) -> &PRO_BT_MAC_INT_MAP {
        &self.pro_bt_mac_int_map
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn pro_bt_bb_int_map(&self) -> &PRO_BT_BB_INT_MAP {
        &self.pro_bt_bb_int_map
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn pro_bt_bb_nmi_map(&self) -> &PRO_BT_BB_NMI_MAP {
        &self.pro_bt_bb_nmi_map
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn pro_rwbt_irq_map(&self) -> &PRO_RWBT_IRQ_MAP {
        &self.pro_rwbt_irq_map
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn pro_rwble_irq_map(&self) -> &PRO_RWBLE_IRQ_MAP {
        &self.pro_rwble_irq_map
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn pro_rwbt_nmi_map(&self) -> &PRO_RWBT_NMI_MAP {
        &self.pro_rwbt_nmi_map
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn pro_rwble_nmi_map(&self) -> &PRO_RWBLE_NMI_MAP {
        &self.pro_rwble_nmi_map
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn pro_slc0_intr_map(&self) -> &PRO_SLC0_INTR_MAP {
        &self.pro_slc0_intr_map
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn pro_slc1_intr_map(&self) -> &PRO_SLC1_INTR_MAP {
        &self.pro_slc1_intr_map
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn pro_uhci0_intr_map(&self) -> &PRO_UHCI0_INTR_MAP {
        &self.pro_uhci0_intr_map
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn pro_uhci1_intr_map(&self) -> &PRO_UHCI1_INTR_MAP {
        &self.pro_uhci1_intr_map
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn pro_tg_t0_level_int_map(&self) -> &PRO_TG_T0_LEVEL_INT_MAP {
        &self.pro_tg_t0_level_int_map
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn pro_tg_t1_level_int_map(&self) -> &PRO_TG_T1_LEVEL_INT_MAP {
        &self.pro_tg_t1_level_int_map
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn pro_tg_wdt_level_int_map(&self) -> &PRO_TG_WDT_LEVEL_INT_MAP {
        &self.pro_tg_wdt_level_int_map
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn pro_tg_lact_level_int_map(&self) -> &PRO_TG_LACT_LEVEL_INT_MAP {
        &self.pro_tg_lact_level_int_map
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn pro_tg1_t0_level_int_map(&self) -> &PRO_TG1_T0_LEVEL_INT_MAP {
        &self.pro_tg1_t0_level_int_map
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn pro_tg1_t1_level_int_map(&self) -> &PRO_TG1_T1_LEVEL_INT_MAP {
        &self.pro_tg1_t1_level_int_map
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn pro_tg1_wdt_level_int_map(&self) -> &PRO_TG1_WDT_LEVEL_INT_MAP {
        &self.pro_tg1_wdt_level_int_map
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn pro_tg1_lact_level_int_map(&self) -> &PRO_TG1_LACT_LEVEL_INT_MAP {
        &self.pro_tg1_lact_level_int_map
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn pro_gpio_interrupt_map(&self) -> &PRO_GPIO_INTERRUPT_MAP {
        &self.pro_gpio_interrupt_map
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn pro_gpio_interrupt_nmi_map(&self) -> &PRO_GPIO_INTERRUPT_NMI_MAP {
        &self.pro_gpio_interrupt_nmi_map
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_0_map(&self) -> &PRO_CPU_INTR_FROM_CPU_0_MAP {
        &self.pro_cpu_intr_from_cpu_0_map
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_1_map(&self) -> &PRO_CPU_INTR_FROM_CPU_1_MAP {
        &self.pro_cpu_intr_from_cpu_1_map
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_2_map(&self) -> &PRO_CPU_INTR_FROM_CPU_2_MAP {
        &self.pro_cpu_intr_from_cpu_2_map
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_3_map(&self) -> &PRO_CPU_INTR_FROM_CPU_3_MAP {
        &self.pro_cpu_intr_from_cpu_3_map
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn pro_spi_intr_0_map(&self) -> &PRO_SPI_INTR_0_MAP {
        &self.pro_spi_intr_0_map
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn pro_spi_intr_1_map(&self) -> &PRO_SPI_INTR_1_MAP {
        &self.pro_spi_intr_1_map
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn pro_spi_intr_2_map(&self) -> &PRO_SPI_INTR_2_MAP {
        &self.pro_spi_intr_2_map
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn pro_spi_intr_3_map(&self) -> &PRO_SPI_INTR_3_MAP {
        &self.pro_spi_intr_3_map
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn pro_i2s0_int_map(&self) -> &PRO_I2S0_INT_MAP {
        &self.pro_i2s0_int_map
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn pro_i2s1_int_map(&self) -> &PRO_I2S1_INT_MAP {
        &self.pro_i2s1_int_map
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn pro_uart_intr_map(&self) -> &PRO_UART_INTR_MAP {
        &self.pro_uart_intr_map
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn pro_uart1_intr_map(&self) -> &PRO_UART1_INTR_MAP {
        &self.pro_uart1_intr_map
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn pro_uart2_intr_map(&self) -> &PRO_UART2_INTR_MAP {
        &self.pro_uart2_intr_map
    }
    #[doc = "0x198 - "]
    #[inline(always)]
    pub const fn pro_sdio_host_interrupt_map(&self) -> &PRO_SDIO_HOST_INTERRUPT_MAP {
        &self.pro_sdio_host_interrupt_map
    }
    #[doc = "0x19c - "]
    #[inline(always)]
    pub const fn pro_emac_int_map(&self) -> &PRO_EMAC_INT_MAP {
        &self.pro_emac_int_map
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn pro_pwm0_intr_map(&self) -> &PRO_PWM0_INTR_MAP {
        &self.pro_pwm0_intr_map
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn pro_pwm1_intr_map(&self) -> &PRO_PWM1_INTR_MAP {
        &self.pro_pwm1_intr_map
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn pro_pwm2_intr_map(&self) -> &PRO_PWM2_INTR_MAP {
        &self.pro_pwm2_intr_map
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn pro_pwm3_intr_map(&self) -> &PRO_PWM3_INTR_MAP {
        &self.pro_pwm3_intr_map
    }
    #[doc = "0x1b0 - "]
    #[inline(always)]
    pub const fn pro_ledc_int_map(&self) -> &PRO_LEDC_INT_MAP {
        &self.pro_ledc_int_map
    }
    #[doc = "0x1b4 - "]
    #[inline(always)]
    pub const fn pro_efuse_int_map(&self) -> &PRO_EFUSE_INT_MAP {
        &self.pro_efuse_int_map
    }
    #[doc = "0x1b8 - "]
    #[inline(always)]
    pub const fn pro_can_int_map(&self) -> &PRO_CAN_INT_MAP {
        &self.pro_can_int_map
    }
    #[doc = "0x1bc - "]
    #[inline(always)]
    pub const fn pro_rtc_core_intr_map(&self) -> &PRO_RTC_CORE_INTR_MAP {
        &self.pro_rtc_core_intr_map
    }
    #[doc = "0x1c0 - "]
    #[inline(always)]
    pub const fn pro_rmt_intr_map(&self) -> &PRO_RMT_INTR_MAP {
        &self.pro_rmt_intr_map
    }
    #[doc = "0x1c4 - "]
    #[inline(always)]
    pub const fn pro_pcnt_intr_map(&self) -> &PRO_PCNT_INTR_MAP {
        &self.pro_pcnt_intr_map
    }
    #[doc = "0x1c8 - "]
    #[inline(always)]
    pub const fn pro_i2c_ext0_intr_map(&self) -> &PRO_I2C_EXT0_INTR_MAP {
        &self.pro_i2c_ext0_intr_map
    }
    #[doc = "0x1cc - "]
    #[inline(always)]
    pub const fn pro_i2c_ext1_intr_map(&self) -> &PRO_I2C_EXT1_INTR_MAP {
        &self.pro_i2c_ext1_intr_map
    }
    #[doc = "0x1d0 - "]
    #[inline(always)]
    pub const fn pro_rsa_intr_map(&self) -> &PRO_RSA_INTR_MAP {
        &self.pro_rsa_intr_map
    }
    #[doc = "0x1d4 - "]
    #[inline(always)]
    pub const fn pro_spi1_dma_int_map(&self) -> &PRO_SPI1_DMA_INT_MAP {
        &self.pro_spi1_dma_int_map
    }
    #[doc = "0x1d8 - "]
    #[inline(always)]
    pub const fn pro_spi2_dma_int_map(&self) -> &PRO_SPI2_DMA_INT_MAP {
        &self.pro_spi2_dma_int_map
    }
    #[doc = "0x1dc - "]
    #[inline(always)]
    pub const fn pro_spi3_dma_int_map(&self) -> &PRO_SPI3_DMA_INT_MAP {
        &self.pro_spi3_dma_int_map
    }
    #[doc = "0x1e0 - "]
    #[inline(always)]
    pub const fn pro_wdg_int_map(&self) -> &PRO_WDG_INT_MAP {
        &self.pro_wdg_int_map
    }
    #[doc = "0x1e4 - "]
    #[inline(always)]
    pub const fn pro_timer_int1_map(&self) -> &PRO_TIMER_INT1_MAP {
        &self.pro_timer_int1_map
    }
    #[doc = "0x1e8 - "]
    #[inline(always)]
    pub const fn pro_timer_int2_map(&self) -> &PRO_TIMER_INT2_MAP {
        &self.pro_timer_int2_map
    }
    #[doc = "0x1ec - "]
    #[inline(always)]
    pub const fn pro_tg_t0_edge_int_map(&self) -> &PRO_TG_T0_EDGE_INT_MAP {
        &self.pro_tg_t0_edge_int_map
    }
    #[doc = "0x1f0 - "]
    #[inline(always)]
    pub const fn pro_tg_t1_edge_int_map(&self) -> &PRO_TG_T1_EDGE_INT_MAP {
        &self.pro_tg_t1_edge_int_map
    }
    #[doc = "0x1f4 - "]
    #[inline(always)]
    pub const fn pro_tg_wdt_edge_int_map(&self) -> &PRO_TG_WDT_EDGE_INT_MAP {
        &self.pro_tg_wdt_edge_int_map
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn pro_tg_lact_edge_int_map(&self) -> &PRO_TG_LACT_EDGE_INT_MAP {
        &self.pro_tg_lact_edge_int_map
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn pro_tg1_t0_edge_int_map(&self) -> &PRO_TG1_T0_EDGE_INT_MAP {
        &self.pro_tg1_t0_edge_int_map
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn pro_tg1_t1_edge_int_map(&self) -> &PRO_TG1_T1_EDGE_INT_MAP {
        &self.pro_tg1_t1_edge_int_map
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn pro_tg1_wdt_edge_int_map(&self) -> &PRO_TG1_WDT_EDGE_INT_MAP {
        &self.pro_tg1_wdt_edge_int_map
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn pro_tg1_lact_edge_int_map(&self) -> &PRO_TG1_LACT_EDGE_INT_MAP {
        &self.pro_tg1_lact_edge_int_map
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn pro_mmu_ia_int_map(&self) -> &PRO_MMU_IA_INT_MAP {
        &self.pro_mmu_ia_int_map
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn pro_mpu_ia_int_map(&self) -> &PRO_MPU_IA_INT_MAP {
        &self.pro_mpu_ia_int_map
    }
    #[doc = "0x214 - "]
    #[inline(always)]
    pub const fn pro_cache_ia_int_map(&self) -> &PRO_CACHE_IA_INT_MAP {
        &self.pro_cache_ia_int_map
    }
    #[doc = "0x218 - "]
    #[inline(always)]
    pub const fn app_mac_intr_map(&self) -> &APP_MAC_INTR_MAP {
        &self.app_mac_intr_map
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn app_mac_nmi_map(&self) -> &APP_MAC_NMI_MAP {
        &self.app_mac_nmi_map
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn app_bb_int_map(&self) -> &APP_BB_INT_MAP {
        &self.app_bb_int_map
    }
    #[doc = "0x224 - "]
    #[inline(always)]
    pub const fn app_bt_mac_int_map(&self) -> &APP_BT_MAC_INT_MAP {
        &self.app_bt_mac_int_map
    }
    #[doc = "0x228 - "]
    #[inline(always)]
    pub const fn app_bt_bb_int_map(&self) -> &APP_BT_BB_INT_MAP {
        &self.app_bt_bb_int_map
    }
    #[doc = "0x22c - "]
    #[inline(always)]
    pub const fn app_bt_bb_nmi_map(&self) -> &APP_BT_BB_NMI_MAP {
        &self.app_bt_bb_nmi_map
    }
    #[doc = "0x230 - "]
    #[inline(always)]
    pub const fn app_rwbt_irq_map(&self) -> &APP_RWBT_IRQ_MAP {
        &self.app_rwbt_irq_map
    }
    #[doc = "0x234 - "]
    #[inline(always)]
    pub const fn app_rwble_irq_map(&self) -> &APP_RWBLE_IRQ_MAP {
        &self.app_rwble_irq_map
    }
    #[doc = "0x238 - "]
    #[inline(always)]
    pub const fn app_rwbt_nmi_map(&self) -> &APP_RWBT_NMI_MAP {
        &self.app_rwbt_nmi_map
    }
    #[doc = "0x23c - "]
    #[inline(always)]
    pub const fn app_rwble_nmi_map(&self) -> &APP_RWBLE_NMI_MAP {
        &self.app_rwble_nmi_map
    }
    #[doc = "0x240 - "]
    #[inline(always)]
    pub const fn app_slc0_intr_map(&self) -> &APP_SLC0_INTR_MAP {
        &self.app_slc0_intr_map
    }
    #[doc = "0x244 - "]
    #[inline(always)]
    pub const fn app_slc1_intr_map(&self) -> &APP_SLC1_INTR_MAP {
        &self.app_slc1_intr_map
    }
    #[doc = "0x248 - "]
    #[inline(always)]
    pub const fn app_uhci0_intr_map(&self) -> &APP_UHCI0_INTR_MAP {
        &self.app_uhci0_intr_map
    }
    #[doc = "0x24c - "]
    #[inline(always)]
    pub const fn app_uhci1_intr_map(&self) -> &APP_UHCI1_INTR_MAP {
        &self.app_uhci1_intr_map
    }
    #[doc = "0x250 - "]
    #[inline(always)]
    pub const fn app_tg_t0_level_int_map(&self) -> &APP_TG_T0_LEVEL_INT_MAP {
        &self.app_tg_t0_level_int_map
    }
    #[doc = "0x254 - "]
    #[inline(always)]
    pub const fn app_tg_t1_level_int_map(&self) -> &APP_TG_T1_LEVEL_INT_MAP {
        &self.app_tg_t1_level_int_map
    }
    #[doc = "0x258 - "]
    #[inline(always)]
    pub const fn app_tg_wdt_level_int_map(&self) -> &APP_TG_WDT_LEVEL_INT_MAP {
        &self.app_tg_wdt_level_int_map
    }
    #[doc = "0x25c - "]
    #[inline(always)]
    pub const fn app_tg_lact_level_int_map(&self) -> &APP_TG_LACT_LEVEL_INT_MAP {
        &self.app_tg_lact_level_int_map
    }
    #[doc = "0x260 - "]
    #[inline(always)]
    pub const fn app_tg1_t0_level_int_map(&self) -> &APP_TG1_T0_LEVEL_INT_MAP {
        &self.app_tg1_t0_level_int_map
    }
    #[doc = "0x264 - "]
    #[inline(always)]
    pub const fn app_tg1_t1_level_int_map(&self) -> &APP_TG1_T1_LEVEL_INT_MAP {
        &self.app_tg1_t1_level_int_map
    }
    #[doc = "0x268 - "]
    #[inline(always)]
    pub const fn app_tg1_wdt_level_int_map(&self) -> &APP_TG1_WDT_LEVEL_INT_MAP {
        &self.app_tg1_wdt_level_int_map
    }
    #[doc = "0x26c - "]
    #[inline(always)]
    pub const fn app_tg1_lact_level_int_map(&self) -> &APP_TG1_LACT_LEVEL_INT_MAP {
        &self.app_tg1_lact_level_int_map
    }
    #[doc = "0x270 - "]
    #[inline(always)]
    pub const fn app_gpio_interrupt_map(&self) -> &APP_GPIO_INTERRUPT_MAP {
        &self.app_gpio_interrupt_map
    }
    #[doc = "0x274 - "]
    #[inline(always)]
    pub const fn app_gpio_interrupt_nmi_map(&self) -> &APP_GPIO_INTERRUPT_NMI_MAP {
        &self.app_gpio_interrupt_nmi_map
    }
    #[doc = "0x278 - "]
    #[inline(always)]
    pub const fn app_cpu_intr_from_cpu_0_map(&self) -> &APP_CPU_INTR_FROM_CPU_0_MAP {
        &self.app_cpu_intr_from_cpu_0_map
    }
    #[doc = "0x27c - "]
    #[inline(always)]
    pub const fn app_cpu_intr_from_cpu_1_map(&self) -> &APP_CPU_INTR_FROM_CPU_1_MAP {
        &self.app_cpu_intr_from_cpu_1_map
    }
    #[doc = "0x280 - "]
    #[inline(always)]
    pub const fn app_cpu_intr_from_cpu_2_map(&self) -> &APP_CPU_INTR_FROM_CPU_2_MAP {
        &self.app_cpu_intr_from_cpu_2_map
    }
    #[doc = "0x284 - "]
    #[inline(always)]
    pub const fn app_cpu_intr_from_cpu_3_map(&self) -> &APP_CPU_INTR_FROM_CPU_3_MAP {
        &self.app_cpu_intr_from_cpu_3_map
    }
    #[doc = "0x288 - "]
    #[inline(always)]
    pub const fn app_spi_intr_0_map(&self) -> &APP_SPI_INTR_0_MAP {
        &self.app_spi_intr_0_map
    }
    #[doc = "0x28c - "]
    #[inline(always)]
    pub const fn app_spi_intr_1_map(&self) -> &APP_SPI_INTR_1_MAP {
        &self.app_spi_intr_1_map
    }
    #[doc = "0x290 - "]
    #[inline(always)]
    pub const fn app_spi_intr_2_map(&self) -> &APP_SPI_INTR_2_MAP {
        &self.app_spi_intr_2_map
    }
    #[doc = "0x294 - "]
    #[inline(always)]
    pub const fn app_spi_intr_3_map(&self) -> &APP_SPI_INTR_3_MAP {
        &self.app_spi_intr_3_map
    }
    #[doc = "0x298 - "]
    #[inline(always)]
    pub const fn app_i2s0_int_map(&self) -> &APP_I2S0_INT_MAP {
        &self.app_i2s0_int_map
    }
    #[doc = "0x29c - "]
    #[inline(always)]
    pub const fn app_i2s1_int_map(&self) -> &APP_I2S1_INT_MAP {
        &self.app_i2s1_int_map
    }
    #[doc = "0x2a0 - "]
    #[inline(always)]
    pub const fn app_uart_intr_map(&self) -> &APP_UART_INTR_MAP {
        &self.app_uart_intr_map
    }
    #[doc = "0x2a4 - "]
    #[inline(always)]
    pub const fn app_uart1_intr_map(&self) -> &APP_UART1_INTR_MAP {
        &self.app_uart1_intr_map
    }
    #[doc = "0x2a8 - "]
    #[inline(always)]
    pub const fn app_uart2_intr_map(&self) -> &APP_UART2_INTR_MAP {
        &self.app_uart2_intr_map
    }
    #[doc = "0x2ac - "]
    #[inline(always)]
    pub const fn app_sdio_host_interrupt_map(&self) -> &APP_SDIO_HOST_INTERRUPT_MAP {
        &self.app_sdio_host_interrupt_map
    }
    #[doc = "0x2b0 - "]
    #[inline(always)]
    pub const fn app_emac_int_map(&self) -> &APP_EMAC_INT_MAP {
        &self.app_emac_int_map
    }
    #[doc = "0x2b4 - "]
    #[inline(always)]
    pub const fn app_pwm0_intr_map(&self) -> &APP_PWM0_INTR_MAP {
        &self.app_pwm0_intr_map
    }
    #[doc = "0x2b8 - "]
    #[inline(always)]
    pub const fn app_pwm1_intr_map(&self) -> &APP_PWM1_INTR_MAP {
        &self.app_pwm1_intr_map
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn app_pwm2_intr_map(&self) -> &APP_PWM2_INTR_MAP {
        &self.app_pwm2_intr_map
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn app_pwm3_intr_map(&self) -> &APP_PWM3_INTR_MAP {
        &self.app_pwm3_intr_map
    }
    #[doc = "0x2c4 - "]
    #[inline(always)]
    pub const fn app_ledc_int_map(&self) -> &APP_LEDC_INT_MAP {
        &self.app_ledc_int_map
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn app_efuse_int_map(&self) -> &APP_EFUSE_INT_MAP {
        &self.app_efuse_int_map
    }
    #[doc = "0x2cc - "]
    #[inline(always)]
    pub const fn app_can_int_map(&self) -> &APP_CAN_INT_MAP {
        &self.app_can_int_map
    }
    #[doc = "0x2d0 - "]
    #[inline(always)]
    pub const fn app_rtc_core_intr_map(&self) -> &APP_RTC_CORE_INTR_MAP {
        &self.app_rtc_core_intr_map
    }
    #[doc = "0x2d4 - "]
    #[inline(always)]
    pub const fn app_rmt_intr_map(&self) -> &APP_RMT_INTR_MAP {
        &self.app_rmt_intr_map
    }
    #[doc = "0x2d8 - "]
    #[inline(always)]
    pub const fn app_pcnt_intr_map(&self) -> &APP_PCNT_INTR_MAP {
        &self.app_pcnt_intr_map
    }
    #[doc = "0x2dc - "]
    #[inline(always)]
    pub const fn app_i2c_ext0_intr_map(&self) -> &APP_I2C_EXT0_INTR_MAP {
        &self.app_i2c_ext0_intr_map
    }
    #[doc = "0x2e0 - "]
    #[inline(always)]
    pub const fn app_i2c_ext1_intr_map(&self) -> &APP_I2C_EXT1_INTR_MAP {
        &self.app_i2c_ext1_intr_map
    }
    #[doc = "0x2e4 - "]
    #[inline(always)]
    pub const fn app_rsa_intr_map(&self) -> &APP_RSA_INTR_MAP {
        &self.app_rsa_intr_map
    }
    #[doc = "0x2e8 - "]
    #[inline(always)]
    pub const fn app_spi1_dma_int_map(&self) -> &APP_SPI1_DMA_INT_MAP {
        &self.app_spi1_dma_int_map
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn app_spi2_dma_int_map(&self) -> &APP_SPI2_DMA_INT_MAP {
        &self.app_spi2_dma_int_map
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn app_spi3_dma_int_map(&self) -> &APP_SPI3_DMA_INT_MAP {
        &self.app_spi3_dma_int_map
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn app_wdg_int_map(&self) -> &APP_WDG_INT_MAP {
        &self.app_wdg_int_map
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn app_timer_int1_map(&self) -> &APP_TIMER_INT1_MAP {
        &self.app_timer_int1_map
    }
    #[doc = "0x2fc - "]
    #[inline(always)]
    pub const fn app_timer_int2_map(&self) -> &APP_TIMER_INT2_MAP {
        &self.app_timer_int2_map
    }
    #[doc = "0x300 - "]
    #[inline(always)]
    pub const fn app_tg_t0_edge_int_map(&self) -> &APP_TG_T0_EDGE_INT_MAP {
        &self.app_tg_t0_edge_int_map
    }
    #[doc = "0x304 - "]
    #[inline(always)]
    pub const fn app_tg_t1_edge_int_map(&self) -> &APP_TG_T1_EDGE_INT_MAP {
        &self.app_tg_t1_edge_int_map
    }
    #[doc = "0x308 - "]
    #[inline(always)]
    pub const fn app_tg_wdt_edge_int_map(&self) -> &APP_TG_WDT_EDGE_INT_MAP {
        &self.app_tg_wdt_edge_int_map
    }
    #[doc = "0x30c - "]
    #[inline(always)]
    pub const fn app_tg_lact_edge_int_map(&self) -> &APP_TG_LACT_EDGE_INT_MAP {
        &self.app_tg_lact_edge_int_map
    }
    #[doc = "0x310 - "]
    #[inline(always)]
    pub const fn app_tg1_t0_edge_int_map(&self) -> &APP_TG1_T0_EDGE_INT_MAP {
        &self.app_tg1_t0_edge_int_map
    }
    #[doc = "0x314 - "]
    #[inline(always)]
    pub const fn app_tg1_t1_edge_int_map(&self) -> &APP_TG1_T1_EDGE_INT_MAP {
        &self.app_tg1_t1_edge_int_map
    }
    #[doc = "0x318 - "]
    #[inline(always)]
    pub const fn app_tg1_wdt_edge_int_map(&self) -> &APP_TG1_WDT_EDGE_INT_MAP {
        &self.app_tg1_wdt_edge_int_map
    }
    #[doc = "0x31c - "]
    #[inline(always)]
    pub const fn app_tg1_lact_edge_int_map(&self) -> &APP_TG1_LACT_EDGE_INT_MAP {
        &self.app_tg1_lact_edge_int_map
    }
    #[doc = "0x320 - "]
    #[inline(always)]
    pub const fn app_mmu_ia_int_map(&self) -> &APP_MMU_IA_INT_MAP {
        &self.app_mmu_ia_int_map
    }
    #[doc = "0x324 - "]
    #[inline(always)]
    pub const fn app_mpu_ia_int_map(&self) -> &APP_MPU_IA_INT_MAP {
        &self.app_mpu_ia_int_map
    }
    #[doc = "0x328 - "]
    #[inline(always)]
    pub const fn app_cache_ia_int_map(&self) -> &APP_CACHE_IA_INT_MAP {
        &self.app_cache_ia_int_map
    }
    #[doc = "0x32c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_uart(&self) -> &AHBLITE_MPU_TABLE_UART {
        &self.ahblite_mpu_table_uart
    }
    #[doc = "0x330 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_spi1(&self) -> &AHBLITE_MPU_TABLE_SPI1 {
        &self.ahblite_mpu_table_spi1
    }
    #[doc = "0x334 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_spi0(&self) -> &AHBLITE_MPU_TABLE_SPI0 {
        &self.ahblite_mpu_table_spi0
    }
    #[doc = "0x338 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_gpio(&self) -> &AHBLITE_MPU_TABLE_GPIO {
        &self.ahblite_mpu_table_gpio
    }
    #[doc = "0x33c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_fe2(&self) -> &AHBLITE_MPU_TABLE_FE2 {
        &self.ahblite_mpu_table_fe2
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_fe(&self) -> &AHBLITE_MPU_TABLE_FE {
        &self.ahblite_mpu_table_fe
    }
    #[doc = "0x344 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_timer(&self) -> &AHBLITE_MPU_TABLE_TIMER {
        &self.ahblite_mpu_table_timer
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_rtc(&self) -> &AHBLITE_MPU_TABLE_RTC {
        &self.ahblite_mpu_table_rtc
    }
    #[doc = "0x34c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_io_mux(&self) -> &AHBLITE_MPU_TABLE_IO_MUX {
        &self.ahblite_mpu_table_io_mux
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_wdg(&self) -> &AHBLITE_MPU_TABLE_WDG {
        &self.ahblite_mpu_table_wdg
    }
    #[doc = "0x354 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_hinf(&self) -> &AHBLITE_MPU_TABLE_HINF {
        &self.ahblite_mpu_table_hinf
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_uhci1(&self) -> &AHBLITE_MPU_TABLE_UHCI1 {
        &self.ahblite_mpu_table_uhci1
    }
    #[doc = "0x35c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_misc(&self) -> &AHBLITE_MPU_TABLE_MISC {
        &self.ahblite_mpu_table_misc
    }
    #[doc = "0x360 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_i2c(&self) -> &AHBLITE_MPU_TABLE_I2C {
        &self.ahblite_mpu_table_i2c
    }
    #[doc = "0x364 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_i2s0(&self) -> &AHBLITE_MPU_TABLE_I2S0 {
        &self.ahblite_mpu_table_i2s0
    }
    #[doc = "0x368 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_uart1(&self) -> &AHBLITE_MPU_TABLE_UART1 {
        &self.ahblite_mpu_table_uart1
    }
    #[doc = "0x36c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_bt(&self) -> &AHBLITE_MPU_TABLE_BT {
        &self.ahblite_mpu_table_bt
    }
    #[doc = "0x370 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_bt_buffer(&self) -> &AHBLITE_MPU_TABLE_BT_BUFFER {
        &self.ahblite_mpu_table_bt_buffer
    }
    #[doc = "0x374 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_i2c_ext0(&self) -> &AHBLITE_MPU_TABLE_I2C_EXT0 {
        &self.ahblite_mpu_table_i2c_ext0
    }
    #[doc = "0x378 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_uhci0(&self) -> &AHBLITE_MPU_TABLE_UHCI0 {
        &self.ahblite_mpu_table_uhci0
    }
    #[doc = "0x37c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_slchost(&self) -> &AHBLITE_MPU_TABLE_SLCHOST {
        &self.ahblite_mpu_table_slchost
    }
    #[doc = "0x380 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_rmt(&self) -> &AHBLITE_MPU_TABLE_RMT {
        &self.ahblite_mpu_table_rmt
    }
    #[doc = "0x384 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pcnt(&self) -> &AHBLITE_MPU_TABLE_PCNT {
        &self.ahblite_mpu_table_pcnt
    }
    #[doc = "0x388 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_slc(&self) -> &AHBLITE_MPU_TABLE_SLC {
        &self.ahblite_mpu_table_slc
    }
    #[doc = "0x38c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_ledc(&self) -> &AHBLITE_MPU_TABLE_LEDC {
        &self.ahblite_mpu_table_ledc
    }
    #[doc = "0x390 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_efuse(&self) -> &AHBLITE_MPU_TABLE_EFUSE {
        &self.ahblite_mpu_table_efuse
    }
    #[doc = "0x394 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_spi_encrypt(&self) -> &AHBLITE_MPU_TABLE_SPI_ENCRYPT {
        &self.ahblite_mpu_table_spi_encrypt
    }
    #[doc = "0x398 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_bb(&self) -> &AHBLITE_MPU_TABLE_BB {
        &self.ahblite_mpu_table_bb
    }
    #[doc = "0x39c - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pwm0(&self) -> &AHBLITE_MPU_TABLE_PWM0 {
        &self.ahblite_mpu_table_pwm0
    }
    #[doc = "0x3a0 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_timergroup(&self) -> &AHBLITE_MPU_TABLE_TIMERGROUP {
        &self.ahblite_mpu_table_timergroup
    }
    #[doc = "0x3a4 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_timergroup1(&self) -> &AHBLITE_MPU_TABLE_TIMERGROUP1 {
        &self.ahblite_mpu_table_timergroup1
    }
    #[doc = "0x3a8 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_spi2(&self) -> &AHBLITE_MPU_TABLE_SPI2 {
        &self.ahblite_mpu_table_spi2
    }
    #[doc = "0x3ac - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_spi3(&self) -> &AHBLITE_MPU_TABLE_SPI3 {
        &self.ahblite_mpu_table_spi3
    }
    #[doc = "0x3b0 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_apb_ctrl(&self) -> &AHBLITE_MPU_TABLE_APB_CTRL {
        &self.ahblite_mpu_table_apb_ctrl
    }
    #[doc = "0x3b4 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_i2c_ext1(&self) -> &AHBLITE_MPU_TABLE_I2C_EXT1 {
        &self.ahblite_mpu_table_i2c_ext1
    }
    #[doc = "0x3b8 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_sdio_host(&self) -> &AHBLITE_MPU_TABLE_SDIO_HOST {
        &self.ahblite_mpu_table_sdio_host
    }
    #[doc = "0x3bc - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_emac(&self) -> &AHBLITE_MPU_TABLE_EMAC {
        &self.ahblite_mpu_table_emac
    }
    #[doc = "0x3c0 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_can(&self) -> &AHBLITE_MPU_TABLE_CAN {
        &self.ahblite_mpu_table_can
    }
    #[doc = "0x3c4 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pwm1(&self) -> &AHBLITE_MPU_TABLE_PWM1 {
        &self.ahblite_mpu_table_pwm1
    }
    #[doc = "0x3c8 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_i2s1(&self) -> &AHBLITE_MPU_TABLE_I2S1 {
        &self.ahblite_mpu_table_i2s1
    }
    #[doc = "0x3cc - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_uart2(&self) -> &AHBLITE_MPU_TABLE_UART2 {
        &self.ahblite_mpu_table_uart2
    }
    #[doc = "0x3d0 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pwm2(&self) -> &AHBLITE_MPU_TABLE_PWM2 {
        &self.ahblite_mpu_table_pwm2
    }
    #[doc = "0x3d4 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pwm3(&self) -> &AHBLITE_MPU_TABLE_PWM3 {
        &self.ahblite_mpu_table_pwm3
    }
    #[doc = "0x3d8 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_rwbt(&self) -> &AHBLITE_MPU_TABLE_RWBT {
        &self.ahblite_mpu_table_rwbt
    }
    #[doc = "0x3dc - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_btmac(&self) -> &AHBLITE_MPU_TABLE_BTMAC {
        &self.ahblite_mpu_table_btmac
    }
    #[doc = "0x3e0 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_wifimac(&self) -> &AHBLITE_MPU_TABLE_WIFIMAC {
        &self.ahblite_mpu_table_wifimac
    }
    #[doc = "0x3e4 - "]
    #[inline(always)]
    pub const fn ahblite_mpu_table_pwr(&self) -> &AHBLITE_MPU_TABLE_PWR {
        &self.ahblite_mpu_table_pwr
    }
    #[doc = "0x3e8 - "]
    #[inline(always)]
    pub const fn mem_access_dbug0(&self) -> &MEM_ACCESS_DBUG0 {
        &self.mem_access_dbug0
    }
    #[doc = "0x3ec - "]
    #[inline(always)]
    pub const fn mem_access_dbug1(&self) -> &MEM_ACCESS_DBUG1 {
        &self.mem_access_dbug1
    }
    #[doc = "0x3f0 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug0(&self) -> &PRO_DCACHE_DBUG0 {
        &self.pro_dcache_dbug0
    }
    #[doc = "0x3f4 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug1(&self) -> &PRO_DCACHE_DBUG1 {
        &self.pro_dcache_dbug1
    }
    #[doc = "0x3f8 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug2(&self) -> &PRO_DCACHE_DBUG2 {
        &self.pro_dcache_dbug2
    }
    #[doc = "0x3fc - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug3(&self) -> &PRO_DCACHE_DBUG3 {
        &self.pro_dcache_dbug3
    }
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug4(&self) -> &PRO_DCACHE_DBUG4 {
        &self.pro_dcache_dbug4
    }
    #[doc = "0x404 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug5(&self) -> &PRO_DCACHE_DBUG5 {
        &self.pro_dcache_dbug5
    }
    #[doc = "0x408 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug6(&self) -> &PRO_DCACHE_DBUG6 {
        &self.pro_dcache_dbug6
    }
    #[doc = "0x40c - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug7(&self) -> &PRO_DCACHE_DBUG7 {
        &self.pro_dcache_dbug7
    }
    #[doc = "0x410 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug8(&self) -> &PRO_DCACHE_DBUG8 {
        &self.pro_dcache_dbug8
    }
    #[doc = "0x414 - "]
    #[inline(always)]
    pub const fn pro_dcache_dbug9(&self) -> &PRO_DCACHE_DBUG9 {
        &self.pro_dcache_dbug9
    }
    #[doc = "0x418 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug0(&self) -> &APP_DCACHE_DBUG0 {
        &self.app_dcache_dbug0
    }
    #[doc = "0x41c - "]
    #[inline(always)]
    pub const fn app_dcache_dbug1(&self) -> &APP_DCACHE_DBUG1 {
        &self.app_dcache_dbug1
    }
    #[doc = "0x420 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug2(&self) -> &APP_DCACHE_DBUG2 {
        &self.app_dcache_dbug2
    }
    #[doc = "0x424 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug3(&self) -> &APP_DCACHE_DBUG3 {
        &self.app_dcache_dbug3
    }
    #[doc = "0x428 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug4(&self) -> &APP_DCACHE_DBUG4 {
        &self.app_dcache_dbug4
    }
    #[doc = "0x42c - "]
    #[inline(always)]
    pub const fn app_dcache_dbug5(&self) -> &APP_DCACHE_DBUG5 {
        &self.app_dcache_dbug5
    }
    #[doc = "0x430 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug6(&self) -> &APP_DCACHE_DBUG6 {
        &self.app_dcache_dbug6
    }
    #[doc = "0x434 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug7(&self) -> &APP_DCACHE_DBUG7 {
        &self.app_dcache_dbug7
    }
    #[doc = "0x438 - "]
    #[inline(always)]
    pub const fn app_dcache_dbug8(&self) -> &APP_DCACHE_DBUG8 {
        &self.app_dcache_dbug8
    }
    #[doc = "0x43c - "]
    #[inline(always)]
    pub const fn app_dcache_dbug9(&self) -> &APP_DCACHE_DBUG9 {
        &self.app_dcache_dbug9
    }
    #[doc = "0x440 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_ctrl(&self) -> &PRO_CPU_RECORD_CTRL {
        &self.pro_cpu_record_ctrl
    }
    #[doc = "0x444 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_status(&self) -> &PRO_CPU_RECORD_STATUS {
        &self.pro_cpu_record_status
    }
    #[doc = "0x448 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pid(&self) -> &PRO_CPU_RECORD_PID {
        &self.pro_cpu_record_pid
    }
    #[doc = "0x44c - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebuginst(&self) -> &PRO_CPU_RECORD_PDEBUGINST {
        &self.pro_cpu_record_pdebuginst
    }
    #[doc = "0x450 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugstatus(&self) -> &PRO_CPU_RECORD_PDEBUGSTATUS {
        &self.pro_cpu_record_pdebugstatus
    }
    #[doc = "0x454 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugdata(&self) -> &PRO_CPU_RECORD_PDEBUGDATA {
        &self.pro_cpu_record_pdebugdata
    }
    #[doc = "0x458 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugpc(&self) -> &PRO_CPU_RECORD_PDEBUGPC {
        &self.pro_cpu_record_pdebugpc
    }
    #[doc = "0x45c - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugls0stat(&self) -> &PRO_CPU_RECORD_PDEBUGLS0STAT {
        &self.pro_cpu_record_pdebugls0stat
    }
    #[doc = "0x460 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugls0addr(&self) -> &PRO_CPU_RECORD_PDEBUGLS0ADDR {
        &self.pro_cpu_record_pdebugls0addr
    }
    #[doc = "0x464 - "]
    #[inline(always)]
    pub const fn pro_cpu_record_pdebugls0data(&self) -> &PRO_CPU_RECORD_PDEBUGLS0DATA {
        &self.pro_cpu_record_pdebugls0data
    }
    #[doc = "0x468 - "]
    #[inline(always)]
    pub const fn app_cpu_record_ctrl(&self) -> &APP_CPU_RECORD_CTRL {
        &self.app_cpu_record_ctrl
    }
    #[doc = "0x46c - "]
    #[inline(always)]
    pub const fn app_cpu_record_status(&self) -> &APP_CPU_RECORD_STATUS {
        &self.app_cpu_record_status
    }
    #[doc = "0x470 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pid(&self) -> &APP_CPU_RECORD_PID {
        &self.app_cpu_record_pid
    }
    #[doc = "0x474 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebuginst(&self) -> &APP_CPU_RECORD_PDEBUGINST {
        &self.app_cpu_record_pdebuginst
    }
    #[doc = "0x478 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugstatus(&self) -> &APP_CPU_RECORD_PDEBUGSTATUS {
        &self.app_cpu_record_pdebugstatus
    }
    #[doc = "0x47c - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugdata(&self) -> &APP_CPU_RECORD_PDEBUGDATA {
        &self.app_cpu_record_pdebugdata
    }
    #[doc = "0x480 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugpc(&self) -> &APP_CPU_RECORD_PDEBUGPC {
        &self.app_cpu_record_pdebugpc
    }
    #[doc = "0x484 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugls0stat(&self) -> &APP_CPU_RECORD_PDEBUGLS0STAT {
        &self.app_cpu_record_pdebugls0stat
    }
    #[doc = "0x488 - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugls0addr(&self) -> &APP_CPU_RECORD_PDEBUGLS0ADDR {
        &self.app_cpu_record_pdebugls0addr
    }
    #[doc = "0x48c - "]
    #[inline(always)]
    pub const fn app_cpu_record_pdebugls0data(&self) -> &APP_CPU_RECORD_PDEBUGLS0DATA {
        &self.app_cpu_record_pdebugls0data
    }
    #[doc = "0x490 - "]
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    #[doc = "0x494 - "]
    #[inline(always)]
    pub const fn rom_mpu_table0(&self) -> &ROM_MPU_TABLE0 {
        &self.rom_mpu_table0
    }
    #[doc = "0x498 - "]
    #[inline(always)]
    pub const fn rom_mpu_table1(&self) -> &ROM_MPU_TABLE1 {
        &self.rom_mpu_table1
    }
    #[doc = "0x49c - "]
    #[inline(always)]
    pub const fn rom_mpu_table2(&self) -> &ROM_MPU_TABLE2 {
        &self.rom_mpu_table2
    }
    #[doc = "0x4a0 - "]
    #[inline(always)]
    pub const fn rom_mpu_table3(&self) -> &ROM_MPU_TABLE3 {
        &self.rom_mpu_table3
    }
    #[doc = "0x4a4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table0(&self) -> &SHROM_MPU_TABLE0 {
        &self.shrom_mpu_table0
    }
    #[doc = "0x4a8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table1(&self) -> &SHROM_MPU_TABLE1 {
        &self.shrom_mpu_table1
    }
    #[doc = "0x4ac - "]
    #[inline(always)]
    pub const fn shrom_mpu_table2(&self) -> &SHROM_MPU_TABLE2 {
        &self.shrom_mpu_table2
    }
    #[doc = "0x4b0 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table3(&self) -> &SHROM_MPU_TABLE3 {
        &self.shrom_mpu_table3
    }
    #[doc = "0x4b4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table4(&self) -> &SHROM_MPU_TABLE4 {
        &self.shrom_mpu_table4
    }
    #[doc = "0x4b8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table5(&self) -> &SHROM_MPU_TABLE5 {
        &self.shrom_mpu_table5
    }
    #[doc = "0x4bc - "]
    #[inline(always)]
    pub const fn shrom_mpu_table6(&self) -> &SHROM_MPU_TABLE6 {
        &self.shrom_mpu_table6
    }
    #[doc = "0x4c0 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table7(&self) -> &SHROM_MPU_TABLE7 {
        &self.shrom_mpu_table7
    }
    #[doc = "0x4c4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table8(&self) -> &SHROM_MPU_TABLE8 {
        &self.shrom_mpu_table8
    }
    #[doc = "0x4c8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table9(&self) -> &SHROM_MPU_TABLE9 {
        &self.shrom_mpu_table9
    }
    #[doc = "0x4cc - "]
    #[inline(always)]
    pub const fn shrom_mpu_table10(&self) -> &SHROM_MPU_TABLE10 {
        &self.shrom_mpu_table10
    }
    #[doc = "0x4d0 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table11(&self) -> &SHROM_MPU_TABLE11 {
        &self.shrom_mpu_table11
    }
    #[doc = "0x4d4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table12(&self) -> &SHROM_MPU_TABLE12 {
        &self.shrom_mpu_table12
    }
    #[doc = "0x4d8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table13(&self) -> &SHROM_MPU_TABLE13 {
        &self.shrom_mpu_table13
    }
    #[doc = "0x4dc - "]
    #[inline(always)]
    pub const fn shrom_mpu_table14(&self) -> &SHROM_MPU_TABLE14 {
        &self.shrom_mpu_table14
    }
    #[doc = "0x4e0 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table15(&self) -> &SHROM_MPU_TABLE15 {
        &self.shrom_mpu_table15
    }
    #[doc = "0x4e4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table16(&self) -> &SHROM_MPU_TABLE16 {
        &self.shrom_mpu_table16
    }
    #[doc = "0x4e8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table17(&self) -> &SHROM_MPU_TABLE17 {
        &self.shrom_mpu_table17
    }
    #[doc = "0x4ec - "]
    #[inline(always)]
    pub const fn shrom_mpu_table18(&self) -> &SHROM_MPU_TABLE18 {
        &self.shrom_mpu_table18
    }
    #[doc = "0x4f0 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table19(&self) -> &SHROM_MPU_TABLE19 {
        &self.shrom_mpu_table19
    }
    #[doc = "0x4f4 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table20(&self) -> &SHROM_MPU_TABLE20 {
        &self.shrom_mpu_table20
    }
    #[doc = "0x4f8 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table21(&self) -> &SHROM_MPU_TABLE21 {
        &self.shrom_mpu_table21
    }
    #[doc = "0x4fc - "]
    #[inline(always)]
    pub const fn shrom_mpu_table22(&self) -> &SHROM_MPU_TABLE22 {
        &self.shrom_mpu_table22
    }
    #[doc = "0x500 - "]
    #[inline(always)]
    pub const fn shrom_mpu_table23(&self) -> &SHROM_MPU_TABLE23 {
        &self.shrom_mpu_table23
    }
    #[doc = "0x504 - "]
    #[inline(always)]
    pub const fn immu_table0(&self) -> &IMMU_TABLE0 {
        &self.immu_table0
    }
    #[doc = "0x508 - "]
    #[inline(always)]
    pub const fn immu_table1(&self) -> &IMMU_TABLE1 {
        &self.immu_table1
    }
    #[doc = "0x50c - "]
    #[inline(always)]
    pub const fn immu_table2(&self) -> &IMMU_TABLE2 {
        &self.immu_table2
    }
    #[doc = "0x510 - "]
    #[inline(always)]
    pub const fn immu_table3(&self) -> &IMMU_TABLE3 {
        &self.immu_table3
    }
    #[doc = "0x514 - "]
    #[inline(always)]
    pub const fn immu_table4(&self) -> &IMMU_TABLE4 {
        &self.immu_table4
    }
    #[doc = "0x518 - "]
    #[inline(always)]
    pub const fn immu_table5(&self) -> &IMMU_TABLE5 {
        &self.immu_table5
    }
    #[doc = "0x51c - "]
    #[inline(always)]
    pub const fn immu_table6(&self) -> &IMMU_TABLE6 {
        &self.immu_table6
    }
    #[doc = "0x520 - "]
    #[inline(always)]
    pub const fn immu_table7(&self) -> &IMMU_TABLE7 {
        &self.immu_table7
    }
    #[doc = "0x524 - "]
    #[inline(always)]
    pub const fn immu_table8(&self) -> &IMMU_TABLE8 {
        &self.immu_table8
    }
    #[doc = "0x528 - "]
    #[inline(always)]
    pub const fn immu_table9(&self) -> &IMMU_TABLE9 {
        &self.immu_table9
    }
    #[doc = "0x52c - "]
    #[inline(always)]
    pub const fn immu_table10(&self) -> &IMMU_TABLE10 {
        &self.immu_table10
    }
    #[doc = "0x530 - "]
    #[inline(always)]
    pub const fn immu_table11(&self) -> &IMMU_TABLE11 {
        &self.immu_table11
    }
    #[doc = "0x534 - "]
    #[inline(always)]
    pub const fn immu_table12(&self) -> &IMMU_TABLE12 {
        &self.immu_table12
    }
    #[doc = "0x538 - "]
    #[inline(always)]
    pub const fn immu_table13(&self) -> &IMMU_TABLE13 {
        &self.immu_table13
    }
    #[doc = "0x53c - "]
    #[inline(always)]
    pub const fn immu_table14(&self) -> &IMMU_TABLE14 {
        &self.immu_table14
    }
    #[doc = "0x540 - "]
    #[inline(always)]
    pub const fn immu_table15(&self) -> &IMMU_TABLE15 {
        &self.immu_table15
    }
    #[doc = "0x544 - "]
    #[inline(always)]
    pub const fn dmmu_table0(&self) -> &DMMU_TABLE0 {
        &self.dmmu_table0
    }
    #[doc = "0x548 - "]
    #[inline(always)]
    pub const fn dmmu_table1(&self) -> &DMMU_TABLE1 {
        &self.dmmu_table1
    }
    #[doc = "0x54c - "]
    #[inline(always)]
    pub const fn dmmu_table2(&self) -> &DMMU_TABLE2 {
        &self.dmmu_table2
    }
    #[doc = "0x550 - "]
    #[inline(always)]
    pub const fn dmmu_table3(&self) -> &DMMU_TABLE3 {
        &self.dmmu_table3
    }
    #[doc = "0x554 - "]
    #[inline(always)]
    pub const fn dmmu_table4(&self) -> &DMMU_TABLE4 {
        &self.dmmu_table4
    }
    #[doc = "0x558 - "]
    #[inline(always)]
    pub const fn dmmu_table5(&self) -> &DMMU_TABLE5 {
        &self.dmmu_table5
    }
    #[doc = "0x55c - "]
    #[inline(always)]
    pub const fn dmmu_table6(&self) -> &DMMU_TABLE6 {
        &self.dmmu_table6
    }
    #[doc = "0x560 - "]
    #[inline(always)]
    pub const fn dmmu_table7(&self) -> &DMMU_TABLE7 {
        &self.dmmu_table7
    }
    #[doc = "0x564 - "]
    #[inline(always)]
    pub const fn dmmu_table8(&self) -> &DMMU_TABLE8 {
        &self.dmmu_table8
    }
    #[doc = "0x568 - "]
    #[inline(always)]
    pub const fn dmmu_table9(&self) -> &DMMU_TABLE9 {
        &self.dmmu_table9
    }
    #[doc = "0x56c - "]
    #[inline(always)]
    pub const fn dmmu_table10(&self) -> &DMMU_TABLE10 {
        &self.dmmu_table10
    }
    #[doc = "0x570 - "]
    #[inline(always)]
    pub const fn dmmu_table11(&self) -> &DMMU_TABLE11 {
        &self.dmmu_table11
    }
    #[doc = "0x574 - "]
    #[inline(always)]
    pub const fn dmmu_table12(&self) -> &DMMU_TABLE12 {
        &self.dmmu_table12
    }
    #[doc = "0x578 - "]
    #[inline(always)]
    pub const fn dmmu_table13(&self) -> &DMMU_TABLE13 {
        &self.dmmu_table13
    }
    #[doc = "0x57c - "]
    #[inline(always)]
    pub const fn dmmu_table14(&self) -> &DMMU_TABLE14 {
        &self.dmmu_table14
    }
    #[doc = "0x580 - "]
    #[inline(always)]
    pub const fn dmmu_table15(&self) -> &DMMU_TABLE15 {
        &self.dmmu_table15
    }
    #[doc = "0x584 - "]
    #[inline(always)]
    pub const fn pro_intrusion_ctrl(&self) -> &PRO_INTRUSION_CTRL {
        &self.pro_intrusion_ctrl
    }
    #[doc = "0x588 - "]
    #[inline(always)]
    pub const fn pro_intrusion_status(&self) -> &PRO_INTRUSION_STATUS {
        &self.pro_intrusion_status
    }
    #[doc = "0x58c - "]
    #[inline(always)]
    pub const fn app_intrusion_ctrl(&self) -> &APP_INTRUSION_CTRL {
        &self.app_intrusion_ctrl
    }
    #[doc = "0x590 - "]
    #[inline(always)]
    pub const fn app_intrusion_status(&self) -> &APP_INTRUSION_STATUS {
        &self.app_intrusion_status
    }
    #[doc = "0x594 - "]
    #[inline(always)]
    pub const fn front_end_mem_pd(&self) -> &FRONT_END_MEM_PD {
        &self.front_end_mem_pd
    }
    #[doc = "0x598 - "]
    #[inline(always)]
    pub const fn mmu_ia_int_en(&self) -> &MMU_IA_INT_EN {
        &self.mmu_ia_int_en
    }
    #[doc = "0x59c - "]
    #[inline(always)]
    pub const fn mpu_ia_int_en(&self) -> &MPU_IA_INT_EN {
        &self.mpu_ia_int_en
    }
    #[doc = "0x5a0 - "]
    #[inline(always)]
    pub const fn cache_ia_int_en(&self) -> &CACHE_IA_INT_EN {
        &self.cache_ia_int_en
    }
    #[doc = "0x5a4 - "]
    #[inline(always)]
    pub const fn secure_boot_ctrl(&self) -> &SECURE_BOOT_CTRL {
        &self.secure_boot_ctrl
    }
    #[doc = "0x5a8 - "]
    #[inline(always)]
    pub const fn spi_dma_chan_sel(&self) -> &SPI_DMA_CHAN_SEL {
        &self.spi_dma_chan_sel
    }
    #[doc = "0x5ac - "]
    #[inline(always)]
    pub const fn pro_vecbase_ctrl(&self) -> &PRO_VECBASE_CTRL {
        &self.pro_vecbase_ctrl
    }
    #[doc = "0x5b0 - "]
    #[inline(always)]
    pub const fn pro_vecbase_set(&self) -> &PRO_VECBASE_SET {
        &self.pro_vecbase_set
    }
    #[doc = "0x5b4 - "]
    #[inline(always)]
    pub const fn app_vecbase_ctrl(&self) -> &APP_VECBASE_CTRL {
        &self.app_vecbase_ctrl
    }
    #[doc = "0x5b8 - "]
    #[inline(always)]
    pub const fn app_vecbase_set(&self) -> &APP_VECBASE_SET {
        &self.app_vecbase_set
    }
    #[doc = "0xffc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PRO_BOOT_REMAP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_boot_remap_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_boot_remap_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_boot_remap_ctrl`] module"]
pub type PRO_BOOT_REMAP_CTRL = crate::Reg<pro_boot_remap_ctrl::PRO_BOOT_REMAP_CTRL_SPEC>;
#[doc = ""]
pub mod pro_boot_remap_ctrl;
#[doc = "APP_BOOT_REMAP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_boot_remap_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_boot_remap_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_boot_remap_ctrl`] module"]
pub type APP_BOOT_REMAP_CTRL = crate::Reg<app_boot_remap_ctrl::APP_BOOT_REMAP_CTRL_SPEC>;
#[doc = ""]
pub mod app_boot_remap_ctrl;
#[doc = "ACCESS_CHECK (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`access_check::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@access_check`] module"]
pub type ACCESS_CHECK = crate::Reg<access_check::ACCESS_CHECK_SPEC>;
#[doc = ""]
pub mod access_check;
#[doc = "PRO_DPORT_APB_MASK0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_apb_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_apb_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_apb_mask0`] module"]
pub type PRO_DPORT_APB_MASK0 = crate::Reg<pro_dport_apb_mask0::PRO_DPORT_APB_MASK0_SPEC>;
#[doc = ""]
pub mod pro_dport_apb_mask0;
#[doc = "PRO_DPORT_APB_MASK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_apb_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_apb_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_apb_mask1`] module"]
pub type PRO_DPORT_APB_MASK1 = crate::Reg<pro_dport_apb_mask1::PRO_DPORT_APB_MASK1_SPEC>;
#[doc = ""]
pub mod pro_dport_apb_mask1;
#[doc = "APP_DPORT_APB_MASK0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dport_apb_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_dport_apb_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dport_apb_mask0`] module"]
pub type APP_DPORT_APB_MASK0 = crate::Reg<app_dport_apb_mask0::APP_DPORT_APB_MASK0_SPEC>;
#[doc = ""]
pub mod app_dport_apb_mask0;
#[doc = "APP_DPORT_APB_MASK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dport_apb_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_dport_apb_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dport_apb_mask1`] module"]
pub type APP_DPORT_APB_MASK1 = crate::Reg<app_dport_apb_mask1::APP_DPORT_APB_MASK1_SPEC>;
#[doc = ""]
pub mod app_dport_apb_mask1;
#[doc = "PERI_CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_en`] module"]
pub type PERI_CLK_EN = crate::Reg<peri_clk_en::PERI_CLK_EN_SPEC>;
#[doc = ""]
pub mod peri_clk_en;
#[doc = "PERI_RST_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_rst_en`] module"]
pub type PERI_RST_EN = crate::Reg<peri_rst_en::PERI_RST_EN_SPEC>;
#[doc = ""]
pub mod peri_rst_en;
#[doc = "WIFI_BB_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_cfg`] module"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_cfg_2`] module"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg_2;
#[doc = "APPCPU_CTRL_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appcpu_ctrl_a`] module"]
pub type APPCPU_CTRL_A = crate::Reg<appcpu_ctrl_a::APPCPU_CTRL_A_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_a;
#[doc = "APPCPU_CTRL_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appcpu_ctrl_b`] module"]
pub type APPCPU_CTRL_B = crate::Reg<appcpu_ctrl_b::APPCPU_CTRL_B_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_b;
#[doc = "APPCPU_CTRL_C (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appcpu_ctrl_c`] module"]
pub type APPCPU_CTRL_C = crate::Reg<appcpu_ctrl_c::APPCPU_CTRL_C_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_c;
#[doc = "APPCPU_CTRL_D (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appcpu_ctrl_d`] module"]
pub type APPCPU_CTRL_D = crate::Reg<appcpu_ctrl_d::APPCPU_CTRL_D_SPEC>;
#[doc = ""]
pub mod appcpu_ctrl_d;
#[doc = "CPU_PER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_per_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_per_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_per_conf`] module"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = ""]
pub mod cpu_per_conf;
#[doc = "PRO_CACHE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_ctrl`] module"]
pub type PRO_CACHE_CTRL = crate::Reg<pro_cache_ctrl::PRO_CACHE_CTRL_SPEC>;
#[doc = ""]
pub mod pro_cache_ctrl;
#[doc = "PRO_CACHE_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_ctrl1`] module"]
pub type PRO_CACHE_CTRL1 = crate::Reg<pro_cache_ctrl1::PRO_CACHE_CTRL1_SPEC>;
#[doc = ""]
pub mod pro_cache_ctrl1;
#[doc = "PRO_CACHE_LOCK_0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_lock_0_addr`] module"]
pub type PRO_CACHE_LOCK_0_ADDR = crate::Reg<pro_cache_lock_0_addr::PRO_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_0_addr;
#[doc = "PRO_CACHE_LOCK_1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_lock_1_addr`] module"]
pub type PRO_CACHE_LOCK_1_ADDR = crate::Reg<pro_cache_lock_1_addr::PRO_CACHE_LOCK_1_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_1_addr;
#[doc = "PRO_CACHE_LOCK_2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_lock_2_addr`] module"]
pub type PRO_CACHE_LOCK_2_ADDR = crate::Reg<pro_cache_lock_2_addr::PRO_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_2_addr;
#[doc = "PRO_CACHE_LOCK_3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_lock_3_addr`] module"]
pub type PRO_CACHE_LOCK_3_ADDR = crate::Reg<pro_cache_lock_3_addr::PRO_CACHE_LOCK_3_ADDR_SPEC>;
#[doc = ""]
pub mod pro_cache_lock_3_addr;
#[doc = "APP_CACHE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_ctrl`] module"]
pub type APP_CACHE_CTRL = crate::Reg<app_cache_ctrl::APP_CACHE_CTRL_SPEC>;
#[doc = ""]
pub mod app_cache_ctrl;
#[doc = "APP_CACHE_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_ctrl1`] module"]
pub type APP_CACHE_CTRL1 = crate::Reg<app_cache_ctrl1::APP_CACHE_CTRL1_SPEC>;
#[doc = ""]
pub mod app_cache_ctrl1;
#[doc = "APP_CACHE_LOCK_0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_lock_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_lock_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_lock_0_addr`] module"]
pub type APP_CACHE_LOCK_0_ADDR = crate::Reg<app_cache_lock_0_addr::APP_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_0_addr;
#[doc = "APP_CACHE_LOCK_1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_lock_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_lock_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_lock_1_addr`] module"]
pub type APP_CACHE_LOCK_1_ADDR = crate::Reg<app_cache_lock_1_addr::APP_CACHE_LOCK_1_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_1_addr;
#[doc = "APP_CACHE_LOCK_2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_lock_2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_lock_2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_lock_2_addr`] module"]
pub type APP_CACHE_LOCK_2_ADDR = crate::Reg<app_cache_lock_2_addr::APP_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_2_addr;
#[doc = "APP_CACHE_LOCK_3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_lock_3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_lock_3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_lock_3_addr`] module"]
pub type APP_CACHE_LOCK_3_ADDR = crate::Reg<app_cache_lock_3_addr::APP_CACHE_LOCK_3_ADDR_SPEC>;
#[doc = ""]
pub mod app_cache_lock_3_addr;
#[doc = "TRACEMEM_MUX_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracemem_mux_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracemem_mux_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracemem_mux_mode`] module"]
pub type TRACEMEM_MUX_MODE = crate::Reg<tracemem_mux_mode::TRACEMEM_MUX_MODE_SPEC>;
#[doc = ""]
pub mod tracemem_mux_mode;
#[doc = "PRO_TRACEMEM_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tracemem_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tracemem_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tracemem_ena`] module"]
pub type PRO_TRACEMEM_ENA = crate::Reg<pro_tracemem_ena::PRO_TRACEMEM_ENA_SPEC>;
#[doc = ""]
pub mod pro_tracemem_ena;
#[doc = "APP_TRACEMEM_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tracemem_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tracemem_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tracemem_ena`] module"]
pub type APP_TRACEMEM_ENA = crate::Reg<app_tracemem_ena::APP_TRACEMEM_ENA_SPEC>;
#[doc = ""]
pub mod app_tracemem_ena;
#[doc = "CACHE_MUX_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mux_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mux_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mux_mode`] module"]
pub type CACHE_MUX_MODE = crate::Reg<cache_mux_mode::CACHE_MUX_MODE_SPEC>;
#[doc = ""]
pub mod cache_mux_mode;
#[doc = "IMMU_PAGE_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_page_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_page_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_page_mode`] module"]
pub type IMMU_PAGE_MODE = crate::Reg<immu_page_mode::IMMU_PAGE_MODE_SPEC>;
#[doc = ""]
pub mod immu_page_mode;
#[doc = "DMMU_PAGE_MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_page_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_page_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_page_mode`] module"]
pub type DMMU_PAGE_MODE = crate::Reg<dmmu_page_mode::DMMU_PAGE_MODE_SPEC>;
#[doc = ""]
pub mod dmmu_page_mode;
#[doc = "ROM_MPU_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mpu_ena`] module"]
pub type ROM_MPU_ENA = crate::Reg<rom_mpu_ena::ROM_MPU_ENA_SPEC>;
#[doc = ""]
pub mod rom_mpu_ena;
#[doc = "MEM_PD_MASK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pd_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pd_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_pd_mask`] module"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = ""]
pub mod mem_pd_mask;
#[doc = "ROM_PD_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_pd_ctrl`] module"]
pub type ROM_PD_CTRL = crate::Reg<rom_pd_ctrl::ROM_PD_CTRL_SPEC>;
#[doc = ""]
pub mod rom_pd_ctrl;
#[doc = "ROM_FO_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_fo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_fo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_fo_ctrl`] module"]
pub type ROM_FO_CTRL = crate::Reg<rom_fo_ctrl::ROM_FO_CTRL_SPEC>;
#[doc = ""]
pub mod rom_fo_ctrl;
#[doc = "SRAM_PD_CTRL_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_pd_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_pd_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_pd_ctrl_0`] module"]
pub type SRAM_PD_CTRL_0 = crate::Reg<sram_pd_ctrl_0::SRAM_PD_CTRL_0_SPEC>;
#[doc = ""]
pub mod sram_pd_ctrl_0;
#[doc = "SRAM_PD_CTRL_1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_pd_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_pd_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_pd_ctrl_1`] module"]
pub type SRAM_PD_CTRL_1 = crate::Reg<sram_pd_ctrl_1::SRAM_PD_CTRL_1_SPEC>;
#[doc = ""]
pub mod sram_pd_ctrl_1;
#[doc = "SRAM_FO_CTRL_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_fo_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_fo_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_fo_ctrl_0`] module"]
pub type SRAM_FO_CTRL_0 = crate::Reg<sram_fo_ctrl_0::SRAM_FO_CTRL_0_SPEC>;
#[doc = ""]
pub mod sram_fo_ctrl_0;
#[doc = "SRAM_FO_CTRL_1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_fo_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_fo_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_fo_ctrl_1`] module"]
pub type SRAM_FO_CTRL_1 = crate::Reg<sram_fo_ctrl_1::SRAM_FO_CTRL_1_SPEC>;
#[doc = ""]
pub mod sram_fo_ctrl_1;
#[doc = "IRAM_DRAM_AHB_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iram_dram_ahb_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iram_dram_ahb_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iram_dram_ahb_sel`] module"]
pub type IRAM_DRAM_AHB_SEL = crate::Reg<iram_dram_ahb_sel::IRAM_DRAM_AHB_SEL_SPEC>;
#[doc = ""]
pub mod iram_dram_ahb_sel;
#[doc = "TAG_FO_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tag_fo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tag_fo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag_fo_ctrl`] module"]
pub type TAG_FO_CTRL = crate::Reg<tag_fo_ctrl::TAG_FO_CTRL_SPEC>;
#[doc = ""]
pub mod tag_fo_ctrl;
#[doc = "AHB_LITE_MASK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_lite_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_lite_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_lite_mask`] module"]
pub type AHB_LITE_MASK = crate::Reg<ahb_lite_mask::AHB_LITE_MASK_SPEC>;
#[doc = ""]
pub mod ahb_lite_mask;
#[doc = "AHB_MPU_TABLE_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_mpu_table_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_mpu_table_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_mpu_table_0`] module"]
pub type AHB_MPU_TABLE_0 = crate::Reg<ahb_mpu_table_0::AHB_MPU_TABLE_0_SPEC>;
#[doc = ""]
pub mod ahb_mpu_table_0;
#[doc = "AHB_MPU_TABLE_1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_mpu_table_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_mpu_table_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_mpu_table_1`] module"]
pub type AHB_MPU_TABLE_1 = crate::Reg<ahb_mpu_table_1::AHB_MPU_TABLE_1_SPEC>;
#[doc = ""]
pub mod ahb_mpu_table_1;
#[doc = "HOST_INF_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_inf_sel`] module"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = ""]
pub mod host_inf_sel;
#[doc = "PERIP_CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perip_clk_en`] module"]
pub type PERIP_CLK_EN = crate::Reg<perip_clk_en::PERIP_CLK_EN_SPEC>;
#[doc = ""]
pub mod perip_clk_en;
#[doc = "PERIP_RST_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perip_rst_en`] module"]
pub type PERIP_RST_EN = crate::Reg<perip_rst_en::PERIP_RST_EN_SPEC>;
#[doc = ""]
pub mod perip_rst_en;
#[doc = "SLAVE_SPI_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_spi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_spi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_spi_config`] module"]
pub type SLAVE_SPI_CONFIG = crate::Reg<slave_spi_config::SLAVE_SPI_CONFIG_SPEC>;
#[doc = ""]
pub mod slave_spi_config;
#[doc = "WIFI_CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_clk_en`] module"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = ""]
pub mod wifi_clk_en;
#[doc = "CORE_RST_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_rst_en`] module"]
pub type CORE_RST_EN = crate::Reg<core_rst_en::CORE_RST_EN_SPEC>;
#[doc = ""]
pub mod core_rst_en;
#[doc = "BT_LPCK_DIV_INT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_lpck_div_int`] module"]
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
#[doc = ""]
pub mod bt_lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_frac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_frac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_lpck_div_frac`] module"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = ""]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0`] module"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1`] module"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2`] module"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3`] module"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_3;
#[doc = "PRO_INTR_STATUS_0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_intr_status_0`] module"]
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
#[doc = ""]
pub mod pro_intr_status_0;
#[doc = "PRO_INTR_STATUS_1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_intr_status_1`] module"]
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
#[doc = ""]
pub mod pro_intr_status_1;
#[doc = "PRO_INTR_STATUS_2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_intr_status_2`] module"]
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
#[doc = ""]
pub mod pro_intr_status_2;
#[doc = "APP_INTR_STATUS_0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_intr_status_0`] module"]
pub type APP_INTR_STATUS_0 = crate::Reg<app_intr_status_0::APP_INTR_STATUS_0_SPEC>;
#[doc = ""]
pub mod app_intr_status_0;
#[doc = "APP_INTR_STATUS_1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_intr_status_1`] module"]
pub type APP_INTR_STATUS_1 = crate::Reg<app_intr_status_1::APP_INTR_STATUS_1_SPEC>;
#[doc = ""]
pub mod app_intr_status_1;
#[doc = "APP_INTR_STATUS_2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_intr_status_2`] module"]
pub type APP_INTR_STATUS_2 = crate::Reg<app_intr_status_2::APP_INTR_STATUS_2_SPEC>;
#[doc = ""]
pub mod app_intr_status_2;
#[doc = "PRO_MAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_mac_intr_map`] module"]
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_mac_intr_map;
#[doc = "PRO_MAC_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_mac_nmi_map`] module"]
pub type PRO_MAC_NMI_MAP = crate::Reg<pro_mac_nmi_map::PRO_MAC_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_mac_nmi_map;
#[doc = "PRO_BB_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_bb_int_map`] module"]
pub type PRO_BB_INT_MAP = crate::Reg<pro_bb_int_map::PRO_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bb_int_map;
#[doc = "PRO_BT_MAC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_bt_mac_int_map`] module"]
pub type PRO_BT_MAC_INT_MAP = crate::Reg<pro_bt_mac_int_map::PRO_BT_MAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_mac_int_map;
#[doc = "PRO_BT_BB_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_bt_bb_int_map`] module"]
pub type PRO_BT_BB_INT_MAP = crate::Reg<pro_bt_bb_int_map::PRO_BT_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_bb_int_map;
#[doc = "PRO_BT_BB_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_bt_bb_nmi_map`] module"]
pub type PRO_BT_BB_NMI_MAP = crate::Reg<pro_bt_bb_nmi_map::PRO_BT_BB_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_bt_bb_nmi_map;
#[doc = "PRO_RWBT_IRQ_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rwbt_irq_map`] module"]
pub type PRO_RWBT_IRQ_MAP = crate::Reg<pro_rwbt_irq_map::PRO_RWBT_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwbt_irq_map;
#[doc = "PRO_RWBLE_IRQ_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rwble_irq_map`] module"]
pub type PRO_RWBLE_IRQ_MAP = crate::Reg<pro_rwble_irq_map::PRO_RWBLE_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwble_irq_map;
#[doc = "PRO_RWBT_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rwbt_nmi_map`] module"]
pub type PRO_RWBT_NMI_MAP = crate::Reg<pro_rwbt_nmi_map::PRO_RWBT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwbt_nmi_map;
#[doc = "PRO_RWBLE_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rwble_nmi_map`] module"]
pub type PRO_RWBLE_NMI_MAP = crate::Reg<pro_rwble_nmi_map::PRO_RWBLE_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_rwble_nmi_map;
#[doc = "PRO_SLC0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_slc0_intr_map`] module"]
pub type PRO_SLC0_INTR_MAP = crate::Reg<pro_slc0_intr_map::PRO_SLC0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_slc0_intr_map;
#[doc = "PRO_SLC1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_slc1_intr_map`] module"]
pub type PRO_SLC1_INTR_MAP = crate::Reg<pro_slc1_intr_map::PRO_SLC1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_slc1_intr_map;
#[doc = "PRO_UHCI0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_uhci0_intr_map`] module"]
pub type PRO_UHCI0_INTR_MAP = crate::Reg<pro_uhci0_intr_map::PRO_UHCI0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uhci0_intr_map;
#[doc = "PRO_UHCI1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_uhci1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uhci1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_uhci1_intr_map`] module"]
pub type PRO_UHCI1_INTR_MAP = crate::Reg<pro_uhci1_intr_map::PRO_UHCI1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uhci1_intr_map;
#[doc = "PRO_TG_T0_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_t0_level_int_map`] module"]
pub type PRO_TG_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t0_level_int_map::PRO_TG_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t0_level_int_map;
#[doc = "PRO_TG_T1_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_t1_level_int_map`] module"]
pub type PRO_TG_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t1_level_int_map::PRO_TG_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t1_level_int_map;
#[doc = "PRO_TG_WDT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_wdt_level_int_map`] module"]
pub type PRO_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_wdt_level_int_map::PRO_TG_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_wdt_level_int_map;
#[doc = "PRO_TG_LACT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_lact_level_int_map`] module"]
pub type PRO_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_lact_level_int_map::PRO_TG_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_lact_level_int_map;
#[doc = "PRO_TG1_T0_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_t0_level_int_map`] module"]
pub type PRO_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t0_level_int_map::PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t0_level_int_map;
#[doc = "PRO_TG1_T1_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_t1_level_int_map`] module"]
pub type PRO_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t1_level_int_map::PRO_TG1_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t1_level_int_map;
#[doc = "PRO_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_wdt_level_int_map`] module"]
pub type PRO_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_wdt_level_int_map::PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_wdt_level_int_map;
#[doc = "PRO_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_lact_level_int_map`] module"]
pub type PRO_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_lact_level_int_map::PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_lact_level_int_map;
#[doc = "PRO_GPIO_INTERRUPT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_gpio_interrupt_map`] module"]
pub type PRO_GPIO_INTERRUPT_MAP = crate::Reg<pro_gpio_interrupt_map::PRO_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod pro_gpio_interrupt_map;
#[doc = "PRO_GPIO_INTERRUPT_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_gpio_interrupt_nmi_map`] module"]
pub type PRO_GPIO_INTERRUPT_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_nmi_map::PRO_GPIO_INTERRUPT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod pro_gpio_interrupt_nmi_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_intr_from_cpu_0_map`] module"]
pub type PRO_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_0_map::PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_0_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_intr_from_cpu_1_map`] module"]
pub type PRO_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_1_map::PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_1_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_intr_from_cpu_2_map`] module"]
pub type PRO_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_2_map::PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_2_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_intr_from_cpu_3_map`] module"]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_3_map::PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = ""]
pub mod pro_cpu_intr_from_cpu_3_map;
#[doc = "PRO_SPI_INTR_0_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi_intr_0_map`] module"]
pub type PRO_SPI_INTR_0_MAP = crate::Reg<pro_spi_intr_0_map::PRO_SPI_INTR_0_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_0_map;
#[doc = "PRO_SPI_INTR_1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi_intr_1_map`] module"]
pub type PRO_SPI_INTR_1_MAP = crate::Reg<pro_spi_intr_1_map::PRO_SPI_INTR_1_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_1_map;
#[doc = "PRO_SPI_INTR_2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi_intr_2_map`] module"]
pub type PRO_SPI_INTR_2_MAP = crate::Reg<pro_spi_intr_2_map::PRO_SPI_INTR_2_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_2_map;
#[doc = "PRO_SPI_INTR_3_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi_intr_3_map`] module"]
pub type PRO_SPI_INTR_3_MAP = crate::Reg<pro_spi_intr_3_map::PRO_SPI_INTR_3_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi_intr_3_map;
#[doc = "PRO_I2S0_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_i2s0_int_map`] module"]
pub type PRO_I2S0_INT_MAP = crate::Reg<pro_i2s0_int_map::PRO_I2S0_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2s0_int_map;
#[doc = "PRO_I2S1_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_i2s1_int_map`] module"]
pub type PRO_I2S1_INT_MAP = crate::Reg<pro_i2s1_int_map::PRO_I2S1_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2s1_int_map;
#[doc = "PRO_UART_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_uart_intr_map`] module"]
pub type PRO_UART_INTR_MAP = crate::Reg<pro_uart_intr_map::PRO_UART_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart_intr_map;
#[doc = "PRO_UART1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_uart1_intr_map`] module"]
pub type PRO_UART1_INTR_MAP = crate::Reg<pro_uart1_intr_map::PRO_UART1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart1_intr_map;
#[doc = "PRO_UART2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_uart2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_uart2_intr_map`] module"]
pub type PRO_UART2_INTR_MAP = crate::Reg<pro_uart2_intr_map::PRO_UART2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_uart2_intr_map;
#[doc = "PRO_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_sdio_host_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_sdio_host_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_sdio_host_interrupt_map`] module"]
pub type PRO_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<pro_sdio_host_interrupt_map::PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod pro_sdio_host_interrupt_map;
#[doc = "PRO_EMAC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_emac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_emac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_emac_int_map`] module"]
pub type PRO_EMAC_INT_MAP = crate::Reg<pro_emac_int_map::PRO_EMAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_emac_int_map;
#[doc = "PRO_PWM0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_pwm0_intr_map`] module"]
pub type PRO_PWM0_INTR_MAP = crate::Reg<pro_pwm0_intr_map::PRO_PWM0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm0_intr_map;
#[doc = "PRO_PWM1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_pwm1_intr_map`] module"]
pub type PRO_PWM1_INTR_MAP = crate::Reg<pro_pwm1_intr_map::PRO_PWM1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm1_intr_map;
#[doc = "PRO_PWM2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_pwm2_intr_map`] module"]
pub type PRO_PWM2_INTR_MAP = crate::Reg<pro_pwm2_intr_map::PRO_PWM2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm2_intr_map;
#[doc = "PRO_PWM3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_pwm3_intr_map`] module"]
pub type PRO_PWM3_INTR_MAP = crate::Reg<pro_pwm3_intr_map::PRO_PWM3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pwm3_intr_map;
#[doc = "PRO_LEDC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ledc_int_map`] module"]
pub type PRO_LEDC_INT_MAP = crate::Reg<pro_ledc_int_map::PRO_LEDC_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_ledc_int_map;
#[doc = "PRO_EFUSE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_efuse_int_map`] module"]
pub type PRO_EFUSE_INT_MAP = crate::Reg<pro_efuse_int_map::PRO_EFUSE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_efuse_int_map;
#[doc = "PRO_CAN_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_can_int_map`] module"]
pub type PRO_CAN_INT_MAP = crate::Reg<pro_can_int_map::PRO_CAN_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_can_int_map;
#[doc = "PRO_RTC_CORE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rtc_core_intr_map`] module"]
pub type PRO_RTC_CORE_INTR_MAP = crate::Reg<pro_rtc_core_intr_map::PRO_RTC_CORE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rtc_core_intr_map;
#[doc = "PRO_RMT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rmt_intr_map`] module"]
pub type PRO_RMT_INTR_MAP = crate::Reg<pro_rmt_intr_map::PRO_RMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rmt_intr_map;
#[doc = "PRO_PCNT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_pcnt_intr_map`] module"]
pub type PRO_PCNT_INTR_MAP = crate::Reg<pro_pcnt_intr_map::PRO_PCNT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_pcnt_intr_map;
#[doc = "PRO_I2C_EXT0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_i2c_ext0_intr_map`] module"]
pub type PRO_I2C_EXT0_INTR_MAP = crate::Reg<pro_i2c_ext0_intr_map::PRO_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2c_ext0_intr_map;
#[doc = "PRO_I2C_EXT1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_i2c_ext1_intr_map`] module"]
pub type PRO_I2C_EXT1_INTR_MAP = crate::Reg<pro_i2c_ext1_intr_map::PRO_I2C_EXT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_i2c_ext1_intr_map;
#[doc = "PRO_RSA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_rsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_rsa_intr_map`] module"]
pub type PRO_RSA_INTR_MAP = crate::Reg<pro_rsa_intr_map::PRO_RSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pro_rsa_intr_map;
#[doc = "PRO_SPI1_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi1_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi1_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi1_dma_int_map`] module"]
pub type PRO_SPI1_DMA_INT_MAP = crate::Reg<pro_spi1_dma_int_map::PRO_SPI1_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi1_dma_int_map;
#[doc = "PRO_SPI2_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi2_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi2_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi2_dma_int_map`] module"]
pub type PRO_SPI2_DMA_INT_MAP = crate::Reg<pro_spi2_dma_int_map::PRO_SPI2_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi2_dma_int_map;
#[doc = "PRO_SPI3_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_spi3_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi3_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_spi3_dma_int_map`] module"]
pub type PRO_SPI3_DMA_INT_MAP = crate::Reg<pro_spi3_dma_int_map::PRO_SPI3_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_spi3_dma_int_map;
#[doc = "PRO_WDG_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_wdg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_wdg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_wdg_int_map`] module"]
pub type PRO_WDG_INT_MAP = crate::Reg<pro_wdg_int_map::PRO_WDG_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_wdg_int_map;
#[doc = "PRO_TIMER_INT1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_timer_int1_map`] module"]
pub type PRO_TIMER_INT1_MAP = crate::Reg<pro_timer_int1_map::PRO_TIMER_INT1_MAP_SPEC>;
#[doc = ""]
pub mod pro_timer_int1_map;
#[doc = "PRO_TIMER_INT2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_timer_int2_map`] module"]
pub type PRO_TIMER_INT2_MAP = crate::Reg<pro_timer_int2_map::PRO_TIMER_INT2_MAP_SPEC>;
#[doc = ""]
pub mod pro_timer_int2_map;
#[doc = "PRO_TG_T0_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_t0_edge_int_map`] module"]
pub type PRO_TG_T0_EDGE_INT_MAP = crate::Reg<pro_tg_t0_edge_int_map::PRO_TG_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t0_edge_int_map;
#[doc = "PRO_TG_T1_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_t1_edge_int_map`] module"]
pub type PRO_TG_T1_EDGE_INT_MAP = crate::Reg<pro_tg_t1_edge_int_map::PRO_TG_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_t1_edge_int_map;
#[doc = "PRO_TG_WDT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_wdt_edge_int_map`] module"]
pub type PRO_TG_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg_wdt_edge_int_map::PRO_TG_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_wdt_edge_int_map;
#[doc = "PRO_TG_LACT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg_lact_edge_int_map`] module"]
pub type PRO_TG_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg_lact_edge_int_map::PRO_TG_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg_lact_edge_int_map;
#[doc = "PRO_TG1_T0_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_t0_edge_int_map`] module"]
pub type PRO_TG1_T0_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t0_edge_int_map::PRO_TG1_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t0_edge_int_map;
#[doc = "PRO_TG1_T1_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_t1_edge_int_map`] module"]
pub type PRO_TG1_T1_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t1_edge_int_map::PRO_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_t1_edge_int_map;
#[doc = "PRO_TG1_WDT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_wdt_edge_int_map`] module"]
pub type PRO_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_wdt_edge_int_map::PRO_TG1_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_wdt_edge_int_map;
#[doc = "PRO_TG1_LACT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_tg1_lact_edge_int_map`] module"]
pub type PRO_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_lact_edge_int_map::PRO_TG1_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_tg1_lact_edge_int_map;
#[doc = "PRO_MMU_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_mmu_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mmu_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_mmu_ia_int_map`] module"]
pub type PRO_MMU_IA_INT_MAP = crate::Reg<pro_mmu_ia_int_map::PRO_MMU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_mmu_ia_int_map;
#[doc = "PRO_MPU_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_mpu_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mpu_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_mpu_ia_int_map`] module"]
pub type PRO_MPU_IA_INT_MAP = crate::Reg<pro_mpu_ia_int_map::PRO_MPU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_mpu_ia_int_map;
#[doc = "PRO_CACHE_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_ia_int_map`] module"]
pub type PRO_CACHE_IA_INT_MAP = crate::Reg<pro_cache_ia_int_map::PRO_CACHE_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod pro_cache_ia_int_map;
#[doc = "APP_MAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_mac_intr_map`] module"]
pub type APP_MAC_INTR_MAP = crate::Reg<app_mac_intr_map::APP_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_mac_intr_map;
#[doc = "APP_MAC_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_mac_nmi_map`] module"]
pub type APP_MAC_NMI_MAP = crate::Reg<app_mac_nmi_map::APP_MAC_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_mac_nmi_map;
#[doc = "APP_BB_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_bb_int_map`] module"]
pub type APP_BB_INT_MAP = crate::Reg<app_bb_int_map::APP_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bb_int_map;
#[doc = "APP_BT_MAC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_bt_mac_int_map`] module"]
pub type APP_BT_MAC_INT_MAP = crate::Reg<app_bt_mac_int_map::APP_BT_MAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_mac_int_map;
#[doc = "APP_BT_BB_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_bt_bb_int_map`] module"]
pub type APP_BT_BB_INT_MAP = crate::Reg<app_bt_bb_int_map::APP_BT_BB_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_bb_int_map;
#[doc = "APP_BT_BB_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_bt_bb_nmi_map`] module"]
pub type APP_BT_BB_NMI_MAP = crate::Reg<app_bt_bb_nmi_map::APP_BT_BB_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_bt_bb_nmi_map;
#[doc = "APP_RWBT_IRQ_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rwbt_irq_map`] module"]
pub type APP_RWBT_IRQ_MAP = crate::Reg<app_rwbt_irq_map::APP_RWBT_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod app_rwbt_irq_map;
#[doc = "APP_RWBLE_IRQ_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rwble_irq_map`] module"]
pub type APP_RWBLE_IRQ_MAP = crate::Reg<app_rwble_irq_map::APP_RWBLE_IRQ_MAP_SPEC>;
#[doc = ""]
pub mod app_rwble_irq_map;
#[doc = "APP_RWBT_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rwbt_nmi_map`] module"]
pub type APP_RWBT_NMI_MAP = crate::Reg<app_rwbt_nmi_map::APP_RWBT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_rwbt_nmi_map;
#[doc = "APP_RWBLE_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rwble_nmi_map`] module"]
pub type APP_RWBLE_NMI_MAP = crate::Reg<app_rwble_nmi_map::APP_RWBLE_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_rwble_nmi_map;
#[doc = "APP_SLC0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_slc0_intr_map`] module"]
pub type APP_SLC0_INTR_MAP = crate::Reg<app_slc0_intr_map::APP_SLC0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_slc0_intr_map;
#[doc = "APP_SLC1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_slc1_intr_map`] module"]
pub type APP_SLC1_INTR_MAP = crate::Reg<app_slc1_intr_map::APP_SLC1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_slc1_intr_map;
#[doc = "APP_UHCI0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uhci0_intr_map`] module"]
pub type APP_UHCI0_INTR_MAP = crate::Reg<app_uhci0_intr_map::APP_UHCI0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uhci0_intr_map;
#[doc = "APP_UHCI1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_uhci1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_uhci1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uhci1_intr_map`] module"]
pub type APP_UHCI1_INTR_MAP = crate::Reg<app_uhci1_intr_map::APP_UHCI1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uhci1_intr_map;
#[doc = "APP_TG_T0_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_t0_level_int_map`] module"]
pub type APP_TG_T0_LEVEL_INT_MAP =
    crate::Reg<app_tg_t0_level_int_map::APP_TG_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t0_level_int_map;
#[doc = "APP_TG_T1_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_t1_level_int_map`] module"]
pub type APP_TG_T1_LEVEL_INT_MAP =
    crate::Reg<app_tg_t1_level_int_map::APP_TG_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t1_level_int_map;
#[doc = "APP_TG_WDT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_wdt_level_int_map`] module"]
pub type APP_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<app_tg_wdt_level_int_map::APP_TG_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_wdt_level_int_map;
#[doc = "APP_TG_LACT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_lact_level_int_map`] module"]
pub type APP_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<app_tg_lact_level_int_map::APP_TG_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_lact_level_int_map;
#[doc = "APP_TG1_T0_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_t0_level_int_map`] module"]
pub type APP_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<app_tg1_t0_level_int_map::APP_TG1_T0_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t0_level_int_map;
#[doc = "APP_TG1_T1_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_t1_level_int_map`] module"]
pub type APP_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<app_tg1_t1_level_int_map::APP_TG1_T1_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t1_level_int_map;
#[doc = "APP_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_wdt_level_int_map`] module"]
pub type APP_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<app_tg1_wdt_level_int_map::APP_TG1_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_wdt_level_int_map;
#[doc = "APP_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_lact_level_int_map`] module"]
pub type APP_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<app_tg1_lact_level_int_map::APP_TG1_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_lact_level_int_map;
#[doc = "APP_GPIO_INTERRUPT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_gpio_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_gpio_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_gpio_interrupt_map`] module"]
pub type APP_GPIO_INTERRUPT_MAP = crate::Reg<app_gpio_interrupt_map::APP_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod app_gpio_interrupt_map;
#[doc = "APP_GPIO_INTERRUPT_NMI_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_gpio_interrupt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_gpio_interrupt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_gpio_interrupt_nmi_map`] module"]
pub type APP_GPIO_INTERRUPT_NMI_MAP =
    crate::Reg<app_gpio_interrupt_nmi_map::APP_GPIO_INTERRUPT_NMI_MAP_SPEC>;
#[doc = ""]
pub mod app_gpio_interrupt_nmi_map;
#[doc = "APP_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_intr_from_cpu_0_map`] module"]
pub type APP_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<app_cpu_intr_from_cpu_0_map::APP_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_0_map;
#[doc = "APP_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_intr_from_cpu_1_map`] module"]
pub type APP_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<app_cpu_intr_from_cpu_1_map::APP_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_1_map;
#[doc = "APP_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_intr_from_cpu_2_map`] module"]
pub type APP_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<app_cpu_intr_from_cpu_2_map::APP_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_2_map;
#[doc = "APP_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_intr_from_cpu_3_map`] module"]
pub type APP_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<app_cpu_intr_from_cpu_3_map::APP_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = ""]
pub mod app_cpu_intr_from_cpu_3_map;
#[doc = "APP_SPI_INTR_0_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi_intr_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi_intr_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_intr_0_map`] module"]
pub type APP_SPI_INTR_0_MAP = crate::Reg<app_spi_intr_0_map::APP_SPI_INTR_0_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_0_map;
#[doc = "APP_SPI_INTR_1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_intr_1_map`] module"]
pub type APP_SPI_INTR_1_MAP = crate::Reg<app_spi_intr_1_map::APP_SPI_INTR_1_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_1_map;
#[doc = "APP_SPI_INTR_2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_intr_2_map`] module"]
pub type APP_SPI_INTR_2_MAP = crate::Reg<app_spi_intr_2_map::APP_SPI_INTR_2_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_2_map;
#[doc = "APP_SPI_INTR_3_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi_intr_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi_intr_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_intr_3_map`] module"]
pub type APP_SPI_INTR_3_MAP = crate::Reg<app_spi_intr_3_map::APP_SPI_INTR_3_MAP_SPEC>;
#[doc = ""]
pub mod app_spi_intr_3_map;
#[doc = "APP_I2S0_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2s0_int_map`] module"]
pub type APP_I2S0_INT_MAP = crate::Reg<app_i2s0_int_map::APP_I2S0_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_i2s0_int_map;
#[doc = "APP_I2S1_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2s1_int_map`] module"]
pub type APP_I2S1_INT_MAP = crate::Reg<app_i2s1_int_map::APP_I2S1_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_i2s1_int_map;
#[doc = "APP_UART_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart_intr_map`] module"]
pub type APP_UART_INTR_MAP = crate::Reg<app_uart_intr_map::APP_UART_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart_intr_map;
#[doc = "APP_UART1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart1_intr_map`] module"]
pub type APP_UART1_INTR_MAP = crate::Reg<app_uart1_intr_map::APP_UART1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart1_intr_map;
#[doc = "APP_UART2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_uart2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_uart2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart2_intr_map`] module"]
pub type APP_UART2_INTR_MAP = crate::Reg<app_uart2_intr_map::APP_UART2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_uart2_intr_map;
#[doc = "APP_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_sdio_host_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_sdio_host_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_sdio_host_interrupt_map`] module"]
pub type APP_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<app_sdio_host_interrupt_map::APP_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = ""]
pub mod app_sdio_host_interrupt_map;
#[doc = "APP_EMAC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_emac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_emac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_emac_int_map`] module"]
pub type APP_EMAC_INT_MAP = crate::Reg<app_emac_int_map::APP_EMAC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_emac_int_map;
#[doc = "APP_PWM0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_pwm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_pwm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_pwm0_intr_map`] module"]
pub type APP_PWM0_INTR_MAP = crate::Reg<app_pwm0_intr_map::APP_PWM0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm0_intr_map;
#[doc = "APP_PWM1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_pwm1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_pwm1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_pwm1_intr_map`] module"]
pub type APP_PWM1_INTR_MAP = crate::Reg<app_pwm1_intr_map::APP_PWM1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm1_intr_map;
#[doc = "APP_PWM2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_pwm2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_pwm2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_pwm2_intr_map`] module"]
pub type APP_PWM2_INTR_MAP = crate::Reg<app_pwm2_intr_map::APP_PWM2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm2_intr_map;
#[doc = "APP_PWM3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_pwm3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_pwm3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_pwm3_intr_map`] module"]
pub type APP_PWM3_INTR_MAP = crate::Reg<app_pwm3_intr_map::APP_PWM3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pwm3_intr_map;
#[doc = "APP_LEDC_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_ledc_int_map`] module"]
pub type APP_LEDC_INT_MAP = crate::Reg<app_ledc_int_map::APP_LEDC_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_ledc_int_map;
#[doc = "APP_EFUSE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_efuse_int_map`] module"]
pub type APP_EFUSE_INT_MAP = crate::Reg<app_efuse_int_map::APP_EFUSE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_efuse_int_map;
#[doc = "APP_CAN_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_can_int_map`] module"]
pub type APP_CAN_INT_MAP = crate::Reg<app_can_int_map::APP_CAN_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_can_int_map;
#[doc = "APP_RTC_CORE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rtc_core_intr_map`] module"]
pub type APP_RTC_CORE_INTR_MAP = crate::Reg<app_rtc_core_intr_map::APP_RTC_CORE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rtc_core_intr_map;
#[doc = "APP_RMT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rmt_intr_map`] module"]
pub type APP_RMT_INTR_MAP = crate::Reg<app_rmt_intr_map::APP_RMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rmt_intr_map;
#[doc = "APP_PCNT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_pcnt_intr_map`] module"]
pub type APP_PCNT_INTR_MAP = crate::Reg<app_pcnt_intr_map::APP_PCNT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_pcnt_intr_map;
#[doc = "APP_I2C_EXT0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2c_ext0_intr_map`] module"]
pub type APP_I2C_EXT0_INTR_MAP = crate::Reg<app_i2c_ext0_intr_map::APP_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_i2c_ext0_intr_map;
#[doc = "APP_I2C_EXT1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2c_ext1_intr_map`] module"]
pub type APP_I2C_EXT1_INTR_MAP = crate::Reg<app_i2c_ext1_intr_map::APP_I2C_EXT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_i2c_ext1_intr_map;
#[doc = "APP_RSA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_rsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rsa_intr_map`] module"]
pub type APP_RSA_INTR_MAP = crate::Reg<app_rsa_intr_map::APP_RSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod app_rsa_intr_map;
#[doc = "APP_SPI1_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi1_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi1_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi1_dma_int_map`] module"]
pub type APP_SPI1_DMA_INT_MAP = crate::Reg<app_spi1_dma_int_map::APP_SPI1_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi1_dma_int_map;
#[doc = "APP_SPI2_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi2_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi2_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi2_dma_int_map`] module"]
pub type APP_SPI2_DMA_INT_MAP = crate::Reg<app_spi2_dma_int_map::APP_SPI2_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi2_dma_int_map;
#[doc = "APP_SPI3_DMA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_spi3_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_spi3_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi3_dma_int_map`] module"]
pub type APP_SPI3_DMA_INT_MAP = crate::Reg<app_spi3_dma_int_map::APP_SPI3_DMA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_spi3_dma_int_map;
#[doc = "APP_WDG_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_wdg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_wdg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_wdg_int_map`] module"]
pub type APP_WDG_INT_MAP = crate::Reg<app_wdg_int_map::APP_WDG_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_wdg_int_map;
#[doc = "APP_TIMER_INT1_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_timer_int1_map`] module"]
pub type APP_TIMER_INT1_MAP = crate::Reg<app_timer_int1_map::APP_TIMER_INT1_MAP_SPEC>;
#[doc = ""]
pub mod app_timer_int1_map;
#[doc = "APP_TIMER_INT2_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_timer_int2_map`] module"]
pub type APP_TIMER_INT2_MAP = crate::Reg<app_timer_int2_map::APP_TIMER_INT2_MAP_SPEC>;
#[doc = ""]
pub mod app_timer_int2_map;
#[doc = "APP_TG_T0_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_t0_edge_int_map`] module"]
pub type APP_TG_T0_EDGE_INT_MAP = crate::Reg<app_tg_t0_edge_int_map::APP_TG_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t0_edge_int_map;
#[doc = "APP_TG_T1_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_t1_edge_int_map`] module"]
pub type APP_TG_T1_EDGE_INT_MAP = crate::Reg<app_tg_t1_edge_int_map::APP_TG_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_t1_edge_int_map;
#[doc = "APP_TG_WDT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_wdt_edge_int_map`] module"]
pub type APP_TG_WDT_EDGE_INT_MAP =
    crate::Reg<app_tg_wdt_edge_int_map::APP_TG_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_wdt_edge_int_map;
#[doc = "APP_TG_LACT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg_lact_edge_int_map`] module"]
pub type APP_TG_LACT_EDGE_INT_MAP =
    crate::Reg<app_tg_lact_edge_int_map::APP_TG_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg_lact_edge_int_map;
#[doc = "APP_TG1_T0_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_t0_edge_int_map`] module"]
pub type APP_TG1_T0_EDGE_INT_MAP =
    crate::Reg<app_tg1_t0_edge_int_map::APP_TG1_T0_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t0_edge_int_map;
#[doc = "APP_TG1_T1_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_t1_edge_int_map`] module"]
pub type APP_TG1_T1_EDGE_INT_MAP =
    crate::Reg<app_tg1_t1_edge_int_map::APP_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_t1_edge_int_map;
#[doc = "APP_TG1_WDT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_wdt_edge_int_map`] module"]
pub type APP_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<app_tg1_wdt_edge_int_map::APP_TG1_WDT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_wdt_edge_int_map;
#[doc = "APP_TG1_LACT_EDGE_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tg1_lact_edge_int_map`] module"]
pub type APP_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<app_tg1_lact_edge_int_map::APP_TG1_LACT_EDGE_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_tg1_lact_edge_int_map;
#[doc = "APP_MMU_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_mmu_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_mmu_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_mmu_ia_int_map`] module"]
pub type APP_MMU_IA_INT_MAP = crate::Reg<app_mmu_ia_int_map::APP_MMU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_mmu_ia_int_map;
#[doc = "APP_MPU_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_mpu_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_mpu_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_mpu_ia_int_map`] module"]
pub type APP_MPU_IA_INT_MAP = crate::Reg<app_mpu_ia_int_map::APP_MPU_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_mpu_ia_int_map;
#[doc = "APP_CACHE_IA_INT_MAP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cache_ia_int_map`] module"]
pub type APP_CACHE_IA_INT_MAP = crate::Reg<app_cache_ia_int_map::APP_CACHE_IA_INT_MAP_SPEC>;
#[doc = ""]
pub mod app_cache_ia_int_map;
#[doc = "AHBLITE_MPU_TABLE_UART (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_uart`] module"]
pub type AHBLITE_MPU_TABLE_UART = crate::Reg<ahblite_mpu_table_uart::AHBLITE_MPU_TABLE_UART_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart;
#[doc = "AHBLITE_MPU_TABLE_SPI1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_spi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_spi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_spi1`] module"]
pub type AHBLITE_MPU_TABLE_SPI1 = crate::Reg<ahblite_mpu_table_spi1::AHBLITE_MPU_TABLE_SPI1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi1;
#[doc = "AHBLITE_MPU_TABLE_SPI0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_spi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_spi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_spi0`] module"]
pub type AHBLITE_MPU_TABLE_SPI0 = crate::Reg<ahblite_mpu_table_spi0::AHBLITE_MPU_TABLE_SPI0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi0;
#[doc = "AHBLITE_MPU_TABLE_GPIO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_gpio`] module"]
pub type AHBLITE_MPU_TABLE_GPIO = crate::Reg<ahblite_mpu_table_gpio::AHBLITE_MPU_TABLE_GPIO_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_gpio;
#[doc = "AHBLITE_MPU_TABLE_FE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_fe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_fe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_fe2`] module"]
pub type AHBLITE_MPU_TABLE_FE2 = crate::Reg<ahblite_mpu_table_fe2::AHBLITE_MPU_TABLE_FE2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_fe2;
#[doc = "AHBLITE_MPU_TABLE_FE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_fe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_fe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_fe`] module"]
pub type AHBLITE_MPU_TABLE_FE = crate::Reg<ahblite_mpu_table_fe::AHBLITE_MPU_TABLE_FE_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_fe;
#[doc = "AHBLITE_MPU_TABLE_TIMER (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_timer`] module"]
pub type AHBLITE_MPU_TABLE_TIMER =
    crate::Reg<ahblite_mpu_table_timer::AHBLITE_MPU_TABLE_TIMER_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timer;
#[doc = "AHBLITE_MPU_TABLE_RTC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_rtc`] module"]
pub type AHBLITE_MPU_TABLE_RTC = crate::Reg<ahblite_mpu_table_rtc::AHBLITE_MPU_TABLE_RTC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rtc;
#[doc = "AHBLITE_MPU_TABLE_IO_MUX (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_io_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_io_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_io_mux`] module"]
pub type AHBLITE_MPU_TABLE_IO_MUX =
    crate::Reg<ahblite_mpu_table_io_mux::AHBLITE_MPU_TABLE_IO_MUX_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_io_mux;
#[doc = "AHBLITE_MPU_TABLE_WDG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_wdg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_wdg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_wdg`] module"]
pub type AHBLITE_MPU_TABLE_WDG = crate::Reg<ahblite_mpu_table_wdg::AHBLITE_MPU_TABLE_WDG_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_wdg;
#[doc = "AHBLITE_MPU_TABLE_HINF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_hinf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_hinf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_hinf`] module"]
pub type AHBLITE_MPU_TABLE_HINF = crate::Reg<ahblite_mpu_table_hinf::AHBLITE_MPU_TABLE_HINF_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_hinf;
#[doc = "AHBLITE_MPU_TABLE_UHCI1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uhci1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uhci1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_uhci1`] module"]
pub type AHBLITE_MPU_TABLE_UHCI1 =
    crate::Reg<ahblite_mpu_table_uhci1::AHBLITE_MPU_TABLE_UHCI1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uhci1;
#[doc = "AHBLITE_MPU_TABLE_MISC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_misc`] module"]
pub type AHBLITE_MPU_TABLE_MISC = crate::Reg<ahblite_mpu_table_misc::AHBLITE_MPU_TABLE_MISC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_misc;
#[doc = "AHBLITE_MPU_TABLE_I2C (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_i2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_i2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_i2c`] module"]
pub type AHBLITE_MPU_TABLE_I2C = crate::Reg<ahblite_mpu_table_i2c::AHBLITE_MPU_TABLE_I2C_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c;
#[doc = "AHBLITE_MPU_TABLE_I2S0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_i2s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_i2s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_i2s0`] module"]
pub type AHBLITE_MPU_TABLE_I2S0 = crate::Reg<ahblite_mpu_table_i2s0::AHBLITE_MPU_TABLE_I2S0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2s0;
#[doc = "AHBLITE_MPU_TABLE_UART1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uart1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uart1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_uart1`] module"]
pub type AHBLITE_MPU_TABLE_UART1 =
    crate::Reg<ahblite_mpu_table_uart1::AHBLITE_MPU_TABLE_UART1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart1;
#[doc = "AHBLITE_MPU_TABLE_BT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_bt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_bt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_bt`] module"]
pub type AHBLITE_MPU_TABLE_BT = crate::Reg<ahblite_mpu_table_bt::AHBLITE_MPU_TABLE_BT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bt;
#[doc = "AHBLITE_MPU_TABLE_BT_BUFFER (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_bt_buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_bt_buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_bt_buffer`] module"]
pub type AHBLITE_MPU_TABLE_BT_BUFFER =
    crate::Reg<ahblite_mpu_table_bt_buffer::AHBLITE_MPU_TABLE_BT_BUFFER_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bt_buffer;
#[doc = "AHBLITE_MPU_TABLE_I2C_EXT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_i2c_ext0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_i2c_ext0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_i2c_ext0`] module"]
pub type AHBLITE_MPU_TABLE_I2C_EXT0 =
    crate::Reg<ahblite_mpu_table_i2c_ext0::AHBLITE_MPU_TABLE_I2C_EXT0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c_ext0;
#[doc = "AHBLITE_MPU_TABLE_UHCI0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uhci0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uhci0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_uhci0`] module"]
pub type AHBLITE_MPU_TABLE_UHCI0 =
    crate::Reg<ahblite_mpu_table_uhci0::AHBLITE_MPU_TABLE_UHCI0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uhci0;
#[doc = "AHBLITE_MPU_TABLE_SLCHOST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_slchost::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_slchost::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_slchost`] module"]
pub type AHBLITE_MPU_TABLE_SLCHOST =
    crate::Reg<ahblite_mpu_table_slchost::AHBLITE_MPU_TABLE_SLCHOST_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_slchost;
#[doc = "AHBLITE_MPU_TABLE_RMT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_rmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_rmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_rmt`] module"]
pub type AHBLITE_MPU_TABLE_RMT = crate::Reg<ahblite_mpu_table_rmt::AHBLITE_MPU_TABLE_RMT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rmt;
#[doc = "AHBLITE_MPU_TABLE_PCNT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pcnt`] module"]
pub type AHBLITE_MPU_TABLE_PCNT = crate::Reg<ahblite_mpu_table_pcnt::AHBLITE_MPU_TABLE_PCNT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pcnt;
#[doc = "AHBLITE_MPU_TABLE_SLC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_slc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_slc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_slc`] module"]
pub type AHBLITE_MPU_TABLE_SLC = crate::Reg<ahblite_mpu_table_slc::AHBLITE_MPU_TABLE_SLC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_slc;
#[doc = "AHBLITE_MPU_TABLE_LEDC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_ledc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_ledc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_ledc`] module"]
pub type AHBLITE_MPU_TABLE_LEDC = crate::Reg<ahblite_mpu_table_ledc::AHBLITE_MPU_TABLE_LEDC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_ledc;
#[doc = "AHBLITE_MPU_TABLE_EFUSE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_efuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_efuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_efuse`] module"]
pub type AHBLITE_MPU_TABLE_EFUSE =
    crate::Reg<ahblite_mpu_table_efuse::AHBLITE_MPU_TABLE_EFUSE_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_efuse;
#[doc = "AHBLITE_MPU_TABLE_SPI_ENCRYPT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_spi_encrypt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_spi_encrypt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_spi_encrypt`] module"]
pub type AHBLITE_MPU_TABLE_SPI_ENCRYPT =
    crate::Reg<ahblite_mpu_table_spi_encrypt::AHBLITE_MPU_TABLE_SPI_ENCRYPT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi_encrypt;
#[doc = "AHBLITE_MPU_TABLE_BB (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_bb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_bb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_bb`] module"]
pub type AHBLITE_MPU_TABLE_BB = crate::Reg<ahblite_mpu_table_bb::AHBLITE_MPU_TABLE_BB_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_bb;
#[doc = "AHBLITE_MPU_TABLE_PWM0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pwm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pwm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pwm0`] module"]
pub type AHBLITE_MPU_TABLE_PWM0 = crate::Reg<ahblite_mpu_table_pwm0::AHBLITE_MPU_TABLE_PWM0_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm0;
#[doc = "AHBLITE_MPU_TABLE_TIMERGROUP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_timergroup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_timergroup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_timergroup`] module"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP =
    crate::Reg<ahblite_mpu_table_timergroup::AHBLITE_MPU_TABLE_TIMERGROUP_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timergroup;
#[doc = "AHBLITE_MPU_TABLE_TIMERGROUP1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_timergroup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_timergroup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_timergroup1`] module"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP1 =
    crate::Reg<ahblite_mpu_table_timergroup1::AHBLITE_MPU_TABLE_TIMERGROUP1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_timergroup1;
#[doc = "AHBLITE_MPU_TABLE_SPI2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_spi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_spi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_spi2`] module"]
pub type AHBLITE_MPU_TABLE_SPI2 = crate::Reg<ahblite_mpu_table_spi2::AHBLITE_MPU_TABLE_SPI2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi2;
#[doc = "AHBLITE_MPU_TABLE_SPI3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_spi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_spi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_spi3`] module"]
pub type AHBLITE_MPU_TABLE_SPI3 = crate::Reg<ahblite_mpu_table_spi3::AHBLITE_MPU_TABLE_SPI3_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_spi3;
#[doc = "AHBLITE_MPU_TABLE_APB_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_apb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_apb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_apb_ctrl`] module"]
pub type AHBLITE_MPU_TABLE_APB_CTRL =
    crate::Reg<ahblite_mpu_table_apb_ctrl::AHBLITE_MPU_TABLE_APB_CTRL_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_apb_ctrl;
#[doc = "AHBLITE_MPU_TABLE_I2C_EXT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_i2c_ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_i2c_ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_i2c_ext1`] module"]
pub type AHBLITE_MPU_TABLE_I2C_EXT1 =
    crate::Reg<ahblite_mpu_table_i2c_ext1::AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2c_ext1;
#[doc = "AHBLITE_MPU_TABLE_SDIO_HOST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_sdio_host::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_sdio_host::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_sdio_host`] module"]
pub type AHBLITE_MPU_TABLE_SDIO_HOST =
    crate::Reg<ahblite_mpu_table_sdio_host::AHBLITE_MPU_TABLE_SDIO_HOST_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_sdio_host;
#[doc = "AHBLITE_MPU_TABLE_EMAC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_emac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_emac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_emac`] module"]
pub type AHBLITE_MPU_TABLE_EMAC = crate::Reg<ahblite_mpu_table_emac::AHBLITE_MPU_TABLE_EMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_emac;
#[doc = "AHBLITE_MPU_TABLE_CAN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_can::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_can::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_can`] module"]
pub type AHBLITE_MPU_TABLE_CAN = crate::Reg<ahblite_mpu_table_can::AHBLITE_MPU_TABLE_CAN_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_can;
#[doc = "AHBLITE_MPU_TABLE_PWM1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pwm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pwm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pwm1`] module"]
pub type AHBLITE_MPU_TABLE_PWM1 = crate::Reg<ahblite_mpu_table_pwm1::AHBLITE_MPU_TABLE_PWM1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm1;
#[doc = "AHBLITE_MPU_TABLE_I2S1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_i2s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_i2s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_i2s1`] module"]
pub type AHBLITE_MPU_TABLE_I2S1 = crate::Reg<ahblite_mpu_table_i2s1::AHBLITE_MPU_TABLE_I2S1_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_i2s1;
#[doc = "AHBLITE_MPU_TABLE_UART2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uart2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uart2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_uart2`] module"]
pub type AHBLITE_MPU_TABLE_UART2 =
    crate::Reg<ahblite_mpu_table_uart2::AHBLITE_MPU_TABLE_UART2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_uart2;
#[doc = "AHBLITE_MPU_TABLE_PWM2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pwm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pwm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pwm2`] module"]
pub type AHBLITE_MPU_TABLE_PWM2 = crate::Reg<ahblite_mpu_table_pwm2::AHBLITE_MPU_TABLE_PWM2_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm2;
#[doc = "AHBLITE_MPU_TABLE_PWM3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pwm3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pwm3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pwm3`] module"]
pub type AHBLITE_MPU_TABLE_PWM3 = crate::Reg<ahblite_mpu_table_pwm3::AHBLITE_MPU_TABLE_PWM3_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwm3;
#[doc = "AHBLITE_MPU_TABLE_RWBT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_rwbt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_rwbt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_rwbt`] module"]
pub type AHBLITE_MPU_TABLE_RWBT = crate::Reg<ahblite_mpu_table_rwbt::AHBLITE_MPU_TABLE_RWBT_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_rwbt;
#[doc = "AHBLITE_MPU_TABLE_BTMAC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_btmac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_btmac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_btmac`] module"]
pub type AHBLITE_MPU_TABLE_BTMAC =
    crate::Reg<ahblite_mpu_table_btmac::AHBLITE_MPU_TABLE_BTMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_btmac;
#[doc = "AHBLITE_MPU_TABLE_WIFIMAC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_wifimac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_wifimac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_wifimac`] module"]
pub type AHBLITE_MPU_TABLE_WIFIMAC =
    crate::Reg<ahblite_mpu_table_wifimac::AHBLITE_MPU_TABLE_WIFIMAC_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_wifimac;
#[doc = "AHBLITE_MPU_TABLE_PWR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblite_mpu_table_pwr`] module"]
pub type AHBLITE_MPU_TABLE_PWR = crate::Reg<ahblite_mpu_table_pwr::AHBLITE_MPU_TABLE_PWR_SPEC>;
#[doc = ""]
pub mod ahblite_mpu_table_pwr;
#[doc = "MEM_ACCESS_DBUG0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_access_dbug0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_access_dbug0`] module"]
pub type MEM_ACCESS_DBUG0 = crate::Reg<mem_access_dbug0::MEM_ACCESS_DBUG0_SPEC>;
#[doc = ""]
pub mod mem_access_dbug0;
#[doc = "MEM_ACCESS_DBUG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_access_dbug1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_access_dbug1`] module"]
pub type MEM_ACCESS_DBUG1 = crate::Reg<mem_access_dbug1::MEM_ACCESS_DBUG1_SPEC>;
#[doc = ""]
pub mod mem_access_dbug1;
#[doc = "PRO_DCACHE_DBUG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_dbug0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug0`] module"]
pub type PRO_DCACHE_DBUG0 = crate::Reg<pro_dcache_dbug0::PRO_DCACHE_DBUG0_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug0;
#[doc = "PRO_DCACHE_DBUG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug1`] module"]
pub type PRO_DCACHE_DBUG1 = crate::Reg<pro_dcache_dbug1::PRO_DCACHE_DBUG1_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug1;
#[doc = "PRO_DCACHE_DBUG2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug2`] module"]
pub type PRO_DCACHE_DBUG2 = crate::Reg<pro_dcache_dbug2::PRO_DCACHE_DBUG2_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug2;
#[doc = "PRO_DCACHE_DBUG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_dbug3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug3`] module"]
pub type PRO_DCACHE_DBUG3 = crate::Reg<pro_dcache_dbug3::PRO_DCACHE_DBUG3_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug3;
#[doc = "PRO_DCACHE_DBUG4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug4`] module"]
pub type PRO_DCACHE_DBUG4 = crate::Reg<pro_dcache_dbug4::PRO_DCACHE_DBUG4_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug4;
#[doc = "PRO_DCACHE_DBUG5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug5`] module"]
pub type PRO_DCACHE_DBUG5 = crate::Reg<pro_dcache_dbug5::PRO_DCACHE_DBUG5_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug5;
#[doc = "PRO_DCACHE_DBUG6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug6`] module"]
pub type PRO_DCACHE_DBUG6 = crate::Reg<pro_dcache_dbug6::PRO_DCACHE_DBUG6_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug6;
#[doc = "PRO_DCACHE_DBUG7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug7`] module"]
pub type PRO_DCACHE_DBUG7 = crate::Reg<pro_dcache_dbug7::PRO_DCACHE_DBUG7_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug7;
#[doc = "PRO_DCACHE_DBUG8 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug8`] module"]
pub type PRO_DCACHE_DBUG8 = crate::Reg<pro_dcache_dbug8::PRO_DCACHE_DBUG8_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug8;
#[doc = "PRO_DCACHE_DBUG9 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_dbug9`] module"]
pub type PRO_DCACHE_DBUG9 = crate::Reg<pro_dcache_dbug9::PRO_DCACHE_DBUG9_SPEC>;
#[doc = ""]
pub mod pro_dcache_dbug9;
#[doc = "APP_DCACHE_DBUG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_dcache_dbug0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug0`] module"]
pub type APP_DCACHE_DBUG0 = crate::Reg<app_dcache_dbug0::APP_DCACHE_DBUG0_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug0;
#[doc = "APP_DCACHE_DBUG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug1`] module"]
pub type APP_DCACHE_DBUG1 = crate::Reg<app_dcache_dbug1::APP_DCACHE_DBUG1_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug1;
#[doc = "APP_DCACHE_DBUG2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug2`] module"]
pub type APP_DCACHE_DBUG2 = crate::Reg<app_dcache_dbug2::APP_DCACHE_DBUG2_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug2;
#[doc = "APP_DCACHE_DBUG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_dcache_dbug3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug3`] module"]
pub type APP_DCACHE_DBUG3 = crate::Reg<app_dcache_dbug3::APP_DCACHE_DBUG3_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug3;
#[doc = "APP_DCACHE_DBUG4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug4`] module"]
pub type APP_DCACHE_DBUG4 = crate::Reg<app_dcache_dbug4::APP_DCACHE_DBUG4_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug4;
#[doc = "APP_DCACHE_DBUG5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug5`] module"]
pub type APP_DCACHE_DBUG5 = crate::Reg<app_dcache_dbug5::APP_DCACHE_DBUG5_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug5;
#[doc = "APP_DCACHE_DBUG6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug6`] module"]
pub type APP_DCACHE_DBUG6 = crate::Reg<app_dcache_dbug6::APP_DCACHE_DBUG6_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug6;
#[doc = "APP_DCACHE_DBUG7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug7`] module"]
pub type APP_DCACHE_DBUG7 = crate::Reg<app_dcache_dbug7::APP_DCACHE_DBUG7_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug7;
#[doc = "APP_DCACHE_DBUG8 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug8`] module"]
pub type APP_DCACHE_DBUG8 = crate::Reg<app_dcache_dbug8::APP_DCACHE_DBUG8_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug8;
#[doc = "APP_DCACHE_DBUG9 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_dcache_dbug9`] module"]
pub type APP_DCACHE_DBUG9 = crate::Reg<app_dcache_dbug9::APP_DCACHE_DBUG9_SPEC>;
#[doc = ""]
pub mod app_dcache_dbug9;
#[doc = "PRO_CPU_RECORD_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_ctrl`] module"]
pub type PRO_CPU_RECORD_CTRL = crate::Reg<pro_cpu_record_ctrl::PRO_CPU_RECORD_CTRL_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_ctrl;
#[doc = "PRO_CPU_RECORD_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_status`] module"]
pub type PRO_CPU_RECORD_STATUS = crate::Reg<pro_cpu_record_status::PRO_CPU_RECORD_STATUS_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_status;
#[doc = "PRO_CPU_RECORD_PID (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pid`] module"]
pub type PRO_CPU_RECORD_PID = crate::Reg<pro_cpu_record_pid::PRO_CPU_RECORD_PID_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pid;
#[doc = "PRO_CPU_RECORD_PDEBUGINST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebuginst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_pdebuginst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebuginst`] module"]
pub type PRO_CPU_RECORD_PDEBUGINST =
    crate::Reg<pro_cpu_record_pdebuginst::PRO_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebuginst;
#[doc = "PRO_CPU_RECORD_PDEBUGSTATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_pdebugstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugstatus`] module"]
pub type PRO_CPU_RECORD_PDEBUGSTATUS =
    crate::Reg<pro_cpu_record_pdebugstatus::PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugstatus;
#[doc = "PRO_CPU_RECORD_PDEBUGDATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_pdebugdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugdata`] module"]
pub type PRO_CPU_RECORD_PDEBUGDATA =
    crate::Reg<pro_cpu_record_pdebugdata::PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugdata;
#[doc = "PRO_CPU_RECORD_PDEBUGPC (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugpc`] module"]
pub type PRO_CPU_RECORD_PDEBUGPC =
    crate::Reg<pro_cpu_record_pdebugpc::PRO_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugpc;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0STAT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugls0stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_pdebugls0stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugls0stat`] module"]
pub type PRO_CPU_RECORD_PDEBUGLS0STAT =
    crate::Reg<pro_cpu_record_pdebugls0stat::PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0stat;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugls0addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugls0addr`] module"]
pub type PRO_CPU_RECORD_PDEBUGLS0ADDR =
    crate::Reg<pro_cpu_record_pdebugls0addr::PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0addr;
#[doc = "PRO_CPU_RECORD_PDEBUGLS0DATA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugls0data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cpu_record_pdebugls0data`] module"]
pub type PRO_CPU_RECORD_PDEBUGLS0DATA =
    crate::Reg<pro_cpu_record_pdebugls0data::PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
#[doc = ""]
pub mod pro_cpu_record_pdebugls0data;
#[doc = "APP_CPU_RECORD_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cpu_record_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_ctrl`] module"]
pub type APP_CPU_RECORD_CTRL = crate::Reg<app_cpu_record_ctrl::APP_CPU_RECORD_CTRL_SPEC>;
#[doc = ""]
pub mod app_cpu_record_ctrl;
#[doc = "APP_CPU_RECORD_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_status`] module"]
pub type APP_CPU_RECORD_STATUS = crate::Reg<app_cpu_record_status::APP_CPU_RECORD_STATUS_SPEC>;
#[doc = ""]
pub mod app_cpu_record_status;
#[doc = "APP_CPU_RECORD_PID (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pid`] module"]
pub type APP_CPU_RECORD_PID = crate::Reg<app_cpu_record_pid::APP_CPU_RECORD_PID_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pid;
#[doc = "APP_CPU_RECORD_PDEBUGINST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebuginst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebuginst`] module"]
pub type APP_CPU_RECORD_PDEBUGINST =
    crate::Reg<app_cpu_record_pdebuginst::APP_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebuginst;
#[doc = "APP_CPU_RECORD_PDEBUGSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugstatus`] module"]
pub type APP_CPU_RECORD_PDEBUGSTATUS =
    crate::Reg<app_cpu_record_pdebugstatus::APP_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugstatus;
#[doc = "APP_CPU_RECORD_PDEBUGDATA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugdata`] module"]
pub type APP_CPU_RECORD_PDEBUGDATA =
    crate::Reg<app_cpu_record_pdebugdata::APP_CPU_RECORD_PDEBUGDATA_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugdata;
#[doc = "APP_CPU_RECORD_PDEBUGPC (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugpc`] module"]
pub type APP_CPU_RECORD_PDEBUGPC =
    crate::Reg<app_cpu_record_pdebugpc::APP_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugpc;
#[doc = "APP_CPU_RECORD_PDEBUGLS0STAT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugls0stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugls0stat`] module"]
pub type APP_CPU_RECORD_PDEBUGLS0STAT =
    crate::Reg<app_cpu_record_pdebugls0stat::APP_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0stat;
#[doc = "APP_CPU_RECORD_PDEBUGLS0ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugls0addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugls0addr`] module"]
pub type APP_CPU_RECORD_PDEBUGLS0ADDR =
    crate::Reg<app_cpu_record_pdebugls0addr::APP_CPU_RECORD_PDEBUGLS0ADDR_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0addr;
#[doc = "APP_CPU_RECORD_PDEBUGLS0DATA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugls0data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_record_pdebugls0data`] module"]
pub type APP_CPU_RECORD_PDEBUGLS0DATA =
    crate::Reg<app_cpu_record_pdebugls0data::APP_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
#[doc = ""]
pub mod app_cpu_record_pdebugls0data;
#[doc = "RSA_PD_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_pd_ctrl`] module"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = ""]
pub mod rsa_pd_ctrl;
#[doc = "ROM_MPU_TABLE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mpu_table0`] module"]
pub type ROM_MPU_TABLE0 = crate::Reg<rom_mpu_table0::ROM_MPU_TABLE0_SPEC>;
#[doc = ""]
pub mod rom_mpu_table0;
#[doc = "ROM_MPU_TABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mpu_table1`] module"]
pub type ROM_MPU_TABLE1 = crate::Reg<rom_mpu_table1::ROM_MPU_TABLE1_SPEC>;
#[doc = ""]
pub mod rom_mpu_table1;
#[doc = "ROM_MPU_TABLE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mpu_table2`] module"]
pub type ROM_MPU_TABLE2 = crate::Reg<rom_mpu_table2::ROM_MPU_TABLE2_SPEC>;
#[doc = ""]
pub mod rom_mpu_table2;
#[doc = "ROM_MPU_TABLE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mpu_table3`] module"]
pub type ROM_MPU_TABLE3 = crate::Reg<rom_mpu_table3::ROM_MPU_TABLE3_SPEC>;
#[doc = ""]
pub mod rom_mpu_table3;
#[doc = "SHROM_MPU_TABLE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table0`] module"]
pub type SHROM_MPU_TABLE0 = crate::Reg<shrom_mpu_table0::SHROM_MPU_TABLE0_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table0;
#[doc = "SHROM_MPU_TABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table1`] module"]
pub type SHROM_MPU_TABLE1 = crate::Reg<shrom_mpu_table1::SHROM_MPU_TABLE1_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table1;
#[doc = "SHROM_MPU_TABLE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table2`] module"]
pub type SHROM_MPU_TABLE2 = crate::Reg<shrom_mpu_table2::SHROM_MPU_TABLE2_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table2;
#[doc = "SHROM_MPU_TABLE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table3`] module"]
pub type SHROM_MPU_TABLE3 = crate::Reg<shrom_mpu_table3::SHROM_MPU_TABLE3_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table3;
#[doc = "SHROM_MPU_TABLE4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table4`] module"]
pub type SHROM_MPU_TABLE4 = crate::Reg<shrom_mpu_table4::SHROM_MPU_TABLE4_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table4;
#[doc = "SHROM_MPU_TABLE5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table5`] module"]
pub type SHROM_MPU_TABLE5 = crate::Reg<shrom_mpu_table5::SHROM_MPU_TABLE5_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table5;
#[doc = "SHROM_MPU_TABLE6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table6`] module"]
pub type SHROM_MPU_TABLE6 = crate::Reg<shrom_mpu_table6::SHROM_MPU_TABLE6_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table6;
#[doc = "SHROM_MPU_TABLE7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table7`] module"]
pub type SHROM_MPU_TABLE7 = crate::Reg<shrom_mpu_table7::SHROM_MPU_TABLE7_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table7;
#[doc = "SHROM_MPU_TABLE8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table8`] module"]
pub type SHROM_MPU_TABLE8 = crate::Reg<shrom_mpu_table8::SHROM_MPU_TABLE8_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table8;
#[doc = "SHROM_MPU_TABLE9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table9`] module"]
pub type SHROM_MPU_TABLE9 = crate::Reg<shrom_mpu_table9::SHROM_MPU_TABLE9_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table9;
#[doc = "SHROM_MPU_TABLE10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table10`] module"]
pub type SHROM_MPU_TABLE10 = crate::Reg<shrom_mpu_table10::SHROM_MPU_TABLE10_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table10;
#[doc = "SHROM_MPU_TABLE11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table11`] module"]
pub type SHROM_MPU_TABLE11 = crate::Reg<shrom_mpu_table11::SHROM_MPU_TABLE11_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table11;
#[doc = "SHROM_MPU_TABLE12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table12`] module"]
pub type SHROM_MPU_TABLE12 = crate::Reg<shrom_mpu_table12::SHROM_MPU_TABLE12_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table12;
#[doc = "SHROM_MPU_TABLE13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table13`] module"]
pub type SHROM_MPU_TABLE13 = crate::Reg<shrom_mpu_table13::SHROM_MPU_TABLE13_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table13;
#[doc = "SHROM_MPU_TABLE14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table14`] module"]
pub type SHROM_MPU_TABLE14 = crate::Reg<shrom_mpu_table14::SHROM_MPU_TABLE14_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table14;
#[doc = "SHROM_MPU_TABLE15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table15`] module"]
pub type SHROM_MPU_TABLE15 = crate::Reg<shrom_mpu_table15::SHROM_MPU_TABLE15_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table15;
#[doc = "SHROM_MPU_TABLE16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table16`] module"]
pub type SHROM_MPU_TABLE16 = crate::Reg<shrom_mpu_table16::SHROM_MPU_TABLE16_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table16;
#[doc = "SHROM_MPU_TABLE17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table17`] module"]
pub type SHROM_MPU_TABLE17 = crate::Reg<shrom_mpu_table17::SHROM_MPU_TABLE17_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table17;
#[doc = "SHROM_MPU_TABLE18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table18`] module"]
pub type SHROM_MPU_TABLE18 = crate::Reg<shrom_mpu_table18::SHROM_MPU_TABLE18_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table18;
#[doc = "SHROM_MPU_TABLE19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table19`] module"]
pub type SHROM_MPU_TABLE19 = crate::Reg<shrom_mpu_table19::SHROM_MPU_TABLE19_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table19;
#[doc = "SHROM_MPU_TABLE20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table20`] module"]
pub type SHROM_MPU_TABLE20 = crate::Reg<shrom_mpu_table20::SHROM_MPU_TABLE20_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table20;
#[doc = "SHROM_MPU_TABLE21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table21`] module"]
pub type SHROM_MPU_TABLE21 = crate::Reg<shrom_mpu_table21::SHROM_MPU_TABLE21_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table21;
#[doc = "SHROM_MPU_TABLE22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table22`] module"]
pub type SHROM_MPU_TABLE22 = crate::Reg<shrom_mpu_table22::SHROM_MPU_TABLE22_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table22;
#[doc = "SHROM_MPU_TABLE23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrom_mpu_table23`] module"]
pub type SHROM_MPU_TABLE23 = crate::Reg<shrom_mpu_table23::SHROM_MPU_TABLE23_SPEC>;
#[doc = ""]
pub mod shrom_mpu_table23;
#[doc = "IMMU_TABLE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table0`] module"]
pub type IMMU_TABLE0 = crate::Reg<immu_table0::IMMU_TABLE0_SPEC>;
#[doc = ""]
pub mod immu_table0;
#[doc = "IMMU_TABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table1`] module"]
pub type IMMU_TABLE1 = crate::Reg<immu_table1::IMMU_TABLE1_SPEC>;
#[doc = ""]
pub mod immu_table1;
#[doc = "IMMU_TABLE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table2`] module"]
pub type IMMU_TABLE2 = crate::Reg<immu_table2::IMMU_TABLE2_SPEC>;
#[doc = ""]
pub mod immu_table2;
#[doc = "IMMU_TABLE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table3`] module"]
pub type IMMU_TABLE3 = crate::Reg<immu_table3::IMMU_TABLE3_SPEC>;
#[doc = ""]
pub mod immu_table3;
#[doc = "IMMU_TABLE4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table4`] module"]
pub type IMMU_TABLE4 = crate::Reg<immu_table4::IMMU_TABLE4_SPEC>;
#[doc = ""]
pub mod immu_table4;
#[doc = "IMMU_TABLE5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table5`] module"]
pub type IMMU_TABLE5 = crate::Reg<immu_table5::IMMU_TABLE5_SPEC>;
#[doc = ""]
pub mod immu_table5;
#[doc = "IMMU_TABLE6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table6`] module"]
pub type IMMU_TABLE6 = crate::Reg<immu_table6::IMMU_TABLE6_SPEC>;
#[doc = ""]
pub mod immu_table6;
#[doc = "IMMU_TABLE7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table7`] module"]
pub type IMMU_TABLE7 = crate::Reg<immu_table7::IMMU_TABLE7_SPEC>;
#[doc = ""]
pub mod immu_table7;
#[doc = "IMMU_TABLE8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table8`] module"]
pub type IMMU_TABLE8 = crate::Reg<immu_table8::IMMU_TABLE8_SPEC>;
#[doc = ""]
pub mod immu_table8;
#[doc = "IMMU_TABLE9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table9`] module"]
pub type IMMU_TABLE9 = crate::Reg<immu_table9::IMMU_TABLE9_SPEC>;
#[doc = ""]
pub mod immu_table9;
#[doc = "IMMU_TABLE10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table10`] module"]
pub type IMMU_TABLE10 = crate::Reg<immu_table10::IMMU_TABLE10_SPEC>;
#[doc = ""]
pub mod immu_table10;
#[doc = "IMMU_TABLE11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table11`] module"]
pub type IMMU_TABLE11 = crate::Reg<immu_table11::IMMU_TABLE11_SPEC>;
#[doc = ""]
pub mod immu_table11;
#[doc = "IMMU_TABLE12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table12`] module"]
pub type IMMU_TABLE12 = crate::Reg<immu_table12::IMMU_TABLE12_SPEC>;
#[doc = ""]
pub mod immu_table12;
#[doc = "IMMU_TABLE13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table13`] module"]
pub type IMMU_TABLE13 = crate::Reg<immu_table13::IMMU_TABLE13_SPEC>;
#[doc = ""]
pub mod immu_table13;
#[doc = "IMMU_TABLE14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table14`] module"]
pub type IMMU_TABLE14 = crate::Reg<immu_table14::IMMU_TABLE14_SPEC>;
#[doc = ""]
pub mod immu_table14;
#[doc = "IMMU_TABLE15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@immu_table15`] module"]
pub type IMMU_TABLE15 = crate::Reg<immu_table15::IMMU_TABLE15_SPEC>;
#[doc = ""]
pub mod immu_table15;
#[doc = "DMMU_TABLE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table0`] module"]
pub type DMMU_TABLE0 = crate::Reg<dmmu_table0::DMMU_TABLE0_SPEC>;
#[doc = ""]
pub mod dmmu_table0;
#[doc = "DMMU_TABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table1`] module"]
pub type DMMU_TABLE1 = crate::Reg<dmmu_table1::DMMU_TABLE1_SPEC>;
#[doc = ""]
pub mod dmmu_table1;
#[doc = "DMMU_TABLE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table2`] module"]
pub type DMMU_TABLE2 = crate::Reg<dmmu_table2::DMMU_TABLE2_SPEC>;
#[doc = ""]
pub mod dmmu_table2;
#[doc = "DMMU_TABLE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table3`] module"]
pub type DMMU_TABLE3 = crate::Reg<dmmu_table3::DMMU_TABLE3_SPEC>;
#[doc = ""]
pub mod dmmu_table3;
#[doc = "DMMU_TABLE4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table4`] module"]
pub type DMMU_TABLE4 = crate::Reg<dmmu_table4::DMMU_TABLE4_SPEC>;
#[doc = ""]
pub mod dmmu_table4;
#[doc = "DMMU_TABLE5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table5`] module"]
pub type DMMU_TABLE5 = crate::Reg<dmmu_table5::DMMU_TABLE5_SPEC>;
#[doc = ""]
pub mod dmmu_table5;
#[doc = "DMMU_TABLE6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table6`] module"]
pub type DMMU_TABLE6 = crate::Reg<dmmu_table6::DMMU_TABLE6_SPEC>;
#[doc = ""]
pub mod dmmu_table6;
#[doc = "DMMU_TABLE7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table7`] module"]
pub type DMMU_TABLE7 = crate::Reg<dmmu_table7::DMMU_TABLE7_SPEC>;
#[doc = ""]
pub mod dmmu_table7;
#[doc = "DMMU_TABLE8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table8`] module"]
pub type DMMU_TABLE8 = crate::Reg<dmmu_table8::DMMU_TABLE8_SPEC>;
#[doc = ""]
pub mod dmmu_table8;
#[doc = "DMMU_TABLE9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table9`] module"]
pub type DMMU_TABLE9 = crate::Reg<dmmu_table9::DMMU_TABLE9_SPEC>;
#[doc = ""]
pub mod dmmu_table9;
#[doc = "DMMU_TABLE10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table10`] module"]
pub type DMMU_TABLE10 = crate::Reg<dmmu_table10::DMMU_TABLE10_SPEC>;
#[doc = ""]
pub mod dmmu_table10;
#[doc = "DMMU_TABLE11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table11`] module"]
pub type DMMU_TABLE11 = crate::Reg<dmmu_table11::DMMU_TABLE11_SPEC>;
#[doc = ""]
pub mod dmmu_table11;
#[doc = "DMMU_TABLE12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table12`] module"]
pub type DMMU_TABLE12 = crate::Reg<dmmu_table12::DMMU_TABLE12_SPEC>;
#[doc = ""]
pub mod dmmu_table12;
#[doc = "DMMU_TABLE13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table13`] module"]
pub type DMMU_TABLE13 = crate::Reg<dmmu_table13::DMMU_TABLE13_SPEC>;
#[doc = ""]
pub mod dmmu_table13;
#[doc = "DMMU_TABLE14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table14`] module"]
pub type DMMU_TABLE14 = crate::Reg<dmmu_table14::DMMU_TABLE14_SPEC>;
#[doc = ""]
pub mod dmmu_table14;
#[doc = "DMMU_TABLE15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmu_table15`] module"]
pub type DMMU_TABLE15 = crate::Reg<dmmu_table15::DMMU_TABLE15_SPEC>;
#[doc = ""]
pub mod dmmu_table15;
#[doc = "PRO_INTRUSION_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intrusion_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_intrusion_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_intrusion_ctrl`] module"]
pub type PRO_INTRUSION_CTRL = crate::Reg<pro_intrusion_ctrl::PRO_INTRUSION_CTRL_SPEC>;
#[doc = ""]
pub mod pro_intrusion_ctrl;
#[doc = "PRO_INTRUSION_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intrusion_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_intrusion_status`] module"]
pub type PRO_INTRUSION_STATUS = crate::Reg<pro_intrusion_status::PRO_INTRUSION_STATUS_SPEC>;
#[doc = ""]
pub mod pro_intrusion_status;
#[doc = "APP_INTRUSION_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intrusion_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_intrusion_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_intrusion_ctrl`] module"]
pub type APP_INTRUSION_CTRL = crate::Reg<app_intrusion_ctrl::APP_INTRUSION_CTRL_SPEC>;
#[doc = ""]
pub mod app_intrusion_ctrl;
#[doc = "APP_INTRUSION_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intrusion_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_intrusion_status`] module"]
pub type APP_INTRUSION_STATUS = crate::Reg<app_intrusion_status::APP_INTRUSION_STATUS_SPEC>;
#[doc = ""]
pub mod app_intrusion_status;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`front_end_mem_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`front_end_mem_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@front_end_mem_pd`] module"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = ""]
pub mod front_end_mem_pd;
#[doc = "MMU_IA_INT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_ia_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_ia_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_ia_int_en`] module"]
pub type MMU_IA_INT_EN = crate::Reg<mmu_ia_int_en::MMU_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod mmu_ia_int_en;
#[doc = "MPU_IA_INT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ia_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ia_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_ia_int_en`] module"]
pub type MPU_IA_INT_EN = crate::Reg<mpu_ia_int_en::MPU_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod mpu_ia_int_en;
#[doc = "CACHE_IA_INT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ia_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ia_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ia_int_en`] module"]
pub type CACHE_IA_INT_EN = crate::Reg<cache_ia_int_en::CACHE_IA_INT_EN_SPEC>;
#[doc = ""]
pub mod cache_ia_int_en;
#[doc = "SECURE_BOOT_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_boot_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_boot_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_boot_ctrl`] module"]
pub type SECURE_BOOT_CTRL = crate::Reg<secure_boot_ctrl::SECURE_BOOT_CTRL_SPEC>;
#[doc = ""]
pub mod secure_boot_ctrl;
#[doc = "SPI_DMA_CHAN_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_chan_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_chan_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_chan_sel`] module"]
pub type SPI_DMA_CHAN_SEL = crate::Reg<spi_dma_chan_sel::SPI_DMA_CHAN_SEL_SPEC>;
#[doc = ""]
pub mod spi_dma_chan_sel;
#[doc = "PRO_VECBASE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_vecbase_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_vecbase_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_vecbase_ctrl`] module"]
pub type PRO_VECBASE_CTRL = crate::Reg<pro_vecbase_ctrl::PRO_VECBASE_CTRL_SPEC>;
#[doc = ""]
pub mod pro_vecbase_ctrl;
#[doc = "PRO_VECBASE_SET (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_vecbase_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_vecbase_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_vecbase_set`] module"]
pub type PRO_VECBASE_SET = crate::Reg<pro_vecbase_set::PRO_VECBASE_SET_SPEC>;
#[doc = ""]
pub mod pro_vecbase_set;
#[doc = "APP_VECBASE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_vecbase_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_vecbase_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_vecbase_ctrl`] module"]
pub type APP_VECBASE_CTRL = crate::Reg<app_vecbase_ctrl::APP_VECBASE_CTRL_SPEC>;
#[doc = ""]
pub mod app_vecbase_ctrl;
#[doc = "APP_VECBASE_SET (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_vecbase_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_vecbase_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_vecbase_set`] module"]
pub type APP_VECBASE_SET = crate::Reg<app_vecbase_set::APP_VECBASE_SET_SPEC>;
#[doc = ""]
pub mod app_vecbase_set;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
