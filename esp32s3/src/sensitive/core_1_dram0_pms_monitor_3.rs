#[doc = "Register `CORE_1_DRAM0_PMS_MONITOR_3` reader"]
pub struct R(crate::R<CORE_1_DRAM0_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR` reader - recorded core1 dram0 wr status, 1 means store, 0 means load."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - recorded core1 dram0 byteen status."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - recorded core1 dram0 wr status, 1 means store, 0 means load."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_wr(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - recorded core1 dram0 byteen status."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_byteen(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_PMS_MONITOR_3")
            .field(
                "core_1_dram0_pms_monitor_violate_status_wr",
                &format_args!(
                    "{}",
                    self.core_1_dram0_pms_monitor_violate_status_wr().bit()
                ),
            )
            .field(
                "core_1_dram0_pms_monitor_violate_status_byteen",
                &format_args!(
                    "{}",
                    self.core_1_dram0_pms_monitor_violate_status_byteen().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_DRAM0_PMS_MONITOR_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "core1 dram0 permission monitor configuration register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_pms_monitor_3](index.html) module"]
pub struct CORE_1_DRAM0_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_PMS_MONITOR_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
