#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2, U?_CONF3"]
pub struct UNIT {
    conf0: CONF0,
    conf1: CONF1,
    conf2: CONF2,
    conf3: CONF3,
}
impl UNIT {
    #[doc = "0x00 - Configuration register 0 for unit"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - Configuration register 1 for unit 0"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x08 - Configuration register 2 for unit 0"]
    #[inline(always)]
    pub const fn conf2(&self) -> &CONF2 {
        &self.conf2
    }
    #[doc = "0x0c - Configuration register for unit 0's step value."]
    #[inline(always)]
    pub const fn conf3(&self) -> &CONF3 {
        &self.conf3
    }
}
#[doc = "CONF0 (rw) register accessor: Configuration register 0 for unit\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Configuration register 0 for unit"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: Configuration register 1 for unit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1 for unit 0"]
pub mod conf1;
#[doc = "CONF2 (rw) register accessor: Configuration register 2 for unit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf2`] module"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = "Configuration register 2 for unit 0"]
pub mod conf2;
#[doc = "CONF3 (rw) register accessor: Configuration register for unit 0's step value.\n\nYou can [`read`](crate::Reg::read) this register and get [`conf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf3`] module"]
pub type CONF3 = crate::Reg<conf3::CONF3_SPEC>;
#[doc = "Configuration register for unit 0's step value."]
pub mod conf3;
