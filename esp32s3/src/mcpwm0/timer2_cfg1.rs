#[doc = "Register `TIMER2_CFG1` reader"]
pub type R = crate::R<TIMER2_CFG1_SPEC>;
#[doc = "Register `TIMER2_CFG1` writer"]
pub type W = crate::W<TIMER2_CFG1_SPEC>;
#[doc = "Field `TIMER2_START` reader - PWM timer2 start and stop control. 0: if PWM timer2 starts, then stops at TEZ, 1: if timer2 starts, then stops at TEP, 2: PWM timer2 starts and runs on, 3: timer2 starts and stops at the next TEZ, 4: timer2 starts and stops at the next TEP. TEP here and below means the event that happens when the timer equals to period"]
pub type TIMER2_START_R = crate::FieldReader;
#[doc = "Field `TIMER2_START` writer - PWM timer2 start and stop control. 0: if PWM timer2 starts, then stops at TEZ, 1: if timer2 starts, then stops at TEP, 2: PWM timer2 starts and runs on, 3: timer2 starts and stops at the next TEZ, 4: timer2 starts and stops at the next TEP. TEP here and below means the event that happens when the timer equals to period"]
pub type TIMER2_START_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMER2_MOD` reader - PWM timer2 working mode, 0: freeze, 1: increase mode, 2: decrease mode, 3: up-down mode"]
pub type TIMER2_MOD_R = crate::FieldReader;
#[doc = "Field `TIMER2_MOD` writer - PWM timer2 working mode, 0: freeze, 1: increase mode, 2: decrease mode, 3: up-down mode"]
pub type TIMER2_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - PWM timer2 start and stop control. 0: if PWM timer2 starts, then stops at TEZ, 1: if timer2 starts, then stops at TEP, 2: PWM timer2 starts and runs on, 3: timer2 starts and stops at the next TEZ, 4: timer2 starts and stops at the next TEP. TEP here and below means the event that happens when the timer equals to period"]
    #[inline(always)]
    pub fn timer2_start(&self) -> TIMER2_START_R {
        TIMER2_START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - PWM timer2 working mode, 0: freeze, 1: increase mode, 2: decrease mode, 3: up-down mode"]
    #[inline(always)]
    pub fn timer2_mod(&self) -> TIMER2_MOD_R {
        TIMER2_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_CFG1")
            .field(
                "timer2_start",
                &format_args!("{}", self.timer2_start().bits()),
            )
            .field("timer2_mod", &format_args!("{}", self.timer2_mod().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM timer2 start and stop control. 0: if PWM timer2 starts, then stops at TEZ, 1: if timer2 starts, then stops at TEP, 2: PWM timer2 starts and runs on, 3: timer2 starts and stops at the next TEZ, 4: timer2 starts and stops at the next TEP. TEP here and below means the event that happens when the timer equals to period"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_start(&mut self) -> TIMER2_START_W<TIMER2_CFG1_SPEC> {
        TIMER2_START_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - PWM timer2 working mode, 0: freeze, 1: increase mode, 2: decrease mode, 3: up-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_mod(&mut self) -> TIMER2_MOD_W<TIMER2_CFG1_SPEC> {
        TIMER2_MOD_W::new(self, 3)
    }
}
#[doc = "PWM timer2 working mode and start/stop control configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_CFG1_SPEC;
impl crate::RegisterSpec for TIMER2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_cfg1::R`](R) reader structure"]
impl crate::Readable for TIMER2_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer2_cfg1::W`](W) writer structure"]
impl crate::Writable for TIMER2_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2_CFG1 to value 0"]
impl crate::Resettable for TIMER2_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
