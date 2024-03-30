#[doc = "Register `LO` reader"]
pub type R = crate::R<LO_SPEC>;
#[doc = "Field `LO` reader - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
pub type LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO")
            .field("lo", &format_args!("{}", self.lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Timer 0 current value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LO_SPEC {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
