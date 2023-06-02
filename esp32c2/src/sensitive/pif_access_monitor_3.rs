#[doc = "Register `PIF_ACCESS_MONITOR_3` reader"]
pub struct R(crate::R<PIF_ACCESS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIF_ACCESS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIF_ACCESS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIF_ACCESS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_status_haddr(
        &self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_ACCESS_MONITOR_3")
            .field(
                "pif_access_monitor_nonword_violate_status_haddr",
                &format_args!(
                    "{}",
                    self.pif_access_monitor_nonword_violate_status_haddr()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIF_ACCESS_MONITOR_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pif_access_monitor_3](index.html) module"]
pub struct PIF_ACCESS_MONITOR_3_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pif_access_monitor_3::R](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_3 to value 0"]
impl crate::Resettable for PIF_ACCESS_MONITOR_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
