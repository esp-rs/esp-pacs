#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    store0: STORE0,
    store1: STORE1,
    store2: STORE2,
    store3: STORE3,
    store4: STORE4,
    store5: STORE5,
    store6: STORE6,
    store7: STORE7,
    store8: STORE8,
    store9: STORE9,
    gpio_mux: GPIO_MUX,
    gpio_hold0: GPIO_HOLD0,
    gpio_hold1: GPIO_HOLD1,
    sys_cfg: SYS_CFG,
    cpucore0_cfg: CPUCORE0_CFG,
    io_mux: IO_MUX,
    ext_wakeup_cntl: EXT_WAKEUP_CNTL,
    usb: USB,
    lpbus: LPBUS,
    sdio_active: SDIO_ACTIVE,
    lpcore: LPCORE,
    sar_cct: SAR_CCT,
    jtag_sel: JTAG_SEL,
    _reserved23: [u8; 0x03a0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn store8(&self) -> &STORE8 {
        &self.store8
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn store9(&self) -> &STORE9 {
        &self.store9
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn gpio_mux(&self) -> &GPIO_MUX {
        &self.gpio_mux
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn gpio_hold0(&self) -> &GPIO_HOLD0 {
        &self.gpio_hold0
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn gpio_hold1(&self) -> &GPIO_HOLD1 {
        &self.gpio_hold1
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn sys_cfg(&self) -> &SYS_CFG {
        &self.sys_cfg
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn cpucore0_cfg(&self) -> &CPUCORE0_CFG {
        &self.cpucore0_cfg
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn io_mux(&self) -> &IO_MUX {
        &self.io_mux
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup_cntl(&self) -> &EXT_WAKEUP_CNTL {
        &self.ext_wakeup_cntl
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn usb(&self) -> &USB {
        &self.usb
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn lpbus(&self) -> &LPBUS {
        &self.lpbus
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn sdio_active(&self) -> &SDIO_ACTIVE {
        &self.sdio_active
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn lpcore(&self) -> &LPCORE {
        &self.lpcore
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn sar_cct(&self) -> &SAR_CCT {
        &self.sar_cct
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn jtag_sel(&self) -> &JTAG_SEL {
        &self.jtag_sel
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "STORE0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "need_des"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "need_des"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "need_des"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "need_des"]
pub mod store3;
#[doc = "STORE4 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "need_des"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "need_des"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "need_des"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "need_des"]
pub mod store7;
#[doc = "STORE8 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store8`] module"]
pub type STORE8 = crate::Reg<store8::STORE8_SPEC>;
#[doc = "need_des"]
pub mod store8;
#[doc = "STORE9 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store9`] module"]
pub type STORE9 = crate::Reg<store9::STORE9_SPEC>;
#[doc = "need_des"]
pub mod store9;
#[doc = "GPIO_MUX (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_mux`] module"]
pub type GPIO_MUX = crate::Reg<gpio_mux::GPIO_MUX_SPEC>;
#[doc = "need_des"]
pub mod gpio_mux;
#[doc = "GPIO_HOLD0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_hold0`] module"]
pub type GPIO_HOLD0 = crate::Reg<gpio_hold0::GPIO_HOLD0_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold0;
#[doc = "GPIO_HOLD1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_hold1`] module"]
pub type GPIO_HOLD1 = crate::Reg<gpio_hold1::GPIO_HOLD1_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold1;
#[doc = "SYS_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_cfg`] module"]
pub type SYS_CFG = crate::Reg<sys_cfg::SYS_CFG_SPEC>;
#[doc = "need_des"]
pub mod sys_cfg;
#[doc = "CPUCORE0_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpucore0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpucore0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucore0_cfg`] module"]
pub type CPUCORE0_CFG = crate::Reg<cpucore0_cfg::CPUCORE0_CFG_SPEC>;
#[doc = "need_des"]
pub mod cpucore0_cfg;
#[doc = "IO_MUX (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_mux`] module"]
pub type IO_MUX = crate::Reg<io_mux::IO_MUX_SPEC>;
#[doc = "need_des"]
pub mod io_mux;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_cntl`] module"]
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
#[doc = "need_des"]
pub mod ext_wakeup_cntl;
#[doc = "USB (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb`] module"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "need_des"]
pub mod usb;
#[doc = "LPBUS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbus`] module"]
pub type LPBUS = crate::Reg<lpbus::LPBUS_SPEC>;
#[doc = "need_des"]
pub mod lpbus;
#[doc = "SDIO_ACTIVE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_active::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_active::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_active`] module"]
pub type SDIO_ACTIVE = crate::Reg<sdio_active::SDIO_ACTIVE_SPEC>;
#[doc = "need_des"]
pub mod sdio_active;
#[doc = "LPCORE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcore::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcore::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcore`] module"]
pub type LPCORE = crate::Reg<lpcore::LPCORE_SPEC>;
#[doc = "need_des"]
pub mod lpcore;
#[doc = "SAR_CCT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cct`] module"]
pub type SAR_CCT = crate::Reg<sar_cct::SAR_CCT_SPEC>;
#[doc = "need_des"]
pub mod sar_cct;
#[doc = "JTAG_SEL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_sel`] module"]
pub type JTAG_SEL = crate::Reg<jtag_sel::JTAG_SEL_SPEC>;
#[doc = "need_des"]
pub mod jtag_sel;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
