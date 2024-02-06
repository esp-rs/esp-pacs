#[doc = "Register `DIEPDMAB0` reader"]
pub type R = crate::R<DIEPDMAB0_SPEC>;
#[doc = "Field `D_DMABUFFERADDR0` reader - "]
pub type D_DMABUFFERADDR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr0(&self) -> D_DMABUFFERADDR0_R {
        D_DMABUFFERADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB0")
            .field(
                "d_dmabufferaddr0",
                &format_args!("{}", self.d_dmabufferaddr0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB0_SPEC;
impl crate::RegisterSpec for DIEPDMAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab0::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB0_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB0 to value 0"]
impl crate::Resettable for DIEPDMAB0_SPEC {
    const RESET_VALUE: u32 = 0;
}
