#[doc = "Register `TIME_HIGH1` reader"]
pub type R = crate::R<TIME_HIGH1_SPEC>;
#[doc = "Register `TIME_HIGH1` writer"]
pub type W = crate::W<TIME_HIGH1_SPEC>;
#[doc = "Field `TIMER_VALUE1_HIGH` reader - RTC timer high 16 bits"]
pub type TIMER_VALUE1_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_VALUE1_HIGH` writer - RTC timer high 16 bits"]
pub type TIMER_VALUE1_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn timer_value1_high(&self) -> TIMER_VALUE1_HIGH_R {
        TIMER_VALUE1_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_HIGH1")
            .field(
                "timer_value1_high",
                &format_args!("{}", self.timer_value1_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_HIGH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_value1_high(&mut self) -> TIMER_VALUE1_HIGH_W<TIME_HIGH1_SPEC> {
        TIMER_VALUE1_HIGH_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_high1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_high1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_HIGH1_SPEC;
impl crate::RegisterSpec for TIME_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_high1::R`](R) reader structure"]
impl crate::Readable for TIME_HIGH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_high1::W`](W) writer structure"]
impl crate::Writable for TIME_HIGH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME_HIGH1 to value 0"]
impl crate::Resettable for TIME_HIGH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
