#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub options0: OPTIONS0,
    #[doc = "0x04 - "]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - "]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - "]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - "]
    pub time0: TIME0,
    #[doc = "0x14 - "]
    pub time1: TIME1,
    #[doc = "0x18 - "]
    pub state0: STATE0,
    #[doc = "0x1c - "]
    pub timer1: TIMER1,
    #[doc = "0x20 - "]
    pub timer2: TIMER2,
    #[doc = "0x24 - "]
    pub timer3: TIMER3,
    #[doc = "0x28 - "]
    pub timer4: TIMER4,
    #[doc = "0x2c - "]
    pub timer5: TIMER5,
    #[doc = "0x30 - "]
    pub ana_conf: ANA_CONF,
    #[doc = "0x34 - "]
    pub reset_state: RESET_STATE,
    #[doc = "0x38 - "]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x3c - "]
    pub int_ena: INT_ENA,
    #[doc = "0x40 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x44 - "]
    pub int_st: INT_ST,
    #[doc = "0x48 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x4c - "]
    pub store0: STORE0,
    #[doc = "0x50 - "]
    pub store1: STORE1,
    #[doc = "0x54 - "]
    pub store2: STORE2,
    #[doc = "0x58 - "]
    pub store3: STORE3,
    #[doc = "0x5c - "]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x60 - "]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x64 - "]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x68 - "]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x6c - "]
    pub sdio_act_conf: SDIO_ACT_CONF,
    #[doc = "0x70 - "]
    pub clk_conf: CLK_CONF,
    #[doc = "0x74 - "]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x78 - "]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x7c - "]
    pub reg: REG,
    #[doc = "0x80 - "]
    pub pwc: PWC,
    #[doc = "0x84 - "]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x88 - "]
    pub dig_iso: DIG_ISO,
    #[doc = "0x8c - "]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x90 - "]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x94 - "]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x98 - "]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x9c - "]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa0 - "]
    pub wdtfeed: WDTFEED,
    #[doc = "0xa4 - "]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xa8 - "]
    pub test_mux: TEST_MUX,
    #[doc = "0xac - "]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xb0 - "]
    pub store4: STORE4,
    #[doc = "0xb4 - "]
    pub store5: STORE5,
    #[doc = "0xb8 - "]
    pub store6: STORE6,
    #[doc = "0xbc - "]
    pub store7: STORE7,
    #[doc = "0xc0 - "]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xc4 - "]
    pub diag1: DIAG1,
    #[doc = "0xc8 - "]
    pub hold_force: HOLD_FORCE,
    #[doc = "0xcc - "]
    pub ext_wakeup1: EXT_WAKEUP1,
    #[doc = "0xd0 - "]
    pub ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    #[doc = "0xd4 - "]
    pub brown_out: BROWN_OUT,
    _reserved54: [u8; 0x64],
    #[doc = "0x13c - "]
    pub date: DATE,
}
#[doc = "OPTIONS0 (rw) register accessor: an alias for `Reg<OPTIONS0_SPEC>`"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = ""]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: an alias for `Reg<SLP_TIMER0_SPEC>`"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = ""]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: an alias for `Reg<SLP_TIMER1_SPEC>`"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = ""]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: an alias for `Reg<TIME_UPDATE_SPEC>`"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = ""]
pub mod time_update;
#[doc = "TIME0 (r) register accessor: an alias for `Reg<TIME0_SPEC>`"]
pub type TIME0 = crate::Reg<time0::TIME0_SPEC>;
#[doc = ""]
pub mod time0;
#[doc = "TIME1 (r) register accessor: an alias for `Reg<TIME1_SPEC>`"]
pub type TIME1 = crate::Reg<time1::TIME1_SPEC>;
#[doc = ""]
pub mod time1;
#[doc = "STATE0 (rw) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = ""]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = ""]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = ""]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = ""]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: an alias for `Reg<TIMER4_SPEC>`"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = ""]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: an alias for `Reg<TIMER5_SPEC>`"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = ""]
pub mod timer5;
#[doc = "ANA_CONF (rw) register accessor: an alias for `Reg<ANA_CONF_SPEC>`"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = ""]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = ""]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: an alias for `Reg<WAKEUP_STATE_SPEC>`"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = ""]
pub mod wakeup_state;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "STORE0 (rw) register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = ""]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = ""]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = ""]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = ""]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: an alias for `Reg<EXT_XTL_CONF_SPEC>`"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = ""]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: an alias for `Reg<EXT_WAKEUP_CONF_SPEC>`"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = ""]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: an alias for `Reg<SLP_REJECT_CONF_SPEC>`"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = ""]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: an alias for `Reg<CPU_PERIOD_CONF_SPEC>`"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = ""]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: an alias for `Reg<SDIO_ACT_CONF_SPEC>`"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = ""]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: an alias for `Reg<SDIO_CONF_SPEC>`"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = ""]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: an alias for `Reg<BIAS_CONF_SPEC>`"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = ""]
pub mod bias_conf;
#[doc = "REG (rw) register accessor: an alias for `Reg<REG_SPEC>`"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = ""]
pub mod reg;
#[doc = "PWC (rw) register accessor: an alias for `Reg<PWC_SPEC>`"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = ""]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = ""]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = ""]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "TEST_MUX (rw) register accessor: an alias for `Reg<TEST_MUX_SPEC>`"]
pub type TEST_MUX = crate::Reg<test_mux::TEST_MUX_SPEC>;
#[doc = ""]
pub mod test_mux;
#[doc = "SW_CPU_STALL (rw) register accessor: an alias for `Reg<SW_CPU_STALL_SPEC>`"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = ""]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = ""]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = ""]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = ""]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = ""]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: an alias for `Reg<LOW_POWER_ST_SPEC>`"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = ""]
pub mod low_power_st;
#[doc = "DIAG1 (r) register accessor: an alias for `Reg<DIAG1_SPEC>`"]
pub type DIAG1 = crate::Reg<diag1::DIAG1_SPEC>;
#[doc = ""]
pub mod diag1;
#[doc = "HOLD_FORCE (rw) register accessor: an alias for `Reg<HOLD_FORCE_SPEC>`"]
pub type HOLD_FORCE = crate::Reg<hold_force::HOLD_FORCE_SPEC>;
#[doc = ""]
pub mod hold_force;
#[doc = "EXT_WAKEUP1 (rw) register accessor: an alias for `Reg<EXT_WAKEUP1_SPEC>`"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = ""]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: an alias for `Reg<EXT_WAKEUP1_STATUS_SPEC>`"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = ""]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: an alias for `Reg<BROWN_OUT_SPEC>`"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = ""]
pub mod brown_out;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
