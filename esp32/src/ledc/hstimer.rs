#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE"]
pub struct HSTIMER {
    conf: CONF,
    value: VALUE,
}
impl HSTIMER {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn value(&self) -> &VALUE {
        &self.value
    }
}
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "VALUE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = ""]
pub mod value;
