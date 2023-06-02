#[doc = "Register `DIEPDMAB2` reader"]
pub struct R(crate::R<DIEPDMAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_DMABUFFERADDR2` reader - "]
pub type D_DMABUFFERADDR2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr2(&self) -> D_DMABUFFERADDR2_R {
        D_DMABUFFERADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB2")
            .field(
                "d_dmabufferaddr2",
                &format_args!("{}", self.d_dmabufferaddr2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMAB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdmab2](index.html) module"]
pub struct DIEPDMAB2_SPEC;
impl crate::RegisterSpec for DIEPDMAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdmab2::R](R) reader structure"]
impl crate::Readable for DIEPDMAB2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPDMAB2 to value 0"]
impl crate::Resettable for DIEPDMAB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
