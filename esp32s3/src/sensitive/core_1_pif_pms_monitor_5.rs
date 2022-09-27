#[doc = "Register `CORE_1_PIF_PMS_MONITOR_5` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_MONITOR_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_MONITOR_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_MONITOR_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_MONITOR_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR` reader - Record core1 unsupported access type interrupt state."]
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE` reader - Record access type when core1 initiate unsupported access type."]
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD` reader - Record world information when core1 initiate unsupported access type."]
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Record core1 unsupported access type interrupt state."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_nonword_violate_intr(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Record access type when core1 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_nonword_violate_status_hsize(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Record world information when core1 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_nonword_violate_status_hworld(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "core1 permission report register 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_monitor_5](index.html) module"]
pub struct CORE_1_PIF_PMS_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_monitor_5::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_MONITOR_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_MONITOR_5 to value 0"]
impl crate::Resettable for CORE_1_PIF_PMS_MONITOR_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
