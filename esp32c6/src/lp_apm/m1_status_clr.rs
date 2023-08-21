#[doc = "Register `M1_STATUS_CLR` writer"]
pub type W = crate::W<M1_STATUS_CLR_SPEC>;
#[doc = "Field `M1_REGION_STATUS_CLR` writer - Clear exception status"]
pub type M1_REGION_STATUS_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M1_STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear exception status"]
    #[inline(always)]
    #[must_use]
    pub fn m1_region_status_clr(&mut self) -> M1_REGION_STATUS_CLR_W<M1_STATUS_CLR_SPEC, 0> {
        M1_REGION_STATUS_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "M1 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1_status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1_STATUS_CLR_SPEC;
impl crate::RegisterSpec for M1_STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m1_status_clr::W`](W) writer structure"]
impl crate::Writable for M1_STATUS_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M1_STATUS_CLR to value 0"]
impl crate::Resettable for M1_STATUS_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
