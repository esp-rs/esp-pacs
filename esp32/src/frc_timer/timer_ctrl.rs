#[doc = "Register `TIMER_CTRL` reader"]
pub struct R(crate::R<TIMER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_CTRL` writer"]
pub struct W(crate::W<TIMER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CTRL_SPEC>;
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
impl From<crate::W<TIMER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_PRESCALER` reader - "]
pub type TIMER_PRESCALER_R = crate::FieldReader;
#[doc = "Field `TIMER_PRESCALER` writer - "]
pub type TIMER_PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_CTRL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn timer_prescaler(&self) -> TIMER_PRESCALER_R {
        TIMER_PRESCALER_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CTRL")
            .field(
                "timer_prescaler",
                &format_args!("{}", self.timer_prescaler().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    #[must_use]
    pub fn timer_prescaler(&mut self) -> TIMER_PRESCALER_W<1> {
        TIMER_PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctrl](index.html) module"]
pub struct TIMER_CTRL_SPEC;
impl crate::RegisterSpec for TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_ctrl::R](R) reader structure"]
impl crate::Readable for TIMER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_ctrl::W](W) writer structure"]
impl crate::Writable for TIMER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_CTRL to value 0"]
impl crate::Resettable for TIMER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
