#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SUPER_WDT_INT_CLR` writer - Configure whether to clear the timeout interrupt signal sent by SWD to CPU.\\\\0: No\\\\1: Yes"]
pub type SUPER_WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - Configure whether to clear the timeout interrupt signal sent by RWDT to CPU.\\\\0: No\\\\1: Yes"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - Configure whether to clear the timeout interrupt signal sent by SWD to CPU.\\\\0: No\\\\1: Yes"]
    #[inline(always)]
    pub fn super_wdt_int_clr(&mut self) -> SUPER_WDT_INT_CLR_W<INT_CLR_SPEC> {
        SUPER_WDT_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configure whether to clear the timeout interrupt signal sent by RWDT to CPU.\\\\0: No\\\\1: Yes"]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_SPEC> {
        WDT_INT_CLR_W::new(self, 31)
    }
}
#[doc = "The interrupt clear register of WDT\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
