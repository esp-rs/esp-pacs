#[doc = "Register `RD_KEY0_DATA6` reader"]
pub struct R(crate::R<RD_KEY0_DATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY0_DATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY0_DATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY0_DATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY0_DATA6` reader - Stores the sixth 32 bits of KEY0."]
pub type KEY0_DATA6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the sixth 32 bits of KEY0."]
    #[inline(always)]
    pub fn key0_data6(&self) -> KEY0_DATA6_R {
        KEY0_DATA6_R::new(self.bits)
    }
}
#[doc = "Register 6 of BLOCK4 (KEY0).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data6](index.html) module"]
pub struct RD_KEY0_DATA6_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key0_data6::R](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY0_DATA6 to value 0"]
impl crate::Resettable for RD_KEY0_DATA6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
