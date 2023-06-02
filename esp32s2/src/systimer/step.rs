#[doc = "Register `STEP` reader"]
pub struct R(crate::R<STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP` writer"]
pub struct W(crate::W<STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_SPEC>;
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
impl From<crate::W<STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_XTAL_STEP` reader - Set system timer increment step when using XTAL_CLK."]
pub type TIMER_XTAL_STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_XTAL_STEP` writer - Set system timer increment step when using XTAL_CLK."]
pub type TIMER_XTAL_STEP_W<'a, const O: u8> = crate::FieldWriter<'a, STEP_SPEC, 10, O, u16, u16>;
#[doc = "Field `TIMER_PLL_STEP` reader - Set system timer increment step when using PLL_CLK"]
pub type TIMER_PLL_STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_PLL_STEP` writer - Set system timer increment step when using PLL_CLK"]
pub type TIMER_PLL_STEP_W<'a, const O: u8> = crate::FieldWriter<'a, STEP_SPEC, 10, O, u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    pub fn timer_xtal_step(&self) -> TIMER_XTAL_STEP_R {
        TIMER_XTAL_STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    pub fn timer_pll_step(&self) -> TIMER_PLL_STEP_R {
        TIMER_PLL_STEP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STEP")
            .field(
                "timer_xtal_step",
                &format_args!("{}", self.timer_xtal_step().bits()),
            )
            .field(
                "timer_pll_step",
                &format_args!("{}", self.timer_pll_step().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STEP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn timer_xtal_step(&mut self) -> TIMER_XTAL_STEP_W<0> {
        TIMER_XTAL_STEP_W::new(self)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn timer_pll_step(&mut self) -> TIMER_PLL_STEP_W<10> {
        TIMER_PLL_STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System timer accumulation step\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step](index.html) module"]
pub struct STEP_SPEC;
impl crate::RegisterSpec for STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [step::R](R) reader structure"]
impl crate::Readable for STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step::W](W) writer structure"]
impl crate::Writable for STEP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STEP to value 0x0450"]
impl crate::Resettable for STEP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0450;
}
