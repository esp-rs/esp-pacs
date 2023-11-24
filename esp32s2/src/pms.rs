#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    sdio_0: SDIO_0,
    sdio_1: SDIO_1,
    mac_dump_0: MAC_DUMP_0,
    mac_dump_1: MAC_DUMP_1,
    pro_iram0_0: PRO_IRAM0_0,
    pro_iram0_1: PRO_IRAM0_1,
    pro_iram0_2: PRO_IRAM0_2,
    pro_iram0_3: PRO_IRAM0_3,
    pro_iram0_4: PRO_IRAM0_4,
    pro_iram0_5: PRO_IRAM0_5,
    pro_dram0_0: PRO_DRAM0_0,
    pro_dram0_1: PRO_DRAM0_1,
    pro_dram0_2: PRO_DRAM0_2,
    pro_dram0_3: PRO_DRAM0_3,
    pro_dram0_4: PRO_DRAM0_4,
    pro_dport_0: PRO_DPORT_0,
    pro_dport_1: PRO_DPORT_1,
    pro_dport_2: PRO_DPORT_2,
    pro_dport_3: PRO_DPORT_3,
    pro_dport_4: PRO_DPORT_4,
    pro_dport_5: PRO_DPORT_5,
    pro_dport_6: PRO_DPORT_6,
    pro_dport_7: PRO_DPORT_7,
    pro_ahb_0: PRO_AHB_0,
    pro_ahb_1: PRO_AHB_1,
    pro_ahb_2: PRO_AHB_2,
    pro_ahb_3: PRO_AHB_3,
    pro_ahb_4: PRO_AHB_4,
    pro_trace_0: PRO_TRACE_0,
    pro_trace_1: PRO_TRACE_1,
    pro_cache_0: PRO_CACHE_0,
    pro_cache_1: PRO_CACHE_1,
    pro_cache_2: PRO_CACHE_2,
    pro_cache_3: PRO_CACHE_3,
    pro_cache_4: PRO_CACHE_4,
    dma_apb_i_0: DMA_APB_I_0,
    dma_apb_i_1: DMA_APB_I_1,
    dma_apb_i_2: DMA_APB_I_2,
    dma_apb_i_3: DMA_APB_I_3,
    dma_rx_i_0: DMA_RX_I_0,
    dma_rx_i_1: DMA_RX_I_1,
    dma_rx_i_2: DMA_RX_I_2,
    dma_rx_i_3: DMA_RX_I_3,
    dma_tx_i_0: DMA_TX_I_0,
    dma_tx_i_1: DMA_TX_I_1,
    dma_tx_i_2: DMA_TX_I_2,
    dma_tx_i_3: DMA_TX_I_3,
    pro_boot_location_0: PRO_BOOT_LOCATION_0,
    pro_boot_location_1: PRO_BOOT_LOCATION_1,
    cache_source_0: CACHE_SOURCE_0,
    cache_source_1: CACHE_SOURCE_1,
    apb_peripheral_0: APB_PERIPHERAL_0,
    apb_peripheral_1: APB_PERIPHERAL_1,
    occupy_0: OCCUPY_0,
    occupy_1: OCCUPY_1,
    occupy_2: OCCUPY_2,
    occupy_3: OCCUPY_3,
    cache_tag_access_0: CACHE_TAG_ACCESS_0,
    cache_tag_access_1: CACHE_TAG_ACCESS_1,
    cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    apb_peripheral_intr: APB_PERIPHERAL_INTR,
    apb_peripheral_status: APB_PERIPHERAL_STATUS,
    cpu_peripheral_intr: CPU_PERIPHERAL_INTR,
    cpu_peripheral_status: CPU_PERIPHERAL_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved66: [u8; 0x0ef4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SDIO permission control register 0."]
    #[inline(always)]
    pub const fn sdio_0(&self) -> &SDIO_0 {
        &self.sdio_0
    }
    #[doc = "0x04 - SDIO permission control register 1."]
    #[inline(always)]
    pub const fn sdio_1(&self) -> &SDIO_1 {
        &self.sdio_1
    }
    #[doc = "0x08 - MAC dump permission control register 0."]
    #[inline(always)]
    pub const fn mac_dump_0(&self) -> &MAC_DUMP_0 {
        &self.mac_dump_0
    }
    #[doc = "0x0c - MAC dump permission control register 1."]
    #[inline(always)]
    pub const fn mac_dump_1(&self) -> &MAC_DUMP_1 {
        &self.mac_dump_1
    }
    #[doc = "0x10 - IBUS permission control register 0."]
    #[inline(always)]
    pub const fn pro_iram0_0(&self) -> &PRO_IRAM0_0 {
        &self.pro_iram0_0
    }
    #[doc = "0x14 - IBUS permission control register 1."]
    #[inline(always)]
    pub const fn pro_iram0_1(&self) -> &PRO_IRAM0_1 {
        &self.pro_iram0_1
    }
    #[doc = "0x18 - IBUS permission control register 2."]
    #[inline(always)]
    pub const fn pro_iram0_2(&self) -> &PRO_IRAM0_2 {
        &self.pro_iram0_2
    }
    #[doc = "0x1c - IBUS permission control register 3."]
    #[inline(always)]
    pub const fn pro_iram0_3(&self) -> &PRO_IRAM0_3 {
        &self.pro_iram0_3
    }
    #[doc = "0x20 - IBUS permission control register 4."]
    #[inline(always)]
    pub const fn pro_iram0_4(&self) -> &PRO_IRAM0_4 {
        &self.pro_iram0_4
    }
    #[doc = "0x24 - IBUS status register."]
    #[inline(always)]
    pub const fn pro_iram0_5(&self) -> &PRO_IRAM0_5 {
        &self.pro_iram0_5
    }
    #[doc = "0x28 - DBUS permission control register 0."]
    #[inline(always)]
    pub const fn pro_dram0_0(&self) -> &PRO_DRAM0_0 {
        &self.pro_dram0_0
    }
    #[doc = "0x2c - DBUS permission control register 1."]
    #[inline(always)]
    pub const fn pro_dram0_1(&self) -> &PRO_DRAM0_1 {
        &self.pro_dram0_1
    }
    #[doc = "0x30 - DBUS permission control register 2."]
    #[inline(always)]
    pub const fn pro_dram0_2(&self) -> &PRO_DRAM0_2 {
        &self.pro_dram0_2
    }
    #[doc = "0x34 - DBUS permission control register 3."]
    #[inline(always)]
    pub const fn pro_dram0_3(&self) -> &PRO_DRAM0_3 {
        &self.pro_dram0_3
    }
    #[doc = "0x38 - DBUS status register."]
    #[inline(always)]
    pub const fn pro_dram0_4(&self) -> &PRO_DRAM0_4 {
        &self.pro_dram0_4
    }
    #[doc = "0x3c - PeriBus1 permission control register 0."]
    #[inline(always)]
    pub const fn pro_dport_0(&self) -> &PRO_DPORT_0 {
        &self.pro_dport_0
    }
    #[doc = "0x40 - PeriBus1 permission control register 1."]
    #[inline(always)]
    pub const fn pro_dport_1(&self) -> &PRO_DPORT_1 {
        &self.pro_dport_1
    }
    #[doc = "0x44 - PeriBus1 permission control register 2."]
    #[inline(always)]
    pub const fn pro_dport_2(&self) -> &PRO_DPORT_2 {
        &self.pro_dport_2
    }
    #[doc = "0x48 - PeriBus1 permission control register 3."]
    #[inline(always)]
    pub const fn pro_dport_3(&self) -> &PRO_DPORT_3 {
        &self.pro_dport_3
    }
    #[doc = "0x4c - PeriBus1 permission control register 4."]
    #[inline(always)]
    pub const fn pro_dport_4(&self) -> &PRO_DPORT_4 {
        &self.pro_dport_4
    }
    #[doc = "0x50 - PeriBus1 permission control register 5."]
    #[inline(always)]
    pub const fn pro_dport_5(&self) -> &PRO_DPORT_5 {
        &self.pro_dport_5
    }
    #[doc = "0x54 - PeriBus1 permission control register 6."]
    #[inline(always)]
    pub const fn pro_dport_6(&self) -> &PRO_DPORT_6 {
        &self.pro_dport_6
    }
    #[doc = "0x58 - PeriBus1 status register."]
    #[inline(always)]
    pub const fn pro_dport_7(&self) -> &PRO_DPORT_7 {
        &self.pro_dport_7
    }
    #[doc = "0x5c - PeriBus2 permission control register 0."]
    #[inline(always)]
    pub const fn pro_ahb_0(&self) -> &PRO_AHB_0 {
        &self.pro_ahb_0
    }
    #[doc = "0x60 - PeriBus2 permission control register 1."]
    #[inline(always)]
    pub const fn pro_ahb_1(&self) -> &PRO_AHB_1 {
        &self.pro_ahb_1
    }
    #[doc = "0x64 - PeriBus2 permission control register 2."]
    #[inline(always)]
    pub const fn pro_ahb_2(&self) -> &PRO_AHB_2 {
        &self.pro_ahb_2
    }
    #[doc = "0x68 - PeriBus2 permission control register 3."]
    #[inline(always)]
    pub const fn pro_ahb_3(&self) -> &PRO_AHB_3 {
        &self.pro_ahb_3
    }
    #[doc = "0x6c - PeriBus2 status register."]
    #[inline(always)]
    pub const fn pro_ahb_4(&self) -> &PRO_AHB_4 {
        &self.pro_ahb_4
    }
    #[doc = "0x70 - Trace memory permission control register 0."]
    #[inline(always)]
    pub const fn pro_trace_0(&self) -> &PRO_TRACE_0 {
        &self.pro_trace_0
    }
    #[doc = "0x74 - Trace memory permission control register 1."]
    #[inline(always)]
    pub const fn pro_trace_1(&self) -> &PRO_TRACE_1 {
        &self.pro_trace_1
    }
    #[doc = "0x78 - Cache permission control register 0."]
    #[inline(always)]
    pub const fn pro_cache_0(&self) -> &PRO_CACHE_0 {
        &self.pro_cache_0
    }
    #[doc = "0x7c - Cache permission control register 1."]
    #[inline(always)]
    pub const fn pro_cache_1(&self) -> &PRO_CACHE_1 {
        &self.pro_cache_1
    }
    #[doc = "0x80 - Cache permission control register 2."]
    #[inline(always)]
    pub const fn pro_cache_2(&self) -> &PRO_CACHE_2 {
        &self.pro_cache_2
    }
    #[doc = "0x84 - Icache status register."]
    #[inline(always)]
    pub const fn pro_cache_3(&self) -> &PRO_CACHE_3 {
        &self.pro_cache_3
    }
    #[doc = "0x88 - Dcache status register."]
    #[inline(always)]
    pub const fn pro_cache_4(&self) -> &PRO_CACHE_4 {
        &self.pro_cache_4
    }
    #[doc = "0x8c - Internal DMA permission control register 0."]
    #[inline(always)]
    pub const fn dma_apb_i_0(&self) -> &DMA_APB_I_0 {
        &self.dma_apb_i_0
    }
    #[doc = "0x90 - Internal DMA permission control register 1."]
    #[inline(always)]
    pub const fn dma_apb_i_1(&self) -> &DMA_APB_I_1 {
        &self.dma_apb_i_1
    }
    #[doc = "0x94 - Internal DMA permission control register 2."]
    #[inline(always)]
    pub const fn dma_apb_i_2(&self) -> &DMA_APB_I_2 {
        &self.dma_apb_i_2
    }
    #[doc = "0x98 - Internal DMA status register."]
    #[inline(always)]
    pub const fn dma_apb_i_3(&self) -> &DMA_APB_I_3 {
        &self.dma_apb_i_3
    }
    #[doc = "0x9c - RX Copy DMA permission control register 0."]
    #[inline(always)]
    pub const fn dma_rx_i_0(&self) -> &DMA_RX_I_0 {
        &self.dma_rx_i_0
    }
    #[doc = "0xa0 - RX Copy DMA permission control register 1."]
    #[inline(always)]
    pub const fn dma_rx_i_1(&self) -> &DMA_RX_I_1 {
        &self.dma_rx_i_1
    }
    #[doc = "0xa4 - RX Copy DMA permission control register 2."]
    #[inline(always)]
    pub const fn dma_rx_i_2(&self) -> &DMA_RX_I_2 {
        &self.dma_rx_i_2
    }
    #[doc = "0xa8 - RX Copy DMA status register."]
    #[inline(always)]
    pub const fn dma_rx_i_3(&self) -> &DMA_RX_I_3 {
        &self.dma_rx_i_3
    }
    #[doc = "0xac - TX Copy DMA permission control register 0."]
    #[inline(always)]
    pub const fn dma_tx_i_0(&self) -> &DMA_TX_I_0 {
        &self.dma_tx_i_0
    }
    #[doc = "0xb0 - TX Copy DMA permission control register 1."]
    #[inline(always)]
    pub const fn dma_tx_i_1(&self) -> &DMA_TX_I_1 {
        &self.dma_tx_i_1
    }
    #[doc = "0xb4 - TX Copy DMA permission control register 2."]
    #[inline(always)]
    pub const fn dma_tx_i_2(&self) -> &DMA_TX_I_2 {
        &self.dma_tx_i_2
    }
    #[doc = "0xb8 - TX Copy DMA status register."]
    #[inline(always)]
    pub const fn dma_tx_i_3(&self) -> &DMA_TX_I_3 {
        &self.dma_tx_i_3
    }
    #[doc = "0xbc - Boot permission control register 0."]
    #[inline(always)]
    pub const fn pro_boot_location_0(&self) -> &PRO_BOOT_LOCATION_0 {
        &self.pro_boot_location_0
    }
    #[doc = "0xc0 - Boot permission control register 1."]
    #[inline(always)]
    pub const fn pro_boot_location_1(&self) -> &PRO_BOOT_LOCATION_1 {
        &self.pro_boot_location_1
    }
    #[doc = "0xc4 - Cache access permission control register 0."]
    #[inline(always)]
    pub const fn cache_source_0(&self) -> &CACHE_SOURCE_0 {
        &self.cache_source_0
    }
    #[doc = "0xc8 - Cache access permission control register 1."]
    #[inline(always)]
    pub const fn cache_source_1(&self) -> &CACHE_SOURCE_1 {
        &self.cache_source_1
    }
    #[doc = "0xcc - Peripheral access permission control register 0."]
    #[inline(always)]
    pub const fn apb_peripheral_0(&self) -> &APB_PERIPHERAL_0 {
        &self.apb_peripheral_0
    }
    #[doc = "0xd0 - Peripheral access permission control register 1."]
    #[inline(always)]
    pub const fn apb_peripheral_1(&self) -> &APB_PERIPHERAL_1 {
        &self.apb_peripheral_1
    }
    #[doc = "0xd4 - Occupy permission control register 0."]
    #[inline(always)]
    pub const fn occupy_0(&self) -> &OCCUPY_0 {
        &self.occupy_0
    }
    #[doc = "0xd8 - Occupy permission control register 1."]
    #[inline(always)]
    pub const fn occupy_1(&self) -> &OCCUPY_1 {
        &self.occupy_1
    }
    #[doc = "0xdc - Occupy permission control register 2."]
    #[inline(always)]
    pub const fn occupy_2(&self) -> &OCCUPY_2 {
        &self.occupy_2
    }
    #[doc = "0xe0 - Occupy permission control register 3."]
    #[inline(always)]
    pub const fn occupy_3(&self) -> &OCCUPY_3 {
        &self.occupy_3
    }
    #[doc = "0xe4 - Cache tag permission control register 0."]
    #[inline(always)]
    pub const fn cache_tag_access_0(&self) -> &CACHE_TAG_ACCESS_0 {
        &self.cache_tag_access_0
    }
    #[doc = "0xe8 - Cache tag permission control register 1."]
    #[inline(always)]
    pub const fn cache_tag_access_1(&self) -> &CACHE_TAG_ACCESS_1 {
        &self.cache_tag_access_1
    }
    #[doc = "0xec - Cache MMU permission control register 0."]
    #[inline(always)]
    pub const fn cache_mmu_access_0(&self) -> &CACHE_MMU_ACCESS_0 {
        &self.cache_mmu_access_0
    }
    #[doc = "0xf0 - Cache MMU permission control register 1."]
    #[inline(always)]
    pub const fn cache_mmu_access_1(&self) -> &CACHE_MMU_ACCESS_1 {
        &self.cache_mmu_access_1
    }
    #[doc = "0xf4 - PeribBus2 permission control register."]
    #[inline(always)]
    pub const fn apb_peripheral_intr(&self) -> &APB_PERIPHERAL_INTR {
        &self.apb_peripheral_intr
    }
    #[doc = "0xf8 - PeribBus2 peripheral access status register."]
    #[inline(always)]
    pub const fn apb_peripheral_status(&self) -> &APB_PERIPHERAL_STATUS {
        &self.apb_peripheral_status
    }
    #[doc = "0xfc - PeribBus1 permission control register."]
    #[inline(always)]
    pub const fn cpu_peripheral_intr(&self) -> &CPU_PERIPHERAL_INTR {
        &self.cpu_peripheral_intr
    }
    #[doc = "0x100 - PeribBus1 peripheral access status register."]
    #[inline(always)]
    pub const fn cpu_peripheral_status(&self) -> &CPU_PERIPHERAL_STATUS {
        &self.cpu_peripheral_status
    }
    #[doc = "0x104 - Clock gate register of permission control."]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Version control register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SDIO_0 (rw) register accessor: SDIO permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_0`] module"]
pub type SDIO_0 = crate::Reg<sdio_0::SDIO_0_SPEC>;
#[doc = "SDIO permission control register 0."]
pub mod sdio_0;
#[doc = "SDIO_1 (rw) register accessor: SDIO permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_1`] module"]
pub type SDIO_1 = crate::Reg<sdio_1::SDIO_1_SPEC>;
#[doc = "SDIO permission control register 1."]
pub mod sdio_1;
#[doc = "MAC_DUMP_0 (rw) register accessor: MAC dump permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_dump_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_dump_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_dump_0`] module"]
pub type MAC_DUMP_0 = crate::Reg<mac_dump_0::MAC_DUMP_0_SPEC>;
#[doc = "MAC dump permission control register 0."]
pub mod mac_dump_0;
#[doc = "MAC_DUMP_1 (rw) register accessor: MAC dump permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_dump_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_dump_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_dump_1`] module"]
pub type MAC_DUMP_1 = crate::Reg<mac_dump_1::MAC_DUMP_1_SPEC>;
#[doc = "MAC dump permission control register 1."]
pub mod mac_dump_1;
#[doc = "PRO_IRAM0_0 (rw) register accessor: IBUS permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_iram0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_0`] module"]
pub type PRO_IRAM0_0 = crate::Reg<pro_iram0_0::PRO_IRAM0_0_SPEC>;
#[doc = "IBUS permission control register 0."]
pub mod pro_iram0_0;
#[doc = "PRO_IRAM0_1 (rw) register accessor: IBUS permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_iram0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_1`] module"]
pub type PRO_IRAM0_1 = crate::Reg<pro_iram0_1::PRO_IRAM0_1_SPEC>;
#[doc = "IBUS permission control register 1."]
pub mod pro_iram0_1;
#[doc = "PRO_IRAM0_2 (rw) register accessor: IBUS permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_iram0_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_2`] module"]
pub type PRO_IRAM0_2 = crate::Reg<pro_iram0_2::PRO_IRAM0_2_SPEC>;
#[doc = "IBUS permission control register 2."]
pub mod pro_iram0_2;
#[doc = "PRO_IRAM0_3 (rw) register accessor: IBUS permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_iram0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_3`] module"]
pub type PRO_IRAM0_3 = crate::Reg<pro_iram0_3::PRO_IRAM0_3_SPEC>;
#[doc = "IBUS permission control register 3."]
pub mod pro_iram0_3;
#[doc = "PRO_IRAM0_4 (rw) register accessor: IBUS permission control register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_iram0_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_4`] module"]
pub type PRO_IRAM0_4 = crate::Reg<pro_iram0_4::PRO_IRAM0_4_SPEC>;
#[doc = "IBUS permission control register 4."]
pub mod pro_iram0_4;
#[doc = "PRO_IRAM0_5 (r) register accessor: IBUS status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_iram0_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_iram0_5`] module"]
pub type PRO_IRAM0_5 = crate::Reg<pro_iram0_5::PRO_IRAM0_5_SPEC>;
#[doc = "IBUS status register."]
pub mod pro_iram0_5;
#[doc = "PRO_DRAM0_0 (rw) register accessor: DBUS permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dram0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dram0_0`] module"]
pub type PRO_DRAM0_0 = crate::Reg<pro_dram0_0::PRO_DRAM0_0_SPEC>;
#[doc = "DBUS permission control register 0."]
pub mod pro_dram0_0;
#[doc = "PRO_DRAM0_1 (rw) register accessor: DBUS permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dram0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dram0_1`] module"]
pub type PRO_DRAM0_1 = crate::Reg<pro_dram0_1::PRO_DRAM0_1_SPEC>;
#[doc = "DBUS permission control register 1."]
pub mod pro_dram0_1;
#[doc = "PRO_DRAM0_2 (rw) register accessor: DBUS permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dram0_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dram0_2`] module"]
pub type PRO_DRAM0_2 = crate::Reg<pro_dram0_2::PRO_DRAM0_2_SPEC>;
#[doc = "DBUS permission control register 2."]
pub mod pro_dram0_2;
#[doc = "PRO_DRAM0_3 (rw) register accessor: DBUS permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dram0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dram0_3`] module"]
pub type PRO_DRAM0_3 = crate::Reg<pro_dram0_3::PRO_DRAM0_3_SPEC>;
#[doc = "DBUS permission control register 3."]
pub mod pro_dram0_3;
#[doc = "PRO_DRAM0_4 (r) register accessor: DBUS status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dram0_4`] module"]
pub type PRO_DRAM0_4 = crate::Reg<pro_dram0_4::PRO_DRAM0_4_SPEC>;
#[doc = "DBUS status register."]
pub mod pro_dram0_4;
#[doc = "PRO_DPORT_0 (rw) register accessor: PeriBus1 permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_0`] module"]
pub type PRO_DPORT_0 = crate::Reg<pro_dport_0::PRO_DPORT_0_SPEC>;
#[doc = "PeriBus1 permission control register 0."]
pub mod pro_dport_0;
#[doc = "PRO_DPORT_1 (rw) register accessor: PeriBus1 permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_1`] module"]
pub type PRO_DPORT_1 = crate::Reg<pro_dport_1::PRO_DPORT_1_SPEC>;
#[doc = "PeriBus1 permission control register 1."]
pub mod pro_dport_1;
#[doc = "PRO_DPORT_2 (rw) register accessor: PeriBus1 permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_2`] module"]
pub type PRO_DPORT_2 = crate::Reg<pro_dport_2::PRO_DPORT_2_SPEC>;
#[doc = "PeriBus1 permission control register 2."]
pub mod pro_dport_2;
#[doc = "PRO_DPORT_3 (rw) register accessor: PeriBus1 permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_3`] module"]
pub type PRO_DPORT_3 = crate::Reg<pro_dport_3::PRO_DPORT_3_SPEC>;
#[doc = "PeriBus1 permission control register 3."]
pub mod pro_dport_3;
#[doc = "PRO_DPORT_4 (rw) register accessor: PeriBus1 permission control register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_4`] module"]
pub type PRO_DPORT_4 = crate::Reg<pro_dport_4::PRO_DPORT_4_SPEC>;
#[doc = "PeriBus1 permission control register 4."]
pub mod pro_dport_4;
#[doc = "PRO_DPORT_5 (rw) register accessor: PeriBus1 permission control register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_5`] module"]
pub type PRO_DPORT_5 = crate::Reg<pro_dport_5::PRO_DPORT_5_SPEC>;
#[doc = "PeriBus1 permission control register 5."]
pub mod pro_dport_5;
#[doc = "PRO_DPORT_6 (rw) register accessor: PeriBus1 permission control register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_6`] module"]
pub type PRO_DPORT_6 = crate::Reg<pro_dport_6::PRO_DPORT_6_SPEC>;
#[doc = "PeriBus1 permission control register 6."]
pub mod pro_dport_6;
#[doc = "PRO_DPORT_7 (r) register accessor: PeriBus1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dport_7`] module"]
pub type PRO_DPORT_7 = crate::Reg<pro_dport_7::PRO_DPORT_7_SPEC>;
#[doc = "PeriBus1 status register."]
pub mod pro_dport_7;
#[doc = "PRO_AHB_0 (rw) register accessor: PeriBus2 permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ahb_0`] module"]
pub type PRO_AHB_0 = crate::Reg<pro_ahb_0::PRO_AHB_0_SPEC>;
#[doc = "PeriBus2 permission control register 0."]
pub mod pro_ahb_0;
#[doc = "PRO_AHB_1 (rw) register accessor: PeriBus2 permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ahb_1`] module"]
pub type PRO_AHB_1 = crate::Reg<pro_ahb_1::PRO_AHB_1_SPEC>;
#[doc = "PeriBus2 permission control register 1."]
pub mod pro_ahb_1;
#[doc = "PRO_AHB_2 (rw) register accessor: PeriBus2 permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ahb_2`] module"]
pub type PRO_AHB_2 = crate::Reg<pro_ahb_2::PRO_AHB_2_SPEC>;
#[doc = "PeriBus2 permission control register 2."]
pub mod pro_ahb_2;
#[doc = "PRO_AHB_3 (rw) register accessor: PeriBus2 permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ahb_3`] module"]
pub type PRO_AHB_3 = crate::Reg<pro_ahb_3::PRO_AHB_3_SPEC>;
#[doc = "PeriBus2 permission control register 3."]
pub mod pro_ahb_3;
#[doc = "PRO_AHB_4 (r) register accessor: PeriBus2 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_ahb_4`] module"]
pub type PRO_AHB_4 = crate::Reg<pro_ahb_4::PRO_AHB_4_SPEC>;
#[doc = "PeriBus2 status register."]
pub mod pro_ahb_4;
#[doc = "PRO_TRACE_0 (rw) register accessor: Trace memory permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_trace_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_trace_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_trace_0`] module"]
pub type PRO_TRACE_0 = crate::Reg<pro_trace_0::PRO_TRACE_0_SPEC>;
#[doc = "Trace memory permission control register 0."]
pub mod pro_trace_0;
#[doc = "PRO_TRACE_1 (rw) register accessor: Trace memory permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_trace_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_trace_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_trace_1`] module"]
pub type PRO_TRACE_1 = crate::Reg<pro_trace_1::PRO_TRACE_1_SPEC>;
#[doc = "Trace memory permission control register 1."]
pub mod pro_trace_1;
#[doc = "PRO_CACHE_0 (rw) register accessor: Cache permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_0`] module"]
pub type PRO_CACHE_0 = crate::Reg<pro_cache_0::PRO_CACHE_0_SPEC>;
#[doc = "Cache permission control register 0."]
pub mod pro_cache_0;
#[doc = "PRO_CACHE_1 (rw) register accessor: Cache permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_1`] module"]
pub type PRO_CACHE_1 = crate::Reg<pro_cache_1::PRO_CACHE_1_SPEC>;
#[doc = "Cache permission control register 1."]
pub mod pro_cache_1;
#[doc = "PRO_CACHE_2 (rw) register accessor: Cache permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_2`] module"]
pub type PRO_CACHE_2 = crate::Reg<pro_cache_2::PRO_CACHE_2_SPEC>;
#[doc = "Cache permission control register 2."]
pub mod pro_cache_2;
#[doc = "PRO_CACHE_3 (r) register accessor: Icache status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_3`] module"]
pub type PRO_CACHE_3 = crate::Reg<pro_cache_3::PRO_CACHE_3_SPEC>;
#[doc = "Icache status register."]
pub mod pro_cache_3;
#[doc = "PRO_CACHE_4 (r) register accessor: Dcache status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_4`] module"]
pub type PRO_CACHE_4 = crate::Reg<pro_cache_4::PRO_CACHE_4_SPEC>;
#[doc = "Dcache status register."]
pub mod pro_cache_4;
#[doc = "DMA_APB_I_0 (rw) register accessor: Internal DMA permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apb_i_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apb_i_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apb_i_0`] module"]
pub type DMA_APB_I_0 = crate::Reg<dma_apb_i_0::DMA_APB_I_0_SPEC>;
#[doc = "Internal DMA permission control register 0."]
pub mod dma_apb_i_0;
#[doc = "DMA_APB_I_1 (rw) register accessor: Internal DMA permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apb_i_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apb_i_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apb_i_1`] module"]
pub type DMA_APB_I_1 = crate::Reg<dma_apb_i_1::DMA_APB_I_1_SPEC>;
#[doc = "Internal DMA permission control register 1."]
pub mod dma_apb_i_1;
#[doc = "DMA_APB_I_2 (rw) register accessor: Internal DMA permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apb_i_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apb_i_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apb_i_2`] module"]
pub type DMA_APB_I_2 = crate::Reg<dma_apb_i_2::DMA_APB_I_2_SPEC>;
#[doc = "Internal DMA permission control register 2."]
pub mod dma_apb_i_2;
#[doc = "DMA_APB_I_3 (r) register accessor: Internal DMA status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apb_i_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_apb_i_3`] module"]
pub type DMA_APB_I_3 = crate::Reg<dma_apb_i_3::DMA_APB_I_3_SPEC>;
#[doc = "Internal DMA status register."]
pub mod dma_apb_i_3;
#[doc = "DMA_RX_I_0 (rw) register accessor: RX Copy DMA permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rx_i_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rx_i_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_i_0`] module"]
pub type DMA_RX_I_0 = crate::Reg<dma_rx_i_0::DMA_RX_I_0_SPEC>;
#[doc = "RX Copy DMA permission control register 0."]
pub mod dma_rx_i_0;
#[doc = "DMA_RX_I_1 (rw) register accessor: RX Copy DMA permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rx_i_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rx_i_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_i_1`] module"]
pub type DMA_RX_I_1 = crate::Reg<dma_rx_i_1::DMA_RX_I_1_SPEC>;
#[doc = "RX Copy DMA permission control register 1."]
pub mod dma_rx_i_1;
#[doc = "DMA_RX_I_2 (rw) register accessor: RX Copy DMA permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rx_i_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rx_i_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_i_2`] module"]
pub type DMA_RX_I_2 = crate::Reg<dma_rx_i_2::DMA_RX_I_2_SPEC>;
#[doc = "RX Copy DMA permission control register 2."]
pub mod dma_rx_i_2;
#[doc = "DMA_RX_I_3 (r) register accessor: RX Copy DMA status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rx_i_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_i_3`] module"]
pub type DMA_RX_I_3 = crate::Reg<dma_rx_i_3::DMA_RX_I_3_SPEC>;
#[doc = "RX Copy DMA status register."]
pub mod dma_rx_i_3;
#[doc = "DMA_TX_I_0 (rw) register accessor: TX Copy DMA permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_i_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tx_i_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_i_0`] module"]
pub type DMA_TX_I_0 = crate::Reg<dma_tx_i_0::DMA_TX_I_0_SPEC>;
#[doc = "TX Copy DMA permission control register 0."]
pub mod dma_tx_i_0;
#[doc = "DMA_TX_I_1 (rw) register accessor: TX Copy DMA permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_i_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tx_i_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_i_1`] module"]
pub type DMA_TX_I_1 = crate::Reg<dma_tx_i_1::DMA_TX_I_1_SPEC>;
#[doc = "TX Copy DMA permission control register 1."]
pub mod dma_tx_i_1;
#[doc = "DMA_TX_I_2 (rw) register accessor: TX Copy DMA permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_i_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tx_i_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_i_2`] module"]
pub type DMA_TX_I_2 = crate::Reg<dma_tx_i_2::DMA_TX_I_2_SPEC>;
#[doc = "TX Copy DMA permission control register 2."]
pub mod dma_tx_i_2;
#[doc = "DMA_TX_I_3 (r) register accessor: TX Copy DMA status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_i_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_i_3`] module"]
pub type DMA_TX_I_3 = crate::Reg<dma_tx_i_3::DMA_TX_I_3_SPEC>;
#[doc = "TX Copy DMA status register."]
pub mod dma_tx_i_3;
#[doc = "PRO_BOOT_LOCATION_0 (rw) register accessor: Boot permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_boot_location_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_boot_location_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_boot_location_0`] module"]
pub type PRO_BOOT_LOCATION_0 = crate::Reg<pro_boot_location_0::PRO_BOOT_LOCATION_0_SPEC>;
#[doc = "Boot permission control register 0."]
pub mod pro_boot_location_0;
#[doc = "PRO_BOOT_LOCATION_1 (rw) register accessor: Boot permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_boot_location_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_boot_location_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_boot_location_1`] module"]
pub type PRO_BOOT_LOCATION_1 = crate::Reg<pro_boot_location_1::PRO_BOOT_LOCATION_1_SPEC>;
#[doc = "Boot permission control register 1."]
pub mod pro_boot_location_1;
#[doc = "CACHE_SOURCE_0 (rw) register accessor: Cache access permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_source_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_source_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_source_0`] module"]
pub type CACHE_SOURCE_0 = crate::Reg<cache_source_0::CACHE_SOURCE_0_SPEC>;
#[doc = "Cache access permission control register 0."]
pub mod cache_source_0;
#[doc = "CACHE_SOURCE_1 (rw) register accessor: Cache access permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_source_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_source_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_source_1`] module"]
pub type CACHE_SOURCE_1 = crate::Reg<cache_source_1::CACHE_SOURCE_1_SPEC>;
#[doc = "Cache access permission control register 1."]
pub mod cache_source_1;
#[doc = "APB_PERIPHERAL_0 (rw) register accessor: Peripheral access permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_0`] module"]
pub type APB_PERIPHERAL_0 = crate::Reg<apb_peripheral_0::APB_PERIPHERAL_0_SPEC>;
#[doc = "Peripheral access permission control register 0."]
pub mod apb_peripheral_0;
#[doc = "APB_PERIPHERAL_1 (rw) register accessor: Peripheral access permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_1`] module"]
pub type APB_PERIPHERAL_1 = crate::Reg<apb_peripheral_1::APB_PERIPHERAL_1_SPEC>;
#[doc = "Peripheral access permission control register 1."]
pub mod apb_peripheral_1;
#[doc = "OCCUPY_0 (rw) register accessor: Occupy permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occupy_0`] module"]
pub type OCCUPY_0 = crate::Reg<occupy_0::OCCUPY_0_SPEC>;
#[doc = "Occupy permission control register 0."]
pub mod occupy_0;
#[doc = "OCCUPY_1 (rw) register accessor: Occupy permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occupy_1`] module"]
pub type OCCUPY_1 = crate::Reg<occupy_1::OCCUPY_1_SPEC>;
#[doc = "Occupy permission control register 1."]
pub mod occupy_1;
#[doc = "OCCUPY_2 (rw) register accessor: Occupy permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occupy_2`] module"]
pub type OCCUPY_2 = crate::Reg<occupy_2::OCCUPY_2_SPEC>;
#[doc = "Occupy permission control register 2."]
pub mod occupy_2;
#[doc = "OCCUPY_3 (rw) register accessor: Occupy permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occupy_3`] module"]
pub type OCCUPY_3 = crate::Reg<occupy_3::OCCUPY_3_SPEC>;
#[doc = "Occupy permission control register 3."]
pub mod occupy_3;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: Cache tag permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_0`] module"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "Cache tag permission control register 0."]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: Cache tag permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_1`] module"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "Cache tag permission control register 1."]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: Cache MMU permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_access_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_access_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_0`] module"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "Cache MMU permission control register 0."]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: Cache MMU permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_access_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_access_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_1`] module"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "Cache MMU permission control register 1."]
pub mod cache_mmu_access_1;
#[doc = "APB_PERIPHERAL_INTR (rw) register accessor: PeribBus2 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_intr`] module"]
pub type APB_PERIPHERAL_INTR = crate::Reg<apb_peripheral_intr::APB_PERIPHERAL_INTR_SPEC>;
#[doc = "PeribBus2 permission control register."]
pub mod apb_peripheral_intr;
#[doc = "APB_PERIPHERAL_STATUS (r) register accessor: PeribBus2 peripheral access status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_status`] module"]
pub type APB_PERIPHERAL_STATUS = crate::Reg<apb_peripheral_status::APB_PERIPHERAL_STATUS_SPEC>;
#[doc = "PeribBus2 peripheral access status register."]
pub mod apb_peripheral_status;
#[doc = "CPU_PERIPHERAL_INTR (rw) register accessor: PeribBus1 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peripheral_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peripheral_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peripheral_intr`] module"]
pub type CPU_PERIPHERAL_INTR = crate::Reg<cpu_peripheral_intr::CPU_PERIPHERAL_INTR_SPEC>;
#[doc = "PeribBus1 permission control register."]
pub mod cpu_peripheral_intr;
#[doc = "CPU_PERIPHERAL_STATUS (r) register accessor: PeribBus1 peripheral access status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peripheral_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peripheral_status`] module"]
pub type CPU_PERIPHERAL_STATUS = crate::Reg<cpu_peripheral_status::CPU_PERIPHERAL_STATUS_SPEC>;
#[doc = "PeribBus1 peripheral access status register."]
pub mod cpu_peripheral_status;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gate register of permission control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gate register of permission control."]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register."]
pub mod date;
