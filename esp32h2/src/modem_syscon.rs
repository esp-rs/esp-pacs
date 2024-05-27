#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    test_conf: TEST_CONF,
    clk_conf: CLK_CONF,
    clk_conf_force_on: CLK_CONF_FORCE_ON,
    modem_rst_conf: MODEM_RST_CONF,
    clk_conf1: CLK_CONF1,
    clk_conf1_force_on: CLK_CONF1_FORCE_ON,
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
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x08 -
    #[inline(always)]
    pub const fn clk_conf_force_on(&self) -> &CLK_CONF_FORCE_ON {
        &self.clk_conf_force_on
    }
    ///0x0c -
    #[inline(always)]
    pub const fn modem_rst_conf(&self) -> &MODEM_RST_CONF {
        &self.modem_rst_conf
    }
    ///0x10 -
    #[inline(always)]
    pub const fn clk_conf1(&self) -> &CLK_CONF1 {
        &self.clk_conf1
    }
    ///0x14 -
    #[inline(always)]
    pub const fn clk_conf1_force_on(&self) -> &CLK_CONF1_FORCE_ON {
        &self.clk_conf1_force_on
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
/**MODEM_RST_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`modem_rst_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_rst_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@modem_rst_conf`] module*/
pub type MODEM_RST_CONF = crate::Reg<modem_rst_conf::MODEM_RST_CONF_SPEC>;
///
pub mod modem_rst_conf;
/**CLK_CONF1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf1`] module*/
pub type CLK_CONF1 = crate::Reg<clk_conf1::CLK_CONF1_SPEC>;
///
pub mod clk_conf1;
/**CLK_CONF1_FORCE_ON (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf1_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf1_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf1_force_on`] module*/
pub type CLK_CONF1_FORCE_ON = crate::Reg<clk_conf1_force_on::CLK_CONF1_FORCE_ON_SPEC>;
///
pub mod clk_conf1_force_on;
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
