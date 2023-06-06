#[doc = "Register `APB_SARADC2_DATA_STATUS` reader"]
pub struct R(crate::R<APB_SARADC2_DATA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC2_DATA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC2_DATA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC2_DATA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_SARADC2_DATA` reader - apb saradc2 sample data"]
pub type APB_SARADC2_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - apb saradc2 sample data"]
    #[inline(always)]
    pub fn apb_saradc2_data(&self) -> APB_SARADC2_DATA_R {
        APB_SARADC2_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC2_DATA_STATUS")
            .field(
                "apb_saradc2_data",
                &format_args!("{}", self.apb_saradc2_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_SARADC2_DATA_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "get apb saradc2 sample data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc2_data_status](index.html) module"]
pub struct APB_SARADC2_DATA_STATUS_SPEC;
impl crate::RegisterSpec for APB_SARADC2_DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc2_data_status::R](R) reader structure"]
impl crate::Readable for APB_SARADC2_DATA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB_SARADC2_DATA_STATUS to value 0"]
impl crate::Resettable for APB_SARADC2_DATA_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
