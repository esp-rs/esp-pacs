#[doc = "Register `DTXFSTS2` reader"]
pub struct R(crate::R<DTXFSTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INEPTXFSPCAVAIL2` reader - "]
pub type D_INEPTXFSPCAVAIL2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail2(&self) -> D_INEPTXFSPCAVAIL2_R {
        D_INEPTXFSPCAVAIL2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts2](index.html) module"]
pub struct DTXFSTS2_SPEC;
impl crate::RegisterSpec for DTXFSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts2::R](R) reader structure"]
impl crate::Readable for DTXFSTS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS2 to value 0"]
impl crate::Resettable for DTXFSTS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
