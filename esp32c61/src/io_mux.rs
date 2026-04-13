#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio: [GPIO; 30],
    _reserved1: [u8; 0x0184],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x78 - IO_MUX Configure Registers for different pads"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x78 - IO_MUX Configure Registers for different pads"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0x1fc - IO_MUX Version Control Register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "GPIO (rw) register accessor: IO_MUX Configure Registers for different pads\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`] module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "IO_MUX Configure Registers for different pads"]
pub mod gpio;
pub use crate::dma::{date, DATE};
