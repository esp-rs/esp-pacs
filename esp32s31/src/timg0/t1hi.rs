#[doc = "Register `T1HI` reader"]
pub type R = crate::R<T1HI_SPEC>;
#[doc = "Field `T_HI` reader - Represents the high 22 bits of the time-base counter of timer 1. Valid only after writing to TIMG_T1UPDATE_REG. \\\\ Measurement unit: T1_clk \\\\"]
pub type T_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Represents the high 22 bits of the time-base counter of timer 1. Valid only after writing to TIMG_T1UPDATE_REG. \\\\ Measurement unit: T1_clk \\\\"]
    #[inline(always)]
    pub fn t_hi(&self) -> T_HI_R {
        T_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1HI").field("t_hi", &self.t_hi()).finish()
    }
}
#[doc = "Timer 1 current value, high 22 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t1hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1HI_SPEC;
impl crate::RegisterSpec for T1HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1hi::R`](R) reader structure"]
impl crate::Readable for T1HI_SPEC {}
#[doc = "`reset()` method sets T1HI to value 0"]
impl crate::Resettable for T1HI_SPEC {}
