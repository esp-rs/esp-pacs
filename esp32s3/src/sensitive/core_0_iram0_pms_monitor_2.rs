#[doc = "Register `CORE_0_IRAM0_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR` reader - recorded core0 iram0 pms monitor interrupt status."]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR` reader - recorded core0 iram0 wr status, only if loadstore is 1 have meaning, 1(store), 0(load)."]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE` reader - recorded core0 iram0 loadstore status, indicated the type of operation, 0(fetch), 1(load/store)."]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - recorded core0 iram0 world status, 0x01 means world0, 0x10 means world1."]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::FieldReader;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - recorded core0 iram0 address \\[25:2\\] status when core0 iram0 violated permission, the real address is 0x40000000+addr*4"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - recorded core0 iram0 pms monitor interrupt status."]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_intr(&self) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - recorded core0 iram0 wr status, only if loadstore is 1 have meaning, 1(store), 0(load)."]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_wr(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - recorded core0 iram0 loadstore status, indicated the type of operation, 0(fetch), 1(load/store)."]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_loadstore(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - recorded core0 iram0 world status, 0x01 means world0, 0x10 means world1."]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_world(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:28 - recorded core0 iram0 address \\[25:2\\] status when core0 iram0 violated permission, the real address is 0x40000000+addr*4"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_addr(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new((self.bits >> 5) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_IRAM0_PMS_MONITOR_2")
            .field(
                "core_0_iram0_pms_monitor_violate_intr",
                &format_args!("{}", self.core_0_iram0_pms_monitor_violate_intr().bit()),
            )
            .field(
                "core_0_iram0_pms_monitor_violate_status_wr",
                &format_args!(
                    "{}",
                    self.core_0_iram0_pms_monitor_violate_status_wr().bit()
                ),
            )
            .field(
                "core_0_iram0_pms_monitor_violate_status_loadstore",
                &format_args!(
                    "{}",
                    self.core_0_iram0_pms_monitor_violate_status_loadstore()
                        .bit()
                ),
            )
            .field(
                "core_0_iram0_pms_monitor_violate_status_world",
                &format_args!(
                    "{}",
                    self.core_0_iram0_pms_monitor_violate_status_world().bits()
                ),
            )
            .field(
                "core_0_iram0_pms_monitor_violate_status_addr",
                &format_args!(
                    "{}",
                    self.core_0_iram0_pms_monitor_violate_status_addr().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_IRAM0_PMS_MONITOR_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "core0 iram0 permission monitor configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_iram0_pms_monitor_2](index.html) module"]
pub struct CORE_0_IRAM0_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_iram0_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_IRAM0_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
