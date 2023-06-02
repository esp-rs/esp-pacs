#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Output Configuration Register"]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04..0xc8 - IO MUX Configure Register for pad GPIO0"]
    pub gpio: [GPIO; 49],
    _reserved2: [u8; 0x34],
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
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "IO MUX Version Control Register"]
pub mod date;
