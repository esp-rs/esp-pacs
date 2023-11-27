#[doc = "Register `TIMER%s_CFG0` reader"]
pub type R = crate::R<TIMER_CFG0_SPEC>;
#[doc = "Register `TIMER%s_CFG0` writer"]
pub type W = crate::W<TIMER_CFG0_SPEC>;
#[doc = "Field `TIMER_PRESCALE` reader - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
pub type TIMER_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER_PRESCALE` writer - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
pub type TIMER_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER_PERIOD` reader - Configures the period shadow of PWM timer%s"]
pub type TIMER_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_PERIOD` writer - Configures the period shadow of PWM timer%s"]
pub type TIMER_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER_PERIOD_UPMETHOD` reader - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type TIMER_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER_PERIOD_UPMETHOD` writer - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type TIMER_PERIOD_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer_prescale(&self) -> TIMER_PRESCALE_R {
        TIMER_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Configures the period shadow of PWM timer%s"]
    #[inline(always)]
    pub fn timer_period(&self) -> TIMER_PERIOD_R {
        TIMER_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer_period_upmethod(&self) -> TIMER_PERIOD_UPMETHOD_R {
        TIMER_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CFG0")
            .field(
                "timer_prescale",
                &format_args!("{}", self.timer_prescale().bits()),
            )
            .field(
                "timer_period",
                &format_args!("{}", self.timer_period().bits()),
            )
            .field(
                "timer_period_upmethod",
                &format_args!("{}", self.timer_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn timer_prescale(&mut self) -> TIMER_PRESCALE_W<TIMER_CFG0_SPEC> {
        TIMER_PRESCALE_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Configures the period shadow of PWM timer%s"]
    #[inline(always)]
    #[must_use]
    pub fn timer_period(&mut self) -> TIMER_PERIOD_W<TIMER_CFG0_SPEC> {
        TIMER_PERIOD_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    #[must_use]
    pub fn timer_period_upmethod(&mut self) -> TIMER_PERIOD_UPMETHOD_W<TIMER_CFG0_SPEC> {
        TIMER_PERIOD_UPMETHOD_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM timer%s period and update method configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CFG0_SPEC;
impl crate::RegisterSpec for TIMER_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cfg0::R`](R) reader structure"]
impl crate::Readable for TIMER_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_cfg0::W`](W) writer structure"]
impl crate::Writable for TIMER_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER%s_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
