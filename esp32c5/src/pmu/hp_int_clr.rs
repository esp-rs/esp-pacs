#[doc = "Register `HP_INT_CLR` writer"]
pub type W = crate::W<HP_INT_CLR_SPEC>;
#[doc = "Field `LP_CPU_EXC_INT_CLR` writer - need_des"]
pub type LP_CPU_EXC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - need_des"]
pub type SDIO_IDLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INT_CLR` writer - need_des"]
pub type SW_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_CLR` writer - need_des"]
pub type SOC_SLEEP_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP_INT_CLR` writer - need_des"]
pub type SOC_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_clr(&mut self) -> LP_CPU_EXC_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        LP_CPU_EXC_INT_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        SDIO_IDLE_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_clr(&mut self) -> SW_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        SW_INT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_clr(&mut self) -> SOC_SLEEP_REJECT_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        SOC_SLEEP_REJECT_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_clr(&mut self) -> SOC_WAKEUP_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        SOC_WAKEUP_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_INT_CLR to value 0"]
impl crate::Resettable for HP_INT_CLR_SPEC {}
