#[doc = "Register `SSTAT0` reader"]
pub type R = crate::R<SSTAT0_SPEC>;
#[doc = "Field `CH1_SSTAT` reader - NA"]
pub type CH1_SSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstat(&self) -> CH1_SSTAT_R {
        CH1_SSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTAT0")
            .field("ch1_sstat", &self.ch1_sstat())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTAT0_SPEC;
impl crate::RegisterSpec for SSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat0::R`](R) reader structure"]
impl crate::Readable for SSTAT0_SPEC {}
#[doc = "`reset()` method sets SSTAT0 to value 0"]
impl crate::Resettable for SSTAT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
