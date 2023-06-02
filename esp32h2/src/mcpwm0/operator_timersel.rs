#[doc = "Register `OPERATOR_TIMERSEL` reader"]
pub struct R(crate::R<OPERATOR_TIMERSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPERATOR_TIMERSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPERATOR_TIMERSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPERATOR_TIMERSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPERATOR_TIMERSEL` writer"]
pub struct W(crate::W<OPERATOR_TIMERSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPERATOR_TIMERSEL_SPEC>;
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
impl From<crate::W<OPERATOR_TIMERSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPERATOR_TIMERSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPERATOR0_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR0_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR0_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR0_TIMERSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, OPERATOR_TIMERSEL_SPEC, 2, O>;
#[doc = "Field `OPERATOR1_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR1_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR1_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR1_TIMERSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, OPERATOR_TIMERSEL_SPEC, 2, O>;
#[doc = "Field `OPERATOR2_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR2_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR2_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2"]
pub type OPERATOR2_TIMERSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, OPERATOR_TIMERSEL_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR0_TIMERSEL_R {
        OPERATOR0_TIMERSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR1_TIMERSEL_R {
        OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    pub fn operator2_timersel(&self) -> OPERATOR2_TIMERSEL_R {
        OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPERATOR_TIMERSEL")
            .field(
                "operator0_timersel",
                &format_args!("{}", self.operator0_timersel().bits()),
            )
            .field(
                "operator1_timersel",
                &format_args!("{}", self.operator1_timersel().bits()),
            )
            .field(
                "operator2_timersel",
                &format_args!("{}", self.operator2_timersel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OPERATOR_TIMERSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator0_timersel(&mut self) -> OPERATOR0_TIMERSEL_W<0> {
        OPERATOR0_TIMERSEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator1_timersel(&mut self) -> OPERATOR1_TIMERSEL_W<2> {
        OPERATOR1_TIMERSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator2_timersel(&mut self) -> OPERATOR2_TIMERSEL_W<4> {
        OPERATOR2_TIMERSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select specific timer for PWM operators.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [operator_timersel](index.html) module"]
pub struct OPERATOR_TIMERSEL_SPEC;
impl crate::RegisterSpec for OPERATOR_TIMERSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [operator_timersel::R](R) reader structure"]
impl crate::Readable for OPERATOR_TIMERSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [operator_timersel::W](W) writer structure"]
impl crate::Writable for OPERATOR_TIMERSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPERATOR_TIMERSEL to value 0"]
impl crate::Resettable for OPERATOR_TIMERSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
