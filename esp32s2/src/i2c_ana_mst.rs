#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config2: CONFIG2,
    _reserved1: [u8; 0x40],
    config1: CONFIG1,
    config0: CONFIG0,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C RTC Configuration register 2"]
    #[inline(always)]
    pub const fn config2(&self) -> &CONFIG2 {
        &self.config2
    }
    #[doc = "0x44 - I2C RTC Configuration register 1"]
    #[inline(always)]
    pub const fn config1(&self) -> &CONFIG1 {
        &self.config1
    }
    #[doc = "0x48 - I2C RTC Configuration register 0"]
    #[inline(always)]
    pub const fn config0(&self) -> &CONFIG0 {
        &self.config0
    }
}
#[doc = "CONFIG2 (rw) register accessor: I2C RTC Configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config2`] module"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "I2C RTC Configuration register 2"]
pub mod config2;
#[doc = "CONFIG1 (rw) register accessor: I2C RTC Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`] module"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "I2C RTC Configuration register 1"]
pub mod config1;
#[doc = "CONFIG0 (rw) register accessor: I2C RTC Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`] module"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "I2C RTC Configuration register 0"]
pub mod config0;
