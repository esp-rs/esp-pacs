#[doc = "Register `T0LO` reader"]
pub type R = crate::R<T0LO_SPEC>;
#[doc = "Field `LO` reader - t0_lo"]
pub type LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - t0_lo"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LO")
            .field("lo", &format_args!("{}", self.lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TIMG_T0LO_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LO_SPEC;
impl crate::RegisterSpec for T0LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0lo::R`](R) reader structure"]
impl crate::Readable for T0LO_SPEC {}
#[doc = "`reset()` method sets T0LO to value 0"]
impl crate::Resettable for T0LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
