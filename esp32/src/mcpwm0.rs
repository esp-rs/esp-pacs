#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_cfg: CLK_CFG,
    timer: [TIMER; 3],
    timer_synci_cfg: TIMER_SYNCI_CFG,
    operator_timersel: OPERATOR_TIMERSEL,
    ch: [CH; 3],
    fault_detect: FAULT_DETECT,
    cap_timer_cfg: CAP_TIMER_CFG,
    cap_timer_phase: CAP_TIMER_PHASE,
    cap_ch_cfg: [CAP_CH_CFG; 3],
    cap_ch: [CAP_CH; 3],
    cap_status: CAP_STATUS,
    update_cfg: UPDATE_CFG,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    clk: CLK,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    #[doc = "0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &TIMER {
        &self.timer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &TIMER> {
        self.timer.iter()
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn timer_synci_cfg(&self) -> &TIMER_SYNCI_CFG {
        &self.timer_synci_cfg
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn operator_timersel(&self) -> &OPERATOR_TIMERSEL {
        &self.operator_timersel
    }
    #[doc = "0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn fault_detect(&self) -> &FAULT_DETECT {
        &self.fault_detect
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn cap_timer_cfg(&self) -> &CAP_TIMER_CFG {
        &self.cap_timer_cfg
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn cap_timer_phase(&self) -> &CAP_TIMER_PHASE {
        &self.cap_timer_phase
    }
    #[doc = "0xf0..0xfc - Capture channel %s configuration and enable"]
    #[inline(always)]
    pub const fn cap_ch_cfg(&self, n: usize) -> &CAP_CH_CFG {
        &self.cap_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0xfc - Capture channel %s configuration and enable"]
    #[inline(always)]
    pub fn cap_ch_cfg_iter(&self) -> impl Iterator<Item = &CAP_CH_CFG> {
        self.cap_ch_cfg.iter()
    }
    #[doc = "0xf0 - Capture channel 0 configuration and enable"]
    #[inline(always)]
    pub const fn cap_ch0_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(0)
    }
    #[doc = "0xf4 - Capture channel 1 configuration and enable"]
    #[inline(always)]
    pub const fn cap_ch1_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(1)
    }
    #[doc = "0xf8 - Capture channel 2 configuration and enable"]
    #[inline(always)]
    pub const fn cap_ch2_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(2)
    }
    #[doc = "0xfc..0x108 - Value of last capture on channel %s"]
    #[inline(always)]
    pub const fn cap_ch(&self, n: usize) -> &CAP_CH {
        &self.cap_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x108 - Value of last capture on channel %s"]
    #[inline(always)]
    pub fn cap_ch_iter(&self) -> impl Iterator<Item = &CAP_CH> {
        self.cap_ch.iter()
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn cap_status(&self) -> &CAP_STATUS {
        &self.cap_status
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn update_cfg(&self) -> &UPDATE_CFG {
        &self.update_cfg
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLK_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`] module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = ""]
pub mod clk_cfg;
#[doc = "Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
pub mod timer;
#[doc = "TIMER_SYNCI_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_synci_cfg`] module"]
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
#[doc = ""]
pub mod timer_synci_cfg;
#[doc = "OPERATOR_TIMERSEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operator_timersel`] module"]
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
#[doc = ""]
pub mod operator_timersel;
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
pub mod ch;
#[doc = "FAULT_DETECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_detect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_detect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_detect`] module"]
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
#[doc = ""]
pub mod fault_detect;
#[doc = "CAP_TIMER_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_cfg`] module"]
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
#[doc = ""]
pub mod cap_timer_cfg;
#[doc = "CAP_TIMER_PHASE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_phase`] module"]
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
#[doc = ""]
pub mod cap_timer_phase;
#[doc = "CAP_CH_CFG (rw) register accessor: Capture channel %s configuration and enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch_cfg`] module"]
pub type CAP_CH_CFG = crate::Reg<cap_ch_cfg::CAP_CH_CFG_SPEC>;
#[doc = "Capture channel %s configuration and enable"]
pub mod cap_ch_cfg;
#[doc = "CAP_CH (r) register accessor: Value of last capture on channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch`] module"]
pub type CAP_CH = crate::Reg<cap_ch::CAP_CH_SPEC>;
#[doc = "Value of last capture on channel %s"]
pub mod cap_ch;
#[doc = "CAP_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_status`] module"]
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
#[doc = ""]
pub mod cap_status;
#[doc = "UPDATE_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`update_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update_cfg`] module"]
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
#[doc = ""]
pub mod update_cfg;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = ""]
pub mod clk;
#[doc = "VERSION (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = ""]
pub mod version;
