#[doc = "Register `WDATABE` writer"]
pub type W = crate::W<WDATABE_SPEC>;
#[doc = "Field `WDATABE` writer - NA"]
pub type WDATABE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDATABE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn wdatabe(&mut self) -> WDATABE_W<WDATABE_SPEC> {
        WDATABE_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdatabe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATABE_SPEC;
impl crate::RegisterSpec for WDATABE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdatabe::W`](W) writer structure"]
impl crate::Writable for WDATABE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDATABE to value 0"]
impl crate::Resettable for WDATABE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
