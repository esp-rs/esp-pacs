#[doc = "Register `DIEPDMAB3` reader"]
pub struct R(crate::R<DIEPDMAB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMAB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMAB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_DMABUFFERADDR3` reader - "]
pub type D_DMABUFFERADDR3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr3(&self) -> D_DMABUFFERADDR3_R {
        D_DMABUFFERADDR3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB3")
            .field(
                "d_dmabufferaddr3",
                &format_args!("{}", self.d_dmabufferaddr3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdmab3](index.html) module"]
pub struct DIEPDMAB3_SPEC;
impl crate::RegisterSpec for DIEPDMAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdmab3::R](R) reader structure"]
impl crate::Readable for DIEPDMAB3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPDMAB3 to value 0"]
impl crate::Resettable for DIEPDMAB3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
