#[doc = "Register `DUTY_R` reader"]
pub type R = crate::R<DUTY_R_SPEC>;
#[doc = "Field `DUTY_CH0_R` reader - This register stores the current duty of output signal on channel %s."]
pub type DUTY_CH0_R_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - This register stores the current duty of output signal on channel %s."]
    #[inline(always)]
    pub fn duty_ch0_r(&self) -> DUTY_CH0_R_R {
        DUTY_CH0_R_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTY_R")
            .field("duty_ch0_r", &format_args!("{}", self.duty_ch0_r().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DUTY_R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Current duty cycle for channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`duty_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUTY_R_SPEC;
impl crate::RegisterSpec for DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty_r::R`](R) reader structure"]
impl crate::Readable for DUTY_R_SPEC {}
#[doc = "`reset()` method sets DUTY_R to value 0"]
impl crate::Resettable for DUTY_R_SPEC {
    const RESET_VALUE: u32 = 0;
}
