#[doc = "Register `CORE_1_PIF_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR` reader - Record core1 illegal access interrupt state."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0` reader - Record hport information when core1 initiate illegal access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE` reader - Record access type when core1 initate illegal access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE` reader - Record access direction when core1 initiate illegal access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD` reader - Record world information when core1 initiate illegal access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Record core1 illegal access interrupt state."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_intr(&self) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Record hport information when core1 initiate illegal access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_status_hport_0(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Record access type when core1 initate illegal access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_status_hsize(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Record access direction when core1 initiate illegal access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_status_hwrite(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Record world information when core1 initiate illegal access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_status_hworld(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "core1 permission report register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_monitor_2](index.html) module"]
pub struct CORE_1_PIF_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_1_PIF_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
