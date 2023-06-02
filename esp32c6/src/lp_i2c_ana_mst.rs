#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub i2c0_ctrl: I2C0_CTRL,
    #[doc = "0x04 - need_des"]
    pub i2c0_conf: I2C0_CONF,
    #[doc = "0x08 - need_des"]
    pub i2c0_data: I2C0_DATA,
    #[doc = "0x0c - need_des"]
    pub ana_conf1: ANA_CONF1,
    #[doc = "0x10 - need_des"]
    pub nouse: NOUSE,
    #[doc = "0x14 - need_des"]
    pub device_en: DEVICE_EN,
    _reserved6: [u8; 0x03e4],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "I2C0_CTRL (rw) register accessor: an alias for `Reg<I2C0_CTRL_SPEC>`"]
pub type I2C0_CTRL = crate::Reg<i2c0_ctrl::I2C0_CTRL_SPEC>;
#[doc = "need_des"]
pub mod i2c0_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: an alias for `Reg<I2C0_CONF_SPEC>`"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "need_des"]
pub mod i2c0_conf;
#[doc = "I2C0_DATA (rw) register accessor: an alias for `Reg<I2C0_DATA_SPEC>`"]
pub type I2C0_DATA = crate::Reg<i2c0_data::I2C0_DATA_SPEC>;
#[doc = "need_des"]
pub mod i2c0_data;
#[doc = "ANA_CONF1 (rw) register accessor: an alias for `Reg<ANA_CONF1_SPEC>`"]
pub type ANA_CONF1 = crate::Reg<ana_conf1::ANA_CONF1_SPEC>;
#[doc = "need_des"]
pub mod ana_conf1;
#[doc = "NOUSE (rw) register accessor: an alias for `Reg<NOUSE_SPEC>`"]
pub type NOUSE = crate::Reg<nouse::NOUSE_SPEC>;
#[doc = "need_des"]
pub mod nouse;
#[doc = "DEVICE_EN (rw) register accessor: an alias for `Reg<DEVICE_EN_SPEC>`"]
pub type DEVICE_EN = crate::Reg<device_en::DEVICE_EN_SPEC>;
#[doc = "need_des"]
pub mod device_en;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
