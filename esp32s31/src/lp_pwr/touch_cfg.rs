#[doc = "Register `TOUCH_CFG` reader"]
pub type R = crate::R<TOUCH_CFG_SPEC>;
#[doc = "Register `TOUCH_CFG` writer"]
pub type W = crate::W<TOUCH_CFG_SPEC>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles config for touch"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles config for touch"]
pub type TOUCH_SLEEP_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` reader - 1:enables touch timer wakeup source 0:disables touch timer wakeup source"]
pub type TOUCH_SLEEP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` writer - 1:enables touch timer wakeup source 0:disables touch timer wakeup source"]
pub type TOUCH_SLEEP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_FORCE_DONE` reader - 1: force touch task to done state, turn off touch 0: no effect"]
pub type TOUCH_FORCE_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_FORCE_DONE` writer - 1: force touch task to done state, turn off touch 0: no effect"]
pub type TOUCH_FORCE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - sleep cycles config for touch"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - 1:enables touch timer wakeup source 0:disables touch timer wakeup source"]
    #[inline(always)]
    pub fn touch_sleep_timer_en(&self) -> TOUCH_SLEEP_TIMER_EN_R {
        TOUCH_SLEEP_TIMER_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: force touch task to done state, turn off touch 0: no effect"]
    #[inline(always)]
    pub fn touch_force_done(&self) -> TOUCH_FORCE_DONE_R {
        TOUCH_FORCE_DONE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CFG")
            .field("touch_sleep_cycles", &self.touch_sleep_cycles())
            .field("touch_sleep_timer_en", &self.touch_sleep_timer_en())
            .field("touch_force_done", &self.touch_force_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25 - sleep cycles config for touch"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<'_, TOUCH_CFG_SPEC> {
        TOUCH_SLEEP_CYCLES_W::new(self, 0)
    }
    #[doc = "Bit 26 - 1:enables touch timer wakeup source 0:disables touch timer wakeup source"]
    #[inline(always)]
    pub fn touch_sleep_timer_en(&mut self) -> TOUCH_SLEEP_TIMER_EN_W<'_, TOUCH_CFG_SPEC> {
        TOUCH_SLEEP_TIMER_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: force touch task to done state, turn off touch 0: no effect"]
    #[inline(always)]
    pub fn touch_force_done(&mut self) -> TOUCH_FORCE_DONE_W<'_, TOUCH_CFG_SPEC> {
        TOUCH_FORCE_DONE_W::new(self, 27)
    }
}
#[doc = "ctrl register for touch power control\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CFG_SPEC;
impl crate::RegisterSpec for TOUCH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_cfg::R`](R) reader structure"]
impl crate::Readable for TOUCH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_cfg::W`](W) writer structure"]
impl crate::Writable for TOUCH_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_CFG to value 0x64"]
impl crate::Resettable for TOUCH_CFG_SPEC {
    const RESET_VALUE: u32 = 0x64;
}
