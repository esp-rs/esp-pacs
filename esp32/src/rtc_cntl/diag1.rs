#[doc = "Register `DIAG1` reader"]
pub type R = crate::R<DIAG1_SPEC>;
#[doc = "Field `LOW_POWER_DIAG1` reader - "]
pub type LOW_POWER_DIAG1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIAG1")
            .field("low_power_diag1", &self.low_power_diag1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIAG1_SPEC;
impl crate::RegisterSpec for DIAG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diag1::R`](R) reader structure"]
impl crate::Readable for DIAG1_SPEC {}
#[doc = "`reset()` method sets DIAG1 to value 0"]
impl crate::Resettable for DIAG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
