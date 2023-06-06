#[doc = "Register `TIMER2_CFG0` reader"]
pub struct R(crate::R<TIMER2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_CFG0` writer"]
pub struct W(crate::W<TIMER2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMER2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER2_PRESCALE` reader - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub type TIMER2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER2_PRESCALE` writer - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub type TIMER2_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER2_CFG0_SPEC, 8, O>;
#[doc = "Field `TIMER2_PERIOD` reader - period shadow register of PWM timer2"]
pub type TIMER2_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER2_PERIOD` writer - period shadow register of PWM timer2"]
pub type TIMER2_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER2_CFG0_SPEC, 16, O, u16>;
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` reader - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER2_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` writer - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER2_PERIOD_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER2_CFG0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer2_prescale(&self) -> TIMER2_PRESCALE_R {
        TIMER2_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    pub fn timer2_period(&self) -> TIMER2_PERIOD_R {
        TIMER2_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer2_period_upmethod(&self) -> TIMER2_PERIOD_UPMETHOD_R {
        TIMER2_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_CFG0")
            .field(
                "timer2_prescale",
                &format_args!("{}", self.timer2_prescale().bits()),
            )
            .field(
                "timer2_period",
                &format_args!("{}", self.timer2_period().bits()),
            )
            .field(
                "timer2_period_upmethod",
                &format_args!("{}", self.timer2_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_prescale(&mut self) -> TIMER2_PRESCALE_W<0> {
        TIMER2_PRESCALE_W::new(self)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_period(&mut self) -> TIMER2_PERIOD_W<8> {
        TIMER2_PERIOD_W::new(self)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_period_upmethod(&mut self) -> TIMER2_PERIOD_UPMETHOD_W<24> {
        TIMER2_PERIOD_UPMETHOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM timer2 period and update method configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_cfg0](index.html) module"]
pub struct TIMER2_CFG0_SPEC;
impl crate::RegisterSpec for TIMER2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_cfg0::R](R) reader structure"]
impl crate::Readable for TIMER2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_cfg0::W](W) writer structure"]
impl crate::Writable for TIMER2_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER2_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
