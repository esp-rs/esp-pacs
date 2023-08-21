#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - register description"]
    pub pmu_intr_map: PMU_INTR_MAP,
    #[doc = "0x04 - register description"]
    pub efuse_intr_map: EFUSE_INTR_MAP,
    #[doc = "0x08 - register description"]
    pub lp_rtc_timer_intr_map: LP_RTC_TIMER_INTR_MAP,
    #[doc = "0x0c - register description"]
    pub lp_ble_timer_intr_map: LP_BLE_TIMER_INTR_MAP,
    #[doc = "0x10 - register description"]
    pub lp_wdt_intr_map: LP_WDT_INTR_MAP,
    #[doc = "0x14 - register description"]
    pub lp_peri_timeout_intr_map: LP_PERI_TIMEOUT_INTR_MAP,
    #[doc = "0x18 - register description"]
    pub lp_apm_m0_intr_map: LP_APM_M0_INTR_MAP,
    #[doc = "0x1c - register description"]
    pub cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x20 - register description"]
    pub cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x24 - register description"]
    pub cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x28 - register description"]
    pub cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x2c - register description"]
    pub assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    #[doc = "0x30 - register description"]
    pub trace_intr_map: TRACE_INTR_MAP,
    #[doc = "0x34 - register description"]
    pub cache_intr_map: CACHE_INTR_MAP,
    #[doc = "0x38 - register description"]
    pub cpu_peri_timeout_intr_map: CPU_PERI_TIMEOUT_INTR_MAP,
    #[doc = "0x3c - register description"]
    pub bt_mac_intr_map: BT_MAC_INTR_MAP,
    #[doc = "0x40 - register description"]
    pub bt_bb_intr_map: BT_BB_INTR_MAP,
    #[doc = "0x44 - register description"]
    pub bt_bb_nmi_map: BT_BB_NMI_MAP,
    #[doc = "0x48 - register description"]
    pub coex_intr_map: COEX_INTR_MAP,
    #[doc = "0x4c - register description"]
    pub ble_timer_intr_map: BLE_TIMER_INTR_MAP,
    #[doc = "0x50 - register description"]
    pub ble_sec_intr_map: BLE_SEC_INTR_MAP,
    #[doc = "0x54 - register description"]
    pub zb_mac_intr_map: ZB_MAC_INTR_MAP,
    #[doc = "0x58 - register description"]
    pub gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x5c - register description"]
    pub gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x60 - register description"]
    pub pau_intr_map: PAU_INTR_MAP,
    #[doc = "0x64 - register description"]
    pub hp_peri_timeout_intr_map: HP_PERI_TIMEOUT_INTR_MAP,
    #[doc = "0x68 - register description"]
    pub hp_apm_m0_intr_map: HP_APM_M0_INTR_MAP,
    #[doc = "0x6c - register description"]
    pub hp_apm_m1_intr_map: HP_APM_M1_INTR_MAP,
    #[doc = "0x70 - register description"]
    pub hp_apm_m2_intr_map: HP_APM_M2_INTR_MAP,
    #[doc = "0x74 - register description"]
    pub hp_apm_m3_intr_map: HP_APM_M3_INTR_MAP,
    #[doc = "0x78 - register description"]
    pub mspi_intr_map: MSPI_INTR_MAP,
    #[doc = "0x7c - register description"]
    pub i2s1_intr_map: I2S1_INTR_MAP,
    #[doc = "0x80 - register description"]
    pub uhci0_intr_map: UHCI0_INTR_MAP,
    #[doc = "0x84 - register description"]
    pub uart0_intr_map: UART0_INTR_MAP,
    #[doc = "0x88 - register description"]
    pub uart1_intr_map: UART1_INTR_MAP,
    #[doc = "0x8c - register description"]
    pub ledc_intr_map: LEDC_INTR_MAP,
    #[doc = "0x90 - register description"]
    pub can0_intr_map: CAN0_INTR_MAP,
    #[doc = "0x94 - register description"]
    pub usb_intr_map: USB_INTR_MAP,
    #[doc = "0x98 - register description"]
    pub rmt_intr_map: RMT_INTR_MAP,
    #[doc = "0x9c - register description"]
    pub i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    #[doc = "0xa0 - register description"]
    pub i2c_ext1_intr_map: I2C_EXT1_INTR_MAP,
    #[doc = "0xa4 - register description"]
    pub tg0_t0_intr_map: TG0_T0_INTR_MAP,
    #[doc = "0xa8 - register description"]
    pub tg0_wdt_intr_map: TG0_WDT_INTR_MAP,
    #[doc = "0xac - register description"]
    pub tg1_t0_intr_map: TG1_T0_INTR_MAP,
    #[doc = "0xb0 - register description"]
    pub tg1_wdt_intr_map: TG1_WDT_INTR_MAP,
    #[doc = "0xb4 - register description"]
    pub systimer_target0_intr_map: SYSTIMER_TARGET0_INTR_MAP,
    #[doc = "0xb8 - register description"]
    pub systimer_target1_intr_map: SYSTIMER_TARGET1_INTR_MAP,
    #[doc = "0xbc - register description"]
    pub systimer_target2_intr_map: SYSTIMER_TARGET2_INTR_MAP,
    #[doc = "0xc0 - register description"]
    pub apb_adc_intr_map: APB_ADC_INTR_MAP,
    #[doc = "0xc4 - register description"]
    pub pwm_intr_map: PWM_INTR_MAP,
    #[doc = "0xc8 - register description"]
    pub pcnt_intr_map: PCNT_INTR_MAP,
    #[doc = "0xcc - register description"]
    pub parl_io_tx_intr_map: PARL_IO_TX_INTR_MAP,
    #[doc = "0xd0 - register description"]
    pub parl_io_rx_intr_map: PARL_IO_RX_INTR_MAP,
    #[doc = "0xd4 - register description"]
    pub dma_in_ch0_intr_map: DMA_IN_CH0_INTR_MAP,
    #[doc = "0xd8 - register description"]
    pub dma_in_ch1_intr_map: DMA_IN_CH1_INTR_MAP,
    #[doc = "0xdc - register description"]
    pub dma_in_ch2_intr_map: DMA_IN_CH2_INTR_MAP,
    #[doc = "0xe0 - register description"]
    pub dma_out_ch0_intr_map: DMA_OUT_CH0_INTR_MAP,
    #[doc = "0xe4 - register description"]
    pub dma_out_ch1_intr_map: DMA_OUT_CH1_INTR_MAP,
    #[doc = "0xe8 - register description"]
    pub dma_out_ch2_intr_map: DMA_OUT_CH2_INTR_MAP,
    #[doc = "0xec - register description"]
    pub gpspi2_intr_map: GPSPI2_INTR_MAP,
    #[doc = "0xf0 - register description"]
    pub aes_intr_map: AES_INTR_MAP,
    #[doc = "0xf4 - register description"]
    pub sha_intr_map: SHA_INTR_MAP,
    #[doc = "0xf8 - register description"]
    pub rsa_intr_map: RSA_INTR_MAP,
    #[doc = "0xfc - register description"]
    pub ecc_intr_map: ECC_INTR_MAP,
    #[doc = "0x100 - register description"]
    pub ecdsa_intr_map: ECDSA_INTR_MAP,
    #[doc = "0x104 - register description"]
    pub intr_status_reg_0: INTR_STATUS_REG_0,
    #[doc = "0x108 - register description"]
    pub intr_status_reg_1: INTR_STATUS_REG_1,
    #[doc = "0x10c - register description"]
    pub int_status_reg_2: INT_STATUS_REG_2,
    #[doc = "0x110 - register description"]
    pub clock_gate: CLOCK_GATE,
    _reserved69: [u8; 0x06e8],
    #[doc = "0x7fc - register description"]
    pub interrupt_reg_date: INTERRUPT_REG_DATE,
}
#[doc = "PMU_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmu_intr_map`] module"]
pub type PMU_INTR_MAP = crate::Reg<pmu_intr_map::PMU_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pmu_intr_map;
#[doc = "EFUSE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`efuse_intr_map`] module"]
pub type EFUSE_INTR_MAP = crate::Reg<efuse_intr_map::EFUSE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod efuse_intr_map;
#[doc = "LP_RTC_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rtc_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rtc_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_rtc_timer_intr_map`] module"]
pub type LP_RTC_TIMER_INTR_MAP = crate::Reg<lp_rtc_timer_intr_map::LP_RTC_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_rtc_timer_intr_map;
#[doc = "LP_BLE_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ble_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ble_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_ble_timer_intr_map`] module"]
pub type LP_BLE_TIMER_INTR_MAP = crate::Reg<lp_ble_timer_intr_map::LP_BLE_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_ble_timer_intr_map;
#[doc = "LP_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_wdt_intr_map`] module"]
pub type LP_WDT_INTR_MAP = crate::Reg<lp_wdt_intr_map::LP_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_wdt_intr_map;
#[doc = "LP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_peri_timeout_intr_map`] module"]
pub type LP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<lp_peri_timeout_intr_map::LP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_peri_timeout_intr_map;
#[doc = "LP_APM_M0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_apm_m0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_apm_m0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_apm_m0_intr_map`] module"]
pub type LP_APM_M0_INTR_MAP = crate::Reg<lp_apm_m0_intr_map::LP_APM_M0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_apm_m0_intr_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`assist_debug_intr_map`] module"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod assist_debug_intr_map;
#[doc = "TRACE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`trace_intr_map`] module"]
pub type TRACE_INTR_MAP = crate::Reg<trace_intr_map::TRACE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod trace_intr_map;
#[doc = "CACHE_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_intr_map`] module"]
pub type CACHE_INTR_MAP = crate::Reg<cache_intr_map::CACHE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod cache_intr_map;
#[doc = "CPU_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_peri_timeout_intr_map`] module"]
pub type CPU_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<cpu_peri_timeout_intr_map::CPU_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_peri_timeout_intr_map;
#[doc = "BT_MAC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_mac_intr_map`] module"]
pub type BT_MAC_INTR_MAP = crate::Reg<bt_mac_intr_map::BT_MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_mac_intr_map;
#[doc = "BT_BB_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_bb_intr_map`] module"]
pub type BT_BB_INTR_MAP = crate::Reg<bt_bb_intr_map::BT_BB_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_intr_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_bb_nmi_map`] module"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_nmi_map;
#[doc = "COEX_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`coex_intr_map`] module"]
pub type COEX_INTR_MAP = crate::Reg<coex_intr_map::COEX_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod coex_intr_map;
#[doc = "BLE_TIMER_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ble_timer_intr_map`] module"]
pub type BLE_TIMER_INTR_MAP = crate::Reg<ble_timer_intr_map::BLE_TIMER_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_timer_intr_map;
#[doc = "BLE_SEC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_sec_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_sec_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ble_sec_intr_map`] module"]
pub type BLE_SEC_INTR_MAP = crate::Reg<ble_sec_intr_map::BLE_SEC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_sec_intr_map;
#[doc = "ZB_MAC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zb_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zb_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`zb_mac_intr_map`] module"]
pub type ZB_MAC_INTR_MAP = crate::Reg<zb_mac_intr_map::ZB_MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod zb_mac_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_pro_map`] module"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_pro_nmi_map`] module"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "PAU_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pau_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pau_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pau_intr_map`] module"]
pub type PAU_INTR_MAP = crate::Reg<pau_intr_map::PAU_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pau_intr_map;
#[doc = "HP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hp_peri_timeout_intr_map`] module"]
pub type HP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<hp_peri_timeout_intr_map::HP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_peri_timeout_intr_map;
#[doc = "HP_APM_M0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hp_apm_m0_intr_map`] module"]
pub type HP_APM_M0_INTR_MAP = crate::Reg<hp_apm_m0_intr_map::HP_APM_M0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m0_intr_map;
#[doc = "HP_APM_M1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hp_apm_m1_intr_map`] module"]
pub type HP_APM_M1_INTR_MAP = crate::Reg<hp_apm_m1_intr_map::HP_APM_M1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m1_intr_map;
#[doc = "HP_APM_M2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hp_apm_m2_intr_map`] module"]
pub type HP_APM_M2_INTR_MAP = crate::Reg<hp_apm_m2_intr_map::HP_APM_M2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m2_intr_map;
#[doc = "HP_APM_M3_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hp_apm_m3_intr_map`] module"]
pub type HP_APM_M3_INTR_MAP = crate::Reg<hp_apm_m3_intr_map::HP_APM_M3_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod hp_apm_m3_intr_map;
#[doc = "MSPI_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspi_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mspi_intr_map`] module"]
pub type MSPI_INTR_MAP = crate::Reg<mspi_intr_map::MSPI_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod mspi_intr_map;
#[doc = "I2S1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2s1_intr_map`] module"]
pub type I2S1_INTR_MAP = crate::Reg<i2s1_intr_map::I2S1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2s1_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uhci0_intr_map`] module"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uhci0_intr_map;
#[doc = "UART0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart0_intr_map`] module"]
pub type UART0_INTR_MAP = crate::Reg<uart0_intr_map::UART0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart0_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart1_intr_map;
#[doc = "LEDC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ledc_intr_map`] module"]
pub type LEDC_INTR_MAP = crate::Reg<ledc_intr_map::LEDC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ledc_intr_map;
#[doc = "CAN0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`can0_intr_map`] module"]
pub type CAN0_INTR_MAP = crate::Reg<can0_intr_map::CAN0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod can0_intr_map;
#[doc = "USB_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb_intr_map`] module"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod usb_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rmt_intr_map`] module"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod rmt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_ext0_intr_map`] module"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_ext0_intr_map;
#[doc = "I2C_EXT1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_ext1_intr_map`] module"]
pub type I2C_EXT1_INTR_MAP = crate::Reg<i2c_ext1_intr_map::I2C_EXT1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_ext1_intr_map;
#[doc = "TG0_T0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_t0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_t0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg0_t0_intr_map`] module"]
pub type TG0_T0_INTR_MAP = crate::Reg<tg0_t0_intr_map::TG0_T0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg0_t0_intr_map;
#[doc = "TG0_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg0_wdt_intr_map`] module"]
pub type TG0_WDT_INTR_MAP = crate::Reg<tg0_wdt_intr_map::TG0_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg0_wdt_intr_map;
#[doc = "TG1_T0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg1_t0_intr_map`] module"]
pub type TG1_T0_INTR_MAP = crate::Reg<tg1_t0_intr_map::TG1_T0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg1_t0_intr_map;
#[doc = "TG1_WDT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg1_wdt_intr_map`] module"]
pub type TG1_WDT_INTR_MAP = crate::Reg<tg1_wdt_intr_map::TG1_WDT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod tg1_wdt_intr_map;
#[doc = "SYSTIMER_TARGET0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target0_intr_map`] module"]
pub type SYSTIMER_TARGET0_INTR_MAP =
    crate::Reg<systimer_target0_intr_map::SYSTIMER_TARGET0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target0_intr_map;
