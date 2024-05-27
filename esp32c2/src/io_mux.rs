#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pin_ctrl: PIN_CTRL,
    gpio: [GPIO; 21],
    _reserved2: [u8; 0xa4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Clock Output Configuration Register
    #[inline(always)]
    pub const fn pin_ctrl(&self) -> &PIN_CTRL {
        &self.pin_ctrl
    }
    ///0x04..0x58 - IO MUX Configure Register for pad XTAL_32K_P
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    ///Iterator for array of:
    ///0x04..0x58 - IO MUX Configure Register for pad XTAL_32K_P
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    ///0xfc - IO MUX Version Control Register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PIN_CTRL (rw) register accessor: Clock Output Configuration Register

You can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin_ctrl`] module*/
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
///Clock Output Configuration Register
pub mod pin_ctrl;
/**GPIO (rw) register accessor: IO MUX Configure Register for pad XTAL_32K_P

You can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio`] module*/
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
///IO MUX Configure Register for pad XTAL_32K_P
pub mod gpio;
/**DATE (rw) register accessor: IO MUX Version Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///IO MUX Version Control Register
pub mod date;
