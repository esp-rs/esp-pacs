#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster UART%s, containing UART?_CONF, UART?_SCLK_CONF, UART?_MEM_LP_CTRL"]
pub struct UART {
    conf: CONF,
    clk_conf: CLK_CONF,
    mem_lp_ctrl: MEM_LP_CTRL,
}
impl UART {
    #[doc = "0x00 - UART0 configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - UART0_SCLK configuration register"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x08 - UART0 memory power control register"]
    #[inline(always)]
    pub const fn mem_lp_ctrl(&self) -> &MEM_LP_CTRL {
        &self.mem_lp_ctrl
    }
}
#[doc = "CONF (rw) register accessor: UART0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "UART0 configuration register"]
pub mod conf;
#[doc = "CLK_CONF (rw) register accessor: UART0_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "UART0_SCLK configuration register"]
pub mod clk_conf;
#[doc = "MEM_LP_CTRL (rw) register accessor: UART0 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_lp_ctrl`] module"]
pub type MEM_LP_CTRL = crate::Reg<mem_lp_ctrl::MEM_LP_CTRL_SPEC>;
#[doc = "UART0 memory power control register"]
pub mod mem_lp_ctrl;
