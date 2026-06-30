#[doc = "Register `HP_CORE_PAD_WAKEUP_EVENT` reader"]
pub type R = crate::R<HP_CORE_PAD_WAKEUP_EVENT_SPEC>;
#[doc = "Register `HP_CORE_PAD_WAKEUP_EVENT` writer"]
pub type W = crate::W<HP_CORE_PAD_WAKEUP_EVENT_SPEC>;
#[doc = "Field `HP_CORE0_PAD_WAKEUP_EVENT` reader - wakeup core0 register"]
pub type HP_CORE0_PAD_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `HP_CORE0_PAD_WAKEUP_EVENT` writer - wakeup core0 register"]
pub type HP_CORE0_PAD_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_PAD_WAKEUP_EVENT` reader - wakeup core1 register"]
pub type HP_CORE1_PAD_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `HP_CORE1_PAD_WAKEUP_EVENT` writer - wakeup core1 register"]
pub type HP_CORE1_PAD_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - wakeup core0 register"]
    #[inline(always)]
    pub fn hp_core0_pad_wakeup_event(&self) -> HP_CORE0_PAD_WAKEUP_EVENT_R {
        HP_CORE0_PAD_WAKEUP_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup core1 register"]
    #[inline(always)]
    pub fn hp_core1_pad_wakeup_event(&self) -> HP_CORE1_PAD_WAKEUP_EVENT_R {
        HP_CORE1_PAD_WAKEUP_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_PAD_WAKEUP_EVENT")
            .field(
                "hp_core0_pad_wakeup_event",
                &self.hp_core0_pad_wakeup_event(),
            )
            .field(
                "hp_core1_pad_wakeup_event",
                &self.hp_core1_pad_wakeup_event(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - wakeup core0 register"]
    #[inline(always)]
    pub fn hp_core0_pad_wakeup_event(
        &mut self,
    ) -> HP_CORE0_PAD_WAKEUP_EVENT_W<'_, HP_CORE_PAD_WAKEUP_EVENT_SPEC> {
        HP_CORE0_PAD_WAKEUP_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup core1 register"]
    #[inline(always)]
    pub fn hp_core1_pad_wakeup_event(
        &mut self,
    ) -> HP_CORE1_PAD_WAKEUP_EVENT_W<'_, HP_CORE_PAD_WAKEUP_EVENT_SPEC> {
        HP_CORE1_PAD_WAKEUP_EVENT_W::new(self, 1)
    }
}
#[doc = "pad wakeup event register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_pad_wakeup_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_pad_wakeup_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_PAD_WAKEUP_EVENT_SPEC;
impl crate::RegisterSpec for HP_CORE_PAD_WAKEUP_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_pad_wakeup_event::R`](R) reader structure"]
impl crate::Readable for HP_CORE_PAD_WAKEUP_EVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_core_pad_wakeup_event::W`](W) writer structure"]
impl crate::Writable for HP_CORE_PAD_WAKEUP_EVENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CORE_PAD_WAKEUP_EVENT to value 0"]
impl crate::Resettable for HP_CORE_PAD_WAKEUP_EVENT_SPEC {}
