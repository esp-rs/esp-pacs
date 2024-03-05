#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pin_ctrl: PIN_CTRL,
    gpio0: GPIO0,
    gpio1: GPIO1,
    gpio2: GPIO2,
    gpio3: GPIO3,
    gpio4: GPIO4,
    gpio5: GPIO5,
    gpio6: GPIO6,
    gpio7: GPIO7,
    gpio8: GPIO8,
    gpio9: GPIO9,
    gpio10: GPIO10,
    gpio11: GPIO11,
    gpio12: GPIO12,
    gpio13: GPIO13,
    gpio14: GPIO14,
    gpio15: GPIO15,
    gpio16: GPIO16,
    gpio17: GPIO17,
    gpio18: GPIO18,
    gpio19: GPIO19,
    gpio20: GPIO20,
    gpio21: GPIO21,
    _reserved23: [u8; 0x10],
    gpio26: GPIO26,
    gpio27: GPIO27,
    gpio28: GPIO28,
    gpio29: GPIO29,
    gpio30: GPIO30,
    gpio31: GPIO31,
    gpio32: GPIO32,
    gpio33: GPIO33,
    gpio34: GPIO34,
    gpio35: GPIO35,
    gpio36: GPIO36,
    gpio37: GPIO37,
    gpio38: GPIO38,
    gpio39: GPIO39,
    gpio40: GPIO40,
    gpio41: GPIO41,
    gpio42: GPIO42,
    gpio43: GPIO43,
    gpio44: GPIO44,
    gpio45: GPIO45,
    gpio46: GPIO46,
    _reserved44: [u8; 0x3c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock output configuration register"]
    #[inline(always)]
    pub const fn pin_ctrl(&self) -> &PIN_CTRL {
        &self.pin_ctrl
    }
    #[doc = "0x04 - Configuration register for pin GPIO0"]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0x08 - Configuration register for pin GPIO1"]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0x0c - Configuration register for pin GPIO2"]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0x10 - Configuration register for pin GPIO3"]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0x14 - Configuration register for pin GPIO4"]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0x18 - Configuration register for pin GPIO5"]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0x1c - Configuration register for pin GPIO6"]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0x20 - Configuration register for pin GPIO7"]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0x24 - Configuration register for pin GPIO8"]
    #[inline(always)]
    pub const fn gpio8(&self) -> &GPIO8 {
        &self.gpio8
    }
    #[doc = "0x28 - Configuration register for pin GPIO9"]
    #[inline(always)]
    pub const fn gpio9(&self) -> &GPIO9 {
        &self.gpio9
    }
    #[doc = "0x2c - Configuration register for pin GPIO10"]
    #[inline(always)]
    pub const fn gpio10(&self) -> &GPIO10 {
        &self.gpio10
    }
    #[doc = "0x30 - Configuration register for pin GPIO11"]
    #[inline(always)]
    pub const fn gpio11(&self) -> &GPIO11 {
        &self.gpio11
    }
    #[doc = "0x34 - Configuration register for pin GPIO12"]
    #[inline(always)]
    pub const fn gpio12(&self) -> &GPIO12 {
        &self.gpio12
    }
    #[doc = "0x38 - Configuration register for pin GPIO13"]
    #[inline(always)]
    pub const fn gpio13(&self) -> &GPIO13 {
        &self.gpio13
    }
    #[doc = "0x3c - Configuration register for pin GPIO14"]
    #[inline(always)]
    pub const fn gpio14(&self) -> &GPIO14 {
        &self.gpio14
    }
    #[doc = "0x40 - Configuration register for pin GPIO15"]
    #[inline(always)]
    pub const fn gpio15(&self) -> &GPIO15 {
        &self.gpio15
    }
    #[doc = "0x44 - Configuration register for pin GPIO16"]
    #[inline(always)]
    pub const fn gpio16(&self) -> &GPIO16 {
        &self.gpio16
    }
    #[doc = "0x48 - Configuration register for pin GPIO17"]
    #[inline(always)]
    pub const fn gpio17(&self) -> &GPIO17 {
        &self.gpio17
    }
    #[doc = "0x4c - Configuration register for pin GPIO18"]
    #[inline(always)]
    pub const fn gpio18(&self) -> &GPIO18 {
        &self.gpio18
    }
    #[doc = "0x50 - Configuration register for pin GPIO19"]
    #[inline(always)]
    pub const fn gpio19(&self) -> &GPIO19 {
        &self.gpio19
    }
    #[doc = "0x54 - Configuration register for pin GPIO20"]
    #[inline(always)]
    pub const fn gpio20(&self) -> &GPIO20 {
        &self.gpio20
    }
    #[doc = "0x58 - Configuration register for pin GPIO21"]
    #[inline(always)]
    pub const fn gpio21(&self) -> &GPIO21 {
        &self.gpio21
    }
    #[doc = "0x6c - Configuration register for pin GPIO26"]
    #[inline(always)]
    pub const fn gpio26(&self) -> &GPIO26 {
        &self.gpio26
    }
    #[doc = "0x70 - Configuration register for pin GPIO27"]
    #[inline(always)]
    pub const fn gpio27(&self) -> &GPIO27 {
        &self.gpio27
    }
    #[doc = "0x74 - Configuration register for pin GPIO28"]
    #[inline(always)]
    pub const fn gpio28(&self) -> &GPIO28 {
        &self.gpio28
    }
    #[doc = "0x78 - Configuration register for pin GPIO29"]
    #[inline(always)]
    pub const fn gpio29(&self) -> &GPIO29 {
        &self.gpio29
    }
    #[doc = "0x7c - Configuration register for pin GPIO30"]
    #[inline(always)]
    pub const fn gpio30(&self) -> &GPIO30 {
        &self.gpio30
    }
    #[doc = "0x80 - Configuration register for pin GPIO31"]
    #[inline(always)]
    pub const fn gpio31(&self) -> &GPIO31 {
        &self.gpio31
    }
    #[doc = "0x84 - Configuration register for pin GPIO32"]
    #[inline(always)]
    pub const fn gpio32(&self) -> &GPIO32 {
        &self.gpio32
    }
    #[doc = "0x88 - Configuration register for pin GPIO33"]
    #[inline(always)]
    pub const fn gpio33(&self) -> &GPIO33 {
        &self.gpio33
    }
    #[doc = "0x8c - Configuration register for pin GPIO34"]
    #[inline(always)]
    pub const fn gpio34(&self) -> &GPIO34 {
        &self.gpio34
    }
    #[doc = "0x90 - Configuration register for pin GPIO35"]
    #[inline(always)]
    pub const fn gpio35(&self) -> &GPIO35 {
        &self.gpio35
    }
    #[doc = "0x94 - Configuration register for pin GPIO36"]
    #[inline(always)]
    pub const fn gpio36(&self) -> &GPIO36 {
        &self.gpio36
    }
    #[doc = "0x98 - Configuration register for pin GPIO37"]
    #[inline(always)]
    pub const fn gpio37(&self) -> &GPIO37 {
        &self.gpio37
    }
    #[doc = "0x9c - Configuration register for pin GPIO38"]
    #[inline(always)]
    pub const fn gpio38(&self) -> &GPIO38 {
        &self.gpio38
    }
    #[doc = "0xa0 - Configuration register for pin GPIO39"]
    #[inline(always)]
    pub const fn gpio39(&self) -> &GPIO39 {
        &self.gpio39
    }
    #[doc = "0xa4 - Configuration register for pin GPIO40"]
    #[inline(always)]
    pub const fn gpio40(&self) -> &GPIO40 {
        &self.gpio40
    }
    #[doc = "0xa8 - Configuration register for pin GPIO41"]
    #[inline(always)]
    pub const fn gpio41(&self) -> &GPIO41 {
        &self.gpio41
    }
    #[doc = "0xac - Configuration register for pin GPIO42"]
    #[inline(always)]
    pub const fn gpio42(&self) -> &GPIO42 {
        &self.gpio42
    }
    #[doc = "0xb0 - Configuration register for pin GPIO43"]
    #[inline(always)]
    pub const fn gpio43(&self) -> &GPIO43 {
        &self.gpio43
    }
    #[doc = "0xb4 - Configuration register for pin GPIO44"]
    #[inline(always)]
    pub const fn gpio44(&self) -> &GPIO44 {
        &self.gpio44
    }
    #[doc = "0xb8 - Configuration register for pin GPIO45"]
    #[inline(always)]
    pub const fn gpio45(&self) -> &GPIO45 {
        &self.gpio45
    }
    #[doc = "0xbc - Configuration register for pin GPIO46"]
    #[inline(always)]
    pub const fn gpio46(&self) -> &GPIO46 {
        &self.gpio46
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PIN_CTRL (rw) register accessor: Clock output configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock output configuration register"]
pub mod pin_ctrl;
#[doc = "GPIO0 (rw) register accessor: Configuration register for pin GPIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "Configuration register for pin GPIO0"]
pub mod gpio0;
#[doc = "GPIO1 (rw) register accessor: Configuration register for pin GPIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "Configuration register for pin GPIO1"]
pub mod gpio1;
#[doc = "GPIO2 (rw) register accessor: Configuration register for pin GPIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "Configuration register for pin GPIO2"]
pub mod gpio2;
#[doc = "GPIO3 (rw) register accessor: Configuration register for pin GPIO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "Configuration register for pin GPIO3"]
pub mod gpio3;
#[doc = "GPIO4 (rw) register accessor: Configuration register for pin GPIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "Configuration register for pin GPIO4"]
pub mod gpio4;
#[doc = "GPIO5 (rw) register accessor: Configuration register for pin GPIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "Configuration register for pin GPIO5"]
pub mod gpio5;
#[doc = "GPIO6 (rw) register accessor: Configuration register for pin GPIO6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "Configuration register for pin GPIO6"]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: Configuration register for pin GPIO7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "Configuration register for pin GPIO7"]
pub mod gpio7;
#[doc = "GPIO8 (rw) register accessor: Configuration register for pin GPIO8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`] module"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = "Configuration register for pin GPIO8"]
pub mod gpio8;
#[doc = "GPIO9 (rw) register accessor: Configuration register for pin GPIO9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`] module"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = "Configuration register for pin GPIO9"]
pub mod gpio9;
#[doc = "GPIO10 (rw) register accessor: Configuration register for pin GPIO10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`] module"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = "Configuration register for pin GPIO10"]
pub mod gpio10;
#[doc = "GPIO11 (rw) register accessor: Configuration register for pin GPIO11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`] module"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = "Configuration register for pin GPIO11"]
pub mod gpio11;
#[doc = "GPIO12 (rw) register accessor: Configuration register for pin GPIO12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`] module"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = "Configuration register for pin GPIO12"]
pub mod gpio12;
#[doc = "GPIO13 (rw) register accessor: Configuration register for pin GPIO13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`] module"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = "Configuration register for pin GPIO13"]
pub mod gpio13;
#[doc = "GPIO14 (rw) register accessor: Configuration register for pin GPIO14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`] module"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = "Configuration register for pin GPIO14"]
pub mod gpio14;
#[doc = "GPIO19 (rw) register accessor: Configuration register for pin GPIO19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`] module"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = "Configuration register for pin GPIO19"]
pub mod gpio19;
#[doc = "GPIO20 (rw) register accessor: Configuration register for pin GPIO20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`] module"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = "Configuration register for pin GPIO20"]
pub mod gpio20;
#[doc = "GPIO21 (rw) register accessor: Configuration register for pin GPIO21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`] module"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = "Configuration register for pin GPIO21"]
pub mod gpio21;
#[doc = "GPIO33 (rw) register accessor: Configuration register for pin GPIO33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio33`] module"]
pub type GPIO33 = crate::Reg<gpio33::GPIO33_SPEC>;
#[doc = "Configuration register for pin GPIO33"]
pub mod gpio33;
#[doc = "GPIO34 (rw) register accessor: Configuration register for pin GPIO34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio34`] module"]
pub type GPIO34 = crate::Reg<gpio34::GPIO34_SPEC>;
#[doc = "Configuration register for pin GPIO34"]
pub mod gpio34;
#[doc = "GPIO35 (rw) register accessor: Configuration register for pin GPIO35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio35`] module"]
pub type GPIO35 = crate::Reg<gpio35::GPIO35_SPEC>;
#[doc = "Configuration register for pin GPIO35"]
pub mod gpio35;
#[doc = "GPIO36 (rw) register accessor: Configuration register for pin GPIO36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio36`] module"]
pub type GPIO36 = crate::Reg<gpio36::GPIO36_SPEC>;
#[doc = "Configuration register for pin GPIO36"]
pub mod gpio36;
#[doc = "GPIO37 (rw) register accessor: Configuration register for pin GPIO37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio37`] module"]
pub type GPIO37 = crate::Reg<gpio37::GPIO37_SPEC>;
#[doc = "Configuration register for pin GPIO37"]
pub mod gpio37;
#[doc = "GPIO38 (rw) register accessor: Configuration register for pin GPIO38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio38`] module"]
pub type GPIO38 = crate::Reg<gpio38::GPIO38_SPEC>;
#[doc = "Configuration register for pin GPIO38"]
pub mod gpio38;
#[doc = "GPIO45 (rw) register accessor: Configuration register for pin GPIO45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio45`] module"]
pub type GPIO45 = crate::Reg<gpio45::GPIO45_SPEC>;
#[doc = "Configuration register for pin GPIO45"]
pub mod gpio45;
#[doc = "GPIO46 (rw) register accessor: Configuration register for pin GPIO46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio46`] module"]
pub type GPIO46 = crate::Reg<gpio46::GPIO46_SPEC>;
#[doc = "Configuration register for pin GPIO46"]
pub mod gpio46;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "GPIO15 (rw) register accessor: Configuration register for pin GPIO15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`] module"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = "Configuration register for pin GPIO15"]
pub mod gpio15;
#[doc = "GPIO16 (rw) register accessor: Configuration register for pin GPIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`] module"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = "Configuration register for pin GPIO16"]
pub mod gpio16;
#[doc = "GPIO17 (rw) register accessor: Configuration register for pin GPIO17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`] module"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = "Configuration register for pin GPIO17"]
pub mod gpio17;
#[doc = "GPIO18 (rw) register accessor: Configuration register for pin GPIO18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`] module"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = "Configuration register for pin GPIO18"]
pub mod gpio18;
#[doc = "GPIO26 (rw) register accessor: Configuration register for pin GPIO26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`] module"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = "Configuration register for pin GPIO26"]
pub mod gpio26;
#[doc = "GPIO27 (rw) register accessor: Configuration register for pin GPIO27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`] module"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = "Configuration register for pin GPIO27"]
pub mod gpio27;
#[doc = "GPIO28 (rw) register accessor: Configuration register for pin GPIO28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`] module"]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = "Configuration register for pin GPIO28"]
pub mod gpio28;
#[doc = "GPIO29 (rw) register accessor: Configuration register for pin GPIO29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio29`] module"]
pub type GPIO29 = crate::Reg<gpio29::GPIO29_SPEC>;
#[doc = "Configuration register for pin GPIO29"]
pub mod gpio29;
#[doc = "GPIO30 (rw) register accessor: Configuration register for pin GPIO30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio30`] module"]
pub type GPIO30 = crate::Reg<gpio30::GPIO30_SPEC>;
#[doc = "Configuration register for pin GPIO30"]
pub mod gpio30;
#[doc = "GPIO31 (rw) register accessor: Configuration register for pin GPIO31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio31`] module"]
pub type GPIO31 = crate::Reg<gpio31::GPIO31_SPEC>;
#[doc = "Configuration register for pin GPIO31"]
pub mod gpio31;
#[doc = "GPIO32 (rw) register accessor: Configuration register for pin GPIO32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio32`] module"]
pub type GPIO32 = crate::Reg<gpio32::GPIO32_SPEC>;
#[doc = "Configuration register for pin GPIO32"]
pub mod gpio32;
#[doc = "GPIO39 (rw) register accessor: Configuration register for pin GPIO39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio39`] module"]
pub type GPIO39 = crate::Reg<gpio39::GPIO39_SPEC>;
#[doc = "Configuration register for pin GPIO39"]
pub mod gpio39;
#[doc = "GPIO40 (rw) register accessor: Configuration register for pin GPIO40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio40`] module"]
pub type GPIO40 = crate::Reg<gpio40::GPIO40_SPEC>;
#[doc = "Configuration register for pin GPIO40"]
pub mod gpio40;
#[doc = "GPIO41 (rw) register accessor: Configuration register for pin GPIO41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio41`] module"]
pub type GPIO41 = crate::Reg<gpio41::GPIO41_SPEC>;
#[doc = "Configuration register for pin GPIO41"]
pub mod gpio41;
#[doc = "GPIO42 (rw) register accessor: Configuration register for pin GPIO42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio42`] module"]
pub type GPIO42 = crate::Reg<gpio42::GPIO42_SPEC>;
#[doc = "Configuration register for pin GPIO42"]
pub mod gpio42;
#[doc = "GPIO43 (rw) register accessor: Configuration register for pin GPIO43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio43`] module"]
pub type GPIO43 = crate::Reg<gpio43::GPIO43_SPEC>;
#[doc = "Configuration register for pin GPIO43"]
pub mod gpio43;
#[doc = "GPIO44 (rw) register accessor: Configuration register for pin GPIO44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio44`] module"]
pub type GPIO44 = crate::Reg<gpio44::GPIO44_SPEC>;
#[doc = "Configuration register for pin GPIO44"]
pub mod gpio44;
