#[doc = "Register `TIME1` reader"]
pub type R = crate::R<TIME1_SPEC>;
#[doc = "Field `TIME_HI` reader - RTC timer high 16 bits"]
pub type TIME_HI_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn time_hi(&self) -> TIME_HI_R {
        TIME_HI_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME1")
            .field("time_hi", &self.time_hi())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME1_SPEC;
impl crate::RegisterSpec for TIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time1::R`](R) reader structure"]
impl crate::Readable for TIME1_SPEC {}
#[doc = "`reset()` method sets TIME1 to value 0"]
impl crate::Resettable for TIME1_SPEC {
    const RESET_VALUE: u32 = 0;
}
