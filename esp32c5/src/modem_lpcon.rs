#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    test_conf: TEST_CONF,
    lp_timer_conf: LP_TIMER_CONF,
    coex_lp_clk_conf: COEX_LP_CLK_CONF,
    wifi_lp_clk_conf: WIFI_LP_CLK_CONF,
    modem_src_clk_conf: MODEM_SRC_CLK_CONF,
    modem_32k_clk_conf: MODEM_32K_CLK_CONF,
    clk_conf: CLK_CONF,
    clk_conf_force_on: CLK_CONF_FORCE_ON,
    clk_conf_power_st: CLK_CONF_POWER_ST,
    rst_conf: RST_CONF,
    tick_conf: TICK_CONF,
    mem_conf: MEM_CONF,
    mem_rf1_aux_ctrl: MEM_RF1_AUX_CTRL,
    mem_rf2_aux_ctrl: MEM_RF2_AUX_CTRL,
    apb_mem_sel: APB_MEM_SEL,
    dcmem_valid_0: DCMEM_VALID_0,
    dcmem_valid_1: DCMEM_VALID_1,
    dcmem_valid_2: DCMEM_VALID_2,
    dcmem_valid_3: DCMEM_VALID_3,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - TEST_CONF"]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    #[doc = "0x04 - LP_TIMER_CONF"]
    #[inline(always)]
    pub const fn lp_timer_conf(&self) -> &LP_TIMER_CONF {
        &self.lp_timer_conf
    }
    #[doc = "0x08 - COEX_LP_CLK_CONF"]
    #[inline(always)]
    pub const fn coex_lp_clk_conf(&self) -> &COEX_LP_CLK_CONF {
        &self.coex_lp_clk_conf
    }
    #[doc = "0x0c - WIFI_LP_CLK_CONF"]
    #[inline(always)]
    pub const fn wifi_lp_clk_conf(&self) -> &WIFI_LP_CLK_CONF {
        &self.wifi_lp_clk_conf
    }
    #[doc = "0x10 - MODEM_SRC_CLK_CONF"]
    #[inline(always)]
    pub const fn modem_src_clk_conf(&self) -> &MODEM_SRC_CLK_CONF {
        &self.modem_src_clk_conf
    }
    #[doc = "0x14 - MODEM_32K_CLK_CONF"]
    #[inline(always)]
    pub const fn modem_32k_clk_conf(&self) -> &MODEM_32K_CLK_CONF {
        &self.modem_32k_clk_conf
    }
    #[doc = "0x18 - CLK_CONF"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x1c - CLK_CONF_FORCE_ON"]
    #[inline(always)]
    pub const fn clk_conf_force_on(&self) -> &CLK_CONF_FORCE_ON {
        &self.clk_conf_force_on
    }
    #[doc = "0x20 - CLK_CONF_POWER_ST"]
    #[inline(always)]
    pub const fn clk_conf_power_st(&self) -> &CLK_CONF_POWER_ST {
        &self.clk_conf_power_st
    }
    #[doc = "0x24 - RST_CONF"]
    #[inline(always)]
    pub const fn rst_conf(&self) -> &RST_CONF {
        &self.rst_conf
    }
    #[doc = "0x28 - TICK_CONF"]
    #[inline(always)]
    pub const fn tick_conf(&self) -> &TICK_CONF {
        &self.tick_conf
    }
    #[doc = "0x2c - MEM_CONF"]
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MEM_CONF {
        &self.mem_conf
    }
    #[doc = "0x30 - MEM_RF1_AUX_CTRL"]
    #[inline(always)]
    pub const fn mem_rf1_aux_ctrl(&self) -> &MEM_RF1_AUX_CTRL {
        &self.mem_rf1_aux_ctrl
    }
    #[doc = "0x34 - MEM_RF2_AUX_CTRL"]
    #[inline(always)]
    pub const fn mem_rf2_aux_ctrl(&self) -> &MEM_RF2_AUX_CTRL {
        &self.mem_rf2_aux_ctrl
    }
    #[doc = "0x38 - APB_MEM_SEL"]
    #[inline(always)]
    pub const fn apb_mem_sel(&self) -> &APB_MEM_SEL {
        &self.apb_mem_sel
    }
    #[doc = "0x3c - DCMEM_VALID_0"]
    #[inline(always)]
    pub const fn dcmem_valid_0(&self) -> &DCMEM_VALID_0 {
        &self.dcmem_valid_0
    }
    #[doc = "0x40 - DCMEM_VALID_1"]
    #[inline(always)]
    pub const fn dcmem_valid_1(&self) -> &DCMEM_VALID_1 {
        &self.dcmem_valid_1
    }
    #[doc = "0x44 - DCMEM_VALID_2"]
    #[inline(always)]
    pub const fn dcmem_valid_2(&self) -> &DCMEM_VALID_2 {
        &self.dcmem_valid_2
    }
    #[doc = "0x48 - DCMEM_VALID_3"]
    #[inline(always)]
    pub const fn dcmem_valid_3(&self) -> &DCMEM_VALID_3 {
        &self.dcmem_valid_3
    }
    #[doc = "0x4c - DATE"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "TEST_CONF (rw) register accessor: TEST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "TEST_CONF"]
