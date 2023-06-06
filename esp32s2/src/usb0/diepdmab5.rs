#[doc = "Register `DIEPDMAB5` reader"]
pub struct R(crate::R<DIEPDMAB5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMAB5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMAB5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMAB5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_DMABUFFERADDR5` reader - "]
pub type D_DMABUFFERADDR5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr5(&self) -> D_DMABUFFERADDR5_R {
        D_DMABUFFERADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB5")
            .field(
                "d_dmabufferaddr5",
                &format_args!("{}", self.d_dmabufferaddr5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdmab5](index.html) module"]
pub struct DIEPDMAB5_SPEC;
impl crate::RegisterSpec for DIEPDMAB5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdmab5::R](R) reader structure"]
impl crate::Readable for DIEPDMAB5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPDMAB5 to value 0"]
impl crate::Resettable for DIEPDMAB5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
