#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_conf: CLK_CONF,
    modem_lp_timer_conf: MODEM_LP_TIMER_CONF,
    coex_lp_clk_conf: COEX_LP_CLK_CONF,
    _reserved_3_date: [u8; 0x04],
    etm_clk_conf: ETM_CLK_CONF,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn modem_lp_timer_conf(&self) -> &MODEM_LP_TIMER_CONF {
        &self.modem_lp_timer_conf
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn coex_lp_clk_conf(&self) -> &COEX_LP_CLK_CONF {
        &self.coex_lp_clk_conf
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ble_timer_clk_conf(&self) -> &BLE_TIMER_CLK_CONF {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn etm_clk_conf(&self) -> &ETM_CLK_CONF {
        &self.etm_clk_conf
    }
}
#[doc = "CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "MODEM_LP_TIMER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_lp_timer_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_lp_timer_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_lp_timer_conf`] module"]
pub type MODEM_LP_TIMER_CONF = crate::Reg<modem_lp_timer_conf::MODEM_LP_TIMER_CONF_SPEC>;
#[doc = ""]
pub mod modem_lp_timer_conf;
#[doc = "COEX_LP_CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_lp_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_lp_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_lp_clk_conf`] module"]
pub type COEX_LP_CLK_CONF = crate::Reg<coex_lp_clk_conf::COEX_LP_CLK_CONF_SPEC>;
#[doc = ""]
pub mod coex_lp_clk_conf;
#[doc = "BLE_TIMER_CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_timer_clk_conf`] module"]
pub type BLE_TIMER_CLK_CONF = crate::Reg<ble_timer_clk_conf::BLE_TIMER_CLK_CONF_SPEC>;
#[doc = ""]
pub mod ble_timer_clk_conf;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ETM_CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_clk_conf`] module"]
pub type ETM_CLK_CONF = crate::Reg<etm_clk_conf::ETM_CLK_CONF_SPEC>;
#[doc = ""]
pub mod etm_clk_conf;
