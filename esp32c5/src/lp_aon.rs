#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    store0: STORE0,
    store1: STORE1,
    store2: STORE2,
    store3: STORE3,
    store4: STORE4,
    store5: STORE5,
    store6: STORE6,
    store7: STORE7,
    store8: STORE8,
    store9: STORE9,
    gpio_mux: GPIO_MUX,
    gpio_hold0: GPIO_HOLD0,
    gpio_hold1: GPIO_HOLD1,
    sys_cfg: SYS_CFG,
    cpucore0_cfg: CPUCORE0_CFG,
    io_mux: IO_MUX,
    ext_wakeup_cntl: EXT_WAKEUP_CNTL,
    usb: USB,
    lpbus: LPBUS,
    sdio_active: SDIO_ACTIVE,
    lpcore: LPCORE,
    sar_cct: SAR_CCT,
    modem_bus: MODEM_BUS,
    _reserved23: [u8; 0x04],
    spram_ctrl: SPRAM_CTRL,
    sprf_ctrl: SPRF_CTRL,
    debug_sel0: DEBUG_SEL0,
    debug_sel1: DEBUG_SEL1,
    backup_dma_cfg0: BACKUP_DMA_CFG0,
    backup_dma_cfg1: BACKUP_DMA_CFG1,
    backup_dma_cfg2: BACKUP_DMA_CFG2,
    mem_ctrl: MEM_CTRL,
    _reserved31: [u8; 0x037c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - store the software massege0 in always-on field"]
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    #[doc = "0x04 - store the software massege1 in always-on field"]
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    #[doc = "0x08 - store the software massege2 in always-on field"]
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    #[doc = "0x0c - store the software massege3 in always-on field"]
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    #[doc = "0x10 - store the software massege4 in always-on field"]
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    #[doc = "0x14 - store the software massege5 in always-on field"]
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    #[doc = "0x18 - store the software massege6 in always-on field"]
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    #[doc = "0x1c - store the software massege7 in always-on field"]
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    #[doc = "0x20 - store the software massege8 in always-on field"]
    #[inline(always)]
    pub const fn store8(&self) -> &STORE8 {
        &self.store8
    }
    #[doc = "0x24 - store the software massege9 in always-on field"]
    #[inline(always)]
    pub const fn store9(&self) -> &STORE9 {
        &self.store9
    }
    #[doc = "0x28 - select the lp io controlled by hp iomux or lp iomux"]
    #[inline(always)]
    pub const fn gpio_mux(&self) -> &GPIO_MUX {
        &self.gpio_mux
    }
    #[doc = "0x2c - configure all io hold"]
    #[inline(always)]
    pub const fn gpio_hold0(&self) -> &GPIO_HOLD0 {
        &self.gpio_hold0
    }
    #[doc = "0x30 - reserved"]
    #[inline(always)]
    pub const fn gpio_hold1(&self) -> &GPIO_HOLD1 {
        &self.gpio_hold1
    }
    #[doc = "0x34 - configure system register"]
    #[inline(always)]
    pub const fn sys_cfg(&self) -> &SYS_CFG {
        &self.sys_cfg
    }
    #[doc = "0x38 - configure core reset register"]
    #[inline(always)]
    pub const fn cpucore0_cfg(&self) -> &CPUCORE0_CFG {
        &self.cpucore0_cfg
    }
    #[doc = "0x3c - configure hp iomux reset bypass"]
    #[inline(always)]
    pub const fn io_mux(&self) -> &IO_MUX {
        &self.io_mux
    }
    #[doc = "0x40 - configure alwayson external io wakeup"]
    #[inline(always)]
    pub const fn ext_wakeup_cntl(&self) -> &EXT_WAKEUP_CNTL {
        &self.ext_wakeup_cntl
    }
    #[doc = "0x44 - configure usb reset bypass"]
    #[inline(always)]
    pub const fn usb(&self) -> &USB {
        &self.usb
    }
    #[doc = "0x48 - Select lp memory bus"]
    #[inline(always)]
    pub const fn lpbus(&self) -> &LPBUS {
        &self.lpbus
    }
    #[doc = "0x4c - configure sdio act dnum"]
    #[inline(always)]
    pub const fn sdio_active(&self) -> &SDIO_ACTIVE {
        &self.sdio_active
    }
    #[doc = "0x50 - configure etm wakeup register"]
    #[inline(always)]
    pub const fn lpcore(&self) -> &LPCORE {
        &self.lpcore
    }
    #[doc = "0x54 - configure sar cct"]
    #[inline(always)]
    pub const fn sar_cct(&self) -> &SAR_CCT {
        &self.sar_cct
    }
    #[doc = "0x58 - configure modem sync bridge"]
    #[inline(always)]
    pub const fn modem_bus(&self) -> &MODEM_BUS {
        &self.modem_bus
    }
    #[doc = "0x60 - configure lp memory power status"]
    #[inline(always)]
    pub const fn spram_ctrl(&self) -> &SPRAM_CTRL {
        &self.spram_ctrl
    }
    #[doc = "0x64 - configure memory in lp system power status"]
    #[inline(always)]
    pub const fn sprf_ctrl(&self) -> &SPRF_CTRL {
        &self.sprf_ctrl
    }
    #[doc = "0x68 - reserved"]
    #[inline(always)]
    pub const fn debug_sel0(&self) -> &DEBUG_SEL0 {
        &self.debug_sel0
    }
    #[doc = "0x6c - need des"]
    #[inline(always)]
    pub const fn debug_sel1(&self) -> &DEBUG_SEL1 {
        &self.debug_sel1
    }
    #[doc = "0x70 - configure regdma always on register"]
    #[inline(always)]
    pub const fn backup_dma_cfg0(&self) -> &BACKUP_DMA_CFG0 {
        &self.backup_dma_cfg0
    }
    #[doc = "0x74 - configure regdma always on register"]
    #[inline(always)]
    pub const fn backup_dma_cfg1(&self) -> &BACKUP_DMA_CFG1 {
        &self.backup_dma_cfg1
    }
    #[doc = "0x78 - configure regdma always on register"]
    #[inline(always)]
    pub const fn backup_dma_cfg2(&self) -> &BACKUP_DMA_CFG2 {
        &self.backup_dma_cfg2
    }
    #[doc = "0x7c - configure rmemory power in lp system register"]
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MEM_CTRL {
        &self.mem_ctrl
    }
    #[doc = "0x3fc - reserved"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "STORE0 (rw) register accessor: store the software massege0 in always-on field\n\nYou can [`read`](crate::Reg::read) this register and get [`store0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "store the software massege0 in always-on field"]
pub mod store0;
pub use store0 as store1;
pub use store0 as store2;
pub use store0 as store3;
pub use store0 as store4;
pub use store0 as store5;
pub use store0 as store6;
pub use store0 as store7;
pub use store0 as store8;
pub use store0 as store9;
pub use STORE0 as STORE1;
pub use STORE0 as STORE2;
pub use STORE0 as STORE3;
pub use STORE0 as STORE4;
pub use STORE0 as STORE5;
pub use STORE0 as STORE6;
pub use STORE0 as STORE7;
pub use STORE0 as STORE8;
pub use STORE0 as STORE9;
#[doc = "GPIO_MUX (rw) register accessor: select the lp io controlled by hp iomux or lp iomux\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_mux`] module"]
pub type GPIO_MUX = crate::Reg<gpio_mux::GPIO_MUX_SPEC>;
#[doc = "select the lp io controlled by hp iomux or lp iomux"]
pub mod gpio_mux;
#[doc = "GPIO_HOLD0 (rw) register accessor: configure all io hold\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_hold0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hold0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_hold0`] module"]
pub type GPIO_HOLD0 = crate::Reg<gpio_hold0::GPIO_HOLD0_SPEC>;
#[doc = "configure all io hold"]
pub mod gpio_hold0;
#[doc = "GPIO_HOLD1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_hold1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hold1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_hold1`] module"]
pub type GPIO_HOLD1 = crate::Reg<gpio_hold1::GPIO_HOLD1_SPEC>;
#[doc = "reserved"]
pub mod gpio_hold1;
#[doc = "SYS_CFG (rw) register accessor: configure system register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_cfg`] module"]
pub type SYS_CFG = crate::Reg<sys_cfg::SYS_CFG_SPEC>;
#[doc = "configure system register"]
pub mod sys_cfg;
#[doc = "CPUCORE0_CFG (rw) register accessor: configure core reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucore0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucore0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucore0_cfg`] module"]
pub type CPUCORE0_CFG = crate::Reg<cpucore0_cfg::CPUCORE0_CFG_SPEC>;
#[doc = "configure core reset register"]
pub mod cpucore0_cfg;
#[doc = "IO_MUX (rw) register accessor: configure hp iomux reset bypass\n\nYou can [`read`](crate::Reg::read) this register and get [`io_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_mux`] module"]
pub type IO_MUX = crate::Reg<io_mux::IO_MUX_SPEC>;
#[doc = "configure hp iomux reset bypass"]
pub mod io_mux;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: configure alwayson external io wakeup\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_cntl`] module"]
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
#[doc = "configure alwayson external io wakeup"]
pub mod ext_wakeup_cntl;
#[doc = "USB (rw) register accessor: configure usb reset bypass\n\nYou can [`read`](crate::Reg::read) this register and get [`usb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb`] module"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "configure usb reset bypass"]
pub mod usb;
#[doc = "LPBUS (rw) register accessor: Select lp memory bus\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbus`] module"]
pub type LPBUS = crate::Reg<lpbus::LPBUS_SPEC>;
#[doc = "Select lp memory bus"]
pub mod lpbus;
#[doc = "SDIO_ACTIVE (rw) register accessor: configure sdio act dnum\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_active::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_active::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_active`] module"]
pub type SDIO_ACTIVE = crate::Reg<sdio_active::SDIO_ACTIVE_SPEC>;
#[doc = "configure sdio act dnum"]
pub mod sdio_active;
#[doc = "LPCORE (rw) register accessor: configure etm wakeup register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcore::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcore::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcore`] module"]
pub type LPCORE = crate::Reg<lpcore::LPCORE_SPEC>;
#[doc = "configure etm wakeup register"]
pub mod lpcore;
#[doc = "SAR_CCT (rw) register accessor: configure sar cct\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cct`] module"]
pub type SAR_CCT = crate::Reg<sar_cct::SAR_CCT_SPEC>;
#[doc = "configure sar cct"]
pub mod sar_cct;
#[doc = "MODEM_BUS (rw) register accessor: configure modem sync bridge\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_bus`] module"]
pub type MODEM_BUS = crate::Reg<modem_bus::MODEM_BUS_SPEC>;
#[doc = "configure modem sync bridge"]
pub mod modem_bus;
#[doc = "SPRAM_CTRL (rw) register accessor: configure lp memory power status\n\nYou can [`read`](crate::Reg::read) this register and get [`spram_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spram_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spram_ctrl`] module"]
pub type SPRAM_CTRL = crate::Reg<spram_ctrl::SPRAM_CTRL_SPEC>;
#[doc = "configure lp memory power status"]
pub mod spram_ctrl;
#[doc = "SPRF_CTRL (rw) register accessor: configure memory in lp system power status\n\nYou can [`read`](crate::Reg::read) this register and get [`sprf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprf_ctrl`] module"]
pub type SPRF_CTRL = crate::Reg<sprf_ctrl::SPRF_CTRL_SPEC>;
#[doc = "configure memory in lp system power status"]
pub mod sprf_ctrl;
#[doc = "DEBUG_SEL0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel0`] module"]
pub type DEBUG_SEL0 = crate::Reg<debug_sel0::DEBUG_SEL0_SPEC>;
#[doc = "reserved"]
pub mod debug_sel0;
#[doc = "DEBUG_SEL1 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel1`] module"]
pub type DEBUG_SEL1 = crate::Reg<debug_sel1::DEBUG_SEL1_SPEC>;
#[doc = "need des"]
pub mod debug_sel1;
#[doc = "BACKUP_DMA_CFG0 (rw) register accessor: configure regdma always on register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg0`] module"]
pub type BACKUP_DMA_CFG0 = crate::Reg<backup_dma_cfg0::BACKUP_DMA_CFG0_SPEC>;
#[doc = "configure regdma always on register"]
pub mod backup_dma_cfg0;
#[doc = "BACKUP_DMA_CFG1 (rw) register accessor: configure regdma always on register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg1`] module"]
pub type BACKUP_DMA_CFG1 = crate::Reg<backup_dma_cfg1::BACKUP_DMA_CFG1_SPEC>;
#[doc = "configure regdma always on register"]
pub mod backup_dma_cfg1;
#[doc = "BACKUP_DMA_CFG2 (rw) register accessor: configure regdma always on register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg2`] module"]
pub type BACKUP_DMA_CFG2 = crate::Reg<backup_dma_cfg2::BACKUP_DMA_CFG2_SPEC>;
#[doc = "configure regdma always on register"]
pub mod backup_dma_cfg2;
#[doc = "MEM_CTRL (rw) register accessor: configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl`] module"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "configure rmemory power in lp system register"]
pub mod mem_ctrl;
#[doc = "DATE (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "reserved"]
pub mod date;
