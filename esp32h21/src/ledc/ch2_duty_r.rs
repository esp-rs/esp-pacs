#[doc = "Register `CH2_DUTY_R` reader"]
pub type R = crate::R<CH2_DUTY_R_SPEC>;
#[doc = "Field `DUTY_CH2_R` reader - This register stores the current duty of output signal on channel 2."]
pub type DUTY_CH2_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - This register stores the current duty of output signal on channel 2."]
    #[inline(always)]
    pub fn duty_ch2_r(&self) -> DUTY_CH2_R_R {
        DUTY_CH2_R_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_DUTY_R")
            .field("duty_ch2_r", &self.duty_ch2_r())
            .finish()
    }
}
#[doc = "Current duty cycle for channel $n\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_duty_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_DUTY_R_SPEC;
impl crate::RegisterSpec for CH2_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_duty_r::R`](R) reader structure"]
impl crate::Readable for CH2_DUTY_R_SPEC {}
#[doc = "`reset()` method sets CH2_DUTY_R to value 0"]
impl crate::Resettable for CH2_DUTY_R_SPEC {}
