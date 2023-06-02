#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub test_conf: TEST_CONF,
    #[doc = "0x04 - "]
    pub clk_conf: CLK_CONF,
    #[doc = "0x08 - "]
    pub clk_conf_force_on: CLK_CONF_FORCE_ON,
    #[doc = "0x0c - "]
    pub clk_conf_power_st: CLK_CONF_POWER_ST,
    #[doc = "0x10 - "]
    pub modem_rst_conf: MODEM_RST_CONF,
    #[doc = "0x14 - "]
    pub clk_conf1: CLK_CONF1,
    #[doc = "0x18 - "]
    pub clk_conf1_force_on: CLK_CONF1_FORCE_ON,
    #[doc = "0x1c - "]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x20 - "]
    pub mem_conf: MEM_CONF,
    #[doc = "0x24 - "]
    pub date: DATE,
}
#[doc = "TEST_CONF (rw) register accessor: an alias for `Reg<TEST_CONF_SPEC>`"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = ""]
pub mod test_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "CLK_CONF_FORCE_ON (rw) register accessor: an alias for `Reg<CLK_CONF_FORCE_ON_SPEC>`"]
pub type CLK_CONF_FORCE_ON = crate::Reg<clk_conf_force_on::CLK_CONF_FORCE_ON_SPEC>;
#[doc = ""]
pub mod clk_conf_force_on;
#[doc = "CLK_CONF_POWER_ST (rw) register accessor: an alias for `Reg<CLK_CONF_POWER_ST_SPEC>`"]
pub type CLK_CONF_POWER_ST = crate::Reg<clk_conf_power_st::CLK_CONF_POWER_ST_SPEC>;
#[doc = ""]
pub mod clk_conf_power_st;
#[doc = "MODEM_RST_CONF (rw) register accessor: an alias for `Reg<MODEM_RST_CONF_SPEC>`"]
pub type MODEM_RST_CONF = crate::Reg<modem_rst_conf::MODEM_RST_CONF_SPEC>;
#[doc = ""]
pub mod modem_rst_conf;
#[doc = "CLK_CONF1 (rw) register accessor: an alias for `Reg<CLK_CONF1_SPEC>`"]
pub type CLK_CONF1 = crate::Reg<clk_conf1::CLK_CONF1_SPEC>;
#[doc = ""]
pub mod clk_conf1;
#[doc = "CLK_CONF1_FORCE_ON (rw) register accessor: an alias for `Reg<CLK_CONF1_FORCE_ON_SPEC>`"]
pub type CLK_CONF1_FORCE_ON = crate::Reg<clk_conf1_force_on::CLK_CONF1_FORCE_ON_SPEC>;
#[doc = ""]
pub mod clk_conf1_force_on;
#[doc = "WIFI_BB_CFG (rw) register accessor: an alias for `Reg<WIFI_BB_CFG_SPEC>`"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg;
#[doc = "MEM_CONF (rw) register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = ""]
pub mod mem_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
