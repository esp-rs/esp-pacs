#[doc = "Register `BACKUP_BUS_PMS_MONITOR_3` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR` reader - backup_bus_pms_monitor_violate_haddr"]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - backup_bus_pms_monitor_violate_haddr"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_haddr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_MONITOR_3")
            .field(
                "backup_bus_pms_monitor_violate_haddr",
                &format_args!("{}", self.backup_bus_pms_monitor_violate_haddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_MONITOR_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_monitor_3](index.html) module"]
pub struct BACKUP_BUS_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
