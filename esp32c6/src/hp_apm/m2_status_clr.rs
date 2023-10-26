#[doc = "Register `M2_STATUS_CLR` writer"]
pub type W = crate::W<M2_STATUS_CLR_SPEC>;
#[doc = "Field `M2_REGION_STATUS_CLR` writer - Clear exception status"]
pub type M2_REGION_STATUS_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M2_STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear exception status"]
    #[inline(always)]
    #[must_use]
    pub fn m2_region_status_clr(&mut self) -> M2_REGION_STATUS_CLR_W<M2_STATUS_CLR_SPEC, 0> {
        M2_REGION_STATUS_CLR_W::new(self)
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
#[doc = "M2 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2_status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2_STATUS_CLR_SPEC;
impl crate::RegisterSpec for M2_STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m2_status_clr::W`](W) writer structure"]
impl crate::Writable for M2_STATUS_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M2_STATUS_CLR to value 0"]
impl crate::Resettable for M2_STATUS_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
