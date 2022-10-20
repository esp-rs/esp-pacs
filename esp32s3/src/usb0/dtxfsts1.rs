#[doc = "Register `DTXFSTS1` reader"]
pub struct R(crate::R<DTXFSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INEPTXFSPCAVAIL1` reader - "]
pub type D_INEPTXFSPCAVAIL1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail1(&self) -> D_INEPTXFSPCAVAIL1_R {
        D_INEPTXFSPCAVAIL1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts1](index.html) module"]
pub struct DTXFSTS1_SPEC;
impl crate::RegisterSpec for DTXFSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts1::R](R) reader structure"]
impl crate::Readable for DTXFSTS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS1 to value 0"]
impl crate::Resettable for DTXFSTS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
