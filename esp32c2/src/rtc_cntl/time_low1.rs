#[doc = "Register `TIME_LOW1` reader"]
pub type R = crate::R<TIME_LOW1_SPEC>;
#[doc = "Register `TIME_LOW1` writer"]
pub type W = crate::W<TIME_LOW1_SPEC>;
#[doc = "Field `TIMER_VALUE1_LOW` reader - RTC timer low 32 bits"]
pub type TIMER_VALUE1_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_VALUE1_LOW` writer - RTC timer low 32 bits"]
pub type TIMER_VALUE1_LOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
            .field(
                "timer_value1_low",
                &format_args!("{}", self.timer_value1_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_LOW1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_value1_low(&mut self) -> TIMER_VALUE1_LOW_W<TIME_LOW1_SPEC, 0> {
        TIMER_VALUE1_LOW_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_low1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_LOW1_SPEC;
impl crate::RegisterSpec for TIME_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_low1::R`](R) reader structure"]
impl crate::Readable for TIME_LOW1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_low1::W`](W) writer structure"]
impl crate::Writable for TIME_LOW1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME_LOW1 to value 0"]
impl crate::Resettable for TIME_LOW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
