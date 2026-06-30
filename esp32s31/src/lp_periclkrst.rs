#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    cpu_ctrl: CPU_CTRL,
    rom_ctrl: ROM_CTRL,
    ram_ctrl: RAM_CTRL,
    uart_ctrl: UART_CTRL,
    i2c_ctrl: I2C_CTRL,
    i2cmst_ctrl: I2CMST_CTRL,
    spi_ctrl: SPI_CTRL,
    adc_ctrl: ADC_CTRL,
    efuse_ctrl: EFUSE_CTRL,
    intr_ctrl: INTR_CTRL,
    touch_ctrl: TOUCH_CTRL,
    tsens_ctrl: TSENS_CTRL,
    iomux_ctrl: IOMUX_CTRL,
    mailbox: MAILBOX,
    rng_ctrl: RNG_CTRL,
    uart_misc_ctrl: UART_MISC_CTRL,
    cpu: CPU,
    ahb_dma_ctrl: AHB_DMA_CTRL,
    dac_ctrl: DAC_CTRL,
    dm_ctrl: DM_CTRL,
    _reserved20: [u8; 0x03a8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn cpu_ctrl(&self) -> &CPU_CTRL {
        &self.cpu_ctrl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn rom_ctrl(&self) -> &ROM_CTRL {
        &self.rom_ctrl
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn ram_ctrl(&self) -> &RAM_CTRL {
        &self.ram_ctrl
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn uart_ctrl(&self) -> &UART_CTRL {
        &self.uart_ctrl
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn i2c_ctrl(&self) -> &I2C_CTRL {
        &self.i2c_ctrl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn i2cmst_ctrl(&self) -> &I2CMST_CTRL {
        &self.i2cmst_ctrl
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn spi_ctrl(&self) -> &SPI_CTRL {
        &self.spi_ctrl
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn adc_ctrl(&self) -> &ADC_CTRL {
        &self.adc_ctrl
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn efuse_ctrl(&self) -> &EFUSE_CTRL {
        &self.efuse_ctrl
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn intr_ctrl(&self) -> &INTR_CTRL {
        &self.intr_ctrl
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn touch_ctrl(&self) -> &TOUCH_CTRL {
        &self.touch_ctrl
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn tsens_ctrl(&self) -> &TSENS_CTRL {
        &self.tsens_ctrl
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn iomux_ctrl(&self) -> &IOMUX_CTRL {
        &self.iomux_ctrl
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn mailbox(&self) -> &MAILBOX {
        &self.mailbox
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn rng_ctrl(&self) -> &RNG_CTRL {
        &self.rng_ctrl
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn uart_misc_ctrl(&self) -> &UART_MISC_CTRL {
        &self.uart_misc_ctrl
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn cpu(&self) -> &CPU {
        &self.cpu
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn ahb_dma_ctrl(&self) -> &AHB_DMA_CTRL {
        &self.ahb_dma_ctrl
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn dac_ctrl(&self) -> &DAC_CTRL {
        &self.dac_ctrl
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn dm_ctrl(&self) -> &DM_CTRL {
        &self.dm_ctrl
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CPU_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_ctrl`] module"]
pub type CPU_CTRL = crate::Reg<cpu_ctrl::CPU_CTRL_SPEC>;
#[doc = "need_des"]
pub mod cpu_ctrl;
#[doc = "ROM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_ctrl`] module"]
pub type ROM_CTRL = crate::Reg<rom_ctrl::ROM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod rom_ctrl;
#[doc = "RAM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_ctrl`] module"]
pub type RAM_CTRL = crate::Reg<ram_ctrl::RAM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod ram_ctrl;
#[doc = "UART_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ctrl`] module"]
pub type UART_CTRL = crate::Reg<uart_ctrl::UART_CTRL_SPEC>;
#[doc = "need_des"]
pub mod uart_ctrl;
#[doc = "I2C_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ctrl`] module"]
pub type I2C_CTRL = crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>;
#[doc = "need_des"]
pub mod i2c_ctrl;
#[doc = "I2CMST_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cmst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cmst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cmst_ctrl`] module"]
pub type I2CMST_CTRL = crate::Reg<i2cmst_ctrl::I2CMST_CTRL_SPEC>;
#[doc = "need_des"]
pub mod i2cmst_ctrl;
#[doc = "SPI_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl`] module"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "need_des"]
pub mod spi_ctrl;
#[doc = "ADC_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctrl`] module"]
pub type ADC_CTRL = crate::Reg<adc_ctrl::ADC_CTRL_SPEC>;
#[doc = "need_des"]
pub mod adc_ctrl;
#[doc = "EFUSE_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_ctrl`] module"]
pub type EFUSE_CTRL = crate::Reg<efuse_ctrl::EFUSE_CTRL_SPEC>;
#[doc = "need_des"]
pub mod efuse_ctrl;
#[doc = "INTR_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_ctrl`] module"]
pub type INTR_CTRL = crate::Reg<intr_ctrl::INTR_CTRL_SPEC>;
#[doc = "need_des"]
pub mod intr_ctrl;
#[doc = "TOUCH_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_ctrl`] module"]
pub type TOUCH_CTRL = crate::Reg<touch_ctrl::TOUCH_CTRL_SPEC>;
#[doc = "need_des"]
pub mod touch_ctrl;
#[doc = "TSENS_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_ctrl`] module"]
pub type TSENS_CTRL = crate::Reg<tsens_ctrl::TSENS_CTRL_SPEC>;
#[doc = "need_des"]
pub mod tsens_ctrl;
#[doc = "IOMUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_ctrl`] module"]
pub type IOMUX_CTRL = crate::Reg<iomux_ctrl::IOMUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod iomux_ctrl;
#[doc = "MAILBOX (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox`] module"]
pub type MAILBOX = crate::Reg<mailbox::MAILBOX_SPEC>;
#[doc = "need_des"]
pub mod mailbox;
#[doc = "RNG_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_ctrl`] module"]
pub type RNG_CTRL = crate::Reg<rng_ctrl::RNG_CTRL_SPEC>;
#[doc = "need_des"]
pub mod rng_ctrl;
#[doc = "UART_MISC_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_misc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_misc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_misc_ctrl`] module"]
pub type UART_MISC_CTRL = crate::Reg<uart_misc_ctrl::UART_MISC_CTRL_SPEC>;
#[doc = "need_des"]
pub mod uart_misc_ctrl;
#[doc = "CPU (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu`] module"]
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "AHB_DMA_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_ctrl`] module"]
pub type AHB_DMA_CTRL = crate::Reg<ahb_dma_ctrl::AHB_DMA_CTRL_SPEC>;
#[doc = "need_des"]
pub mod ahb_dma_ctrl;
#[doc = "DAC_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_ctrl`] module"]
pub type DAC_CTRL = crate::Reg<dac_ctrl::DAC_CTRL_SPEC>;
#[doc = "need_des"]
pub mod dac_ctrl;
#[doc = "DM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dm_ctrl`] module"]
pub type DM_CTRL = crate::Reg<dm_ctrl::DM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod dm_ctrl;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
