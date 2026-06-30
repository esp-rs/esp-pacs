#[doc = "Register `TIMER%s_SYNC` reader"]
pub type R = crate::R<TIMER_SYNC_SPEC>;
#[doc = "Register `TIMER%s_SYNC` writer"]
pub type W = crate::W<TIMER_SYNC_SPEC>;
#[doc = "Field `TIMER_SYNCI_EN` reader - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\\\0: Disable\\\\1: Enable"]
pub type TIMER_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `TIMER_SYNCI_EN` writer - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\\\0: Disable\\\\1: Enable"]
pub type TIMER_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` reader - Configures the generation of software sync. Toggling this bit will trigger a software sync."]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - Configures the generation of software sync. Toggling this bit will trigger a software sync."]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_SYNCO_SEL` reader - Configures the selection of PWM timer%s sync_out.\\\\0: Sync_in\\\\1: TEZ\\\\2: TEP\\\\3: Invalid, sync_out selects noting"]
pub type TIMER_SYNCO_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SYNCO_SEL` writer - Configures the selection of PWM timer%s sync_out.\\\\0: Sync_in\\\\1: TEZ\\\\2: TEP\\\\3: Invalid, sync_out selects noting"]
pub type TIMER_SYNCO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER_PHASE` reader - Configures the phase for timer%s reload on sync event."]
pub type TIMER_PHASE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_PHASE` writer - Configures the phase for timer%s reload on sync event."]
pub type TIMER_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER_PHASE_DIRECTION` reader - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\\\0: Increase\\\\1: Decrease"]
pub type TIMER_PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `TIMER_PHASE_DIRECTION` writer - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\\\0: Increase\\\\1: Decrease"]
pub type TIMER_PHASE_DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer_synci_en(&self) -> TIMER_SYNCI_EN_R {
        TIMER_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the generation of software sync. Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the selection of PWM timer%s sync_out.\\\\0: Sync_in\\\\1: TEZ\\\\2: TEP\\\\3: Invalid, sync_out selects noting"]
    #[inline(always)]
    pub fn timer_synco_sel(&self) -> TIMER_SYNCO_SEL_R {
        TIMER_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19 - Configures the phase for timer%s reload on sync event."]
    #[inline(always)]
    pub fn timer_phase(&self) -> TIMER_PHASE_R {
        TIMER_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\\\0: Increase\\\\1: Decrease"]
    #[inline(always)]
    pub fn timer_phase_direction(&self) -> TIMER_PHASE_DIRECTION_R {
        TIMER_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_SYNC")
            .field("timer_synci_en", &self.timer_synci_en())
            .field("sw", &self.sw())
            .field("timer_synco_sel", &self.timer_synco_sel())
            .field("timer_phase", &self.timer_phase())
            .field("timer_phase_direction", &self.timer_phase_direction())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer_synci_en(&mut self) -> TIMER_SYNCI_EN_W<'_, TIMER_SYNC_SPEC> {
        TIMER_SYNCI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the generation of software sync. Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, TIMER_SYNC_SPEC> {
        SW_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the selection of PWM timer%s sync_out.\\\\0: Sync_in\\\\1: TEZ\\\\2: TEP\\\\3: Invalid, sync_out selects noting"]
    #[inline(always)]
    pub fn timer_synco_sel(&mut self) -> TIMER_SYNCO_SEL_W<'_, TIMER_SYNC_SPEC> {
        TIMER_SYNCO_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:19 - Configures the phase for timer%s reload on sync event."]
    #[inline(always)]
    pub fn timer_phase(&mut self) -> TIMER_PHASE_W<'_, TIMER_SYNC_SPEC> {
        TIMER_PHASE_W::new(self, 4)
    }
    #[doc = "Bit 20 - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\\\0: Increase\\\\1: Decrease"]
    #[inline(always)]
    pub fn timer_phase_direction(&mut self) -> TIMER_PHASE_DIRECTION_W<'_, TIMER_SYNC_SPEC> {
        TIMER_PHASE_DIRECTION_W::new(self, 20)
    }
}
#[doc = "PWM timer%s sync function configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_SYNC_SPEC;
impl crate::RegisterSpec for TIMER_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_sync::R`](R) reader structure"]
impl crate::Readable for TIMER_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_sync::W`](W) writer structure"]
impl crate::Writable for TIMER_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER%s_SYNC to value 0"]
impl crate::Resettable for TIMER_SYNC_SPEC {}