pub mod test_conf;
#[doc = "LP_TIMER_CONF (rw) register accessor: LP_TIMER_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_conf`] module"]
pub type LP_TIMER_CONF = crate::Reg<lp_timer_conf::LP_TIMER_CONF_SPEC>;
#[doc = "LP_TIMER_CONF"]
pub mod lp_timer_conf;
#[doc = "COEX_LP_CLK_CONF (rw) register accessor: COEX_LP_CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_lp_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_lp_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_lp_clk_conf`] module"]
pub type COEX_LP_CLK_CONF = crate::Reg<coex_lp_clk_conf::COEX_LP_CLK_CONF_SPEC>;
#[doc = "COEX_LP_CLK_CONF"]
pub mod coex_lp_clk_conf;
#[doc = "WIFI_LP_CLK_CONF (rw) register accessor: WIFI_LP_CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_lp_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_lp_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_lp_clk_conf`] module"]
pub type WIFI_LP_CLK_CONF = crate::Reg<wifi_lp_clk_conf::WIFI_LP_CLK_CONF_SPEC>;
#[doc = "WIFI_LP_CLK_CONF"]
pub mod wifi_lp_clk_conf;
#[doc = "MODEM_SRC_CLK_CONF (rw) register accessor: MODEM_SRC_CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_src_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_src_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_src_clk_conf`] module"]
pub type MODEM_SRC_CLK_CONF = crate::Reg<modem_src_clk_conf::MODEM_SRC_CLK_CONF_SPEC>;
#[doc = "MODEM_SRC_CLK_CONF"]
pub mod modem_src_clk_conf;
#[doc = "MODEM_32K_CLK_CONF (rw) register accessor: MODEM_32K_CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_32k_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_32k_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_32k_clk_conf`] module"]
pub type MODEM_32K_CLK_CONF = crate::Reg<modem_32k_clk_conf::MODEM_32K_CLK_CONF_SPEC>;
#[doc = "MODEM_32K_CLK_CONF"]
pub mod modem_32k_clk_conf;
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
#[doc = "RST_CONF (w) register accessor: RST_CONF\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_conf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_conf`] module"]
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
#[doc = "RST_CONF"]
pub mod rst_conf;
#[doc = "TICK_CONF (rw) register accessor: TICK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`tick_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tick_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tick_conf`] module"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = "TICK_CONF"]
pub mod tick_conf;
#[doc = "MEM_CONF (rw) register accessor: MEM_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "MEM_CONF"]
pub mod mem_conf;
#[doc = "MEM_RF1_AUX_CTRL (rw) register accessor: MEM_RF1_AUX_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rf1_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rf1_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rf1_aux_ctrl`] module"]
pub type MEM_RF1_AUX_CTRL = crate::Reg<mem_rf1_aux_ctrl::MEM_RF1_AUX_CTRL_SPEC>;
#[doc = "MEM_RF1_AUX_CTRL"]
pub mod mem_rf1_aux_ctrl;
#[doc = "MEM_RF2_AUX_CTRL (rw) register accessor: MEM_RF2_AUX_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rf2_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rf2_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rf2_aux_ctrl`] module"]
pub type MEM_RF2_AUX_CTRL = crate::Reg<mem_rf2_aux_ctrl::MEM_RF2_AUX_CTRL_SPEC>;
#[doc = "MEM_RF2_AUX_CTRL"]
pub mod mem_rf2_aux_ctrl;
#[doc = "APB_MEM_SEL (rw) register accessor: APB_MEM_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_mem_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_mem_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_mem_sel`] module"]
pub type APB_MEM_SEL = crate::Reg<apb_mem_sel::APB_MEM_SEL_SPEC>;
#[doc = "APB_MEM_SEL"]
pub mod apb_mem_sel;
#[doc = "DCMEM_VALID_0 (r) register accessor: DCMEM_VALID_0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmem_valid_0`] module"]
pub type DCMEM_VALID_0 = crate::Reg<dcmem_valid_0::DCMEM_VALID_0_SPEC>;
#[doc = "DCMEM_VALID_0"]
pub mod dcmem_valid_0;
#[doc = "DCMEM_VALID_1 (r) register accessor: DCMEM_VALID_1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmem_valid_1`] module"]
pub type DCMEM_VALID_1 = crate::Reg<dcmem_valid_1::DCMEM_VALID_1_SPEC>;
#[doc = "DCMEM_VALID_1"]
pub mod dcmem_valid_1;
#[doc = "DCMEM_VALID_2 (r) register accessor: DCMEM_VALID_2\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmem_valid_2`] module"]
pub type DCMEM_VALID_2 = crate::Reg<dcmem_valid_2::DCMEM_VALID_2_SPEC>;
#[doc = "DCMEM_VALID_2"]
pub mod dcmem_valid_2;
#[doc = "DCMEM_VALID_3 (r) register accessor: DCMEM_VALID_3\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmem_valid_3`] module"]
pub type DCMEM_VALID_3 = crate::Reg<dcmem_valid_3::DCMEM_VALID_3_SPEC>;
#[doc = "DCMEM_VALID_3"]
pub mod dcmem_valid_3;
pub use crate::aes::date;
pub use crate::aes::DATE;
