#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ex_clkout_conf: EX_CLKOUT_CONF,
    ex_oscclk_conf: EX_OSCCLK_CONF,
    ex_clk_ctrl: EX_CLK_CTRL,
    ex_phyinf_conf: EX_PHYINF_CONF,
    pd_sel: PD_SEL,
    _reserved5: [u8; 0xe8],
    ex_date: EX_DATE,
}
impl RegisterBlock {
    ///0x00 - RMII clock divider setting
    #[inline(always)]
    pub const fn ex_clkout_conf(&self) -> &EX_CLKOUT_CONF {
        &self.ex_clkout_conf
    }
    ///0x04 - RMII clock half and whole divider settings
    #[inline(always)]
    pub const fn ex_oscclk_conf(&self) -> &EX_OSCCLK_CONF {
        &self.ex_oscclk_conf
    }
    ///0x08 - Clock enable and external/internal clock selection
    #[inline(always)]
    pub const fn ex_clk_ctrl(&self) -> &EX_CLK_CTRL {
        &self.ex_clk_ctrl
    }
    ///0x0c - Selection of MII/RMII phy
    #[inline(always)]
    pub const fn ex_phyinf_conf(&self) -> &EX_PHYINF_CONF {
        &self.ex_phyinf_conf
    }
    ///0x10 - Ethernet RAM power-down enable
    #[inline(always)]
    pub const fn pd_sel(&self) -> &PD_SEL {
        &self.pd_sel
    }
    ///0xfc -
    #[inline(always)]
    pub const fn ex_date(&self) -> &EX_DATE {
        &self.ex_date
    }
}
/**EX_CLKOUT_CONF (rw) register accessor: RMII clock divider setting

You can [`read`](crate::generic::Reg::read) this register and get [`ex_clkout_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_clkout_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ex_clkout_conf`] module*/
pub type EX_CLKOUT_CONF = crate::Reg<ex_clkout_conf::EX_CLKOUT_CONF_SPEC>;
///RMII clock divider setting
pub mod ex_clkout_conf;
/**EX_OSCCLK_CONF (rw) register accessor: RMII clock half and whole divider settings

You can [`read`](crate::generic::Reg::read) this register and get [`ex_oscclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_oscclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ex_oscclk_conf`] module*/
pub type EX_OSCCLK_CONF = crate::Reg<ex_oscclk_conf::EX_OSCCLK_CONF_SPEC>;
///RMII clock half and whole divider settings
pub mod ex_oscclk_conf;
/**EX_CLK_CTRL (rw) register accessor: Clock enable and external/internal clock selection

You can [`read`](crate::generic::Reg::read) this register and get [`ex_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ex_clk_ctrl`] module*/
pub type EX_CLK_CTRL = crate::Reg<ex_clk_ctrl::EX_CLK_CTRL_SPEC>;
///Clock enable and external/internal clock selection
pub mod ex_clk_ctrl;
/**EX_PHYINF_CONF (rw) register accessor: Selection of MII/RMII phy

You can [`read`](crate::generic::Reg::read) this register and get [`ex_phyinf_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_phyinf_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ex_phyinf_conf`] module*/
pub type EX_PHYINF_CONF = crate::Reg<ex_phyinf_conf::EX_PHYINF_CONF_SPEC>;
///Selection of MII/RMII phy
pub mod ex_phyinf_conf;
/**PD_SEL (rw) register accessor: Ethernet RAM power-down enable

You can [`read`](crate::generic::Reg::read) this register and get [`pd_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pd_sel`] module*/
pub type PD_SEL = crate::Reg<pd_sel::PD_SEL_SPEC>;
///Ethernet RAM power-down enable
pub mod pd_sel;
/**EX_DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ex_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ex_date`] module*/
pub type EX_DATE = crate::Reg<ex_date::EX_DATE_SPEC>;
///
pub mod ex_date;
