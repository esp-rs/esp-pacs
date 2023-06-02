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
#[doc = "PIN_CTRL (rw) register accessor: an alias for `Reg<PIN_CTRL_SPEC>`"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod pin_ctrl;
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "IO MUX Configure Register for pad GPIO0"]
pub mod gpio;
#[doc = "MODEM_DIAG_EN (rw) register accessor: an alias for `Reg<MODEM_DIAG_EN_SPEC>`"]
pub type MODEM_DIAG_EN = crate::Reg<modem_diag_en::MODEM_DIAG_EN_SPEC>;
#[doc = "GPIO MATRIX Configure Register for modem diag"]
pub mod modem_diag_en;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "IO MUX Version Control Register"]
pub mod date;
