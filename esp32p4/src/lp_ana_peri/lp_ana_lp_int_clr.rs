#[doc = "Register `LP_ANA_LP_INT_CLR` writer"]
pub type W = crate::W<LP_ANA_LP_INT_CLR_SPEC>;
#[doc = "Field `LP_ANA_BOD_MODE0_LP_INT_CLR` writer - need_des"]
pub type LP_ANA_BOD_MODE0_LP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_bod_mode0_lp_int_clr(
        &mut self,
    ) -> LP_ANA_BOD_MODE0_LP_INT_CLR_W<LP_ANA_LP_INT_CLR_SPEC> {
        LP_ANA_BOD_MODE0_LP_INT_CLR_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_ANA_LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_ana_lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LP_ANA_LP_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_LP_INT_CLR to value 0"]
impl crate::Resettable for LP_ANA_LP_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
