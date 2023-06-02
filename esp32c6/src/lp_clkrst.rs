#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub lp_clk_conf: LP_CLK_CONF,
    #[doc = "0x04 - need_des"]
    pub lp_clk_po_en: LP_CLK_PO_EN,
    #[doc = "0x08 - need_des"]
    pub lp_clk_en: LP_CLK_EN,
    #[doc = "0x0c - need_des"]
    pub lp_rst_en: LP_RST_EN,
    #[doc = "0x10 - need_des"]
    pub reset_cause: RESET_CAUSE,
    #[doc = "0x14 - need_des"]
    pub cpu_reset: CPU_RESET,
    #[doc = "0x18 - need_des"]
    pub fosc_cntl: FOSC_CNTL,
    #[doc = "0x1c - need_des"]
    pub rc32k_cntl: RC32K_CNTL,
    #[doc = "0x20 - need_des"]
    pub clk_to_hp: CLK_TO_HP,
    #[doc = "0x24 - need_des"]
    pub lpmem_force: LPMEM_FORCE,
    #[doc = "0x28 - need_des"]
    pub lpperi: LPPERI,
    #[doc = "0x2c - need_des"]
    pub xtal32k: XTAL32K,
    _reserved12: [u8; 0x03cc],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "LP_CLK_CONF (rw) register accessor: an alias for `Reg<LP_CLK_CONF_SPEC>`"]
pub type LP_CLK_CONF = crate::Reg<lp_clk_conf::LP_CLK_CONF_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_conf;
#[doc = "LP_CLK_PO_EN (rw) register accessor: an alias for `Reg<LP_CLK_PO_EN_SPEC>`"]
pub type LP_CLK_PO_EN = crate::Reg<lp_clk_po_en::LP_CLK_PO_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_po_en;
#[doc = "LP_CLK_EN (rw) register accessor: an alias for `Reg<LP_CLK_EN_SPEC>`"]
pub type LP_CLK_EN = crate::Reg<lp_clk_en::LP_CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_en;
#[doc = "LP_RST_EN (rw) register accessor: an alias for `Reg<LP_RST_EN_SPEC>`"]
pub type LP_RST_EN = crate::Reg<lp_rst_en::LP_RST_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_rst_en;
#[doc = "RESET_CAUSE (rw) register accessor: an alias for `Reg<RESET_CAUSE_SPEC>`"]
pub type RESET_CAUSE = crate::Reg<reset_cause::RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod reset_cause;
#[doc = "CPU_RESET (rw) register accessor: an alias for `Reg<CPU_RESET_SPEC>`"]
pub type CPU_RESET = crate::Reg<cpu_reset::CPU_RESET_SPEC>;
#[doc = "need_des"]
pub mod cpu_reset;
#[doc = "FOSC_CNTL (rw) register accessor: an alias for `Reg<FOSC_CNTL_SPEC>`"]
pub type FOSC_CNTL = crate::Reg<fosc_cntl::FOSC_CNTL_SPEC>;
#[doc = "need_des"]
pub mod fosc_cntl;
#[doc = "RC32K_CNTL (rw) register accessor: an alias for `Reg<RC32K_CNTL_SPEC>`"]
pub type RC32K_CNTL = crate::Reg<rc32k_cntl::RC32K_CNTL_SPEC>;
#[doc = "need_des"]
pub mod rc32k_cntl;
#[doc = "CLK_TO_HP (rw) register accessor: an alias for `Reg<CLK_TO_HP_SPEC>`"]
pub type CLK_TO_HP = crate::Reg<clk_to_hp::CLK_TO_HP_SPEC>;
#[doc = "need_des"]
pub mod clk_to_hp;
#[doc = "LPMEM_FORCE (rw) register accessor: an alias for `Reg<LPMEM_FORCE_SPEC>`"]
pub type LPMEM_FORCE = crate::Reg<lpmem_force::LPMEM_FORCE_SPEC>;
#[doc = "need_des"]
pub mod lpmem_force;
#[doc = "LPPERI (rw) register accessor: an alias for `Reg<LPPERI_SPEC>`"]
pub type LPPERI = crate::Reg<lpperi::LPPERI_SPEC>;
#[doc = "need_des"]
pub mod lpperi;
#[doc = "XTAL32K (rw) register accessor: an alias for `Reg<XTAL32K_SPEC>`"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "need_des"]
pub mod xtal32k;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
