#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Output Configuration Register"]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04..0x58 - IO MUX Configure Register for pad XTAL_32K_P"]
    pub gpio: [GPIO; 21],
    _reserved2: [u8; 0xa4],
    #[doc = "0xfc - IO MUX Version Control Register"]
    pub date: DATE,
}
#[doc = "PIN_CTRL (rw) register accessor: an alias for `Reg<PIN_CTRL_SPEC>`"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod pin_ctrl;
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "IO MUX Configure Register for pad XTAL_32K_P"]
pub mod gpio;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "IO MUX Version Control Register"]
pub mod date;
