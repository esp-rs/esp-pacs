#[doc = "Register `INT_CLR_TIMERS` writer"]
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
#[doc = "Field `T0_INT_CLR` writer - interrupt when timer0 alarm"]
pub type T0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `T1_INT_CLR` writer - interrupt when timer1 alarm"]
pub type T1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_INT_CLR` writer - Interrupt when an interrupt stage timeout"]
pub type WDT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LACT_INT_CLR` writer - "]
pub type LACT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_clr(&mut self) -> T0_INT_CLR_W<INT_CLR_TIMERS_SPEC, 0> {
        T0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t1_int_clr(&mut self) -> T1_INT_CLR_W<INT_CLR_TIMERS_SPEC, 1> {
        T1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_TIMERS_SPEC, 2> {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lact_int_clr(&mut self) -> LACT_INT_CLR_W<INT_CLR_TIMERS_SPEC, 3> {
        LACT_INT_CLR_W::new(self)
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
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_timers::W`](W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR_TIMERS to value 0"]
impl crate::Resettable for INT_CLR_TIMERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
