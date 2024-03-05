#[doc = "Register `START` writer"]
pub type W = crate::W<START_SPEC>;
#[doc = "Field `START` writer - Reserved."]
pub type START_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 1:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<START_SPEC> {
        START_W::new(self, 1)
    }
}
#[doc = "Typical SHA configuration register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: u32 = 0;
}
