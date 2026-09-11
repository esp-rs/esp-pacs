#[doc = "Register `TBTT_CFG%s` reader"]
pub type R = crate::R<TBTT_CFG_SPEC>;
#[doc = "Register `TBTT_CFG%s` writer"]
pub type W = crate::W<TBTT_CFG_SPEC>;
#[doc = "Field `INTERVAL` reader - Beacon interval in TUs (the blob shifts microseconds right by 10)"]
pub type INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - Beacon interval in TUs (the blob shifts microseconds right by 10)"]
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EARLY_TIME` reader - How much earlier than the TBTT the interrupt fires, set by tsf_hal_set_tbtt_early_time"]
pub type EARLY_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `EARLY_TIME` writer - How much earlier than the TBTT the interrupt fires, set by tsf_hal_set_tbtt_early_time"]
pub type EARLY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Beacon interval in TUs (the blob shifts microseconds right by 10)"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - How much earlier than the TBTT the interrupt fires, set by tsf_hal_set_tbtt_early_time"]
    #[inline(always)]
    pub fn early_time(&self) -> EARLY_TIME_R {
        EARLY_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBTT_CFG")
            .field("interval", &self.interval())
            .field("early_time", &self.early_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Beacon interval in TUs (the blob shifts microseconds right by 10)"]
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W<'_, TBTT_CFG_SPEC> {
        INTERVAL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - How much earlier than the TBTT the interrupt fires, set by tsf_hal_set_tbtt_early_time"]
    #[inline(always)]
    pub fn early_time(&mut self) -> EARLY_TIME_W<'_, TBTT_CFG_SPEC> {
        EARLY_TIME_W::new(self, 16)
    }
}
#[doc = "TBTT interval and early time of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBTT_CFG_SPEC;
impl crate::RegisterSpec for TBTT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbtt_cfg::R`](R) reader structure"]
impl crate::Readable for TBTT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbtt_cfg::W`](W) writer structure"]
impl crate::Writable for TBTT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBTT_CFG%s to value 0"]
impl crate::Resettable for TBTT_CFG_SPEC {}
