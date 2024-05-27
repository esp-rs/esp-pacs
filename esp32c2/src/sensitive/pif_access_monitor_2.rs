#[doc = "Register `PIF_ACCESS_MONITOR_2` reader"]
pub type R = crate::R<PIF_ACCESS_MONITOR_2_SPEC>;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_INTR` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_intr(
        &self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_INTR_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_status_hsize(
        &self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_ACCESS_MONITOR_2")
            .field(
                "pif_access_monitor_nonword_violate_intr",
                &self.pif_access_monitor_nonword_violate_intr(),
            )
            .field(
                "pif_access_monitor_nonword_violate_status_hsize",
                &self.pif_access_monitor_nonword_violate_status_hsize(),
            )
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pif_access_monitor_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIF_ACCESS_MONITOR_2_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pif_access_monitor_2::R`](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_2_SPEC {}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_2 to value 0"]
impl crate::Resettable for PIF_ACCESS_MONITOR_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
