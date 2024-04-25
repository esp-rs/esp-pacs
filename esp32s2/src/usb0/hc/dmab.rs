#[doc = "Register `DMAB` reader"]
pub type R = crate::R<DMAB_SPEC>;
#[doc = "Field `HCDMAB` reader - "]
pub type HCDMAB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAB")
            .field("hcdmab", &format_args!("{}", self.hcdmab().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMAB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAB_SPEC;
impl crate::RegisterSpec for DMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmab::R`](R) reader structure"]
impl crate::Readable for DMAB_SPEC {}
#[doc = "`reset()` method sets DMAB to value 0"]
impl crate::Resettable for DMAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
