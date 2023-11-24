#[doc = "Register `HCDMAB5` reader"]
pub type R = crate::R<HCDMAB5_SPEC>;
#[doc = "Field `H_HCDMAB5` reader - "]
pub type H_HCDMAB5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab5(&self) -> H_HCDMAB5_R {
        H_HCDMAB5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB5")
            .field("h_hcdmab5", &format_args!("{}", self.h_hcdmab5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB5_SPEC;
impl crate::RegisterSpec for HCDMAB5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab5::R`](R) reader structure"]
impl crate::Readable for HCDMAB5_SPEC {}
#[doc = "`reset()` method sets HCDMAB5 to value 0"]
impl crate::Resettable for HCDMAB5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
