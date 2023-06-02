#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub clk_conf: CLK_CONF,
    #[doc = "0x04 - "]
    pub modem_lp_timer_conf: MODEM_LP_TIMER_CONF,
    #[doc = "0x08 - "]
    pub coex_lp_clk_conf: COEX_LP_CLK_CONF,
    _reserved_3_date: [u8; 0x04],
    #[doc = "0x10 - "]
    pub etm_clk_conf: ETM_CLK_CONF,
}
impl RegisterBlock {
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ble_timer_clk_conf(&self) -> &BLE_TIMER_CLK_CONF {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
}
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "MODEM_LP_TIMER_CONF (rw) register accessor: an alias for `Reg<MODEM_LP_TIMER_CONF_SPEC>`"]
pub type MODEM_LP_TIMER_CONF = crate::Reg<modem_lp_timer_conf::MODEM_LP_TIMER_CONF_SPEC>;
#[doc = ""]
pub mod modem_lp_timer_conf;
#[doc = "COEX_LP_CLK_CONF (rw) register accessor: an alias for `Reg<COEX_LP_CLK_CONF_SPEC>`"]
pub type COEX_LP_CLK_CONF = crate::Reg<coex_lp_clk_conf::COEX_LP_CLK_CONF_SPEC>;
#[doc = ""]
pub mod coex_lp_clk_conf;
#[doc = "BLE_TIMER_CLK_CONF (rw) register accessor: an alias for `Reg<BLE_TIMER_CLK_CONF_SPEC>`"]
pub type BLE_TIMER_CLK_CONF = crate::Reg<ble_timer_clk_conf::BLE_TIMER_CLK_CONF_SPEC>;
#[doc = ""]
pub mod ble_timer_clk_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ETM_CLK_CONF (rw) register accessor: an alias for `Reg<ETM_CLK_CONF_SPEC>`"]
pub type ETM_CLK_CONF = crate::Reg<etm_clk_conf::ETM_CLK_CONF_SPEC>;
#[doc = ""]
pub mod etm_clk_conf;
