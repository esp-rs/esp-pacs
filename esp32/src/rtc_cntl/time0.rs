#[doc = "Register `TIME0` reader"]
pub type R = crate::R<TIME0_SPEC>;
#[doc = "Field `TIME_LO` reader - RTC timer low 32 bits"]
pub type TIME_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn time_lo(&self) -> TIME_LO_R {
        TIME_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME0")
            .field("time_lo", &self.time_lo())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME0_SPEC;
impl crate::RegisterSpec for TIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time0::R`](R) reader structure"]
impl crate::Readable for TIME0_SPEC {}
#[doc = "`reset()` method sets TIME0 to value 0"]
impl crate::Resettable for TIME0_SPEC {
    const RESET_VALUE: u32 = 0;
}
