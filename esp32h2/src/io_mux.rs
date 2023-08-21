#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Output Configuration Register"]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04..0x74 - IO MUX Configure Register for pad GPIO0"]
    pub gpio: [GPIO; 28],
    _reserved2: [u8; 0x48],
    #[doc = "0xbc - GPIO MATRIX Configure Register for modem diag"]
    pub modem_diag_en: MODEM_DIAG_EN,
    _reserved3: [u8; 0x3c],
    #[doc = "0xfc - IO MUX Version Control Register"]
    pub date: DATE,
}
#[doc = "PIN_CTRL (rw) register accessor: Clock Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod pin_ctrl;
#[doc = "GPIO (rw) register accessor: IO MUX Configure Register for pad GPIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio`] module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "IO MUX Configure Register for pad GPIO0"]
pub mod gpio;
#[doc = "MODEM_DIAG_EN (rw) register accessor: GPIO MATRIX Configure Register for modem diag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_diag_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_diag_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modem_diag_en`] module"]
pub type MODEM_DIAG_EN = crate::Reg<modem_diag_en::MODEM_DIAG_EN_SPEC>;
#[doc = "GPIO MATRIX Configure Register for modem diag"]
pub mod modem_diag_en;
#[doc = "DATE (rw) register accessor: IO MUX Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "IO MUX Version Control Register"]
pub mod date;