#[doc = "SYSTIMER_TARGET1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target1_intr_map`] module"]
pub type SYSTIMER_TARGET1_INTR_MAP =
    crate::Reg<systimer_target1_intr_map::SYSTIMER_TARGET1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target1_intr_map;
#[doc = "SYSTIMER_TARGET2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target2_intr_map`] module"]
pub type SYSTIMER_TARGET2_INTR_MAP =
    crate::Reg<systimer_target2_intr_map::SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target2_intr_map;
#[doc = "APB_ADC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_adc_intr_map`] module"]
pub type APB_ADC_INTR_MAP = crate::Reg<apb_adc_intr_map::APB_ADC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod apb_adc_intr_map;
#[doc = "PWM_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm_intr_map`] module"]
pub type PWM_INTR_MAP = crate::Reg<pwm_intr_map::PWM_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pwm_intr_map;
#[doc = "PCNT_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcnt_intr_map`] module"]
pub type PCNT_INTR_MAP = crate::Reg<pcnt_intr_map::PCNT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod pcnt_intr_map;
#[doc = "PARL_IO_TX_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parl_io_tx_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_tx_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`parl_io_tx_intr_map`] module"]
pub type PARL_IO_TX_INTR_MAP = crate::Reg<parl_io_tx_intr_map::PARL_IO_TX_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod parl_io_tx_intr_map;
#[doc = "PARL_IO_RX_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parl_io_rx_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_rx_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`parl_io_rx_intr_map`] module"]
pub type PARL_IO_RX_INTR_MAP = crate::Reg<parl_io_rx_intr_map::PARL_IO_RX_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod parl_io_rx_intr_map;
#[doc = "DMA_IN_CH0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch0_intr_map`] module"]
pub type DMA_IN_CH0_INTR_MAP = crate::Reg<dma_in_ch0_intr_map::DMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch0_intr_map;
#[doc = "DMA_IN_CH1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch1_intr_map`] module"]
pub type DMA_IN_CH1_INTR_MAP = crate::Reg<dma_in_ch1_intr_map::DMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch1_intr_map;
#[doc = "DMA_IN_CH2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch2_intr_map`] module"]
pub type DMA_IN_CH2_INTR_MAP = crate::Reg<dma_in_ch2_intr_map::DMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_in_ch2_intr_map;
#[doc = "DMA_OUT_CH0_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch0_intr_map`] module"]
pub type DMA_OUT_CH0_INTR_MAP = crate::Reg<dma_out_ch0_intr_map::DMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch0_intr_map;
#[doc = "DMA_OUT_CH1_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch1_intr_map`] module"]
pub type DMA_OUT_CH1_INTR_MAP = crate::Reg<dma_out_ch1_intr_map::DMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch1_intr_map;
#[doc = "DMA_OUT_CH2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch2_intr_map`] module"]
pub type DMA_OUT_CH2_INTR_MAP = crate::Reg<dma_out_ch2_intr_map::DMA_OUT_CH2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_out_ch2_intr_map;
#[doc = "GPSPI2_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpspi2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpspi2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpspi2_intr_map`] module"]
pub type GPSPI2_INTR_MAP = crate::Reg<gpspi2_intr_map::GPSPI2_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod gpspi2_intr_map;
#[doc = "AES_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aes_intr_map`] module"]
pub type AES_INTR_MAP = crate::Reg<aes_intr_map::AES_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod aes_intr_map;
#[doc = "SHA_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sha_intr_map`] module"]
pub type SHA_INTR_MAP = crate::Reg<sha_intr_map::SHA_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod sha_intr_map;
#[doc = "RSA_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsa_intr_map`] module"]
pub type RSA_INTR_MAP = crate::Reg<rsa_intr_map::RSA_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod rsa_intr_map;
#[doc = "ECC_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecc_intr_map`] module"]
pub type ECC_INTR_MAP = crate::Reg<ecc_intr_map::ECC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ecc_intr_map;
#[doc = "ECDSA_INTR_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecdsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecdsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecdsa_intr_map`] module"]
pub type ECDSA_INTR_MAP = crate::Reg<ecdsa_intr_map::ECDSA_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod ecdsa_intr_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_status_reg_0`] module"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_status_reg_1`] module"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_1;
#[doc = "INT_STATUS_REG_2 (r) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_reg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_status_reg_2`] module"]
pub type INT_STATUS_REG_2 = crate::Reg<int_status_reg_2::INT_STATUS_REG_2_SPEC>;
#[doc = "register description"]
pub mod int_status_reg_2;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`interrupt_reg_date`] module"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "register description"]
pub mod interrupt_reg_date;
