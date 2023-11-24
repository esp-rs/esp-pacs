#[doc = "Register `T%sLOAD` writer"]
pub type W = crate::W<TLOAD_SPEC>;
#[doc = "Field `LOAD` writer - Write any value to trigger a timer %s time-base counter reload."]
pub type LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to trigger a timer %s time-base counter reload."]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<TLOAD_SPEC> {
        LOAD_W::new(self, 0)
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
#[doc = "Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLOAD_SPEC;
impl crate::RegisterSpec for TLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tload::W`](W) writer structure"]
impl crate::Writable for TLOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sLOAD to value 0"]
impl crate::Resettable for TLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
