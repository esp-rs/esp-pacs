#[doc = "Register `RD_KEY1_DATA%s` reader"]
pub struct R(crate::R<RD_KEY1_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY1_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY1_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY1_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY1_DATA0` reader - Stores the %sth 32 bits of KEY1."]
pub type KEY1_DATA0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY1."]
    #[inline(always)]
    pub fn key1_data0(&self) -> KEY1_DATA0_R {
        KEY1_DATA0_R::new(self.bits)
    }
}
#[doc = "Register %s of BLOCK5 (KEY1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data](index.html) module"]
pub struct RD_KEY1_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY1_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key1_data::R](R) reader structure"]
impl crate::Readable for RD_KEY1_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY1_DATA%s to value 0"]
impl crate::Resettable for RD_KEY1_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
