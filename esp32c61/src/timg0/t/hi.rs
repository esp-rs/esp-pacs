#[doc = "Register `HI` reader"]
pub type R = crate::R<HI_SPEC>;
#[doc = "Field `T0_HI` reader - Represents the high 22 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Represents the high 22 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_hi(&self) -> T0_HI_R {
        T0_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("t0_hi", &self.t0_hi()).finish()
    }
}
#[doc = "Timer 0 current value, high 22 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi::R`](R) reader structure"]
impl crate::Readable for HI_SPEC {}
#[doc = "`reset()` method sets HI to value 0"]
impl crate::Resettable for HI_SPEC {}
