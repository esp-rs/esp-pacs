#[doc = "Register `APB_SARADC1_DATA_STATUS` reader"]
pub struct R(crate::R<APB_SARADC1_DATA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC1_DATA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC1_DATA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC1_DATA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_SARADC1_DATA` reader - apbsaradc sample data"]
pub type APB_SARADC1_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:16 - apbsaradc sample data"]
    #[inline(always)]
    pub fn apb_saradc1_data(&self) -> APB_SARADC1_DATA_R {
        APB_SARADC1_DATA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
#[doc = "get apb saradc sample data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc1_data_status](index.html) module"]
pub struct APB_SARADC1_DATA_STATUS_SPEC;
impl crate::RegisterSpec for APB_SARADC1_DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc1_data_status::R](R) reader structure"]
impl crate::Readable for APB_SARADC1_DATA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB_SARADC1_DATA_STATUS to value 0"]
impl crate::Resettable for APB_SARADC1_DATA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
