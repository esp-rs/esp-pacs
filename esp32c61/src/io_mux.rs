#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
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
    _reserved30: [u8; 0x0184],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - IO_MUX Configure Register for pad XTAL_32K_P"]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0x04 - IO_MUX Configure Register for pad XTAL_32K_N"]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0x08 - IO_MUX Configure Register for pad GPIO2"]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0x0c - IO_MUX Configure Register for pad MTMS"]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0x10 - IO_MUX Configure Register for pad MTDI"]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0x14 - IO_MUX Configure Register for pad MTCK"]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0x18 - IO_MUX Configure Register for pad MTDO"]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0x1c - IO_MUX Configure Register for pad GPIO7"]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0x20 - IO_MUX Configure Register for pad GPIO8"]
    #[inline(always)]
    pub const fn gpio8(&self) -> &GPIO8 {
        &self.gpio8
    }
    #[doc = "0x24 - IO_MUX Configure Register for pad GPIO9"]
    #[inline(always)]
    pub const fn gpio9(&self) -> &GPIO9 {
        &self.gpio9
    }
    #[doc = "0x28 - IO_MUX Configure Register for pad U0RXD"]
    #[inline(always)]
    pub const fn gpio10(&self) -> &GPIO10 {
        &self.gpio10
    }
    #[doc = "0x2c - IO_MUX Configure Register for pad U0TXD"]
    #[inline(always)]
    pub const fn gpio11(&self) -> &GPIO11 {
        &self.gpio11
    }
    #[doc = "0x30 - IO_MUX Configure Register for pad GPIO12"]
    #[inline(always)]
    pub const fn gpio12(&self) -> &GPIO12 {
        &self.gpio12
    }
    #[doc = "0x34 - IO_MUX Configure Register for pad GPIO13"]
    #[inline(always)]
    pub const fn gpio13(&self) -> &GPIO13 {
        &self.gpio13
    }
    #[doc = "0x38 - IO_MUX Configure Register for pad SPICS1"]
    #[inline(always)]
    pub const fn gpio14(&self) -> &GPIO14 {
        &self.gpio14
    }
    #[doc = "0x3c - IO_MUX Configure Register for pad SPICS0"]
    #[inline(always)]
    pub const fn gpio15(&self) -> &GPIO15 {
        &self.gpio15
    }
    #[doc = "0x40 - IO_MUX Configure Register for pad SPIQ"]
    #[inline(always)]
    pub const fn gpio16(&self) -> &GPIO16 {
        &self.gpio16
    }
    #[doc = "0x44 - IO_MUX Configure Register for pad SPIWP"]
    #[inline(always)]
    pub const fn gpio17(&self) -> &GPIO17 {
        &self.gpio17
    }
    #[doc = "0x48 - IO_MUX Configure Register for pad VDD_SPI"]
    #[inline(always)]
    pub const fn gpio18(&self) -> &GPIO18 {
        &self.gpio18
    }
    #[doc = "0x4c - IO_MUX Configure Register for pad SPIHD"]
    #[inline(always)]
    pub const fn gpio19(&self) -> &GPIO19 {
        &self.gpio19
    }
    #[doc = "0x50 - IO_MUX Configure Register for pad SPICLK"]
    #[inline(always)]
    pub const fn gpio20(&self) -> &GPIO20 {
        &self.gpio20
    }
    #[doc = "0x54 - IO_MUX Configure Register for pad SPID"]
    #[inline(always)]
    pub const fn gpio21(&self) -> &GPIO21 {
        &self.gpio21
    }
    #[doc = "0x58 - IO_MUX Configure Register for pad SDIO_DATA2"]
    #[inline(always)]
    pub const fn gpio22(&self) -> &GPIO22 {
        &self.gpio22
    }
    #[doc = "0x5c - IO_MUX Configure Register for pad SDIO_DATA3"]
    #[inline(always)]
    pub const fn gpio23(&self) -> &GPIO23 {
        &self.gpio23
    }
    #[doc = "0x60 - IO_MUX Configure Register for pad GPIO24"]
    #[inline(always)]
    pub const fn gpio24(&self) -> &GPIO24 {
        &self.gpio24
    }
    #[doc = "0x64 - IO_MUX Configure Register for pad SDIO_CMD"]
    #[inline(always)]
    pub const fn gpio25(&self) -> &GPIO25 {
        &self.gpio25
    }
    #[doc = "0x68 - IO_MUX Configure Register for pad SDIO_CLK"]
    #[inline(always)]
    pub const fn gpio26(&self) -> &GPIO26 {
        &self.gpio26
    }
    #[doc = "0x6c - IO_MUX Configure Register for pad SDIO_DATA0"]
    #[inline(always)]
    pub const fn gpio27(&self) -> &GPIO27 {
        &self.gpio27
    }
    #[doc = "0x70 - IO_MUX Configure Register for pad SDIO_DATA1"]
    #[inline(always)]
    pub const fn gpio28(&self) -> &GPIO28 {
        &self.gpio28
    }
    #[doc = "0x74 - IO_MUX Configure Register for pad GPIO29"]
    #[inline(always)]
    pub const fn gpio29(&self) -> &GPIO29 {
        &self.gpio29
    }
    #[doc = "0x1fc - IO_MUX Version Control Register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "GPIO0 (rw) register accessor: IO_MUX Configure Register for pad XTAL_32K_P\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "IO_MUX Configure Register for pad XTAL_32K_P"]
