#[doc = "Register `T0LO` reader"]
pub type R = crate::R<T0LO_SPEC>;
#[doc = "Field `T_LO` reader - Represents the low 32 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
pub type T_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the low 32 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t_lo(&self) -> T_LO_R {
        T_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LO").field("t_lo", &self.t_lo()).finish()
    }
}
#[doc = "Timer 0 current value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t0lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LO_SPEC;
impl crate::RegisterSpec for T0LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0lo::R`](R) reader structure"]
impl crate::Readable for T0LO_SPEC {}
#[doc = "`reset()` method sets T0LO to value 0"]
impl crate::Resettable for T0LO_SPEC {}
