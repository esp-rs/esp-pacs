#[doc = "Register `CH%s_DUTY_R` reader"]
pub type R = crate::R<CH_DUTY_R_SPEC>;
#[doc = "Field `DUTY_R` reader - reg_duty_lsch0_r."]
pub type DUTY_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch0_r."]
    #[inline(always)]
    pub fn duty_r(&self) -> DUTY_R_R {
        DUTY_R_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DUTY_R")
            .field("duty_r", &format_args!("{}", self.duty_r().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_DUTY_R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_duty_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DUTY_R_SPEC;
impl crate::RegisterSpec for CH_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_duty_r::R`](R) reader structure"]
impl crate::Readable for CH_DUTY_R_SPEC {}
#[doc = "`reset()` method sets CH%s_DUTY_R to value 0"]
impl crate::Resettable for CH_DUTY_R_SPEC {
    const RESET_VALUE: u32 = 0;
}
