#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio: [GPIO; 29],
    _reserved1: [u8; 0x0188],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x74 - IO MUX configuration register for GPIO%s"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x74 - IO MUX configuration register for GPIO%s"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0x1fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "GPIO (rw) register accessor: IO MUX configuration register for GPIO%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`] module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "IO MUX configuration register for GPIO%s"]
pub mod gpio;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
