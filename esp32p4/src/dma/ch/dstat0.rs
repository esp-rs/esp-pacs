#[doc = "Register `DSTAT0` reader"]
pub type R = crate::R<DSTAT0_SPEC>;
#[doc = "Field `CH1_DSTAT` reader - NA"]
pub type CH1_DSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dstat(&self) -> CH1_DSTAT_R {
        CH1_DSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTAT0")
            .field("ch1_dstat", &self.ch1_dstat())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTAT0_SPEC;
impl crate::RegisterSpec for DSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat0::R`](R) reader structure"]
impl crate::Readable for DSTAT0_SPEC {}
#[doc = "`reset()` method sets DSTAT0 to value 0"]
impl crate::Resettable for DSTAT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
