#[doc = "Register `LSCH%s_DUTY_R` reader"]
pub type R = crate::R<LSCH_DUTY_R_SPEC>;
#[doc = "Field `DUTY_R` reader - This register represents the current duty cycle of the output signal for low-speed channel %s"]
pub type DUTY_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty cycle of the output signal for low-speed channel %s"]
    #[inline(always)]
    pub fn duty_r(&self) -> DUTY_R_R {
        DUTY_R_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSCH_DUTY_R")
            .field("duty_r", &format_args!("{}", self.duty_r().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LSCH_DUTY_R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_duty_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSCH_DUTY_R_SPEC;
impl crate::RegisterSpec for LSCH_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsch_duty_r::R`](R) reader structure"]
impl crate::Readable for LSCH_DUTY_R_SPEC {}
#[doc = "`reset()` method sets LSCH%s_DUTY_R to value 0"]
impl crate::Resettable for LSCH_DUTY_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
