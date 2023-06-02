#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub test_conf: TEST_CONF,
    #[doc = "0x04 - "]
    pub coex_lp_clk_conf: COEX_LP_CLK_CONF,
    #[doc = "0x08 - "]
    pub clk_conf: CLK_CONF,
    #[doc = "0x0c - "]
    pub clk_conf_force_on: CLK_CONF_FORCE_ON,
    #[doc = "0x10 - "]
    pub tick_conf: TICK_CONF,
    #[doc = "0x14 - "]
    pub rst_conf: RST_CONF,
    #[doc = "0x18 - "]
    pub mem_conf: MEM_CONF,
    #[doc = "0x1c - "]
    pub date: DATE,
}
#[doc = "TEST_CONF (rw) register accessor: an alias for `Reg<TEST_CONF_SPEC>`"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = ""]
pub mod test_conf;
#[doc = "COEX_LP_CLK_CONF (rw) register accessor: an alias for `Reg<COEX_LP_CLK_CONF_SPEC>`"]
pub type COEX_LP_CLK_CONF = crate::Reg<coex_lp_clk_conf::COEX_LP_CLK_CONF_SPEC>;
#[doc = ""]
pub mod coex_lp_clk_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "CLK_CONF_FORCE_ON (rw) register accessor: an alias for `Reg<CLK_CONF_FORCE_ON_SPEC>`"]
pub type CLK_CONF_FORCE_ON = crate::Reg<clk_conf_force_on::CLK_CONF_FORCE_ON_SPEC>;
#[doc = ""]
pub mod clk_conf_force_on;
#[doc = "TICK_CONF (rw) register accessor: an alias for `Reg<TICK_CONF_SPEC>`"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = ""]
pub mod tick_conf;
#[doc = "RST_CONF (w) register accessor: an alias for `Reg<RST_CONF_SPEC>`"]
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
#[doc = ""]
pub mod rst_conf;
#[doc = "MEM_CONF (rw) register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = ""]
pub mod mem_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
