#[doc = "Register `TIME_LOW1` reader"]
pub type R = crate::R<TIME_LOW1_SPEC>;
#[doc = "Register `TIME_LOW1` writer"]
pub type W = crate::W<TIME_LOW1_SPEC>;
#[doc = "Field `TIMER_VALUE1_LOW` reader - RTC timer low 32 bits"]
pub type TIMER_VALUE1_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_VALUE1_LOW` writer - RTC timer low 32 bits"]
pub type TIMER_VALUE1_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn timer_value1_low(&self) -> TIMER_VALUE1_LOW_R {
        TIMER_VALUE1_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_LOW1")
            .field("timer_value1_low", &self.timer_value1_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_value1_low(&mut self) -> TIMER_VALUE1_LOW_W<TIME_LOW1_SPEC> {
        TIMER_VALUE1_LOW_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_low1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_low1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_LOW1_SPEC;
impl crate::RegisterSpec for TIME_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_low1::R`](R) reader structure"]
impl crate::Readable for TIME_LOW1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_low1::W`](W) writer structure"]
impl crate::Writable for TIME_LOW1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME_LOW1 to value 0"]
impl crate::Resettable for TIME_LOW1_SPEC {
    const RESET_VALUE: u32 = 0;
}
