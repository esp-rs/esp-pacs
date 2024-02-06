#[doc = "Register `T%sHI` reader"]
pub type R = crate::R<THI_SPEC>;
#[doc = "Field `HI` reader - After writing to TIMG_T%sUPDATE_REG, the high 32 bits of the time-base counter of timer %s can be read here."]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the high 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THI")
            .field("hi", &format_args!("{}", self.hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Timer %s current value, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THI_SPEC;
impl crate::RegisterSpec for THI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thi::R`](R) reader structure"]
impl crate::Readable for THI_SPEC {}
#[doc = "`reset()` method sets T%sHI to value 0"]
impl crate::Resettable for THI_SPEC {
    const RESET_VALUE: u32 = 0;
}
