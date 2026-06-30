#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    trace0_ctrl: TRACE0_CTRL,
    trace1_ctrl: TRACE1_CTRL,
    cpu_bus_mon_ctrl: CPU_BUS_MON_CTRL,
    psram_mon_ctrl: PSRAM_MON_CTRL,
    tcm_mon_ctrl: TCM_MON_CTRL,
    cache_ctrl: CACHE_CTRL,
    hp_usbotg_core_ctrl: HP_USBOTG_CORE_CTRL,
    hp_regdma_ctrl: HP_REGDMA_CTRL,
    hp_sdmmc_ctrl: HP_SDMMC_CTRL,
    hp_ahb_pdma_ctrl: HP_AHB_PDMA_CTRL,
    hp_jpeg_ctrl: HP_JPEG_CTRL,
    hp_ppa_ctrl: HP_PPA_CTRL,
    hp_dma2d_ctrl: HP_DMA2D_CTRL,
    hp_axi_pdma_ctrl: HP_AXI_PDMA_CTRL,
    hp_gmac_ctrl: HP_GMAC_CTRL,
    hp_pvt0_ctrl: HP_PVT0_CTRL,
    hp_rmt_ctrl: HP_RMT_CTRL,
    hp_bitsrambler_ctrl: HP_BITSRAMBLER_CTRL,
    hp_asrc_ctrl: HP_ASRC_CTRL,
    cnnt_sys_reg_ctrl: CNNT_SYS_REG_CTRL,
    hp_flash_ctrl: HP_FLASH_CTRL,
    hp_psram_ctrl: HP_PSRAM_CTRL,
    tee_ctrl: TEE_CTRL,
    hp_apm_ctrl: HP_APM_CTRL,
    hp_mem_apm_ctrl: HP_MEM_APM_CTRL,
    cpu_apm_ctrl: CPU_APM_CTRL,
    hp_peri0_pms_ctrl: HP_PERI0_PMS_CTRL,
    hp_key_manager_ctrl: HP_KEY_MANAGER_CTRL,
    hp_crypto_ctrl: HP_CRYPTO_CTRL,
    hp_axi_icm_ctrl: HP_AXI_ICM_CTRL,
    axi_perf_mon_ctrl: AXI_PERF_MON_CTRL,
    _reserved31: [u8; 0x0184],
    hp_cpuperi0_0: HP_CPUPERI0_0,
    hp_cpuperi0_1: HP_CPUPERI0_1,
    hp_cpuperi1_0: HP_CPUPERI1_0,
    hp_cpuperi1_1: HP_CPUPERI1_1,
    hp_peri0_0: HP_PERI0_0,
    hp_peri0_1: HP_PERI0_1,
    _reserved37: [u8; 0xe8],
    int_en: INT_EN,
    _reserved38: [u8; 0x0cec],
    bus_err_conf: BUS_ERR_CONF,
    _reserved39: [u8; 0x04],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - trace0 read/write control register"]
    #[inline(always)]
    pub const fn trace0_ctrl(&self) -> &TRACE0_CTRL {
        &self.trace0_ctrl
    }
    #[doc = "0x04 - trace1 read/write control register"]
    #[inline(always)]
    pub const fn trace1_ctrl(&self) -> &TRACE1_CTRL {
        &self.trace1_ctrl
    }
    #[doc = "0x08 - cpu_bus_mon read/write control register"]
    #[inline(always)]
    pub const fn cpu_bus_mon_ctrl(&self) -> &CPU_BUS_MON_CTRL {
        &self.cpu_bus_mon_ctrl
    }
    #[doc = "0x0c - psram_mon read/write control register"]
    #[inline(always)]
    pub const fn psram_mon_ctrl(&self) -> &PSRAM_MON_CTRL {
        &self.psram_mon_ctrl
    }
    #[doc = "0x10 - tcm_mon read/write control register"]
    #[inline(always)]
    pub const fn tcm_mon_ctrl(&self) -> &TCM_MON_CTRL {
        &self.tcm_mon_ctrl
    }
    #[doc = "0x14 - cache read/write control register"]
    #[inline(always)]
    pub const fn cache_ctrl(&self) -> &CACHE_CTRL {
        &self.cache_ctrl
    }
    #[doc = "0x18 - hp_usbotg_core read/write control register"]
    #[inline(always)]
    pub const fn hp_usbotg_core_ctrl(&self) -> &HP_USBOTG_CORE_CTRL {
        &self.hp_usbotg_core_ctrl
    }
    #[doc = "0x1c - hp_regdma read/write control register"]
    #[inline(always)]
    pub const fn hp_regdma_ctrl(&self) -> &HP_REGDMA_CTRL {
        &self.hp_regdma_ctrl
    }
    #[doc = "0x20 - hp_sdmmc read/write control register"]
    #[inline(always)]
    pub const fn hp_sdmmc_ctrl(&self) -> &HP_SDMMC_CTRL {
        &self.hp_sdmmc_ctrl
    }
    #[doc = "0x24 - hp_ahb_pdma read/write control register"]
    #[inline(always)]
    pub const fn hp_ahb_pdma_ctrl(&self) -> &HP_AHB_PDMA_CTRL {
        &self.hp_ahb_pdma_ctrl
    }
    #[doc = "0x28 - hp_jpeg read/write control register"]
    #[inline(always)]
    pub const fn hp_jpeg_ctrl(&self) -> &HP_JPEG_CTRL {
        &self.hp_jpeg_ctrl
    }
    #[doc = "0x2c - hp_ppa read/write control register"]
    #[inline(always)]
    pub const fn hp_ppa_ctrl(&self) -> &HP_PPA_CTRL {
        &self.hp_ppa_ctrl
    }
    #[doc = "0x30 - hp_dma2d read/write control register"]
    #[inline(always)]
    pub const fn hp_dma2d_ctrl(&self) -> &HP_DMA2D_CTRL {
        &self.hp_dma2d_ctrl
    }
    #[doc = "0x34 - hp_axi_pdma read/write control register"]
    #[inline(always)]
    pub const fn hp_axi_pdma_ctrl(&self) -> &HP_AXI_PDMA_CTRL {
        &self.hp_axi_pdma_ctrl
    }
    #[doc = "0x38 - hp_gmac read/write control register"]
    #[inline(always)]
    pub const fn hp_gmac_ctrl(&self) -> &HP_GMAC_CTRL {
        &self.hp_gmac_ctrl
    }
    #[doc = "0x3c - hp_pvt0 read/write control register"]
    #[inline(always)]
    pub const fn hp_pvt0_ctrl(&self) -> &HP_PVT0_CTRL {
        &self.hp_pvt0_ctrl
    }
    #[doc = "0x40 - hp_rmt read/write control register"]
    #[inline(always)]
    pub const fn hp_rmt_ctrl(&self) -> &HP_RMT_CTRL {
        &self.hp_rmt_ctrl
    }
    #[doc = "0x44 - hp_bitsrambler read/write control register"]
    #[inline(always)]
    pub const fn hp_bitsrambler_ctrl(&self) -> &HP_BITSRAMBLER_CTRL {
        &self.hp_bitsrambler_ctrl
    }
    #[doc = "0x48 - hp_asrc read/write control register"]
    #[inline(always)]
    pub const fn hp_asrc_ctrl(&self) -> &HP_ASRC_CTRL {
        &self.hp_asrc_ctrl
    }
    #[doc = "0x4c - cnnt_sys_reg read/write control register"]
    #[inline(always)]
    pub const fn cnnt_sys_reg_ctrl(&self) -> &CNNT_SYS_REG_CTRL {
        &self.cnnt_sys_reg_ctrl
    }
    #[doc = "0x50 - hp_flash read/write control register"]
    #[inline(always)]
    pub const fn hp_flash_ctrl(&self) -> &HP_FLASH_CTRL {
        &self.hp_flash_ctrl
    }
    #[doc = "0x54 - hp_psram read/write control register"]
    #[inline(always)]
    pub const fn hp_psram_ctrl(&self) -> &HP_PSRAM_CTRL {
        &self.hp_psram_ctrl
    }
    #[doc = "0x58 - tee read/write control register"]
    #[inline(always)]
    pub const fn tee_ctrl(&self) -> &TEE_CTRL {
        &self.tee_ctrl
    }
    #[doc = "0x5c - hp_apm read/write control register"]
    #[inline(always)]
    pub const fn hp_apm_ctrl(&self) -> &HP_APM_CTRL {
        &self.hp_apm_ctrl
    }
    #[doc = "0x60 - hp_mem_apm read/write control register"]
    #[inline(always)]
    pub const fn hp_mem_apm_ctrl(&self) -> &HP_MEM_APM_CTRL {
        &self.hp_mem_apm_ctrl
    }
    #[doc = "0x64 - cpu_apm read/write control register"]
    #[inline(always)]
    pub const fn cpu_apm_ctrl(&self) -> &CPU_APM_CTRL {
        &self.cpu_apm_ctrl
    }
    #[doc = "0x68 - hp_peri0_pms read/write control register"]
    #[inline(always)]
    pub const fn hp_peri0_pms_ctrl(&self) -> &HP_PERI0_PMS_CTRL {
        &self.hp_peri0_pms_ctrl
    }
    #[doc = "0x6c - hp_key_manager read/write control register"]
    #[inline(always)]
    pub const fn hp_key_manager_ctrl(&self) -> &HP_KEY_MANAGER_CTRL {
        &self.hp_key_manager_ctrl
    }
    #[doc = "0x70 - hp_crypto read/write control register"]
    #[inline(always)]
    pub const fn hp_crypto_ctrl(&self) -> &HP_CRYPTO_CTRL {
        &self.hp_crypto_ctrl
    }
    #[doc = "0x74 - hp_axi_icm read/write control register"]
    #[inline(always)]
    pub const fn hp_axi_icm_ctrl(&self) -> &HP_AXI_ICM_CTRL {
        &self.hp_axi_icm_ctrl
    }
    #[doc = "0x78 - axi_perf_mon read/write control register"]
    #[inline(always)]
    pub const fn axi_perf_mon_ctrl(&self) -> &AXI_PERF_MON_CTRL {
        &self.axi_perf_mon_ctrl
    }
    #[doc = "0x200 - HP_CPUPERI0 PMS configuration & info register"]
    #[inline(always)]
    pub const fn hp_cpuperi0_0(&self) -> &HP_CPUPERI0_0 {
        &self.hp_cpuperi0_0
    }
    #[doc = "0x204 - HP_CPUPERI0 PMS exception addr record register"]
    #[inline(always)]
    pub const fn hp_cpuperi0_1(&self) -> &HP_CPUPERI0_1 {
        &self.hp_cpuperi0_1
    }
    #[doc = "0x208 - HP_CPUPERI1 PMS configuration & info register"]
    #[inline(always)]
    pub const fn hp_cpuperi1_0(&self) -> &HP_CPUPERI1_0 {
        &self.hp_cpuperi1_0
    }
    #[doc = "0x20c - HP_CPUPERI1 PMS exception addr record register"]
    #[inline(always)]
    pub const fn hp_cpuperi1_1(&self) -> &HP_CPUPERI1_1 {
        &self.hp_cpuperi1_1
    }
    #[doc = "0x210 - HP_PERI0 PMS configuration & info register"]
    #[inline(always)]
    pub const fn hp_peri0_0(&self) -> &HP_PERI0_0 {
        &self.hp_peri0_0
    }
    #[doc = "0x214 - HP_PERI0 PMS exception addr record register"]
    #[inline(always)]
    pub const fn hp_peri0_1(&self) -> &HP_PERI0_1 {
        &self.hp_peri0_1
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
#[doc = "TRACE0_CTRL (rw) register accessor: trace0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace0_ctrl`] module"]
pub type TRACE0_CTRL = crate::Reg<trace0_ctrl::TRACE0_CTRL_SPEC>;
#[doc = "trace0 read/write control register"]
pub mod trace0_ctrl;
#[doc = "TRACE1_CTRL (rw) register accessor: trace1 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace1_ctrl`] module"]
pub type TRACE1_CTRL = crate::Reg<trace1_ctrl::TRACE1_CTRL_SPEC>;
#[doc = "trace1 read/write control register"]
pub mod trace1_ctrl;
#[doc = "CPU_BUS_MON_CTRL (rw) register accessor: cpu_bus_mon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_bus_mon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_bus_mon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_bus_mon_ctrl`] module"]
pub type CPU_BUS_MON_CTRL = crate::Reg<cpu_bus_mon_ctrl::CPU_BUS_MON_CTRL_SPEC>;
#[doc = "cpu_bus_mon read/write control register"]
pub mod cpu_bus_mon_ctrl;
#[doc = "PSRAM_MON_CTRL (rw) register accessor: psram_mon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mon_ctrl`] module"]
pub type PSRAM_MON_CTRL = crate::Reg<psram_mon_ctrl::PSRAM_MON_CTRL_SPEC>;
#[doc = "psram_mon read/write control register"]
pub mod psram_mon_ctrl;
#[doc = "TCM_MON_CTRL (rw) register accessor: tcm_mon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_mon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_mon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_mon_ctrl`] module"]
pub type TCM_MON_CTRL = crate::Reg<tcm_mon_ctrl::TCM_MON_CTRL_SPEC>;
#[doc = "tcm_mon read/write control register"]
pub mod tcm_mon_ctrl;
#[doc = "CACHE_CTRL (rw) register accessor: cache read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ctrl`] module"]
pub type CACHE_CTRL = crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>;
#[doc = "cache read/write control register"]
pub mod cache_ctrl;
#[doc = "HP_USBOTG_CORE_CTRL (rw) register accessor: hp_usbotg_core read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usbotg_core_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usbotg_core_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_usbotg_core_ctrl`] module"]
pub type HP_USBOTG_CORE_CTRL = crate::Reg<hp_usbotg_core_ctrl::HP_USBOTG_CORE_CTRL_SPEC>;
#[doc = "hp_usbotg_core read/write control register"]
pub mod hp_usbotg_core_ctrl;
#[doc = "HP_REGDMA_CTRL (rw) register accessor: hp_regdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_regdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_regdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_regdma_ctrl`] module"]
pub type HP_REGDMA_CTRL = crate::Reg<hp_regdma_ctrl::HP_REGDMA_CTRL_SPEC>;
#[doc = "hp_regdma read/write control register"]
pub mod hp_regdma_ctrl;
#[doc = "HP_SDMMC_CTRL (rw) register accessor: hp_sdmmc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sdmmc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sdmmc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sdmmc_ctrl`] module"]
pub type HP_SDMMC_CTRL = crate::Reg<hp_sdmmc_ctrl::HP_SDMMC_CTRL_SPEC>;
#[doc = "hp_sdmmc read/write control register"]
pub mod hp_sdmmc_ctrl;
#[doc = "HP_AHB_PDMA_CTRL (rw) register accessor: hp_ahb_pdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb_pdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb_pdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb_pdma_ctrl`] module"]
pub type HP_AHB_PDMA_CTRL = crate::Reg<hp_ahb_pdma_ctrl::HP_AHB_PDMA_CTRL_SPEC>;
#[doc = "hp_ahb_pdma read/write control register"]
pub mod hp_ahb_pdma_ctrl;
#[doc = "HP_JPEG_CTRL (rw) register accessor: hp_jpeg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_jpeg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_jpeg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_jpeg_ctrl`] module"]
pub type HP_JPEG_CTRL = crate::Reg<hp_jpeg_ctrl::HP_JPEG_CTRL_SPEC>;
#[doc = "hp_jpeg read/write control register"]
pub mod hp_jpeg_ctrl;
#[doc = "HP_PPA_CTRL (rw) register accessor: hp_ppa read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ppa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ppa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ppa_ctrl`] module"]
pub type HP_PPA_CTRL = crate::Reg<hp_ppa_ctrl::HP_PPA_CTRL_SPEC>;
#[doc = "hp_ppa read/write control register"]
pub mod hp_ppa_ctrl;
#[doc = "HP_DMA2D_CTRL (rw) register accessor: hp_dma2d read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_dma2d_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_dma2d_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_dma2d_ctrl`] module"]
pub type HP_DMA2D_CTRL = crate::Reg<hp_dma2d_ctrl::HP_DMA2D_CTRL_SPEC>;
#[doc = "hp_dma2d read/write control register"]
pub mod hp_dma2d_ctrl;
#[doc = "HP_AXI_PDMA_CTRL (rw) register accessor: hp_axi_pdma read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_axi_pdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_axi_pdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_axi_pdma_ctrl`] module"]
pub type HP_AXI_PDMA_CTRL = crate::Reg<hp_axi_pdma_ctrl::HP_AXI_PDMA_CTRL_SPEC>;
#[doc = "hp_axi_pdma read/write control register"]
pub mod hp_axi_pdma_ctrl;
#[doc = "HP_GMAC_CTRL (rw) register accessor: hp_gmac read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gmac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gmac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gmac_ctrl`] module"]
pub type HP_GMAC_CTRL = crate::Reg<hp_gmac_ctrl::HP_GMAC_CTRL_SPEC>;
#[doc = "hp_gmac read/write control register"]
pub mod hp_gmac_ctrl;
#[doc = "HP_PVT0_CTRL (rw) register accessor: hp_pvt0 read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pvt0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pvt0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pvt0_ctrl`] module"]
pub type HP_PVT0_CTRL = crate::Reg<hp_pvt0_ctrl::HP_PVT0_CTRL_SPEC>;
#[doc = "hp_pvt0 read/write control register"]
pub mod hp_pvt0_ctrl;
#[doc = "HP_RMT_CTRL (rw) register accessor: hp_rmt read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rmt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rmt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rmt_ctrl`] module"]
pub type HP_RMT_CTRL = crate::Reg<hp_rmt_ctrl::HP_RMT_CTRL_SPEC>;
#[doc = "hp_rmt read/write control register"]
pub mod hp_rmt_ctrl;
#[doc = "HP_BITSRAMBLER_CTRL (rw) register accessor: hp_bitsrambler read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_bitsrambler_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_bitsrambler_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_bitsrambler_ctrl`] module"]
pub type HP_BITSRAMBLER_CTRL = crate::Reg<hp_bitsrambler_ctrl::HP_BITSRAMBLER_CTRL_SPEC>;
#[doc = "hp_bitsrambler read/write control register"]
pub mod hp_bitsrambler_ctrl;
#[doc = "HP_ASRC_CTRL (rw) register accessor: hp_asrc read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_asrc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_asrc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_asrc_ctrl`] module"]
pub type HP_ASRC_CTRL = crate::Reg<hp_asrc_ctrl::HP_ASRC_CTRL_SPEC>;
#[doc = "hp_asrc read/write control register"]
pub mod hp_asrc_ctrl;
#[doc = "CNNT_SYS_REG_CTRL (rw) register accessor: cnnt_sys_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_sys_reg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_sys_reg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnnt_sys_reg_ctrl`] module"]
pub type CNNT_SYS_REG_CTRL = crate::Reg<cnnt_sys_reg_ctrl::CNNT_SYS_REG_CTRL_SPEC>;
#[doc = "cnnt_sys_reg read/write control register"]
pub mod cnnt_sys_reg_ctrl;
#[doc = "HP_FLASH_CTRL (rw) register accessor: hp_flash read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_flash_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_flash_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_flash_ctrl`] module"]
pub type HP_FLASH_CTRL = crate::Reg<hp_flash_ctrl::HP_FLASH_CTRL_SPEC>;
#[doc = "hp_flash read/write control register"]
pub mod hp_flash_ctrl;
#[doc = "HP_PSRAM_CTRL (rw) register accessor: hp_psram read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_psram_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_psram_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_psram_ctrl`] module"]
pub type HP_PSRAM_CTRL = crate::Reg<hp_psram_ctrl::HP_PSRAM_CTRL_SPEC>;
#[doc = "hp_psram read/write control register"]
pub mod hp_psram_ctrl;
#[doc = "TEE_CTRL (rw) register accessor: tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tee_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tee_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tee_ctrl`] module"]
pub type TEE_CTRL = crate::Reg<tee_ctrl::TEE_CTRL_SPEC>;
#[doc = "tee read/write control register"]
pub mod tee_ctrl;
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
#[doc = "HP_PERI0_PMS_CTRL (rw) register accessor: hp_peri0_pms read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_pms_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri0_pms_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_pms_ctrl`] module"]
pub type HP_PERI0_PMS_CTRL = crate::Reg<hp_peri0_pms_ctrl::HP_PERI0_PMS_CTRL_SPEC>;
#[doc = "hp_peri0_pms read/write control register"]
pub mod hp_peri0_pms_ctrl;
#[doc = "HP_KEY_MANAGER_CTRL (rw) register accessor: hp_key_manager read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_key_manager_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_key_manager_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_key_manager_ctrl`] module"]
pub type HP_KEY_MANAGER_CTRL = crate::Reg<hp_key_manager_ctrl::HP_KEY_MANAGER_CTRL_SPEC>;
#[doc = "hp_key_manager read/write control register"]
pub mod hp_key_manager_ctrl;
#[doc = "HP_CRYPTO_CTRL (rw) register accessor: hp_crypto read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_crypto_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_crypto_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_crypto_ctrl`] module"]
pub type HP_CRYPTO_CTRL = crate::Reg<hp_crypto_ctrl::HP_CRYPTO_CTRL_SPEC>;
#[doc = "hp_crypto read/write control register"]
pub mod hp_crypto_ctrl;
#[doc = "HP_AXI_ICM_CTRL (rw) register accessor: hp_axi_icm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_axi_icm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_axi_icm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_axi_icm_ctrl`] module"]
pub type HP_AXI_ICM_CTRL = crate::Reg<hp_axi_icm_ctrl::HP_AXI_ICM_CTRL_SPEC>;
#[doc = "hp_axi_icm read/write control register"]
pub mod hp_axi_icm_ctrl;
#[doc = "AXI_PERF_MON_CTRL (rw) register accessor: axi_perf_mon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_perf_mon_ctrl`] module"]
pub type AXI_PERF_MON_CTRL = crate::Reg<axi_perf_mon_ctrl::AXI_PERF_MON_CTRL_SPEC>;
#[doc = "axi_perf_mon read/write control register"]
pub mod axi_perf_mon_ctrl;
#[doc = "HP_CPUPERI0_0 (rw) register accessor: HP_CPUPERI0 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpuperi0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpuperi0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpuperi0_0`] module"]
pub type HP_CPUPERI0_0 = crate::Reg<hp_cpuperi0_0::HP_CPUPERI0_0_SPEC>;
#[doc = "HP_CPUPERI0 PMS configuration & info register"]
pub mod hp_cpuperi0_0;
#[doc = "HP_CPUPERI0_1 (r) register accessor: HP_CPUPERI0 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpuperi0_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpuperi0_1`] module"]
pub type HP_CPUPERI0_1 = crate::Reg<hp_cpuperi0_1::HP_CPUPERI0_1_SPEC>;
#[doc = "HP_CPUPERI0 PMS exception addr record register"]
pub mod hp_cpuperi0_1;
#[doc = "HP_CPUPERI1_0 (rw) register accessor: HP_CPUPERI1 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpuperi1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpuperi1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpuperi1_0`] module"]
pub type HP_CPUPERI1_0 = crate::Reg<hp_cpuperi1_0::HP_CPUPERI1_0_SPEC>;
#[doc = "HP_CPUPERI1 PMS configuration & info register"]
pub mod hp_cpuperi1_0;
#[doc = "HP_CPUPERI1_1 (r) register accessor: HP_CPUPERI1 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpuperi1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpuperi1_1`] module"]
pub type HP_CPUPERI1_1 = crate::Reg<hp_cpuperi1_1::HP_CPUPERI1_1_SPEC>;
#[doc = "HP_CPUPERI1 PMS exception addr record register"]
pub mod hp_cpuperi1_1;
#[doc = "HP_PERI0_0 (rw) register accessor: HP_PERI0 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_0`] module"]
pub type HP_PERI0_0 = crate::Reg<hp_peri0_0::HP_PERI0_0_SPEC>;
#[doc = "HP_PERI0 PMS configuration & info register"]
pub mod hp_peri0_0;
#[doc = "HP_PERI0_1 (r) register accessor: HP_PERI0 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_1`] module"]
pub type HP_PERI0_1 = crate::Reg<hp_peri0_1::HP_PERI0_1_SPEC>;
#[doc = "HP_PERI0 PMS exception addr record register"]
pub mod hp_peri0_1;
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
