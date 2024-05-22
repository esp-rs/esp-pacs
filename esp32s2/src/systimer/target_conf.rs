#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster TARGET%s_CONF, containing TARGET?_CONF"]
pub struct TARGET_CONF {
    target_conf: TARGET_CONF,
}
impl TARGET_CONF {
    #[doc = "0x00 - Configure work mode for system timer target 0"]
    #[inline(always)]
    pub const fn target_conf(&self) -> &TARGET_CONF {
        &self.target_conf
    }
}
#[doc = "TARGET_CONF (rw) register accessor: Configure work mode for system timer target 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_conf`] module"]
pub type TARGET_CONF = crate::Reg<target_conf::TARGET_CONF_SPEC>;
#[doc = "Configure work mode for system timer target 0"]
pub mod target_conf;
