#[doc = "Register `CH%s_DUTY_R` reader"]
pub type R = crate::R<CH_DUTY_R_SPEC>;
#[doc = "Field `DUTY_CH_R` reader - Represents the current duty of output signal on channel %s."]
pub type DUTY_CH_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Represents the current duty of output signal on channel %s."]
    #[inline(always)]
    pub fn duty_ch_r(&self) -> DUTY_CH_R_R {
        DUTY_CH_R_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DUTY_R")
            .field("duty_ch_r", &self.duty_ch_r())
            .finish()
    }
}
#[doc = "Current duty cycle register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_duty_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DUTY_R_SPEC;
impl crate::RegisterSpec for CH_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_duty_r::R`](R) reader structure"]
impl crate::Readable for CH_DUTY_R_SPEC {}
#[doc = "`reset()` method sets CH%s_DUTY_R to value 0"]
impl crate::Resettable for CH_DUTY_R_SPEC {}
