#[doc = "Register `DIEPDMAB1` reader"]
pub struct R(crate::R<DIEPDMAB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMAB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMAB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMAB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_DMABUFFERADDR1` reader - "]
pub type D_DMABUFFERADDR1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmabufferaddr1(&self) -> D_DMABUFFERADDR1_R {
        D_DMABUFFERADDR1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdmab1](index.html) module"]
pub struct DIEPDMAB1_SPEC;
impl crate::RegisterSpec for DIEPDMAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdmab1::R](R) reader structure"]
impl crate::Readable for DIEPDMAB1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPDMAB1 to value 0"]
impl crate::Resettable for DIEPDMAB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
