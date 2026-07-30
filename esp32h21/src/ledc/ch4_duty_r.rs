#[doc = "Register `CH4_DUTY_R` reader"]
pub type R = crate::R<CH4_DUTY_R_SPEC>;
#[doc = "Field `DUTY_CH4_R` reader - This register stores the current duty of output signal on channel 4."]
pub type DUTY_CH4_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - This register stores the current duty of output signal on channel 4."]
    #[inline(always)]
    pub fn duty_ch4_r(&self) -> DUTY_CH4_R_R {
        DUTY_CH4_R_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_DUTY_R")
            .field("duty_ch4_r", &self.duty_ch4_r())
            .finish()
    }
}
#[doc = "Current duty cycle for channel $n\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_duty_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_DUTY_R_SPEC;
impl crate::RegisterSpec for CH4_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_duty_r::R`](R) reader structure"]
impl crate::Readable for CH4_DUTY_R_SPEC {}
#[doc = "`reset()` method sets CH4_DUTY_R to value 0"]
impl crate::Resettable for CH4_DUTY_R_SPEC {}
