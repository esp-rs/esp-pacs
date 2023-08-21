#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub store0: STORE0,
    #[doc = "0x04 - need_des"]
    pub store1: STORE1,
    #[doc = "0x08 - need_des"]
    pub store2: STORE2,
    #[doc = "0x0c - need_des"]
    pub store3: STORE3,
    #[doc = "0x10 - need_des"]
    pub store4: STORE4,
    #[doc = "0x14 - need_des"]
    pub store5: STORE5,
    #[doc = "0x18 - need_des"]
    pub store6: STORE6,
    #[doc = "0x1c - need_des"]
    pub store7: STORE7,
    #[doc = "0x20 - need_des"]
    pub store8: STORE8,
    #[doc = "0x24 - need_des"]
    pub store9: STORE9,
    #[doc = "0x28 - need_des"]
    pub gpio_mux: GPIO_MUX,
    #[doc = "0x2c - need_des"]
    pub gpio_hold0: GPIO_HOLD0,
    #[doc = "0x30 - need_des"]
    pub gpio_hold1: GPIO_HOLD1,
    #[doc = "0x34 - need_des"]
    pub sys_cfg: SYS_CFG,
    #[doc = "0x38 - need_des"]
    pub cpucore0_cfg: CPUCORE0_CFG,
    #[doc = "0x3c - need_des"]
    pub io_mux: IO_MUX,
    #[doc = "0x40 - need_des"]
    pub ext_wakeup_cntl: EXT_WAKEUP_CNTL,
    #[doc = "0x44 - need_des"]
    pub usb: USB,
    #[doc = "0x48 - need_des"]
    pub lpbus: LPBUS,
    #[doc = "0x4c - need_des"]
    pub sdio_active: SDIO_ACTIVE,
    #[doc = "0x50 - need_des"]
    pub lpcore: LPCORE,
    #[doc = "0x54 - need_des"]
    pub sar_cct: SAR_CCT,
    _reserved22: [u8; 0x03a4],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "STORE0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "need_des"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "need_des"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "need_des"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "need_des"]
pub mod store3;
#[doc = "STORE4 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "need_des"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "need_des"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "need_des"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "need_des"]
pub mod store7;
#[doc = "STORE8 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store8`] module"]
pub type STORE8 = crate::Reg<store8::STORE8_SPEC>;
#[doc = "need_des"]
pub mod store8;
#[doc = "STORE9 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store9`] module"]
pub type STORE9 = crate::Reg<store9::STORE9_SPEC>;
#[doc = "need_des"]
pub mod store9;
#[doc = "GPIO_MUX (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_mux`] module"]
pub type GPIO_MUX = crate::Reg<gpio_mux::GPIO_MUX_SPEC>;
#[doc = "need_des"]
pub mod gpio_mux;
#[doc = "GPIO_HOLD0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_hold0`] module"]
pub type GPIO_HOLD0 = crate::Reg<gpio_hold0::GPIO_HOLD0_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold0;
#[doc = "GPIO_HOLD1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_hold1`] module"]
pub type GPIO_HOLD1 = crate::Reg<gpio_hold1::GPIO_HOLD1_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold1;
#[doc = "SYS_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_cfg`] module"]
pub type SYS_CFG = crate::Reg<sys_cfg::SYS_CFG_SPEC>;
#[doc = "need_des"]
pub mod sys_cfg;
#[doc = "CPUCORE0_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpucore0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpucore0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpucore0_cfg`] module"]
pub type CPUCORE0_CFG = crate::Reg<cpucore0_cfg::CPUCORE0_CFG_SPEC>;
#[doc = "need_des"]
pub mod cpucore0_cfg;
#[doc = "IO_MUX (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux`] module"]
pub type IO_MUX = crate::Reg<io_mux::IO_MUX_SPEC>;
#[doc = "need_des"]
pub mod io_mux;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup_cntl`] module"]
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
#[doc = "need_des"]
pub mod ext_wakeup_cntl;
#[doc = "USB (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb`] module"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "need_des"]
pub mod usb;
#[doc = "LPBUS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpbus`] module"]
pub type LPBUS = crate::Reg<lpbus::LPBUS_SPEC>;
#[doc = "need_des"]
pub mod lpbus;
#[doc = "SDIO_ACTIVE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_active::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_active::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_active`] module"]
pub type SDIO_ACTIVE = crate::Reg<sdio_active::SDIO_ACTIVE_SPEC>;
#[doc = "need_des"]
pub mod sdio_active;
#[doc = "LPCORE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcore::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcore::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpcore`] module"]
pub type LPCORE = crate::Reg<lpcore::LPCORE_SPEC>;
#[doc = "need_des"]
pub mod lpcore;
#[doc = "SAR_CCT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_cct`] module"]
pub type SAR_CCT = crate::Reg<sar_cct::SAR_CCT_SPEC>;
#[doc = "need_des"]
pub mod sar_cct;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
