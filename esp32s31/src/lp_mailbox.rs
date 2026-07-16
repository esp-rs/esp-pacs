#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    massege: [MASSEGE; 16],
    lp_int_raw: LP_INT_RAW,
    lp_int_st: LP_INT_ST,
    lp_int_ena: LP_INT_ENA,
    lp_int_clr: LP_INT_CLR,
    hp_int_raw: HP_INT_RAW,
    hp_int_st: HP_INT_ST,
    hp_int_ena: HP_INT_ENA,
    hp_int_clr: HP_INT_CLR,
    reg_clk_en: REG_CLK_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - "]
    #[inline(always)]
    pub const fn massege(&self, n: usize) -> &MASSEGE {
        &self.massege[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - "]
    #[inline(always)]
    pub fn massege_iter(&self) -> impl Iterator<Item = &MASSEGE> {
        self.massege.iter()
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LP_INT_RAW {
        &self.lp_int_raw
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LP_INT_ST {
        &self.lp_int_st
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LP_INT_ENA {
        &self.lp_int_ena
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LP_INT_CLR {
        &self.lp_int_clr
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn hp_int_raw(&self) -> &HP_INT_RAW {
        &self.hp_int_raw
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn hp_int_st(&self) -> &HP_INT_ST {
        &self.hp_int_st
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn hp_int_ena(&self) -> &HP_INT_ENA {
        &self.hp_int_ena
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn hp_int_clr(&self) -> &HP_INT_CLR {
        &self.hp_int_clr
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn reg_clk_en(&self) -> &REG_CLK_EN {
        &self.reg_clk_en
    }
}
#[doc = "MASSEGE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`massege::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`massege::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@massege`] module"]
pub type MASSEGE = crate::Reg<massege::MASSEGE_SPEC>;
#[doc = ""]
pub mod massege;
#[doc = "LP_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_raw`] module"]
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
#[doc = ""]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_st`] module"]
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
#[doc = ""]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_ena`] module"]
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
#[doc = ""]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_clr`] module"]
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
#[doc = ""]
pub mod lp_int_clr;
#[doc = "HP_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_int_raw`] module"]
pub type HP_INT_RAW = crate::Reg<hp_int_raw::HP_INT_RAW_SPEC>;
#[doc = ""]
pub mod hp_int_raw;
#[doc = "HP_INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_int_st`] module"]
pub type HP_INT_ST = crate::Reg<hp_int_st::HP_INT_ST_SPEC>;
#[doc = ""]
pub mod hp_int_st;
#[doc = "HP_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_int_ena`] module"]
pub type HP_INT_ENA = crate::Reg<hp_int_ena::HP_INT_ENA_SPEC>;
#[doc = ""]
pub mod hp_int_ena;
#[doc = "HP_INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_int_clr`] module"]
pub type HP_INT_CLR = crate::Reg<hp_int_clr::HP_INT_CLR_SPEC>;
#[doc = ""]
pub mod hp_int_clr;
#[doc = "REG_CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_clk_en`] module"]
pub type REG_CLK_EN = crate::Reg<reg_clk_en::REG_CLK_EN_SPEC>;
#[doc = ""]
pub mod reg_clk_en;
