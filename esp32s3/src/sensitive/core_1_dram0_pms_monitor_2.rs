#[doc = "Register `CORE_1_DRAM0_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR` reader - recorded core1 dram0 permission monitor interrupt status."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK` reader - recorded core1 dram0 lock status, 1 means s32c1i access."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - recorded core1 dram0 world status, 0x1 means world0, 0x2 means world1."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - recorded core1 dram0 address\\[25:4\\] status when core1 dram0 violated permission,the real address is 0x3c000000+addr*16"]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - recorded core1 dram0 permission monitor interrupt status."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_intr(&self) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - recorded core1 dram0 lock status, 1 means s32c1i access."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_lock(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - recorded core1 dram0 world status, 0x1 means world0, 0x2 means world1."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_world(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:25 - recorded core1 dram0 address\\[25:4\\] status when core1 dram0 violated permission,the real address is 0x3c000000+addr*16"]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_addr(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(((self.bits >> 4) & 0x003f_ffff) as u32)
    }
}
#[doc = "core1 dram0 permission monitor configuration register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_pms_monitor_2](index.html) module"]
pub struct CORE_1_DRAM0_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
