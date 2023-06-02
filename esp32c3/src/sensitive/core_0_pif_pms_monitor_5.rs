#[doc = "Register `CORE_0_PIF_PMS_MONITOR_5` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR` reader - core_0_pif_pms_monitor_nonword_violate_intr"]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE` reader - core_0_pif_pms_monitor_nonword_violate_status_hsize"]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD` reader - core_0_pif_pms_monitor_nonword_violate_status_hworld"]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - core_0_pif_pms_monitor_nonword_violate_intr"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_intr(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - core_0_pif_pms_monitor_nonword_violate_status_hsize"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hsize(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - core_0_pif_pms_monitor_nonword_violate_status_hworld"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hworld(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_MONITOR_5")
            .field(
                "core_0_pif_pms_monitor_nonword_violate_intr",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_intr().bit()
                ),
            )
            .field(
                "core_0_pif_pms_monitor_nonword_violate_status_hsize",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_status_hsize()
                        .bits()
                ),
            )
            .field(
                "core_0_pif_pms_monitor_nonword_violate_status_hworld",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_status_hworld()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_MONITOR_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_monitor_5](index.html) module"]
pub struct CORE_0_PIF_PMS_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_monitor_5::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_5 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
