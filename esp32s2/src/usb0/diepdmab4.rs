#[doc = "Register `DIEPDMAB4` reader"]
pub struct R(crate::R<DIEPDMAB4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMAB4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMAB4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMAB4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_DMABUFFERADDR4` reader - "]
pub type D_DMABUFFERADDR4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr4(&self) -> D_DMABUFFERADDR4_R {
        D_DMABUFFERADDR4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB4")
            .field(
                "d_dmabufferaddr4",
                &format_args!("{}", self.d_dmabufferaddr4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdmab4](index.html) module"]
pub struct DIEPDMAB4_SPEC;
impl crate::RegisterSpec for DIEPDMAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdmab4::R](R) reader structure"]
impl crate::Readable for DIEPDMAB4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPDMAB4 to value 0"]
impl crate::Resettable for DIEPDMAB4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
