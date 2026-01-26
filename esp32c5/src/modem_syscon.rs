#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    test_conf: TEST_CONF,
    clk_conf: CLK_CONF,
    clk_conf_force_on: CLK_CONF_FORCE_ON,
    clk_conf_power_st: CLK_CONF_POWER_ST,
    modem_rst_conf: MODEM_RST_CONF,
    clk_conf1: CLK_CONF1,
    wifi_bb_cfg: WIFI_BB_CFG,
    fe_cfg: FE_CFG,
    mem_rf1_conf: MEM_RF1_CONF,
    mem_rf2_conf: MEM_RF2_CONF,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - TEST_CONF"]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    #[doc = "0x04 - CLK_CONF"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x08 - CLK_CONF_FORCE_ON"]
    #[inline(always)]
    pub const fn clk_conf_force_on(&self) -> &CLK_CONF_FORCE_ON {
        &self.clk_conf_force_on
    }
    #[doc = "0x0c - CLK_CONF_POWER_ST"]
    #[inline(always)]
    pub const fn clk_conf_power_st(&self) -> &CLK_CONF_POWER_ST {
        &self.clk_conf_power_st
    }
    #[doc = "0x10 - MODEM_RST_CONF"]
    #[inline(always)]
    pub const fn modem_rst_conf(&self) -> &MODEM_RST_CONF {
        &self.modem_rst_conf
    }
    #[doc = "0x14 - CLK_CONF1"]
    #[inline(always)]
    pub const fn clk_conf1(&self) -> &CLK_CONF1 {
        &self.clk_conf1
    }
    #[doc = "0x18 - WIFI_BB_CFG"]
    #[inline(always)]
    pub const fn wifi_bb_cfg(&self) -> &WIFI_BB_CFG {
        &self.wifi_bb_cfg
    }
    #[doc = "0x1c - FE_CFG"]
    #[inline(always)]
    pub const fn fe_cfg(&self) -> &FE_CFG {
        &self.fe_cfg
    }
    #[doc = "0x20 - MEM_RF1_CONF"]
    #[inline(always)]
    pub const fn mem_rf1_conf(&self) -> &MEM_RF1_CONF {
        &self.mem_rf1_conf
    }
    #[doc = "0x24 - MEM_RF2_CONF"]
    #[inline(always)]
    pub const fn mem_rf2_conf(&self) -> &MEM_RF2_CONF {
        &self.mem_rf2_conf
    }
    #[doc = "0x28 - DATE"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "TEST_CONF (rw) register accessor: TEST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "TEST_CONF"]
pub mod test_conf;
#[doc = "CLK_CONF (rw) register accessor: CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "CLK_CONF"]
pub mod clk_conf;
#[doc = "CLK_CONF_FORCE_ON (rw) register accessor: CLK_CONF_FORCE_ON\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf_force_on::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf_force_on::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf_force_on`] module"]
pub type CLK_CONF_FORCE_ON = crate::Reg<clk_conf_force_on::CLK_CONF_FORCE_ON_SPEC>;
#[doc = "CLK_CONF_FORCE_ON"]
pub mod clk_conf_force_on;
#[doc = "CLK_CONF_POWER_ST (rw) register accessor: CLK_CONF_POWER_ST\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf_power_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf_power_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf_power_st`] module"]
pub type CLK_CONF_POWER_ST = crate::Reg<clk_conf_power_st::CLK_CONF_POWER_ST_SPEC>;
#[doc = "CLK_CONF_POWER_ST"]
pub mod clk_conf_power_st;
#[doc = "MODEM_RST_CONF (rw) register accessor: MODEM_RST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_rst_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_rst_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_rst_conf`] module"]
pub type MODEM_RST_CONF = crate::Reg<modem_rst_conf::MODEM_RST_CONF_SPEC>;
#[doc = "MODEM_RST_CONF"]
pub mod modem_rst_conf;
#[doc = "CLK_CONF1 (rw) register accessor: CLK_CONF1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf1`] module"]
pub type CLK_CONF1 = crate::Reg<clk_conf1::CLK_CONF1_SPEC>;
#[doc = "CLK_CONF1"]
pub mod clk_conf1;
#[doc = "WIFI_BB_CFG (rw) register accessor: WIFI_BB_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_bb_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_cfg`] module"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = "WIFI_BB_CFG"]
pub mod wifi_bb_cfg;
#[doc = "FE_CFG (rw) register accessor: FE_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`fe_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fe_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fe_cfg`] module"]
pub type FE_CFG = crate::Reg<fe_cfg::FE_CFG_SPEC>;
#[doc = "FE_CFG"]
pub mod fe_cfg;
#[doc = "MEM_RF1_CONF (rw) register accessor: MEM_RF1_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rf1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rf1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rf1_conf`] module"]
pub type MEM_RF1_CONF = crate::Reg<mem_rf1_conf::MEM_RF1_CONF_SPEC>;
#[doc = "MEM_RF1_CONF"]
pub mod mem_rf1_conf;
#[doc = "MEM_RF2_CONF (rw) register accessor: MEM_RF2_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rf2_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rf2_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rf2_conf`] module"]
pub type MEM_RF2_CONF = crate::Reg<mem_rf2_conf::MEM_RF2_CONF_SPEC>;
#[doc = "MEM_RF2_CONF"]
pub mod mem_rf2_conf;
pub use crate::aes::date;
pub use crate::aes::DATE;
