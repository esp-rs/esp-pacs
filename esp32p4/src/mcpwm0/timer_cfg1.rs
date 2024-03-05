#[doc = "Register `TIMER%s_CFG1` reader"]
pub type R = crate::R<TIMER_CFG1_SPEC>;
#[doc = "Register `TIMER%s_CFG1` writer"]
pub type W = crate::W<TIMER_CFG1_SPEC>;
#[doc = "Field `TIMER_START` reader - Configures whether or not to start/stop PWM timer%s.\\\\0: If PWM timer%s starts, then stops at TEZ\\\\1: If timer%s starts, then stops at TEP\\\\2: PWM timer%s starts and runs on\\\\3: Timer%s starts and stops at the next TEZ\\\\4: Timer0 starts and stops at the next TEP.\\\\TEP here and below means the event that happens when the timer equals to period"]
pub type TIMER_START_R = crate::FieldReader;
#[doc = "Field `TIMER_START` writer - Configures whether or not to start/stop PWM timer%s.\\\\0: If PWM timer%s starts, then stops at TEZ\\\\1: If timer%s starts, then stops at TEP\\\\2: PWM timer%s starts and runs on\\\\3: Timer%s starts and stops at the next TEZ\\\\4: Timer0 starts and stops at the next TEP.\\\\TEP here and below means the event that happens when the timer equals to period"]
pub type TIMER_START_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMER_MOD` reader - Configures the working mode of PWM timer%s.\\\\0: Freeze\\\\1: Increase mode\\\\2: Decrease mode\\\\3: Up-down mode"]
pub type TIMER_MOD_R = crate::FieldReader;
#[doc = "Field `TIMER_MOD` writer - Configures the working mode of PWM timer%s.\\\\0: Freeze\\\\1: Increase mode\\\\2: Decrease mode\\\\3: Up-down mode"]
pub type TIMER_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Configures whether or not to start/stop PWM timer%s.\\\\0: If PWM timer%s starts, then stops at TEZ\\\\1: If timer%s starts, then stops at TEP\\\\2: PWM timer%s starts and runs on\\\\3: Timer%s starts and stops at the next TEZ\\\\4: Timer0 starts and stops at the next TEP.\\\\TEP here and below means the event that happens when the timer equals to period"]
    #[inline(always)]
    pub fn timer_start(&self) -> TIMER_START_R {
        TIMER_START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Configures the working mode of PWM timer%s.\\\\0: Freeze\\\\1: Increase mode\\\\2: Decrease mode\\\\3: Up-down mode"]
    #[inline(always)]
    pub fn timer_mod(&self) -> TIMER_MOD_R {
        TIMER_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CFG1")
            .field(
                "timer_start",
                &format_args!("{}", self.timer_start().bits()),
            )
            .field("timer_mod", &format_args!("{}", self.timer_mod().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether or not to start/stop PWM timer%s.\\\\0: If PWM timer%s starts, then stops at TEZ\\\\1: If timer%s starts, then stops at TEP\\\\2: PWM timer%s starts and runs on\\\\3: Timer%s starts and stops at the next TEZ\\\\4: Timer0 starts and stops at the next TEP.\\\\TEP here and below means the event that happens when the timer equals to period"]
    #[inline(always)]
    #[must_use]
    pub fn timer_start(&mut self) -> TIMER_START_W<TIMER_CFG1_SPEC> {
        TIMER_START_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Configures the working mode of PWM timer%s.\\\\0: Freeze\\\\1: Increase mode\\\\2: Decrease mode\\\\3: Up-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mod(&mut self) -> TIMER_MOD_W<TIMER_CFG1_SPEC> {
        TIMER_MOD_W::new(self, 3)
    }
}
#[doc = "PWM timer%s working mode and start/stop control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CFG1_SPEC;
impl crate::RegisterSpec for TIMER_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cfg1::R`](R) reader structure"]
impl crate::Readable for TIMER_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_cfg1::W`](W) writer structure"]
impl crate::Writable for TIMER_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER%s_CFG1 to value 0"]
impl crate::Resettable for TIMER_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
