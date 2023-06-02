#[doc = "Register `TIMER_INT` reader"]
pub struct R(crate::R<TIMER_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_INT` writer"]
pub struct W(crate::W<TIMER_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_INT_SPEC>;
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
impl From<crate::W<TIMER_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - "]
pub type CLR_R = crate::BitReader;
#[doc = "Field `CLR` writer - "]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_INT_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT")
            .field("clr", &format_args!("{}", self.clr().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_int](index.html) module"]
pub struct TIMER_INT_SPEC;
impl crate::RegisterSpec for TIMER_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_int::R](R) reader structure"]
impl crate::Readable for TIMER_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_int::W](W) writer structure"]
impl crate::Writable for TIMER_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_INT to value 0"]
impl crate::Resettable for TIMER_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
