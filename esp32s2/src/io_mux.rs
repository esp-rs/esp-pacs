#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pin_ctrl: PIN_CTRL,
    gpio: [GPIO; 47],
    _reserved2: [u8; 0x3c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock output configuration register"]
    #[inline(always)]
    pub const fn pin_ctrl(&self) -> &PIN_CTRL {
        &self.pin_ctrl
    }
    #[doc = "0x04..0xc0 - Configuration register for pin GPIO%s"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0xc0 - Configuration register for pin GPIO%s"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PIN_CTRL (rw) register accessor: Clock output configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock output configuration register"]
pub mod pin_ctrl;
#[doc = "GPIO (rw) register accessor: Configuration register for pin GPIO%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`] module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "Configuration register for pin GPIO%s"]
pub mod gpio;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
