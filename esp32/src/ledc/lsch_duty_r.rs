#[doc = "Register `LSCH%s_DUTY_R` reader"]
pub struct R(crate::R<LSCH_DUTY_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH_DUTY_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH_DUTY_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH_DUTY_R_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch_duty_r](index.html) module"]
pub struct LSCH_DUTY_R_SPEC;
impl crate::RegisterSpec for LSCH_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch_duty_r::R](R) reader structure"]
impl crate::Readable for LSCH_DUTY_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH%s_DUTY_R to value 0"]
impl crate::Resettable for LSCH_DUTY_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
