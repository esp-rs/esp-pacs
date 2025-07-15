#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pin_ctrl: PIN_CTRL,
    gpio36: GPIO36,
    gpio37: GPIO37,
    gpio38: GPIO38,
    gpio39: GPIO39,
    gpio34: GPIO34,
    gpio35: GPIO35,
    gpio32: GPIO32,
    gpio33: GPIO33,
    gpio25: GPIO25,
    gpio26: GPIO26,
    gpio27: GPIO27,
    gpio14: GPIO14,
    gpio12: GPIO12,
    gpio13: GPIO13,
    gpio15: GPIO15,
    gpio2: GPIO2,
    gpio0: GPIO0,
    gpio4: GPIO4,
    gpio16: GPIO16,
    gpio17: GPIO17,
    gpio9: GPIO9,
    gpio10: GPIO10,
    gpio11: GPIO11,
    gpio6: GPIO6,
    gpio7: GPIO7,
    gpio8: GPIO8,
    gpio5: GPIO5,
    gpio18: GPIO18,
    gpio19: GPIO19,
    gpio20: GPIO20,
    gpio21: GPIO21,
    gpio22: GPIO22,
    gpio3: GPIO3,
    gpio1: GPIO1,
    gpio23: GPIO23,
    gpio24: GPIO24,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn pin_ctrl(&self) -> &PIN_CTRL {
        &self.pin_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn gpio36(&self) -> &GPIO36 {
        &self.gpio36
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn gpio37(&self) -> &GPIO37 {
        &self.gpio37
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn gpio38(&self) -> &GPIO38 {
        &self.gpio38
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn gpio39(&self) -> &GPIO39 {
        &self.gpio39
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn gpio34(&self) -> &GPIO34 {
        &self.gpio34
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn gpio35(&self) -> &GPIO35 {
        &self.gpio35
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn gpio32(&self) -> &GPIO32 {
        &self.gpio32
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn gpio33(&self) -> &GPIO33 {
        &self.gpio33
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn gpio25(&self) -> &GPIO25 {
        &self.gpio25
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn gpio26(&self) -> &GPIO26 {
        &self.gpio26
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn gpio27(&self) -> &GPIO27 {
        &self.gpio27
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn gpio14(&self) -> &GPIO14 {
        &self.gpio14
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn gpio12(&self) -> &GPIO12 {
        &self.gpio12
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn gpio13(&self) -> &GPIO13 {
        &self.gpio13
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn gpio15(&self) -> &GPIO15 {
        &self.gpio15
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn gpio16(&self) -> &GPIO16 {
        &self.gpio16
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn gpio17(&self) -> &GPIO17 {
        &self.gpio17
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn gpio9(&self) -> &GPIO9 {
        &self.gpio9
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn gpio10(&self) -> &GPIO10 {
        &self.gpio10
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn gpio11(&self) -> &GPIO11 {
        &self.gpio11
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn gpio8(&self) -> &GPIO8 {
        &self.gpio8
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn gpio18(&self) -> &GPIO18 {
        &self.gpio18
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn gpio19(&self) -> &GPIO19 {
        &self.gpio19
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn gpio20(&self) -> &GPIO20 {
        &self.gpio20
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn gpio21(&self) -> &GPIO21 {
        &self.gpio21
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn gpio22(&self) -> &GPIO22 {
        &self.gpio22
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn gpio23(&self) -> &GPIO23 {
        &self.gpio23
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn gpio24(&self) -> &GPIO24 {
        &self.gpio24
    }
}
#[doc = "PIN_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = ""]
pub mod pin_ctrl;
pub use gpio0 as gpio36;
pub use gpio0 as gpio37;
pub use gpio0 as gpio38;
pub use gpio0 as gpio39;
pub use gpio0 as gpio34;
pub use gpio0 as gpio35;
pub use gpio0 as gpio32;
pub use gpio0 as gpio33;
pub use gpio0 as gpio25;
pub use gpio0 as gpio26;
pub use gpio0 as gpio27;
pub use gpio0 as gpio14;
pub use gpio0 as gpio12;
pub use gpio0 as gpio13;
pub use gpio0 as gpio15;
pub use gpio0 as gpio2;
pub use GPIO0 as GPIO36;
pub use GPIO0 as GPIO37;
pub use GPIO0 as GPIO38;
pub use GPIO0 as GPIO39;
pub use GPIO0 as GPIO34;
pub use GPIO0 as GPIO35;
pub use GPIO0 as GPIO32;
pub use GPIO0 as GPIO33;
pub use GPIO0 as GPIO25;
pub use GPIO0 as GPIO26;
pub use GPIO0 as GPIO27;
pub use GPIO0 as GPIO14;
pub use GPIO0 as GPIO12;
pub use GPIO0 as GPIO13;
pub use GPIO0 as GPIO15;
pub use GPIO0 as GPIO2;
#[doc = "GPIO0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = ""]
pub mod gpio0;
pub use gpio0 as gpio4;
pub use gpio0 as gpio16;
pub use gpio0 as gpio17;
pub use gpio0 as gpio9;
pub use gpio0 as gpio10;
pub use gpio0 as gpio11;
pub use gpio0 as gpio6;
pub use gpio0 as gpio7;
pub use gpio0 as gpio8;
pub use gpio0 as gpio5;
pub use gpio0 as gpio18;
pub use gpio0 as gpio19;
pub use gpio0 as gpio20;
pub use gpio0 as gpio21;
pub use gpio0 as gpio22;
pub use gpio0 as gpio3;
pub use gpio0 as gpio1;
pub use gpio0 as gpio23;
pub use gpio0 as gpio24;
pub use GPIO0 as GPIO4;
pub use GPIO0 as GPIO16;
pub use GPIO0 as GPIO17;
pub use GPIO0 as GPIO9;
pub use GPIO0 as GPIO10;
pub use GPIO0 as GPIO11;
pub use GPIO0 as GPIO6;
pub use GPIO0 as GPIO7;
pub use GPIO0 as GPIO8;
pub use GPIO0 as GPIO5;
pub use GPIO0 as GPIO18;
pub use GPIO0 as GPIO19;
pub use GPIO0 as GPIO20;
pub use GPIO0 as GPIO21;
pub use GPIO0 as GPIO22;
pub use GPIO0 as GPIO3;
pub use GPIO0 as GPIO1;
pub use GPIO0 as GPIO23;
pub use GPIO0 as GPIO24;
