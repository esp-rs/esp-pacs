#[doc = "Register `HCDMAB5` reader"]
pub struct R(crate::R<HCDMAB5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB5` reader - "]
pub type H_HCDMAB5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab5(&self) -> H_HCDMAB5_R {
        H_HCDMAB5_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab5](index.html) module"]
pub struct HCDMAB5_SPEC;
impl crate::RegisterSpec for HCDMAB5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab5::R](R) reader structure"]
impl crate::Readable for HCDMAB5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB5 to value 0"]
impl crate::Resettable for HCDMAB5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
