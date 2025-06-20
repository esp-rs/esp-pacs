#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `OVERFLOW_CLR` writer - Clear the RTC main timer overflow raw interrupt.."]
pub type OVERFLOW_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP_INT_CLR` writer - Clear the RTC main timer raw interrupt.."]
pub type SOC_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - Clear the RTC main timer overflow raw interrupt.."]
    #[inline(always)]
    pub fn overflow_clr(&mut self) -> OVERFLOW_CLR_W<INT_CLR_SPEC> {
        OVERFLOW_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear the RTC main timer raw interrupt.."]
    #[inline(always)]
    pub fn soc_wakeup_int_clr(&mut self) -> SOC_WAKEUP_INT_CLR_W<INT_CLR_SPEC> {
        SOC_WAKEUP_INT_CLR_W::new(self, 31)
    }
}
#[doc = "RTC timer interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
