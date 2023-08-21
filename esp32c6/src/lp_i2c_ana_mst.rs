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
#[doc = "I2C0_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c0_ctrl`] module"]
pub type I2C0_CTRL = crate::Reg<i2c0_ctrl::I2C0_CTRL_SPEC>;
#[doc = "need_des"]
pub mod i2c0_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c0_conf`] module"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "need_des"]
pub mod i2c0_conf;
#[doc = "I2C0_DATA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c0_data`] module"]
pub type I2C0_DATA = crate::Reg<i2c0_data::I2C0_DATA_SPEC>;
#[doc = "need_des"]
pub mod i2c0_data;
#[doc = "ANA_CONF1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ana_conf1`] module"]
pub type ANA_CONF1 = crate::Reg<ana_conf1::ANA_CONF1_SPEC>;
#[doc = "need_des"]
pub mod ana_conf1;
#[doc = "NOUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nouse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nouse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`nouse`] module"]
pub type NOUSE = crate::Reg<nouse::NOUSE_SPEC>;
#[doc = "need_des"]
pub mod nouse;
#[doc = "DEVICE_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`device_en`] module"]
pub type DEVICE_EN = crate::Reg<device_en::DEVICE_EN_SPEC>;
#[doc = "need_des"]
pub mod device_en;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
