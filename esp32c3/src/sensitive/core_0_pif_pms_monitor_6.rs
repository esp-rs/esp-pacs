#[doc = "Register `CORE_0_PIF_PMS_MONITOR_6` reader"]
pub type R = crate::R<CORE_0_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR` reader - core_0_pif_pms_monitor_nonword_violate_status_haddr"]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core_0_pif_pms_monitor_nonword_violate_status_haddr"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_haddr(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_MONITOR_6")
            .field(
                "core_0_pif_pms_monitor_nonword_violate_status_haddr",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_status_haddr()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_MONITOR_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_PIF_PMS_MONITOR_6_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_pif_pms_monitor_6::R`](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_6_SPEC {}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_6 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
