#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
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
    gpio22: GPIO22,
    gpio23: GPIO23,
    gpio24: GPIO24,
    gpio25: GPIO25,
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
    gpio47: GPIO47,
    gpio48: GPIO48,
    gpio49: GPIO49,
    gpio50: GPIO50,
    gpio51: GPIO51,
    gpio52: GPIO52,
    gpio53: GPIO53,
    gpio54: GPIO54,
    gpio55: GPIO55,
    gpio56: GPIO56,
    _reserved57: [u8; 0x1c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x04 - iomux control register for gpio0"]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0x08 - iomux control register for gpio1"]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0x0c - iomux control register for gpio2"]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0x10 - iomux control register for gpio3"]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0x14 - iomux control register for gpio4"]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0x18 - iomux control register for gpio5"]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0x1c - iomux control register for gpio6"]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0x20 - iomux control register for gpio7"]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0x24 - iomux control register for gpio8"]
    #[inline(always)]
    pub const fn gpio8(&self) -> &GPIO8 {
        &self.gpio8
    }
    #[doc = "0x28 - iomux control register for gpio9"]
    #[inline(always)]
    pub const fn gpio9(&self) -> &GPIO9 {
        &self.gpio9
    }
    #[doc = "0x2c - iomux control register for gpio10"]
    #[inline(always)]
    pub const fn gpio10(&self) -> &GPIO10 {
        &self.gpio10
    }
    #[doc = "0x30 - iomux control register for gpio11"]
    #[inline(always)]
    pub const fn gpio11(&self) -> &GPIO11 {
        &self.gpio11
    }
    #[doc = "0x34 - iomux control register for gpio12"]
    #[inline(always)]
    pub const fn gpio12(&self) -> &GPIO12 {
        &self.gpio12
    }
    #[doc = "0x38 - iomux control register for gpio13"]
    #[inline(always)]
    pub const fn gpio13(&self) -> &GPIO13 {
        &self.gpio13
    }
    #[doc = "0x3c - iomux control register for gpio14"]
    #[inline(always)]
    pub const fn gpio14(&self) -> &GPIO14 {
        &self.gpio14
    }
    #[doc = "0x40 - iomux control register for gpio15"]
    #[inline(always)]
    pub const fn gpio15(&self) -> &GPIO15 {
        &self.gpio15
    }
    #[doc = "0x44 - iomux control register for gpio16"]
    #[inline(always)]
    pub const fn gpio16(&self) -> &GPIO16 {
        &self.gpio16
    }
    #[doc = "0x48 - iomux control register for gpio17"]
    #[inline(always)]
    pub const fn gpio17(&self) -> &GPIO17 {
        &self.gpio17
    }
    #[doc = "0x4c - iomux control register for gpio18"]
    #[inline(always)]
    pub const fn gpio18(&self) -> &GPIO18 {
        &self.gpio18
    }
    #[doc = "0x50 - iomux control register for gpio19"]
    #[inline(always)]
    pub const fn gpio19(&self) -> &GPIO19 {
        &self.gpio19
    }
    #[doc = "0x54 - iomux control register for gpio20"]
    #[inline(always)]
    pub const fn gpio20(&self) -> &GPIO20 {
        &self.gpio20
    }
    #[doc = "0x58 - iomux control register for gpio21"]
    #[inline(always)]
    pub const fn gpio21(&self) -> &GPIO21 {
        &self.gpio21
    }
    #[doc = "0x5c - iomux control register for gpio22"]
    #[inline(always)]
    pub const fn gpio22(&self) -> &GPIO22 {
        &self.gpio22
    }
    #[doc = "0x60 - iomux control register for gpio23"]
    #[inline(always)]
    pub const fn gpio23(&self) -> &GPIO23 {
        &self.gpio23
    }
    #[doc = "0x64 - iomux control register for gpio24"]
    #[inline(always)]
    pub const fn gpio24(&self) -> &GPIO24 {
        &self.gpio24
    }
    #[doc = "0x68 - iomux control register for gpio25"]
    #[inline(always)]
    pub const fn gpio25(&self) -> &GPIO25 {
        &self.gpio25
    }
    #[doc = "0x6c - iomux control register for gpio26"]
    #[inline(always)]
    pub const fn gpio26(&self) -> &GPIO26 {
        &self.gpio26
    }
    #[doc = "0x70 - iomux control register for gpio27"]
    #[inline(always)]
    pub const fn gpio27(&self) -> &GPIO27 {
        &self.gpio27
    }
    #[doc = "0x74 - iomux control register for gpio28"]
    #[inline(always)]
    pub const fn gpio28(&self) -> &GPIO28 {
        &self.gpio28
    }
    #[doc = "0x78 - iomux control register for gpio29"]
    #[inline(always)]
    pub const fn gpio29(&self) -> &GPIO29 {
        &self.gpio29
    }
    #[doc = "0x7c - iomux control register for gpio30"]
    #[inline(always)]
    pub const fn gpio30(&self) -> &GPIO30 {
        &self.gpio30
    }
    #[doc = "0x80 - iomux control register for gpio31"]
    #[inline(always)]
    pub const fn gpio31(&self) -> &GPIO31 {
        &self.gpio31
    }
    #[doc = "0x84 - iomux control register for gpio32"]
    #[inline(always)]
    pub const fn gpio32(&self) -> &GPIO32 {
        &self.gpio32
    }
    #[doc = "0x88 - iomux control register for gpio33"]
    #[inline(always)]
    pub const fn gpio33(&self) -> &GPIO33 {
        &self.gpio33
    }
    #[doc = "0x8c - iomux control register for gpio34"]
    #[inline(always)]
    pub const fn gpio34(&self) -> &GPIO34 {
        &self.gpio34
    }
    #[doc = "0x90 - iomux control register for gpio35"]
    #[inline(always)]
    pub const fn gpio35(&self) -> &GPIO35 {
        &self.gpio35
    }
    #[doc = "0x94 - iomux control register for gpio36"]
    #[inline(always)]
    pub const fn gpio36(&self) -> &GPIO36 {
        &self.gpio36
    }
    #[doc = "0x98 - iomux control register for gpio37"]
    #[inline(always)]
    pub const fn gpio37(&self) -> &GPIO37 {
        &self.gpio37
    }
    #[doc = "0x9c - iomux control register for gpio38"]
    #[inline(always)]
    pub const fn gpio38(&self) -> &GPIO38 {
        &self.gpio38
    }
    #[doc = "0xa0 - iomux control register for gpio39"]
    #[inline(always)]
    pub const fn gpio39(&self) -> &GPIO39 {
        &self.gpio39
    }
    #[doc = "0xa4 - iomux control register for gpio40"]
    #[inline(always)]
    pub const fn gpio40(&self) -> &GPIO40 {
        &self.gpio40
    }
    #[doc = "0xa8 - iomux control register for gpio41"]
    #[inline(always)]
    pub const fn gpio41(&self) -> &GPIO41 {
        &self.gpio41
    }
    #[doc = "0xac - iomux control register for gpio42"]
    #[inline(always)]
    pub const fn gpio42(&self) -> &GPIO42 {
        &self.gpio42
    }
    #[doc = "0xb0 - iomux control register for gpio43"]
    #[inline(always)]
    pub const fn gpio43(&self) -> &GPIO43 {
        &self.gpio43
    }
    #[doc = "0xb4 - iomux control register for gpio44"]
    #[inline(always)]
    pub const fn gpio44(&self) -> &GPIO44 {
        &self.gpio44
    }
    #[doc = "0xb8 - iomux control register for gpio45"]
    #[inline(always)]
    pub const fn gpio45(&self) -> &GPIO45 {
        &self.gpio45
    }
    #[doc = "0xbc - iomux control register for gpio46"]
    #[inline(always)]
    pub const fn gpio46(&self) -> &GPIO46 {
        &self.gpio46
    }
    #[doc = "0xc0 - iomux control register for gpio47"]
    #[inline(always)]
    pub const fn gpio47(&self) -> &GPIO47 {
        &self.gpio47
    }
    #[doc = "0xc4 - iomux control register for gpio48"]
    #[inline(always)]
    pub const fn gpio48(&self) -> &GPIO48 {
        &self.gpio48
    }
    #[doc = "0xc8 - iomux control register for gpio49"]
    #[inline(always)]
    pub const fn gpio49(&self) -> &GPIO49 {
        &self.gpio49
    }
    #[doc = "0xcc - iomux control register for gpio50"]
    #[inline(always)]
    pub const fn gpio50(&self) -> &GPIO50 {
        &self.gpio50
    }
    #[doc = "0xd0 - iomux control register for gpio51"]
    #[inline(always)]
    pub const fn gpio51(&self) -> &GPIO51 {
        &self.gpio51
    }
    #[doc = "0xd4 - iomux control register for gpio52"]
    #[inline(always)]
    pub const fn gpio52(&self) -> &GPIO52 {
        &self.gpio52
    }
    #[doc = "0xd8 - iomux control register for gpio53"]
    #[inline(always)]
    pub const fn gpio53(&self) -> &GPIO53 {
        &self.gpio53
    }
    #[doc = "0xdc - iomux control register for gpio54"]
    #[inline(always)]
    pub const fn gpio54(&self) -> &GPIO54 {
        &self.gpio54
    }
    #[doc = "0xe0 - iomux control register for gpio55"]
    #[inline(always)]
    pub const fn gpio55(&self) -> &GPIO55 {
        &self.gpio55
    }
    #[doc = "0xe4 - iomux control register for gpio56"]
    #[inline(always)]
    pub const fn gpio56(&self) -> &GPIO56 {
        &self.gpio56
    }
    #[doc = "0x104 - iomux version"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "gpio0 (rw) register accessor: iomux control register for gpio0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "iomux control register for gpio0"]
