#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - "]
    pub xtal_tick_conf: XTAL_TICK_CONF,
    #[doc = "0x08 - "]
    pub pll_tick_conf: PLL_TICK_CONF,
    #[doc = "0x0c - "]
    pub ck8m_tick_conf: CK8M_TICK_CONF,
    #[doc = "0x10 - "]
    pub apb_saradc_ctrl: APB_SARADC_CTRL,
    #[doc = "0x14 - "]
    pub apb_saradc_ctrl2: APB_SARADC_CTRL2,
    #[doc = "0x18 - "]
    pub apb_saradc_fsm: APB_SARADC_FSM,
    #[doc = "0x1c - "]
    pub apb_saradc_sar1_patt_tab1: APB_SARADC_SAR1_PATT_TAB1,
    #[doc = "0x20 - "]
    pub apb_saradc_sar1_patt_tab2: APB_SARADC_SAR1_PATT_TAB2,
    #[doc = "0x24 - "]
    pub apb_saradc_sar1_patt_tab3: APB_SARADC_SAR1_PATT_TAB3,
    #[doc = "0x28 - "]
    pub apb_saradc_sar1_patt_tab4: APB_SARADC_SAR1_PATT_TAB4,
    #[doc = "0x2c - "]
    pub apb_saradc_sar2_patt_tab1: APB_SARADC_SAR2_PATT_TAB1,
    #[doc = "0x30 - "]
    pub apb_saradc_sar2_patt_tab2: APB_SARADC_SAR2_PATT_TAB2,
    #[doc = "0x34 - "]
    pub apb_saradc_sar2_patt_tab3: APB_SARADC_SAR2_PATT_TAB3,
    #[doc = "0x38 - "]
    pub apb_saradc_sar2_patt_tab4: APB_SARADC_SAR2_PATT_TAB4,
    #[doc = "0x3c - "]
    pub apll_tick_conf: APLL_TICK_CONF,
    _reserved16: [u8; 0x3c],
    #[doc = "0x7c - "]
    pub date: DATE,
}
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = ""]
pub mod sysclk_conf;
#[doc = "XTAL_TICK_CONF (rw) register accessor: an alias for `Reg<XTAL_TICK_CONF_SPEC>`"]
pub type XTAL_TICK_CONF = crate::Reg<xtal_tick_conf::XTAL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod xtal_tick_conf;
#[doc = "PLL_TICK_CONF (rw) register accessor: an alias for `Reg<PLL_TICK_CONF_SPEC>`"]
pub type PLL_TICK_CONF = crate::Reg<pll_tick_conf::PLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod pll_tick_conf;
#[doc = "CK8M_TICK_CONF (rw) register accessor: an alias for `Reg<CK8M_TICK_CONF_SPEC>`"]
pub type CK8M_TICK_CONF = crate::Reg<ck8m_tick_conf::CK8M_TICK_CONF_SPEC>;
#[doc = ""]
pub mod ck8m_tick_conf;
#[doc = "APB_SARADC_CTRL (rw) register accessor: an alias for `Reg<APB_SARADC_CTRL_SPEC>`"]
pub type APB_SARADC_CTRL = crate::Reg<apb_saradc_ctrl::APB_SARADC_CTRL_SPEC>;
#[doc = ""]
pub mod apb_saradc_ctrl;
#[doc = "APB_SARADC_CTRL2 (rw) register accessor: an alias for `Reg<APB_SARADC_CTRL2_SPEC>`"]
pub type APB_SARADC_CTRL2 = crate::Reg<apb_saradc_ctrl2::APB_SARADC_CTRL2_SPEC>;
#[doc = ""]
pub mod apb_saradc_ctrl2;
#[doc = "APB_SARADC_FSM (rw) register accessor: an alias for `Reg<APB_SARADC_FSM_SPEC>`"]
pub type APB_SARADC_FSM = crate::Reg<apb_saradc_fsm::APB_SARADC_FSM_SPEC>;
#[doc = ""]
pub mod apb_saradc_fsm;
#[doc = "APB_SARADC_SAR1_PATT_TAB1 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR1_PATT_TAB1_SPEC>`"]
pub type APB_SARADC_SAR1_PATT_TAB1 =
    crate::Reg<apb_saradc_sar1_patt_tab1::APB_SARADC_SAR1_PATT_TAB1_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar1_patt_tab1;
#[doc = "APB_SARADC_SAR1_PATT_TAB2 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR1_PATT_TAB2_SPEC>`"]
pub type APB_SARADC_SAR1_PATT_TAB2 =
    crate::Reg<apb_saradc_sar1_patt_tab2::APB_SARADC_SAR1_PATT_TAB2_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar1_patt_tab2;
#[doc = "APB_SARADC_SAR1_PATT_TAB3 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR1_PATT_TAB3_SPEC>`"]
pub type APB_SARADC_SAR1_PATT_TAB3 =
    crate::Reg<apb_saradc_sar1_patt_tab3::APB_SARADC_SAR1_PATT_TAB3_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar1_patt_tab3;
#[doc = "APB_SARADC_SAR1_PATT_TAB4 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR1_PATT_TAB4_SPEC>`"]
pub type APB_SARADC_SAR1_PATT_TAB4 =
    crate::Reg<apb_saradc_sar1_patt_tab4::APB_SARADC_SAR1_PATT_TAB4_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar1_patt_tab4;
#[doc = "APB_SARADC_SAR2_PATT_TAB1 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR2_PATT_TAB1_SPEC>`"]
pub type APB_SARADC_SAR2_PATT_TAB1 =
    crate::Reg<apb_saradc_sar2_patt_tab1::APB_SARADC_SAR2_PATT_TAB1_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar2_patt_tab1;
#[doc = "APB_SARADC_SAR2_PATT_TAB2 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR2_PATT_TAB2_SPEC>`"]
pub type APB_SARADC_SAR2_PATT_TAB2 =
    crate::Reg<apb_saradc_sar2_patt_tab2::APB_SARADC_SAR2_PATT_TAB2_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar2_patt_tab2;
#[doc = "APB_SARADC_SAR2_PATT_TAB3 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR2_PATT_TAB3_SPEC>`"]
pub type APB_SARADC_SAR2_PATT_TAB3 =
    crate::Reg<apb_saradc_sar2_patt_tab3::APB_SARADC_SAR2_PATT_TAB3_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar2_patt_tab3;
#[doc = "APB_SARADC_SAR2_PATT_TAB4 (rw) register accessor: an alias for `Reg<APB_SARADC_SAR2_PATT_TAB4_SPEC>`"]
pub type APB_SARADC_SAR2_PATT_TAB4 =
    crate::Reg<apb_saradc_sar2_patt_tab4::APB_SARADC_SAR2_PATT_TAB4_SPEC>;
#[doc = ""]
pub mod apb_saradc_sar2_patt_tab4;
#[doc = "APLL_TICK_CONF (rw) register accessor: an alias for `Reg<APLL_TICK_CONF_SPEC>`"]
pub type APLL_TICK_CONF = crate::Reg<apll_tick_conf::APLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod apll_tick_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
