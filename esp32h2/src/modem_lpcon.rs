#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    test_conf: TEST_CONF,
    coex_lp_clk_conf: COEX_LP_CLK_CONF,
    clk_conf: CLK_CONF,
    clk_conf_force_on: CLK_CONF_FORCE_ON,
    tick_conf: TICK_CONF,
    rst_conf: RST_CONF,
    mem_conf: MEM_CONF,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    ///0x04 -
    #[inline(always)]
    pub const fn coex_lp_clk_conf(&self) -> &COEX_LP_CLK_CONF {
        &self.coex_lp_clk_conf
    }
    ///0x08 -
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x0c -
    #[inline(always)]
    pub const fn clk_conf_force_on(&self) -> &CLK_CONF_FORCE_ON {
        &self.clk_conf_force_on
    }
    ///0x10 -
    #[inline(always)]
    pub const fn tick_conf(&self) -> &TICK_CONF {
        &self.tick_conf
    }
    ///0x14 -
    #[inline(always)]
    pub const fn rst_conf(&self) -> &RST_CONF {
        &self.rst_conf
    }
    ///0x18 -
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MEM_CONF {
        &self.mem_conf
    }
    ///0x1c -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**TEST_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`test_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test_conf`] module*/
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
///
pub mod test_conf;
/**COEX_LP_CLK_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`coex_lp_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_lp_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@coex_lp_clk_conf`] module*/
pub type COEX_LP_CLK_CONF = crate::Reg<coex_lp_clk_conf::COEX_LP_CLK_CONF_SPEC>;
///
pub mod coex_lp_clk_conf;
/**CLK_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf`] module*/
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
///
pub mod clk_conf;
/**CLK_CONF_FORCE_ON (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf_force_on`] module*/
pub type CLK_CONF_FORCE_ON = crate::Reg<clk_conf_force_on::CLK_CONF_FORCE_ON_SPEC>;
///
pub mod clk_conf_force_on;
/**TICK_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tick_conf`] module*/
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
///
pub mod tick_conf;
/**RST_CONF (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_conf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rst_conf`] module*/
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
///
pub mod rst_conf;
/**MEM_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_conf`] module*/
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
///
pub mod mem_conf;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
