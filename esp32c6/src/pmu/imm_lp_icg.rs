#[doc = "Register `IMM_LP_ICG` writer"]
pub struct W(crate::W<IMM_LP_ICG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMM_LP_ICG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IMM_LP_ICG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMM_LP_ICG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE_LOW_LP_ROOTCLK_SEL` writer - need_des"]
pub type TIE_LOW_LP_ROOTCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, IMM_LP_ICG_SPEC, O>;
#[doc = "Field `TIE_HIGH_LP_ROOTCLK_SEL` writer - need_des"]
pub type TIE_HIGH_LP_ROOTCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, IMM_LP_ICG_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_LP_ICG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_lp_rootclk_sel(&mut self) -> TIE_LOW_LP_ROOTCLK_SEL_W<30> {
        TIE_LOW_LP_ROOTCLK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_lp_rootclk_sel(&mut self) -> TIE_HIGH_LP_ROOTCLK_SEL_W<31> {
        TIE_HIGH_LP_ROOTCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imm_lp_icg](index.html) module"]
pub struct IMM_LP_ICG_SPEC;
impl crate::RegisterSpec for IMM_LP_ICG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [imm_lp_icg::W](W) writer structure"]
impl crate::Writable for IMM_LP_ICG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_LP_ICG to value 0"]
impl crate::Resettable for IMM_LP_ICG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
