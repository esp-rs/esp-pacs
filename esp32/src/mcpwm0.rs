#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    ///0x00 -
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    ///0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &TIMER {
        &self.timer[n]
    }
    ///Iterator for array of:
    ///0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &TIMER> {
        self.timer.iter()
    }
    ///0x34 -
    #[inline(always)]
    pub const fn timer_synci_cfg(&self) -> &TIMER_SYNCI_CFG {
        &self.timer_synci_cfg
    }
    ///0x38 -
    #[inline(always)]
    pub const fn operator_timersel(&self) -> &OPERATOR_TIMERSEL {
        &self.operator_timersel
    }
    ///0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn fault_detect(&self) -> &FAULT_DETECT {
        &self.fault_detect
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn cap_timer_cfg(&self) -> &CAP_TIMER_CFG {
        &self.cap_timer_cfg
    }
    ///0xec -
    #[inline(always)]
    pub const fn cap_timer_phase(&self) -> &CAP_TIMER_PHASE {
        &self.cap_timer_phase
    }
    ///0xf0..0xfc - Capture channel %s configuration and enable
    #[inline(always)]
    pub const fn cap_ch_cfg(&self, n: usize) -> &CAP_CH_CFG {
        &self.cap_ch_cfg[n]
    }
    ///Iterator for array of:
    ///0xf0..0xfc - Capture channel %s configuration and enable
    #[inline(always)]
    pub fn cap_ch_cfg_iter(&self) -> impl Iterator<Item = &CAP_CH_CFG> {
        self.cap_ch_cfg.iter()
    }
    ///0xf0 - Capture channel 0 configuration and enable
    #[inline(always)]
    pub const fn cap_ch0_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(0)
    }
    ///0xf4 - Capture channel 1 configuration and enable
    #[inline(always)]
    pub const fn cap_ch1_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(1)
    }
    ///0xf8 - Capture channel 2 configuration and enable
    #[inline(always)]
    pub const fn cap_ch2_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(2)
    }
    ///0xfc..0x108 - Value of last capture on channel %s
    #[inline(always)]
    pub const fn cap_ch(&self, n: usize) -> &CAP_CH {
        &self.cap_ch[n]
    }
    ///Iterator for array of:
    ///0xfc..0x108 - Value of last capture on channel %s
    #[inline(always)]
    pub fn cap_ch_iter(&self) -> impl Iterator<Item = &CAP_CH> {
        self.cap_ch.iter()
    }
    ///0x108 -
    #[inline(always)]
    pub const fn cap_status(&self) -> &CAP_STATUS {
        &self.cap_status
    }
    ///0x10c -
    #[inline(always)]
    pub const fn update_cfg(&self) -> &UPDATE_CFG {
        &self.update_cfg
    }
    ///0x110 -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x114 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x118 -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x11c -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x120 -
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x124 -
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
/**CLK_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_cfg`] module*/
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
///
pub mod clk_cfg;
///Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS
pub use self::timer::TIMER;
///Cluster
///Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS
pub mod timer;
/**TIMER_SYNCI_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_synci_cfg`] module*/
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
///
pub mod timer_synci_cfg;
/**OPERATOR_TIMERSEL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@operator_timersel`] module*/
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
///
pub mod operator_timersel;
///Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS
pub mod ch;
/**FAULT_DETECT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`fault_detect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_detect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fault_detect`] module*/
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
///
pub mod fault_detect;
/**CAP_TIMER_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cap_timer_cfg`] module*/
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
///
pub mod cap_timer_cfg;
/**CAP_TIMER_PHASE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cap_timer_phase`] module*/
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
///
pub mod cap_timer_phase;
/**CAP_CH_CFG (rw) register accessor: Capture channel %s configuration and enable

You can [`read`](crate::generic::Reg::read) this register and get [`cap_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cap_ch_cfg`] module*/
pub type CAP_CH_CFG = crate::Reg<cap_ch_cfg::CAP_CH_CFG_SPEC>;
///Capture channel %s configuration and enable
pub mod cap_ch_cfg;
/**CAP_CH (r) register accessor: Value of last capture on channel %s

You can [`read`](crate::generic::Reg::read) this register and get [`cap_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cap_ch`] module*/
pub type CAP_CH = crate::Reg<cap_ch::CAP_CH_SPEC>;
///Value of last capture on channel %s
pub mod cap_ch;
/**CAP_STATUS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cap_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cap_status`] module*/
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
///
pub mod cap_status;
/**UPDATE_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`update_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@update_cfg`] module*/
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
///
pub mod update_cfg;
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_RAW (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**CLK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///
pub mod clk;
/**VERSION (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///
pub mod version;
