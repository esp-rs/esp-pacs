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
#[doc = "STORE0 (rw) register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "need_des"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "need_des"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "need_des"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "need_des"]
pub mod store3;
#[doc = "STORE4 (rw) register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "need_des"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "need_des"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "need_des"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "need_des"]
pub mod store7;
#[doc = "STORE8 (rw) register accessor: an alias for `Reg<STORE8_SPEC>`"]
pub type STORE8 = crate::Reg<store8::STORE8_SPEC>;
#[doc = "need_des"]
pub mod store8;
#[doc = "STORE9 (rw) register accessor: an alias for `Reg<STORE9_SPEC>`"]
pub type STORE9 = crate::Reg<store9::STORE9_SPEC>;
#[doc = "need_des"]
pub mod store9;
#[doc = "GPIO_MUX (rw) register accessor: an alias for `Reg<GPIO_MUX_SPEC>`"]
pub type GPIO_MUX = crate::Reg<gpio_mux::GPIO_MUX_SPEC>;
#[doc = "need_des"]
pub mod gpio_mux;
#[doc = "GPIO_HOLD0 (rw) register accessor: an alias for `Reg<GPIO_HOLD0_SPEC>`"]
pub type GPIO_HOLD0 = crate::Reg<gpio_hold0::GPIO_HOLD0_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold0;
#[doc = "GPIO_HOLD1 (rw) register accessor: an alias for `Reg<GPIO_HOLD1_SPEC>`"]
pub type GPIO_HOLD1 = crate::Reg<gpio_hold1::GPIO_HOLD1_SPEC>;
#[doc = "need_des"]
pub mod gpio_hold1;
#[doc = "SYS_CFG (rw) register accessor: an alias for `Reg<SYS_CFG_SPEC>`"]
pub type SYS_CFG = crate::Reg<sys_cfg::SYS_CFG_SPEC>;
#[doc = "need_des"]
pub mod sys_cfg;
#[doc = "CPUCORE0_CFG (rw) register accessor: an alias for `Reg<CPUCORE0_CFG_SPEC>`"]
pub type CPUCORE0_CFG = crate::Reg<cpucore0_cfg::CPUCORE0_CFG_SPEC>;
#[doc = "need_des"]
pub mod cpucore0_cfg;
#[doc = "IO_MUX (rw) register accessor: an alias for `Reg<IO_MUX_SPEC>`"]
pub type IO_MUX = crate::Reg<io_mux::IO_MUX_SPEC>;
#[doc = "need_des"]
pub mod io_mux;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: an alias for `Reg<EXT_WAKEUP_CNTL_SPEC>`"]
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
#[doc = "need_des"]
pub mod ext_wakeup_cntl;
#[doc = "USB (rw) register accessor: an alias for `Reg<USB_SPEC>`"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "need_des"]
pub mod usb;
#[doc = "LPBUS (rw) register accessor: an alias for `Reg<LPBUS_SPEC>`"]
pub type LPBUS = crate::Reg<lpbus::LPBUS_SPEC>;
#[doc = "need_des"]
pub mod lpbus;
#[doc = "SDIO_ACTIVE (rw) register accessor: an alias for `Reg<SDIO_ACTIVE_SPEC>`"]
pub type SDIO_ACTIVE = crate::Reg<sdio_active::SDIO_ACTIVE_SPEC>;
#[doc = "need_des"]
pub mod sdio_active;
#[doc = "LPCORE (rw) register accessor: an alias for `Reg<LPCORE_SPEC>`"]
pub type LPCORE = crate::Reg<lpcore::LPCORE_SPEC>;
#[doc = "need_des"]
pub mod lpcore;
#[doc = "SAR_CCT (rw) register accessor: an alias for `Reg<SAR_CCT_SPEC>`"]
pub type SAR_CCT = crate::Reg<sar_cct::SAR_CCT_SPEC>;
#[doc = "need_des"]
pub mod sar_cct;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
