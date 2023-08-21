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
#[doc = "LP_CLK_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_clk_conf`] module"]
pub type LP_CLK_CONF = crate::Reg<lp_clk_conf::LP_CLK_CONF_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_conf;
#[doc = "LP_CLK_PO_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_po_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_po_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_clk_po_en`] module"]
pub type LP_CLK_PO_EN = crate::Reg<lp_clk_po_en::LP_CLK_PO_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_po_en;
#[doc = "LP_CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_clk_en`] module"]
pub type LP_CLK_EN = crate::Reg<lp_clk_en::LP_CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_en;
#[doc = "LP_RST_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_rst_en`] module"]
pub type LP_RST_EN = crate::Reg<lp_rst_en::LP_RST_EN_SPEC>;
#[doc = "need_des"]
pub mod lp_rst_en;
#[doc = "RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_cause`] module"]
pub type RESET_CAUSE = crate::Reg<reset_cause::RESET_CAUSE_SPEC>;
#[doc = "need_des"]
pub mod reset_cause;
#[doc = "CPU_RESET (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_reset`] module"]
pub type CPU_RESET = crate::Reg<cpu_reset::CPU_RESET_SPEC>;
#[doc = "need_des"]
pub mod cpu_reset;
#[doc = "FOSC_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fosc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fosc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fosc_cntl`] module"]
pub type FOSC_CNTL = crate::Reg<fosc_cntl::FOSC_CNTL_SPEC>;
#[doc = "need_des"]
pub mod fosc_cntl;
#[doc = "RC32K_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32k_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32k_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rc32k_cntl`] module"]
pub type RC32K_CNTL = crate::Reg<rc32k_cntl::RC32K_CNTL_SPEC>;
#[doc = "need_des"]
pub mod rc32k_cntl;
#[doc = "CLK_TO_HP (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_to_hp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_to_hp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_to_hp`] module"]
pub type CLK_TO_HP = crate::Reg<clk_to_hp::CLK_TO_HP_SPEC>;
#[doc = "need_des"]
pub mod clk_to_hp;
#[doc = "LPMEM_FORCE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmem_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmem_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpmem_force`] module"]
pub type LPMEM_FORCE = crate::Reg<lpmem_force::LPMEM_FORCE_SPEC>;
#[doc = "need_des"]
pub mod lpmem_force;
#[doc = "LPPERI (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpperi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpperi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpperi`] module"]
pub type LPPERI = crate::Reg<lpperi::LPPERI_SPEC>;
#[doc = "need_des"]
pub mod lpperi;
#[doc = "XTAL32K (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal32k`] module"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "need_des"]
pub mod xtal32k;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
