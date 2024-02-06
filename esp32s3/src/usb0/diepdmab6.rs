#[doc = "Register `DIEPDMAB6` reader"]
pub type R = crate::R<DIEPDMAB6_SPEC>;
#[doc = "Field `D_DMABUFFERADDR6` reader - "]
pub type D_DMABUFFERADDR6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr6(&self) -> D_DMABUFFERADDR6_R {
        D_DMABUFFERADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB6")
            .field(
                "d_dmabufferaddr6",
                &format_args!("{}", self.d_dmabufferaddr6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB6_SPEC;
impl crate::RegisterSpec for DIEPDMAB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab6::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB6_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB6 to value 0"]
impl crate::Resettable for DIEPDMAB6_SPEC {
    const RESET_VALUE: u32 = 0;
}
