#[doc = "Register `DIEPDMAB1` reader"]
pub type R = crate::R<DIEPDMAB1_SPEC>;
#[doc = "Field `D_DMABUFFERADDR1` reader - "]
pub type D_DMABUFFERADDR1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr1(&self) -> D_DMABUFFERADDR1_R {
        D_DMABUFFERADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB1")
            .field(
                "d_dmabufferaddr1",
                &format_args!("{}", self.d_dmabufferaddr1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB1_SPEC;
impl crate::RegisterSpec for DIEPDMAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab1::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB1_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB1 to value 0"]
impl crate::Resettable for DIEPDMAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
