#[doc = "Register `INT_ENA_TIMERS` reader"]
pub type R = crate::R<INT_ENA_TIMERS_SPEC>;
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub type W = crate::W<INT_ENA_TIMERS_SPEC>;
#[doc = "Field `T0_INT_ENA` reader - t0_int_ena"]
pub type T0_INT_ENA_R = crate::BitReader;
#[doc = "Field `T0_INT_ENA` writer - t0_int_ena"]
pub type T0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA` reader - wdt_int_ena"]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - wdt_int_ena"]
pub type WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0_int_ena", &format_args!("{}", self.t0_int_ena().bit()))
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        T0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        WDT_INT_ENA_W::new(self, 1)
    }
}
#[doc = "INT_ENA_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_timers::R`](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_timers::W`](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
