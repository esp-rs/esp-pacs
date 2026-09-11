#[doc = "Register `TSF_TIMER_CFG%s` reader"]
pub type R = crate::R<TSF_TIMER_CFG_SPEC>;
#[doc = "Register `TSF_TIMER_CFG%s` writer"]
pub type W = crate::W<TSF_TIMER_CFG_SPEC>;
#[doc = "Field `SOC_WAKEUP` reader - Wake the SoC when the timer fires, set by tsf_hal_set_timer_soc_wakeup_enable"]
pub type SOC_WAKEUP_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP` writer - Wake the SoC when the timer fires, set by tsf_hal_set_timer_soc_wakeup_enable"]
pub type SOC_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Set by tsf_hal_set_timer_enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Set by tsf_hal_set_timer_enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Wake the SoC when the timer fires, set by tsf_hal_set_timer_soc_wakeup_enable"]
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set by tsf_hal_set_timer_enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSF_TIMER_CFG")
            .field("soc_wakeup", &self.soc_wakeup())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Wake the SoC when the timer fires, set by tsf_hal_set_timer_soc_wakeup_enable"]
    #[inline(always)]
    pub fn soc_wakeup(&mut self) -> SOC_WAKEUP_W<'_, TSF_TIMER_CFG_SPEC> {
        SOC_WAKEUP_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set by tsf_hal_set_timer_enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, TSF_TIMER_CFG_SPEC> {
        ENABLE_W::new(self, 31)
    }
}
#[doc = "Configuration of a TSF timer\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_TIMER_CFG_SPEC;
impl crate::RegisterSpec for TSF_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_timer_cfg::R`](R) reader structure"]
impl crate::Readable for TSF_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_timer_cfg::W`](W) writer structure"]
impl crate::Writable for TSF_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_TIMER_CFG%s to value 0"]
impl crate::Resettable for TSF_TIMER_CFG_SPEC {}
