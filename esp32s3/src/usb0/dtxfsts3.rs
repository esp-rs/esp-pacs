#[doc = "Register `DTXFSTS3` reader"]
pub struct R(crate::R<DTXFSTS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INEPTXFSPCAVAIL3` reader - "]
pub type D_INEPTXFSPCAVAIL3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail3(&self) -> D_INEPTXFSPCAVAIL3_R {
        D_INEPTXFSPCAVAIL3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts3](index.html) module"]
pub struct DTXFSTS3_SPEC;
impl crate::RegisterSpec for DTXFSTS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts3::R](R) reader structure"]
impl crate::Readable for DTXFSTS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS3 to value 0"]
impl crate::Resettable for DTXFSTS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
