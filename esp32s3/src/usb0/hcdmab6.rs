#[doc = "Register `HCDMAB6` reader"]
pub type R = crate::R<HCDMAB6_SPEC>;
#[doc = "Field `H_HCDMAB6` reader - "]
pub type H_HCDMAB6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab6(&self) -> H_HCDMAB6_R {
        H_HCDMAB6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB6")
            .field("h_hcdmab6", &format_args!("{}", self.h_hcdmab6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB6_SPEC;
impl crate::RegisterSpec for HCDMAB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab6::R`](R) reader structure"]
impl crate::Readable for HCDMAB6_SPEC {}
#[doc = "`reset()` method sets HCDMAB6 to value 0"]
impl crate::Resettable for HCDMAB6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
