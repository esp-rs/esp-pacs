#[doc = "Register `IMM_SLEEP_SYSCLK` writer"]
pub type W = crate::W<IMM_SLEEP_SYSCLK_SPEC>;
#[doc = "Field `UPDATE_DIG_ICG_SWITCH` writer - need_des"]
pub type UPDATE_DIG_ICG_SWITCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE_LOW_ICG_SLP_SEL` writer - need_des"]
pub type TIE_LOW_ICG_SLP_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE_HIGH_ICG_SLP_SEL` writer - need_des"]
pub type TIE_HIGH_ICG_SLP_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDATE_DIG_SYS_CLK_SEL` writer - need_des"]
pub type UPDATE_DIG_SYS_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_SLEEP_SYSCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn update_dig_icg_switch(&mut self) -> UPDATE_DIG_ICG_SWITCH_W<IMM_SLEEP_SYSCLK_SPEC, 28> {
        UPDATE_DIG_ICG_SWITCH_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_icg_slp_sel(&mut self) -> TIE_LOW_ICG_SLP_SEL_W<IMM_SLEEP_SYSCLK_SPEC, 29> {
        TIE_LOW_ICG_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_icg_slp_sel(&mut self) -> TIE_HIGH_ICG_SLP_SEL_W<IMM_SLEEP_SYSCLK_SPEC, 30> {
        TIE_HIGH_ICG_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn update_dig_sys_clk_sel(
        &mut self,
    ) -> UPDATE_DIG_SYS_CLK_SEL_W<IMM_SLEEP_SYSCLK_SPEC, 31> {
        UPDATE_DIG_SYS_CLK_SEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_SLEEP_SYSCLK_SPEC;
impl crate::RegisterSpec for IMM_SLEEP_SYSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_sleep_sysclk::W`](W) writer structure"]
impl crate::Writable for IMM_SLEEP_SYSCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_SLEEP_SYSCLK to value 0"]
impl crate::Resettable for IMM_SLEEP_SYSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
