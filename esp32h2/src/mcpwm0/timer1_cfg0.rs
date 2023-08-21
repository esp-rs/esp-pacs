#[doc = "Register `TIMER1_CFG0` reader"]
pub type R = crate::R<TIMER1_CFG0_SPEC>;
#[doc = "Register `TIMER1_CFG0` writer"]
pub type W = crate::W<TIMER1_CFG0_SPEC>;
#[doc = "Field `TIMER1_PRESCALE` reader - period of PT0_clk = Period of PWM_clk * (PWM_timer1_PRESCALE + 1)"]
pub type TIMER1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER1_PRESCALE` writer - period of PT0_clk = Period of PWM_clk * (PWM_timer1_PRESCALE + 1)"]
pub type TIMER1_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER1_PERIOD` reader - period shadow register of PWM timer1"]
pub type TIMER1_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER1_PERIOD` writer - period shadow register of PWM timer1"]
pub type TIMER1_PERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TIMER1_PERIOD_UPMETHOD` reader - Update method for active register of PWM timer1 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER1_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER1_PERIOD_UPMETHOD` writer - Update method for active register of PWM timer1 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER1_PERIOD_UPMETHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer1_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer1_prescale(&self) -> TIMER1_PRESCALE_R {
        TIMER1_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer1"]
    #[inline(always)]
    pub fn timer1_period(&self) -> TIMER1_PERIOD_R {
        TIMER1_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer1 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer1_period_upmethod(&self) -> TIMER1_PERIOD_UPMETHOD_R {
        TIMER1_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1_CFG0")
            .field(
                "timer1_prescale",
                &format_args!("{}", self.timer1_prescale().bits()),
            )
            .field(
                "timer1_period",
                &format_args!("{}", self.timer1_period().bits()),
            )
            .field(
                "timer1_period_upmethod",
                &format_args!("{}", self.timer1_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER1_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer1_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_prescale(&mut self) -> TIMER1_PRESCALE_W<TIMER1_CFG0_SPEC, 0> {
        TIMER1_PRESCALE_W::new(self)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_period(&mut self) -> TIMER1_PERIOD_W<TIMER1_CFG0_SPEC, 8> {
        TIMER1_PERIOD_W::new(self)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer1 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_period_upmethod(&mut self) -> TIMER1_PERIOD_UPMETHOD_W<TIMER1_CFG0_SPEC, 24> {
        TIMER1_PERIOD_UPMETHOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM timer1 period and update method configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1_CFG0_SPEC;
impl crate::RegisterSpec for TIMER1_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1_cfg0::R`](R) reader structure"]
impl crate::Readable for TIMER1_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer1_cfg0::W`](W) writer structure"]
impl crate::Writable for TIMER1_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER1_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
