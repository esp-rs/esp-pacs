#[doc = "Register `INTR_HP2LP_STATUS_3` reader"]
pub type R = crate::R<INTR_HP2LP_STATUS_3_SPEC>;
#[doc = "Field `INTR_HP2LP_STATUS_3` reader - reserved"]
pub type INTR_HP2LP_STATUS_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn intr_hp2lp_status_3(&self) -> INTR_HP2LP_STATUS_3_R {
        INTR_HP2LP_STATUS_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_HP2LP_STATUS_3")
            .field("intr_hp2lp_status_3", &self.intr_hp2lp_status_3())
            .finish()
    }
}
#[doc = "intr hp2lp status register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_status_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HP2LP_STATUS_3_SPEC;
impl crate::RegisterSpec for INTR_HP2LP_STATUS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_hp2lp_status_3::R`](R) reader structure"]
impl crate::Readable for INTR_HP2LP_STATUS_3_SPEC {}
#[doc = "`reset()` method sets INTR_HP2LP_STATUS_3 to value 0"]
impl crate::Resettable for INTR_HP2LP_STATUS_3_SPEC {}
