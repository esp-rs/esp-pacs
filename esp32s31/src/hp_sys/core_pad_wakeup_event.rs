#[doc = "Register `CORE_PAD_WAKEUP_EVENT` reader"]
pub type R = crate::R<CORE_PAD_WAKEUP_EVENT_SPEC>;
#[doc = "Register `CORE_PAD_WAKEUP_EVENT` writer"]
pub type W = crate::W<CORE_PAD_WAKEUP_EVENT_SPEC>;
#[doc = "Field `CORE0_PAD_WAKEUP_EVENT` reader - wakeup core0 register"]
pub type CORE0_PAD_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CORE0_PAD_WAKEUP_EVENT` writer - wakeup core0 register"]
pub type CORE0_PAD_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_PAD_WAKEUP_EVENT` reader - wakeup core1 register"]
pub type CORE1_PAD_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CORE1_PAD_WAKEUP_EVENT` writer - wakeup core1 register"]
pub type CORE1_PAD_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - wakeup core0 register"]
    #[inline(always)]
    pub fn core0_pad_wakeup_event(&self) -> CORE0_PAD_WAKEUP_EVENT_R {
        CORE0_PAD_WAKEUP_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup core1 register"]
    #[inline(always)]
    pub fn core1_pad_wakeup_event(&self) -> CORE1_PAD_WAKEUP_EVENT_R {
        CORE1_PAD_WAKEUP_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_PAD_WAKEUP_EVENT")
            .field("core0_pad_wakeup_event", &self.core0_pad_wakeup_event())
            .field("core1_pad_wakeup_event", &self.core1_pad_wakeup_event())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - wakeup core0 register"]
    #[inline(always)]
    pub fn core0_pad_wakeup_event(
        &mut self,
    ) -> CORE0_PAD_WAKEUP_EVENT_W<'_, CORE_PAD_WAKEUP_EVENT_SPEC> {
        CORE0_PAD_WAKEUP_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup core1 register"]
    #[inline(always)]
    pub fn core1_pad_wakeup_event(
        &mut self,
    ) -> CORE1_PAD_WAKEUP_EVENT_W<'_, CORE_PAD_WAKEUP_EVENT_SPEC> {
        CORE1_PAD_WAKEUP_EVENT_W::new(self, 1)
    }
}
#[doc = "pad wakeup event register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_pad_wakeup_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_pad_wakeup_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_PAD_WAKEUP_EVENT_SPEC;
impl crate::RegisterSpec for CORE_PAD_WAKEUP_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_pad_wakeup_event::R`](R) reader structure"]
impl crate::Readable for CORE_PAD_WAKEUP_EVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_pad_wakeup_event::W`](W) writer structure"]
impl crate::Writable for CORE_PAD_WAKEUP_EVENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_PAD_WAKEUP_EVENT to value 0"]
impl crate::Resettable for CORE_PAD_WAKEUP_EVENT_SPEC {}
