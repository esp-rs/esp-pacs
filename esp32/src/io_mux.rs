#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04 - "]
    pub gpio36: GPIO36,
    #[doc = "0x08 - "]
    pub gpio37: GPIO37,
    #[doc = "0x0c - "]
    pub gpio38: GPIO38,
    #[doc = "0x10 - "]
    pub gpio39: GPIO39,
    #[doc = "0x14 - "]
    pub gpio34: GPIO34,
    #[doc = "0x18 - "]
    pub gpio35: GPIO35,
    #[doc = "0x1c - "]
    pub gpio32: GPIO32,
    #[doc = "0x20 - "]
    pub gpio33: GPIO33,
    #[doc = "0x24 - "]
    pub gpio25: GPIO25,
    #[doc = "0x28 - "]
    pub gpio26: GPIO26,
    #[doc = "0x2c - "]
    pub gpio27: GPIO27,
    #[doc = "0x30 - "]
    pub gpio14: GPIO14,
    #[doc = "0x34 - "]
    pub gpio12: GPIO12,
    #[doc = "0x38 - "]
    pub gpio13: GPIO13,
    #[doc = "0x3c - "]
    pub gpio15: GPIO15,
    #[doc = "0x40 - "]
    pub gpio2: GPIO2,
    #[doc = "0x44 - "]
    pub gpio0: GPIO0,
    #[doc = "0x48 - "]
    pub gpio4: GPIO4,
    #[doc = "0x4c - "]
    pub gpio16: GPIO16,
    #[doc = "0x50 - "]
    pub gpio17: GPIO17,
    #[doc = "0x54 - "]
    pub gpio9: GPIO9,
    #[doc = "0x58 - "]
    pub gpio10: GPIO10,
    #[doc = "0x5c - "]
    pub gpio11: GPIO11,
    #[doc = "0x60 - "]
    pub gpio6: GPIO6,
    #[doc = "0x64 - "]
    pub gpio7: GPIO7,
    #[doc = "0x68 - "]
    pub gpio8: GPIO8,
    #[doc = "0x6c - "]
    pub gpio5: GPIO5,
    #[doc = "0x70 - "]
    pub gpio18: GPIO18,
    #[doc = "0x74 - "]
    pub gpio19: GPIO19,
    #[doc = "0x78 - "]
    pub gpio20: GPIO20,
    #[doc = "0x7c - "]
    pub gpio21: GPIO21,
    #[doc = "0x80 - "]
    pub gpio22: GPIO22,
    #[doc = "0x84 - "]
    pub gpio3: GPIO3,
    #[doc = "0x88 - "]
    pub gpio1: GPIO1,
    #[doc = "0x8c - "]
    pub gpio23: GPIO23,
    #[doc = "0x90 - "]
    pub gpio24: GPIO24,
}
#[doc = "PIN_CTRL (rw) register accessor: an alias for `Reg<PIN_CTRL_SPEC>`"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = ""]
pub mod pin_ctrl;
#[doc = "GPIO36 (rw) register accessor: an alias for `Reg<GPIO36_SPEC>`"]
pub type GPIO36 = crate::Reg<gpio36::GPIO36_SPEC>;
#[doc = ""]
pub mod gpio36;
#[doc = "GPIO37 (rw) register accessor: an alias for `Reg<GPIO37_SPEC>`"]
pub type GPIO37 = crate::Reg<gpio37::GPIO37_SPEC>;
#[doc = ""]
pub mod gpio37;
#[doc = "GPIO38 (rw) register accessor: an alias for `Reg<GPIO38_SPEC>`"]
pub type GPIO38 = crate::Reg<gpio38::GPIO38_SPEC>;
#[doc = ""]
pub mod gpio38;
#[doc = "GPIO39 (rw) register accessor: an alias for `Reg<GPIO39_SPEC>`"]
pub type GPIO39 = crate::Reg<gpio39::GPIO39_SPEC>;
#[doc = ""]
pub mod gpio39;
#[doc = "GPIO34 (rw) register accessor: an alias for `Reg<GPIO34_SPEC>`"]
pub type GPIO34 = crate::Reg<gpio34::GPIO34_SPEC>;
#[doc = ""]
pub mod gpio34;
#[doc = "GPIO35 (rw) register accessor: an alias for `Reg<GPIO35_SPEC>`"]
pub type GPIO35 = crate::Reg<gpio35::GPIO35_SPEC>;
#[doc = ""]
pub mod gpio35;
#[doc = "GPIO32 (rw) register accessor: an alias for `Reg<GPIO32_SPEC>`"]
pub type GPIO32 = crate::Reg<gpio32::GPIO32_SPEC>;
#[doc = ""]
pub mod gpio32;
#[doc = "GPIO33 (rw) register accessor: an alias for `Reg<GPIO33_SPEC>`"]
pub type GPIO33 = crate::Reg<gpio33::GPIO33_SPEC>;
#[doc = ""]
pub mod gpio33;
#[doc = "GPIO25 (rw) register accessor: an alias for `Reg<GPIO25_SPEC>`"]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = ""]
pub mod gpio25;
#[doc = "GPIO26 (rw) register accessor: an alias for `Reg<GPIO26_SPEC>`"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = ""]
pub mod gpio26;
#[doc = "GPIO27 (rw) register accessor: an alias for `Reg<GPIO27_SPEC>`"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = ""]
pub mod gpio27;
#[doc = "GPIO14 (rw) register accessor: an alias for `Reg<GPIO14_SPEC>`"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = ""]
pub mod gpio14;
#[doc = "GPIO12 (rw) register accessor: an alias for `Reg<GPIO12_SPEC>`"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = ""]
pub mod gpio12;
#[doc = "GPIO13 (rw) register accessor: an alias for `Reg<GPIO13_SPEC>`"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = ""]
pub mod gpio13;
#[doc = "GPIO15 (rw) register accessor: an alias for `Reg<GPIO15_SPEC>`"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = ""]
pub mod gpio15;
#[doc = "GPIO2 (rw) register accessor: an alias for `Reg<GPIO2_SPEC>`"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = ""]
pub mod gpio2;
#[doc = "GPIO0 (rw) register accessor: an alias for `Reg<GPIO0_SPEC>`"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = ""]
pub mod gpio0;
#[doc = "GPIO4 (rw) register accessor: an alias for `Reg<GPIO4_SPEC>`"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = ""]
pub mod gpio4;
#[doc = "GPIO16 (rw) register accessor: an alias for `Reg<GPIO16_SPEC>`"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = ""]
pub mod gpio16;
#[doc = "GPIO17 (rw) register accessor: an alias for `Reg<GPIO17_SPEC>`"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = ""]
pub mod gpio17;
#[doc = "GPIO9 (rw) register accessor: an alias for `Reg<GPIO9_SPEC>`"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = ""]
pub mod gpio9;
#[doc = "GPIO10 (rw) register accessor: an alias for `Reg<GPIO10_SPEC>`"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = ""]
pub mod gpio10;
#[doc = "GPIO11 (rw) register accessor: an alias for `Reg<GPIO11_SPEC>`"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = ""]
pub mod gpio11;
#[doc = "GPIO6 (rw) register accessor: an alias for `Reg<GPIO6_SPEC>`"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = ""]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: an alias for `Reg<GPIO7_SPEC>`"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = ""]
pub mod gpio7;
#[doc = "GPIO8 (rw) register accessor: an alias for `Reg<GPIO8_SPEC>`"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = ""]
pub mod gpio8;
#[doc = "GPIO5 (rw) register accessor: an alias for `Reg<GPIO5_SPEC>`"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = ""]
pub mod gpio5;
#[doc = "GPIO18 (rw) register accessor: an alias for `Reg<GPIO18_SPEC>`"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = ""]
pub mod gpio18;
#[doc = "GPIO19 (rw) register accessor: an alias for `Reg<GPIO19_SPEC>`"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = ""]
pub mod gpio19;
#[doc = "GPIO20 (rw) register accessor: an alias for `Reg<GPIO20_SPEC>`"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = ""]
pub mod gpio20;
#[doc = "GPIO21 (rw) register accessor: an alias for `Reg<GPIO21_SPEC>`"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = ""]
pub mod gpio21;
#[doc = "GPIO22 (rw) register accessor: an alias for `Reg<GPIO22_SPEC>`"]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = ""]
pub mod gpio22;
#[doc = "GPIO3 (rw) register accessor: an alias for `Reg<GPIO3_SPEC>`"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = ""]
pub mod gpio3;
#[doc = "GPIO1 (rw) register accessor: an alias for `Reg<GPIO1_SPEC>`"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = ""]
pub mod gpio1;
#[doc = "GPIO23 (rw) register accessor: an alias for `Reg<GPIO23_SPEC>`"]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = ""]
pub mod gpio23;
#[doc = "GPIO24 (rw) register accessor: an alias for `Reg<GPIO24_SPEC>`"]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = ""]
pub mod gpio24;