pub mod gpio0;
#[doc = "GPIO1 (rw) register accessor: IO_MUX Configure Register for pad XTAL_32K_N\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "IO_MUX Configure Register for pad XTAL_32K_N"]
pub mod gpio1;
#[doc = "GPIO2 (rw) register accessor: IO_MUX Configure Register for pad GPIO2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO2"]
pub mod gpio2;
#[doc = "GPIO3 (rw) register accessor: IO_MUX Configure Register for pad MTMS\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "IO_MUX Configure Register for pad MTMS"]
pub mod gpio3;
#[doc = "GPIO4 (rw) register accessor: IO_MUX Configure Register for pad MTDI\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "IO_MUX Configure Register for pad MTDI"]
pub mod gpio4;
#[doc = "GPIO5 (rw) register accessor: IO_MUX Configure Register for pad MTCK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "IO_MUX Configure Register for pad MTCK"]
pub mod gpio5;
#[doc = "GPIO6 (rw) register accessor: IO_MUX Configure Register for pad MTDO\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "IO_MUX Configure Register for pad MTDO"]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: IO_MUX Configure Register for pad GPIO7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO7"]
pub mod gpio7;
#[doc = "GPIO8 (rw) register accessor: IO_MUX Configure Register for pad GPIO8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`] module"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO8"]
pub mod gpio8;
#[doc = "GPIO9 (rw) register accessor: IO_MUX Configure Register for pad GPIO9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`] module"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO9"]
pub mod gpio9;
#[doc = "GPIO10 (rw) register accessor: IO_MUX Configure Register for pad U0RXD\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`] module"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = "IO_MUX Configure Register for pad U0RXD"]
pub mod gpio10;
#[doc = "GPIO11 (rw) register accessor: IO_MUX Configure Register for pad U0TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`] module"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = "IO_MUX Configure Register for pad U0TXD"]
pub mod gpio11;
#[doc = "GPIO12 (rw) register accessor: IO_MUX Configure Register for pad GPIO12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`] module"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO12"]
pub mod gpio12;
#[doc = "GPIO13 (rw) register accessor: IO_MUX Configure Register for pad GPIO13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`] module"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO13"]
pub mod gpio13;
#[doc = "GPIO14 (rw) register accessor: IO_MUX Configure Register for pad SPICS1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`] module"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPICS1"]
pub mod gpio14;
#[doc = "GPIO15 (rw) register accessor: IO_MUX Configure Register for pad SPICS0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`] module"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPICS0"]
pub mod gpio15;
#[doc = "GPIO16 (rw) register accessor: IO_MUX Configure Register for pad SPIQ\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`] module"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPIQ"]
pub mod gpio16;
#[doc = "GPIO17 (rw) register accessor: IO_MUX Configure Register for pad SPIWP\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`] module"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPIWP"]
pub mod gpio17;
#[doc = "GPIO18 (rw) register accessor: IO_MUX Configure Register for pad VDD_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`] module"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = "IO_MUX Configure Register for pad VDD_SPI"]
pub mod gpio18;
#[doc = "GPIO19 (rw) register accessor: IO_MUX Configure Register for pad SPIHD\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`] module"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPIHD"]
pub mod gpio19;
#[doc = "GPIO20 (rw) register accessor: IO_MUX Configure Register for pad SPICLK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`] module"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPICLK"]
pub mod gpio20;
#[doc = "GPIO21 (rw) register accessor: IO_MUX Configure Register for pad SPID\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`] module"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = "IO_MUX Configure Register for pad SPID"]
pub mod gpio21;
#[doc = "GPIO22 (rw) register accessor: IO_MUX Configure Register for pad SDIO_DATA2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22`] module"]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_DATA2"]
pub mod gpio22;
#[doc = "GPIO23 (rw) register accessor: IO_MUX Configure Register for pad SDIO_DATA3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23`] module"]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_DATA3"]
pub mod gpio23;
#[doc = "GPIO24 (rw) register accessor: IO_MUX Configure Register for pad GPIO24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24`] module"]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO24"]
pub mod gpio24;
#[doc = "GPIO25 (rw) register accessor: IO_MUX Configure Register for pad SDIO_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25`] module"]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_CMD"]
pub mod gpio25;
#[doc = "GPIO26 (rw) register accessor: IO_MUX Configure Register for pad SDIO_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`] module"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_CLK"]
pub mod gpio26;
#[doc = "GPIO27 (rw) register accessor: IO_MUX Configure Register for pad SDIO_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`] module"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_DATA0"]
pub mod gpio27;
#[doc = "GPIO28 (rw) register accessor: IO_MUX Configure Register for pad SDIO_DATA1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`] module"]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = "IO_MUX Configure Register for pad SDIO_DATA1"]
pub mod gpio28;
#[doc = "GPIO29 (rw) register accessor: IO_MUX Configure Register for pad GPIO29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio29`] module"]
pub type GPIO29 = crate::Reg<gpio29::GPIO29_SPEC>;
#[doc = "IO_MUX Configure Register for pad GPIO29"]
pub mod gpio29;
#[doc = "DATE (rw) register accessor: IO_MUX Version Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "IO_MUX Version Control Register"]
pub mod date;
