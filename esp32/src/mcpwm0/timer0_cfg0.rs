#[doc = "Register `TIMER0_CFG0` reader"]
pub struct R(crate::R<TIMER0_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_CFG0` writer"]
pub struct W(crate::W<TIMER0_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CFG0_SPEC>;
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
impl From<crate::W<TIMER0_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_PRESCALE` reader - "]
pub type TIMER0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER0_PRESCALE` writer - "]
pub type TIMER0_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER0_CFG0_SPEC, 8, O>;
#[doc = "Field `TIMER0_PERIOD` reader - "]
pub type TIMER0_PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER0_PERIOD` writer - "]
pub type TIMER0_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, TIMER0_CFG0_SPEC, 16, O, u16, u16>;
#[doc = "Field `TIMER0_PERIOD_UPMETHOD` reader - "]
pub type TIMER0_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER0_PERIOD_UPMETHOD` writer - "]
pub type TIMER0_PERIOD_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER0_CFG0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer0_prescale(&self) -> TIMER0_PRESCALE_R {
        TIMER0_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn timer0_period(&self) -> TIMER0_PERIOD_R {
        TIMER0_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn timer0_period_upmethod(&self) -> TIMER0_PERIOD_UPMETHOD_R {
        TIMER0_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0_CFG0")
            .field(
                "timer0_prescale",
                &format_args!("{}", self.timer0_prescale().bits()),
            )
            .field(
                "timer0_period",
                &format_args!("{}", self.timer0_period().bits()),
            )
            .field(
                "timer0_period_upmethod",
                &format_args!("{}", self.timer0_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER0_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_prescale(&mut self) -> TIMER0_PRESCALE_W<0> {
        TIMER0_PRESCALE_W::new(self)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_period(&mut self) -> TIMER0_PERIOD_W<8> {
        TIMER0_PERIOD_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_period_upmethod(&mut self) -> TIMER0_PERIOD_UPMETHOD_W<24> {
        TIMER0_PERIOD_UPMETHOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_cfg0](index.html) module"]
pub struct TIMER0_CFG0_SPEC;
impl crate::RegisterSpec for TIMER0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_cfg0::R](R) reader structure"]
impl crate::Readable for TIMER0_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_cfg0::W](W) writer structure"]
impl crate::Writable for TIMER0_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER0_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
