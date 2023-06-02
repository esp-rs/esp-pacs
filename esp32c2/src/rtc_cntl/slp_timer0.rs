#[doc = "Register `SLP_TIMER0` reader"]
pub struct R(crate::R<SLP_TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER0` writer"]
pub struct W(crate::W<SLP_TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER0_SPEC>;
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
impl From<crate::W<SLP_TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_LO` reader - Need add desc"]
pub type SLP_VAL_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLP_VAL_LO` writer - Need add desc"]
pub type SLP_VAL_LO_W<'a, const O: u8> = crate::FieldWriter<'a, SLP_TIMER0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER0")
            .field("slp_val_lo", &format_args!("{}", self.slp_val_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_TIMER0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W<0> {
        SLP_VAL_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer0](index.html) module"]
pub struct SLP_TIMER0_SPEC;
impl crate::RegisterSpec for SLP_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer0::R](R) reader structure"]
impl crate::Readable for SLP_TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer0::W](W) writer structure"]
impl crate::Writable for SLP_TIMER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_TIMER0 to value 0"]
impl crate::Resettable for SLP_TIMER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
