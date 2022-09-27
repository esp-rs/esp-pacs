#[doc = "Register `CORE_1_PIF_PMS_MONITOR_3` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR` reader - Record address information when core1 initiate illegal access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Record address information when core1 initiate illegal access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_status_haddr(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R::new(self.bits)
    }
}
#[doc = "core1 permission report register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_monitor_3](index.html) module"]
pub struct CORE_1_PIF_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for CORE_1_PIF_PMS_MONITOR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
