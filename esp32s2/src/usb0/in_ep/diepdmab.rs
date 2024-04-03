#[doc = "Register `DIEPDMAB` reader"]
pub type R = crate::R<DIEPDMAB_SPEC>;
#[doc = "Field `DMABUFFERADDR` reader - "]
pub type DMABUFFERADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr(&self) -> DMABUFFERADDR_R {
        DMABUFFERADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB")
            .field(
                "dmabufferaddr",
                &format_args!("{}", self.dmabufferaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB_SPEC;
impl crate::RegisterSpec for DIEPDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB to value 0"]
impl crate::Resettable for DIEPDMAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
