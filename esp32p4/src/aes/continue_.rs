#[doc = "Register `CONTINUE` writer"]
pub type W = crate::W<CONTINUE_SPEC>;
#[doc = "Field `CONTINUE` writer - Set this bit to continue GCM operation."]
pub type CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to continue GCM operation."]
    #[inline(always)]
    #[must_use]
    pub fn continue_(&mut self) -> CONTINUE_W<CONTINUE_SPEC> {
        CONTINUE_W::new(self, 0)
    }
}
#[doc = "AES continue register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`continue_::W`](W) writer structure"]
impl crate::Writable for CONTINUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
