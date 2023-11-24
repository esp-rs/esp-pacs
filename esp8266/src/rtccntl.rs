#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pll: PLL,
}
impl RegisterBlock {
    #[doc = "0x10 - PLL I2C Register"]
    #[inline(always)]
    pub const fn pll(&self) -> &PLL {
        &self.pll
    }
}
#[doc = "PLL (rw) register accessor: PLL I2C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`] module"]
pub type PLL = crate::Reg<pll::PLL_SPEC>;
#[doc = "PLL I2C Register"]
pub mod pll;
