#[doc = "Register `HCDMAB3` reader"]
pub struct R(crate::R<HCDMAB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB3` reader - "]
pub type H_HCDMAB3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab3(&self) -> H_HCDMAB3_R {
        H_HCDMAB3_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab3](index.html) module"]
pub struct HCDMAB3_SPEC;
impl crate::RegisterSpec for HCDMAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab3::R](R) reader structure"]
impl crate::Readable for HCDMAB3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB3 to value 0"]
impl crate::Resettable for HCDMAB3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