pub mod gpio0;
#[doc = "gpio1 (rw) register accessor: iomux control register for gpio1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "iomux control register for gpio1"]
pub mod gpio1;
#[doc = "gpio2 (rw) register accessor: iomux control register for gpio2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "iomux control register for gpio2"]
pub mod gpio2;
#[doc = "gpio3 (rw) register accessor: iomux control register for gpio3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "iomux control register for gpio3"]
pub mod gpio3;
#[doc = "gpio4 (rw) register accessor: iomux control register for gpio4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "iomux control register for gpio4"]
pub mod gpio4;
#[doc = "gpio5 (rw) register accessor: iomux control register for gpio5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "iomux control register for gpio5"]
pub mod gpio5;
#[doc = "gpio6 (rw) register accessor: iomux control register for gpio6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "iomux control register for gpio6"]
pub mod gpio6;
#[doc = "gpio7 (rw) register accessor: iomux control register for gpio7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "iomux control register for gpio7"]
pub mod gpio7;
#[doc = "gpio8 (rw) register accessor: iomux control register for gpio8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`] module"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = "iomux control register for gpio8"]
pub mod gpio8;
#[doc = "gpio9 (rw) register accessor: iomux control register for gpio9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`] module"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = "iomux control register for gpio9"]
pub mod gpio9;
#[doc = "gpio10 (rw) register accessor: iomux control register for gpio10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`] module"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = "iomux control register for gpio10"]
pub mod gpio10;
#[doc = "gpio11 (rw) register accessor: iomux control register for gpio11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`] module"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = "iomux control register for gpio11"]
pub mod gpio11;
#[doc = "gpio12 (rw) register accessor: iomux control register for gpio12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`] module"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = "iomux control register for gpio12"]
pub mod gpio12;
#[doc = "gpio13 (rw) register accessor: iomux control register for gpio13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`] module"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = "iomux control register for gpio13"]
pub mod gpio13;
#[doc = "gpio14 (rw) register accessor: iomux control register for gpio14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`] module"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = "iomux control register for gpio14"]
pub mod gpio14;
#[doc = "gpio15 (rw) register accessor: iomux control register for gpio15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`] module"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = "iomux control register for gpio15"]
pub mod gpio15;
#[doc = "gpio16 (rw) register accessor: iomux control register for gpio16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`] module"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = "iomux control register for gpio16"]
pub mod gpio16;
#[doc = "gpio17 (rw) register accessor: iomux control register for gpio17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`] module"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = "iomux control register for gpio17"]
pub mod gpio17;
#[doc = "gpio18 (rw) register accessor: iomux control register for gpio18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`] module"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = "iomux control register for gpio18"]
pub mod gpio18;
#[doc = "gpio19 (rw) register accessor: iomux control register for gpio19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`] module"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = "iomux control register for gpio19"]
pub mod gpio19;
#[doc = "gpio20 (rw) register accessor: iomux control register for gpio20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`] module"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = "iomux control register for gpio20"]
pub mod gpio20;
#[doc = "gpio21 (rw) register accessor: iomux control register for gpio21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`] module"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = "iomux control register for gpio21"]
pub mod gpio21;
#[doc = "gpio22 (rw) register accessor: iomux control register for gpio22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22`] module"]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = "iomux control register for gpio22"]
pub mod gpio22;
#[doc = "gpio23 (rw) register accessor: iomux control register for gpio23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23`] module"]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = "iomux control register for gpio23"]
pub mod gpio23;
#[doc = "gpio24 (rw) register accessor: iomux control register for gpio24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24`] module"]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = "iomux control register for gpio24"]
pub mod gpio24;
#[doc = "gpio25 (rw) register accessor: iomux control register for gpio25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25`] module"]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = "iomux control register for gpio25"]
pub mod gpio25;
#[doc = "gpio26 (rw) register accessor: iomux control register for gpio26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`] module"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = "iomux control register for gpio26"]
pub mod gpio26;
#[doc = "gpio27 (rw) register accessor: iomux control register for gpio27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`] module"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = "iomux control register for gpio27"]
pub mod gpio27;
#[doc = "gpio28 (rw) register accessor: iomux control register for gpio28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`] module"]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = "iomux control register for gpio28"]
pub mod gpio28;
#[doc = "gpio29 (rw) register accessor: iomux control register for gpio29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio29`] module"]
pub type GPIO29 = crate::Reg<gpio29::GPIO29_SPEC>;
#[doc = "iomux control register for gpio29"]
pub mod gpio29;
#[doc = "gpio30 (rw) register accessor: iomux control register for gpio30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio30`] module"]
pub type GPIO30 = crate::Reg<gpio30::GPIO30_SPEC>;
#[doc = "iomux control register for gpio30"]
pub mod gpio30;
#[doc = "gpio31 (rw) register accessor: iomux control register for gpio31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio31`] module"]
pub type GPIO31 = crate::Reg<gpio31::GPIO31_SPEC>;
#[doc = "iomux control register for gpio31"]
pub mod gpio31;
#[doc = "gpio32 (rw) register accessor: iomux control register for gpio32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio32`] module"]
pub type GPIO32 = crate::Reg<gpio32::GPIO32_SPEC>;
#[doc = "iomux control register for gpio32"]
pub mod gpio32;
#[doc = "gpio33 (rw) register accessor: iomux control register for gpio33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio33`] module"]
pub type GPIO33 = crate::Reg<gpio33::GPIO33_SPEC>;
#[doc = "iomux control register for gpio33"]
pub mod gpio33;
#[doc = "gpio34 (rw) register accessor: iomux control register for gpio34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio34`] module"]
pub type GPIO34 = crate::Reg<gpio34::GPIO34_SPEC>;
#[doc = "iomux control register for gpio34"]
pub mod gpio34;
#[doc = "gpio35 (rw) register accessor: iomux control register for gpio35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio35`] module"]
pub type GPIO35 = crate::Reg<gpio35::GPIO35_SPEC>;
#[doc = "iomux control register for gpio35"]
pub mod gpio35;
#[doc = "gpio36 (rw) register accessor: iomux control register for gpio36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio36`] module"]
pub type GPIO36 = crate::Reg<gpio36::GPIO36_SPEC>;
#[doc = "iomux control register for gpio36"]
pub mod gpio36;
#[doc = "gpio37 (rw) register accessor: iomux control register for gpio37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio37`] module"]
pub type GPIO37 = crate::Reg<gpio37::GPIO37_SPEC>;
#[doc = "iomux control register for gpio37"]
pub mod gpio37;
#[doc = "gpio38 (rw) register accessor: iomux control register for gpio38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio38`] module"]
pub type GPIO38 = crate::Reg<gpio38::GPIO38_SPEC>;
#[doc = "iomux control register for gpio38"]
pub mod gpio38;
#[doc = "gpio39 (rw) register accessor: iomux control register for gpio39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio39`] module"]
pub type GPIO39 = crate::Reg<gpio39::GPIO39_SPEC>;
#[doc = "iomux control register for gpio39"]
pub mod gpio39;
#[doc = "gpio40 (rw) register accessor: iomux control register for gpio40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio40`] module"]
pub type GPIO40 = crate::Reg<gpio40::GPIO40_SPEC>;
#[doc = "iomux control register for gpio40"]
pub mod gpio40;
#[doc = "gpio41 (rw) register accessor: iomux control register for gpio41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio41`] module"]
pub type GPIO41 = crate::Reg<gpio41::GPIO41_SPEC>;
#[doc = "iomux control register for gpio41"]
pub mod gpio41;
#[doc = "gpio42 (rw) register accessor: iomux control register for gpio42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio42`] module"]
pub type GPIO42 = crate::Reg<gpio42::GPIO42_SPEC>;
#[doc = "iomux control register for gpio42"]
pub mod gpio42;
#[doc = "gpio43 (rw) register accessor: iomux control register for gpio43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio43`] module"]
pub type GPIO43 = crate::Reg<gpio43::GPIO43_SPEC>;
#[doc = "iomux control register for gpio43"]
pub mod gpio43;
#[doc = "gpio44 (rw) register accessor: iomux control register for gpio44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio44`] module"]
pub type GPIO44 = crate::Reg<gpio44::GPIO44_SPEC>;
#[doc = "iomux control register for gpio44"]
pub mod gpio44;
#[doc = "gpio45 (rw) register accessor: iomux control register for gpio45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio45`] module"]
pub type GPIO45 = crate::Reg<gpio45::GPIO45_SPEC>;
#[doc = "iomux control register for gpio45"]
pub mod gpio45;
#[doc = "gpio46 (rw) register accessor: iomux control register for gpio46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio46`] module"]
pub type GPIO46 = crate::Reg<gpio46::GPIO46_SPEC>;
#[doc = "iomux control register for gpio46"]
pub mod gpio46;
#[doc = "gpio47 (rw) register accessor: iomux control register for gpio47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio47`] module"]
pub type GPIO47 = crate::Reg<gpio47::GPIO47_SPEC>;
#[doc = "iomux control register for gpio47"]
pub mod gpio47;
#[doc = "gpio48 (rw) register accessor: iomux control register for gpio48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio48`] module"]
pub type GPIO48 = crate::Reg<gpio48::GPIO48_SPEC>;
#[doc = "iomux control register for gpio48"]
pub mod gpio48;
#[doc = "gpio49 (rw) register accessor: iomux control register for gpio49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio49`] module"]
pub type GPIO49 = crate::Reg<gpio49::GPIO49_SPEC>;
#[doc = "iomux control register for gpio49"]
pub mod gpio49;
#[doc = "gpio50 (rw) register accessor: iomux control register for gpio50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio50`] module"]
pub type GPIO50 = crate::Reg<gpio50::GPIO50_SPEC>;
#[doc = "iomux control register for gpio50"]
pub mod gpio50;
#[doc = "gpio51 (rw) register accessor: iomux control register for gpio51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio51`] module"]
pub type GPIO51 = crate::Reg<gpio51::GPIO51_SPEC>;
#[doc = "iomux control register for gpio51"]
pub mod gpio51;
#[doc = "gpio52 (rw) register accessor: iomux control register for gpio52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio52`] module"]
pub type GPIO52 = crate::Reg<gpio52::GPIO52_SPEC>;
#[doc = "iomux control register for gpio52"]
pub mod gpio52;
#[doc = "gpio53 (rw) register accessor: iomux control register for gpio53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio53`] module"]
pub type GPIO53 = crate::Reg<gpio53::GPIO53_SPEC>;
#[doc = "iomux control register for gpio53"]
pub mod gpio53;
#[doc = "gpio54 (rw) register accessor: iomux control register for gpio54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio54`] module"]
pub type GPIO54 = crate::Reg<gpio54::GPIO54_SPEC>;
#[doc = "iomux control register for gpio54"]
pub mod gpio54;
#[doc = "gpio55 (rw) register accessor: iomux control register for gpio55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio55`] module"]
pub type GPIO55 = crate::Reg<gpio55::GPIO55_SPEC>;
#[doc = "iomux control register for gpio55"]
pub mod gpio55;
#[doc = "gpio56 (rw) register accessor: iomux control register for gpio56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio56`] module"]
pub type GPIO56 = crate::Reg<gpio56::GPIO56_SPEC>;
#[doc = "iomux control register for gpio56"]
pub mod gpio56;
#[doc = "DATE (rw) register accessor: iomux version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "iomux version"]
pub mod date;
